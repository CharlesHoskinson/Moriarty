// -*- mode: Bluespec; -*-
// E3 J8 Soundness Model (Round 2): OnionAir (lane-src/iface_l.rs)
// Reduced-scale formal specification of the BabyBear L_iface log onion AIR.
//
// Correspondence Map:
// - onion2.qnt :: tables_valid         <-> lane-src/iface_l.rs:240-348 (IfaceAir::eval for LogEntry, LogValue, LOG_BUS)
// - onion2.qnt :: row_local_ok         <-> lane-src/iface_l.rs:547-644 (OnionAir::eval local constraints)
// - onion2.qnt :: row_trans_ok         <-> lane-src/iface_l.rs:562-666 (OnionAir::eval transition constraints)
// - onion2.qnt :: system_ok_cfg        <-> lane-src/iface_l.rs:560-578, 601-677 (boundary & bus interactions)
// - onion2.qnt :: p2_native / onion    <-> lane-src/iface.rs:242-284 (p2_with_inputs, enc, onion)
// - onion2.qnt :: build_honest_system  <-> lane-src/iface_l.rs:412-527 (OnionAir::trace)
//
// Parameters (scaled down from RATE = 8, LIMBS = 9 while preserving exact block structure):
// - Block / rate / digest / capacity width: W_BLK = 4 (permutation width 8 = 4 rate + 4 cap)
// - Limbs per logged value: LIMBS = 2
// - Header of enc(e) on FA row: [TAG_ENT, op, param] (3 words, j < 3 skipped on VAL_BUS)
// - Entry total words: total = 3 + 2 * k, where k in {1, 2, 3}:
//     k = 1 (PUB):    total = 5 -> 2 A-blocks (FA with c=4, LAST with pos=4, c=1)
//     k = 2 (DIGEST): total = 7 -> 2 A-blocks (FA with c=4, LAST with pos=4, c=3)
//     k = 3 (H2):     total = 9 -> 3 A-blocks (FA with c=4, mid A with pos=4, c=4, LAST with pos=8, c=1)
// - Step input: [TAG_STEP] ++ d[0..4] ++ h[0..4] (9 words, cap0 = 9) -> 3 B-blocks:
//     B0: [TAG_STEP, d0, d1, d2], cap = [9, 0, 0, 0]
//     B1: [d3, h0, h1, h2],       cap = B0.out[4..8]
//     B2: [h3, 0, 0, 0],          cap = B1.out[4..8]

module onion2 {

  // ---- 1. Types ----------------------------------------------------------------------------

  type LogEntryRow = {
    op: int,
    param: int,
    off: int,
    k_vals: int,
    used: int
  }

  type LogValueRow = {
    limb0: int,
    limb1: int,
    used: int,
    cnt: int
  }

  type OnionRow = {
    w: List[int],
    cap: List[int],
    out: List[int],
    d: List[int],
    h: List[int],
    u: List[int],
    sel_e: int,
    sel_a: int,
    sel_fa: int,
    sel_b0: int,
    sel_b1: int,
    sel_b2: int,
    sel_last: int,
    sel_fin: int,
    pos: int,
    cnt: int,
    op: int,
    param: int,
    off: int,
    k_vals: int
  }

  type TieMsg = {
    d: List[int],
    n_used: int
  }

  type SystemInstance = {
    entries: List[LogEntryRow],
    values: List[LogValueRow],
    stmt_tie: TieMsg,
    trace: List[OnionRow]
  }

  type ConstraintFlags = {
    enforce_first_off_zero: bool,
    enforce_b2_off_step: bool,
    enforce_fa_w0: bool,
    enforce_last_count: bool,
    enforce_fin_no_b1: bool,
    enforce_cap_chain: bool,
    enforce_cnt_b2: bool,
    enforce_d_chain: bool,
    enforce_fa_pos_zero: bool,
    enforce_same_off: bool,
    enforce_same_k: bool,
    enforce_fin_transition: bool
  }

  // Cohesive state record for the state machine (quint-modeling Step 2)
  type ModelState = {
    sys: SystemInstance,
    is_honest: bool,
    step_kind: int,
    adv_accepted: int,
    adv_total: int
  }

  // ---- 2. Constants ------------------------------------------------------------------------

  pure val TAG_EMPTY: int = 1
  pure val TAG_STEP: int = 2
  pure val TAG_NODE: int = 3
  pure val TAG_ENT: int = 4

  pure val OP_NOP: int = 0
  pure val OP_H2: int = 2
  pure val OP_PUB: int = 7
  pure val OP_DIGEST: int = 8

  pure val W_BLK: int = 4
  pure val LIMBS: int = 2
  pure val STEP_LEN: int = 9
  pure val PM: int = 997

  pure val ZERO4: List[int] = [0, 0, 0, 0]
  pure val ZERO8: List[int] = [0, 0, 0, 0, 0, 0, 0, 0]

  pure val REAL_FLAGS: ConstraintFlags = {
    enforce_first_off_zero: true,
    enforce_b2_off_step: true,
    enforce_fa_w0: true,
    enforce_last_count: true,
    enforce_fin_no_b1: true,
    enforce_cap_chain: true,
    enforce_cnt_b2: true,
    enforce_d_chain: true,
    enforce_fa_pos_zero: true,
    enforce_same_off: true,
    enforce_same_k: true,
    enforce_fin_transition: true
  }

  // ---- 3. Pure Helper & Ideal Permutation Functions ----------------------------------------

  pure def is_bit(x: int): bool = {
    x == 0 or x == 1
  }

  pure def list_eq(xs: List[int], ys: List[int]): bool = {
    xs.length() == ys.length() and
    xs.indices().forall(i => xs.nth(i) == ys.nth(i))
  }

  pure def word_or_zero(xs: List[int], idx: int): int = {
    if (idx >= 0 and idx < xs.length()) xs.nth(idx) else 0
  }

  pure def mod_pm(x: int): int = {
    ((x % PM) + PM) % PM
  }

  // Bijection on Z_PM^8 (PM = 997 prime) via Horner evaluation at Vandermonde nodes 2..9.
  // Keeps every intermediate integer < 10000 and avoids 0.to(-1) / range folds.
  pure def horner8(st_in: List[int], node_a: int, c_add: int): int = {
    val x0 = mod_pm(st_in.nth(0))
    val x1 = mod_pm(st_in.nth(1))
    val x2 = mod_pm(st_in.nth(2))
    val x3 = mod_pm(st_in.nth(3))
    val x4 = mod_pm(st_in.nth(4))
    val x5 = mod_pm(st_in.nth(5))
    val x6 = mod_pm(st_in.nth(6))
    val x7 = mod_pm(st_in.nth(7))
    val s6 = (x7 * node_a + x6) % PM
    val s5 = (s6 * node_a + x5) % PM
    val s4 = (s5 * node_a + x4) % PM
    val s3 = (s4 * node_a + x3) % PM
    val s2 = (s3 * node_a + x2) % PM
    val s1 = (s2 * node_a + x1) % PM
    (s1 * node_a + x0 + c_add) % PM
  }

  pure def perm8(st_in: List[int]): List[int] = {
    [
      horner8(st_in, 2, 1),
      horner8(st_in, 3, 2),
      horner8(st_in, 4, 3),
      horner8(st_in, 5, 4),
      horner8(st_in, 6, 5),
      horner8(st_in, 7, 6),
      horner8(st_in, 8, 7),
      horner8(st_in, 9, 8)
    ]
  }

  pure def rate_part(st: List[int]): List[int] = {
    [st.nth(0), st.nth(1), st.nth(2), st.nth(3)]
  }

  pure def cap_part(st: List[int]): List[int] = {
    [st.nth(4), st.nth(5), st.nth(6), st.nth(7)]
  }

  pure def op_vals(op: int): int = {
    if (op == OP_PUB) 1
    else if (op == OP_DIGEST) 2
    else if (op == OP_H2) 3
    else 0
  }

  // ---- 4. Native Reference (lane-src/iface.rs) ---------------------------------------------

  pure def p2_block_indices(total_len: int): List[int] = {
    if (total_len <= 4) [0]
    else if (total_len <= 8) [0, 1]
    else [0, 1, 2]
  }

  pure def p2_words(words: List[int]): List[int] = {
    rate_part(
      p2_block_indices(words.length()).foldl(
        [0, 0, 0, 0, words.length(), 0, 0, 0],
        (st, bi) => perm8([
          word_or_zero(words, 4 * bi + 0),
          word_or_zero(words, 4 * bi + 1),
          word_or_zero(words, 4 * bi + 2),
          word_or_zero(words, 4 * bi + 3),
          st.nth(4),
          st.nth(5),
          st.nth(6),
          st.nth(7)
        ])
      )
    )
  }

  pure def p2_native(tag: int, xs: List[int]): List[int] = {
    p2_words([tag].concat(xs))
  }

  pure def flat_limbs(values: List[LogValueRow], off: int, k_vals: int): List[int] = {
    if (k_vals == 1 and off >= 0 and off < values.length()) [
      values.nth(off).limb0,
      values.nth(off).limb1
    ] else if (k_vals == 2 and off >= 0 and off + 1 < values.length()) [
      values.nth(off).limb0,
      values.nth(off).limb1,
      values.nth(off + 1).limb0,
      values.nth(off + 1).limb1
    ] else if (k_vals == 3 and off >= 0 and off + 2 < values.length()) [
      values.nth(off).limb0,
      values.nth(off).limb1,
      values.nth(off + 1).limb0,
      values.nth(off + 1).limb1,
      values.nth(off + 2).limb0,
      values.nth(off + 2).limb1
    ] else List()
  }

  pure def entry_enc(e: LogEntryRow, values: List[LogValueRow]): List[int] = {
    [e.op, e.param].concat(flat_limbs(values, e.off, e.k_vals))
  }

  pure def d_empty_native: List[int] = {
    p2_native(TAG_EMPTY, List())
  }

  pure def native_onion(entries: List[LogEntryRow], values: List[LogValueRow]): TieMsg = {
    entries.foldl(
      { d: d_empty_native, n_used: 0 },
      (acc, e) =>
        if (e.used == 1) {
          d: p2_native(
            TAG_STEP,
            acc.d.concat(p2_native(TAG_ENT, entry_enc(e, values)))
          ),
          n_used: acc.n_used + 1
        } else acc
    )
  }

  // ---- 5. Bus Balance & Table Constraints (IfaceAir + OnionAir) ----------------------------

  // LogEntry + LogValue + LOG_BUS constraints (lane-src/iface_l.rs:240-348)
  pure def tables_valid(entries: List[LogEntryRow], values: List[LogValueRow]): bool = {
    entries.length() > 0 and
    values.length() > 0 and
    // Entry 0 starts at off = 0 (iface_l.rs:292)
    entries.nth(0).off == 0 and
    // Each entry has valid op, parameter bounds, used flag, and k_vals = op_vals(op) (iface_l.rs:249-285)
    entries.indices().forall(i => {
      val e = entries.nth(i)
      is_bit(e.used) and
      ((e.op == OP_NOP and e.used == 0 and e.k_vals == 0 and e.param == 0) or
       ((e.op == OP_PUB or e.op == OP_DIGEST or e.op == OP_H2) and
        e.used == 1 and
        e.param >= 0 and
        (if (e.op == OP_H2) e.param == 0 else true) and
        e.k_vals == op_vals(e.op)))
    }) and
    // No used entry after NOP (iface_l.rs:260-263), and running sum off_{i+1} = off_i + k_vals_i (iface_l.rs:295-296)
    entries.indices().forall(i =>
      if (i + 1 < entries.length()) (
        ((1 - entries.nth(i).used) * entries.nth(i + 1).used == 0) and
        (entries.nth(i + 1).off == entries.nth(i).off + entries.nth(i).k_vals)
      ) else true
    ) and
    // Last row capacity slack: V - off_end >= 0 (iface_l.rs:300-303)
    (entries.nth(entries.length() - 1).off + entries.nth(entries.length() - 1).k_vals <= values.length()) and
    // Value table: boolean used, non-increasing, unused limbs zero, running cnt (iface_l.rs:335-345)
    values.nth(0).cnt == values.nth(0).used and
    values.indices().forall(r => {
      val v = values.nth(r)
      is_bit(v.used) and
      ((1 - v.used) * v.limb0 == 0) and
      ((1 - v.used) * v.limb1 == 0) and
      (if (r + 1 < values.length()) (
        ((1 - v.used) * values.nth(r + 1).used == 0) and
        (values.nth(r + 1).cnt == v.cnt + values.nth(r + 1).used)
      ) else true)
    }) and
    // LOG_BUS: off_end on last entry row equals cnt on last value row (iface_l.rs:317, 347)
    (entries.nth(entries.length() - 1).off + entries.nth(entries.length() - 1).k_vals ==
     values.nth(values.length() - 1).cnt)
  }

  // Exact signed multiset balance on ENT_BUS (iface_l.rs:309-314, 668-673):
  // LogEntry sends (op, param, off, vals) with +used; OnionAir receives with -sel_fa.
  pure def ent_sent_mult(entries: List[LogEntryRow], op: int, param: int, off: int, kv: int): int = {
    entries.foldl(0, (acc, e) =>
      acc + (if (e.op == op and e.param == param and e.off == off and e.k_vals == kv) e.used else 0)
    )
  }

  pure def ent_recv_mult(tr: List[OnionRow], op: int, param: int, off: int, kv: int): int = {
    tr.foldl(0, (acc, r) =>
      acc + (if (r.op == op and r.param == param and r.off == off and r.k_vals == kv) r.sel_fa else 0)
    )
  }

  pure def ent_bus_balanced(entries: List[LogEntryRow], tr: List[OnionRow]): bool = {
    entries.indices().forall(i => {
      val e = entries.nth(i)
      ent_sent_mult(entries, e.op, e.param, e.off, e.k_vals) ==
      ent_recv_mult(tr, e.op, e.param, e.off, e.k_vals)
    }) and
    tr.indices().forall(ri => {
      val r = tr.nth(ri)
      ent_sent_mult(entries, r.op, r.param, r.off, r.k_vals) ==
      ent_recv_mult(tr, r.op, r.param, r.off, r.k_vals)
    })
  }

  // Multiplicity m on VAL_BUS for row x and slot j in 0..3 (iface_l.rs:628)
  pure def val_mult(x: OnionRow, j: int, fa_skips_header: bool): int = {
    x.sel_a * x.u.nth(j) - (if (fa_skips_header and j < 3) x.sel_fa else 0)
  }

  // Lookup key g on VAL_BUS for row x and slot j in 0..3 (iface_l.rs:629)
  pure def val_key(x: OnionRow, j: int): int = {
    x.off * LIMBS + x.pos + j - 3
  }

  // Exact signed multiset balance on VAL_BUS (iface_l.rs:330-333, 628-630):
  // LogValue sends ((cnt - 1) * LIMBS + t, limb_t) with multiplicity +u;
  // OnionAir receives (off * LIMBS + pos + j - 3, w[j]) with multiplicity -m(x, j).
  // No extra side condition (such as m >= 0) is assumed; m in {0, 1} follows from the AIR constraints.
  pure def val_sent_mult(values: List[LogValueRow], key_g: int, limb_v: int): int = {
    values.foldl(0, (acc, v) =>
      acc +
      (if ((v.cnt - 1) * LIMBS + 0 == key_g and v.limb0 == limb_v) v.used else 0) +
      (if ((v.cnt - 1) * LIMBS + 1 == key_g and v.limb1 == limb_v) v.used else 0)
    )
  }

  pure def val_recv_mult(tr: List[OnionRow], key_g: int, limb_v: int, fa_skips_header: bool): int = {
    tr.foldl(0, (acc, row) =>
      acc + [0, 1, 2, 3].foldl(0, (inner, j) =>
        inner + (if (val_key(row, j) == key_g and row.w.nth(j) == limb_v)
                   val_mult(row, j, fa_skips_header)
                 else 0)
      )
    )
  }

  pure def val_bus_balanced_cfg(values: List[LogValueRow], tr: List[OnionRow], fa_skips_header: bool): bool = {
    values.indices().forall(v_idx => {
      val v = values.nth(v_idx)
      val g0 = (v.cnt - 1) * LIMBS + 0
      val g1 = (v.cnt - 1) * LIMBS + 1
      (val_sent_mult(values, g0, v.limb0) == val_recv_mult(tr, g0, v.limb0, fa_skips_header)) and
      (val_sent_mult(values, g1, v.limb1) == val_recv_mult(tr, g1, v.limb1, fa_skips_header))
    }) and
    tr.indices().forall(ri => {
      val row = tr.nth(ri)
      Set(0, 1, 2, 3).forall(j => {
        val g = val_key(row, j)
        val wj = row.w.nth(j)
        val_sent_mult(values, g, wj) == val_recv_mult(tr, g, wj, fa_skips_header)
      })
    })
  }

  // Exact signed multiset balance on TIE_BUS (iface_l.rs:319-326, 675-677):
  // Stmt receives stmt_tie with multiplicity -1; OnionAir sends (out[0..4], cnt + b2) with +sel_fin.
  pure def tie_row_count(row: OnionRow, cnt_has_b2: bool): int = {
    row.cnt + (if (cnt_has_b2) row.sel_b2 else 0)
  }

  pure def tie_sent_mult(tr: List[OnionRow], d_msg: List[int], n_msg: int, cnt_has_b2: bool): int = {
    tr.foldl(0, (acc, r) =>
      acc + (if (list_eq(rate_part(r.out), d_msg) and tie_row_count(r, cnt_has_b2) == n_msg)
               r.sel_fin
             else 0)
    )
  }

  pure def tie_bus_balanced_cfg(stmt_tie: TieMsg, tr: List[OnionRow], cnt_has_b2: bool): bool = {
    tie_sent_mult(tr, stmt_tie.d, stmt_tie.n_used, cnt_has_b2) == 1 and
    tr.indices().forall(ri => {
      val r = tr.nth(ri)
      val d_r = rate_part(r.out)
      val n_r = tie_row_count(r, cnt_has_b2)
      val expected = if (list_eq(d_r, stmt_tie.d) and n_r == stmt_tie.n_used) 1 else 0
      tie_sent_mult(tr, d_r, n_r, cnt_has_b2) == expected
    })
  }

  // Poseidon2PermuteBus lookup (iface_l.rs:601-608):
  // Since ax in {0, 1}, lookup with multiplicity ax holds iff on every active row out == perm8(w ++ cap).
  pure def perm_bus_ok(tr: List[OnionRow]): bool = {
    tr.indices().forall(r =>
      if (row_act(tr.nth(r)) == 1)
        list_eq(tr.nth(r).out, perm8(tr.nth(r).w.concat(tr.nth(r).cap)))
      else true
    )
  }

  // ---- 6. OnionAir::eval Constraints (lane-src/iface_l.rs:538-678) -------------------------

  pure def row_act(x: OnionRow): int = {
    x.sel_e + x.sel_a + x.sel_b0 + x.sel_b1 + x.sel_b2
  }

  pure def row_u_sum(x: OnionRow): int = {
    x.u.nth(0) + x.u.nth(1) + x.u.nth(2) + x.u.nth(3)
  }

  pure def row_total(x: OnionRow): int = {
    3 + x.k_vals * LIMBS
  }

  pure def row_first_blk(x: OnionRow): int = {
    x.sel_e + x.sel_fa + x.sel_b0
  }

  pure def row_cont(y: OnionRow): int = {
    y.sel_a - y.sel_fa + y.sel_b1 + y.sel_b2
  }

  pure def row_same_entry(y: OnionRow): int = {
    row_act(y) * (1 - y.sel_fa)
  }

  // Local (per-row) constraints of OnionAir::eval
  pure def row_local_ok(x: OnionRow, cfg: ConstraintFlags): bool = {
    x.w.length() == 4 and
    x.cap.length() == 4 and
    x.out.length() == 8 and
    x.d.length() == 4 and
    x.h.length() == 4 and
    x.u.length() == 4 and
    // Selector booleans (iface_l.rs:547-552)
    is_bit(x.sel_e) and is_bit(x.sel_a) and is_bit(x.sel_fa) and
    is_bit(x.sel_b0) and is_bit(x.sel_b1) and is_bit(x.sel_b2) and
    is_bit(x.sel_last) and is_bit(x.sel_fin) and
    is_bit(x.u.nth(0)) and is_bit(x.u.nth(1)) and is_bit(x.u.nth(2)) and is_bit(x.u.nth(3)) and
    // ax boolean and FA => A (iface_l.rs:557-558)
    is_bit(row_act(x)) and
    (x.sel_fa * (1 - x.sel_a) == 0) and
    // FIN exclusion: x[O_FIN] * (x[O_A] + x[O_B0] + x[O_B1]) == 0 (iface_l.rs:578-580)
    (if (cfg.enforce_fin_no_b1)
      (x.sel_fin * (x.sel_a + x.sel_b0 + x.sel_b1) == 0)
     else
      (x.sel_fin * (x.sel_a + x.sel_b0) == 0)) and
    // Capacities and fixed words on E, FA, B0 (iface_l.rs:582-593)
    (x.sel_e * (x.w.nth(0) - TAG_EMPTY) == 0) and
    (x.sel_e * (x.cap.nth(0) - 1) == 0) and
    (x.sel_fa * (x.cap.nth(0) - row_total(x)) == 0) and
    (x.sel_b0 * (x.cap.nth(0) - STEP_LEN) == 0) and
    Set(1, 2, 3).forall(m => row_first_blk(x) * x.cap.nth(m) == 0) and
    Set(1, 2, 3).forall(j => x.sel_e * x.w.nth(j) == 0) and
    // A blocks: [tagEnt, op, param, limbs...], zero padded (iface_l.rs:610-634)
    (if (cfg.enforce_fa_w0) (x.sel_fa * (x.w.nth(0) - TAG_ENT) == 0) else true) and
    (x.sel_fa * (x.w.nth(1) - x.op) == 0) and
    (x.sel_fa * (x.w.nth(2) - x.param) == 0) and
    (if (cfg.enforce_fa_pos_zero) (x.sel_fa * x.pos == 0) else true) and
    (x.sel_a * (1 - x.u.nth(0)) == 0) and
    Set(0, 1, 2).forall(j => x.u.nth(j + 1) * (1 - x.u.nth(j)) == 0) and
    Set(0, 1, 2, 3).forall(j => x.sel_a * (1 - x.u.nth(j)) * x.w.nth(j) == 0) and
    (x.sel_a * (1 - x.sel_last) * (W_BLK - row_u_sum(x)) == 0) and
    (if (cfg.enforce_last_count)
      (x.sel_a * x.sel_last * (x.pos + row_u_sum(x) - row_total(x)) == 0)
     else true) and
    // B blocks: [tagStep, d0, d1, d2], [d3, h0, h1, h2], [h3, 0, 0, 0] (iface_l.rs:636-644)
    (x.sel_b0 * (x.w.nth(0) - TAG_STEP) == 0) and
    Set(1, 2, 3).forall(j =>
      (x.sel_b0 * (x.w.nth(j) - x.d.nth(j - 1)) == 0) and
      (x.sel_b1 * (x.w.nth(j) - x.h.nth(j - 1)) == 0) and
      (x.sel_b2 * x.w.nth(j) == 0)
    ) and
    (x.sel_b1 * (x.w.nth(0) - x.d.nth(3)) == 0) and
    (x.sel_b2 * (x.w.nth(0) - x.h.nth(3)) == 0)
  }

  // Transition constraints between row x and next row y
  pure def row_trans_ok(x: OnionRow, y: OnionRow, cfg: ConstraintFlags): bool = {
    // Row grammar transitions (iface_l.rs:562-576)
    (y.sel_e == 0) and
    ((x.sel_e + x.sel_b2) * row_act(y) * (1 - y.sel_fa) == 0) and
    (x.sel_a * x.sel_last * (1 - y.sel_b0) == 0) and
    (x.sel_a * (1 - x.sel_last) * (1 - y.sel_a) == 0) and
    (x.sel_a * (1 - x.sel_last) * y.sel_fa == 0) and
    (x.sel_b0 * (1 - y.sel_b1) == 0) and
    (if (cfg.enforce_fin_no_b1)
      (x.sel_b1 * (1 - y.sel_b2) == 0)
     else
      (x.sel_b1 * row_act(y) * (1 - y.sel_b2) == 0)) and
    ((1 - row_act(x)) * row_act(y) == 0) and
    (if (cfg.enforce_fin_transition)
      (x.sel_fin - row_act(x) * (1 - row_act(y)) == 0)
     else true) and
    (if (cfg.enforce_cnt_b2)
      (y.cnt - x.cnt - x.sel_b2 == 0)
     else
      (y.cnt - x.cnt == 0)) and
    // Capacity chaining on continuation blocks (iface_l.rs:594-599)
    (if (cfg.enforce_cap_chain)
      Set(0, 1, 2, 3).forall(m => row_cont(y) * (y.cap.nth(m) - x.out.nth(4 + m)) == 0)
     else true) and
    // POS step by W_BLK = 4 on continuation A rows (iface_l.rs:615-618)
    ((y.sel_a - y.sel_fa) * (y.pos - x.pos - W_BLK) == 0) and
    // Carried entry fields across same-entry rows (iface_l.rs:646-650)
    (row_same_entry(y) * (y.op - x.op) == 0) and
    (row_same_entry(y) * (y.param - x.param) == 0) and
    (if (cfg.enforce_same_off) (row_same_entry(y) * (y.off - x.off) == 0) else true) and
    (if (cfg.enforce_same_k) (row_same_entry(y) * (y.k_vals - x.k_vals) == 0) else true) and
    // Chaining d and h (iface_l.rs:652-660)
    Set(0, 1, 2, 3).forall(m =>
      (row_same_entry(y) * (y.d.nth(m) - x.d.nth(m)) == 0) and
      (if (cfg.enforce_d_chain) (y.sel_fa * (y.d.nth(m) - x.out.nth(m)) == 0) else true) and
      (y.sel_b0 * (y.h.nth(m) - x.out.nth(m)) == 0) and
      ((y.sel_b1 + y.sel_b2) * (y.h.nth(m) - x.h.nth(m)) == 0)
    ) and
    // OFF chain across entries (iface_l.rs:662-666)
    (if (cfg.enforce_first_off_zero) (x.sel_e * y.off == 0) else true) and
    (if (cfg.enforce_b2_off_step) (x.sel_b2 * y.sel_fa * (y.off - x.off - x.k_vals) == 0) else true)
  }

  // Full system check parameterized by ConstraintFlags
  pure def system_ok_cfg(sys: SystemInstance, cfg: ConstraintFlags): bool = {
    tables_valid(sys.entries, sys.values) and
    sys.trace.length() >= 1 and
    // First row constraints (iface_l.rs:560-561)
    (sys.trace.nth(0).sel_e == 1) and
    (sys.trace.nth(0).cnt == 0) and
    // Last row constraint (iface_l.rs:577)
    (if (cfg.enforce_fin_transition)
      (sys.trace.nth(sys.trace.length() - 1).sel_fin - row_act(sys.trace.nth(sys.trace.length() - 1)) == 0)
     else true) and
    // Local constraints on all rows
    sys.trace.indices().forall(r => row_local_ok(sys.trace.nth(r), cfg)) and
    // Transition constraints on 0..H-2
    sys.trace.indices().forall(r =>
      if (r + 1 < sys.trace.length())
        row_trans_ok(sys.trace.nth(r), sys.trace.nth(r + 1), cfg)
      else true
    ) and
    // Bus balances
    perm_bus_ok(sys.trace) and
    ent_bus_balanced(sys.entries, sys.trace) and
    val_bus_balanced_cfg(sys.values, sys.trace, true) and
    tie_bus_balanced_cfg(sys.stmt_tie, sys.trace, cfg.enforce_cnt_b2)
  }

  // The real constraint system predicate
  pure def system_ok(sys: SystemInstance): bool = {
    system_ok_cfg(sys, REAL_FLAGS)
  }

  // Soundness predicate: does the statement tie equal the native onion of the tables and n the used count?
  pure def tie_is_sound(sys: SystemInstance): bool = {
    val want = native_onion(sys.entries, sys.values)
    list_eq(sys.stmt_tie.d, want.d) and
    sys.stmt_tie.n_used == want.n_used
  }

  // ---- 7. Mutants (7 from Round 1 + 5 from Round 2 Audit) ----------------------------------

  // Mutant 1 (R1): omits both OFF chain constraints in OnionAir (allows swapping entries)
  pure def mut_no_off_chain(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_first_off_zero: false, enforce_b2_off_step: false })
  }

  // Mutant 2 (R1): omits FA w[0] == TAG_ENT check
  pure def mut_fa_reads_w0(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_fa_w0: false })
  }

  // Mutant 3 (R1): omits last-block count pos + c == total on LAST A-rows
  pure def mut_no_last_count(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_last_count: false })
  }

  // Mutant 4 (R1): allows FIN = 1 on B1 row and transition from B1 to inactive
  pure def mut_fin_on_b1(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_fin_no_b1: false })
  }

  // Mutant 5 (R1): omits capacity chaining from previous out[4..8] on continuation blocks
  pure def mut_cap_not_chained(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_cap_chain: false })
  }

  // Mutant 6 (R1): omits B2 increment in CNT transition and TIE_BUS n_used expression
  pure def mut_cnt_no_b2(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_cnt_b2: false })
  }

  // Mutant 7 (R1): omits chaining of d from previous E/B2 output onto FA row
  pure def mut_d_not_chained(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_d_chain: false })
  }

  // Mutant 8 (R2 new): omits fa * x[O_POS] == 0 (iface_l.rs:614), keeping pos + c == total!
  pure def mut_no_fa_pos_zero(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_fa_pos_zero: false })
  }

  // Mutant 9 (R2 new): omits same * (y[O_OFF] - x[O_OFF]) == 0 (iface_l.rs:647-650)
  pure def mut_no_same_off(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_same_off: false })
  }

  // Mutant 10 (R2 new): omits same * (y[O_K] - x[O_K]) == 0 (iface_l.rs:647-650)
  pure def mut_no_same_k(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_same_k: false })
  }

  // Mutant 11 (R2 new): omits b2 * y[O_FA] * (y[O_OFF] - x[O_OFF] - x[O_K]) == 0 alone, keeping x[O_E] * y[O_OFF] == 0
  pure def mut_no_b2_off_step(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_b2_off_step: false })
  }

  // Mutant 12 (R2 new): omits x[O_FIN] - ax * (1 - ay) == 0 (iface_l.rs:575, 577), keeping TIE_BUS balance
  pure def mut_early_fin(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_fin_transition: false })
  }

  // Defense-in-depth variant: omits x[O_E] * y[O_OFF] == 0 alone while keeping b2 * y[O_FA] * (y[O_OFF] - x[O_OFF] - x[O_K]) == 0
  pure def mut_no_first_off_zero_only(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_first_off_zero: false })
  }

  // ---- 8. Honest Trace Builders (lane-src/iface_l.rs OnionAir::trace) ----------------------

  pure def make_e_row(fin_flag: int): OnionRow = {
    {
      w: [TAG_EMPTY, 0, 0, 0],
      cap: [1, 0, 0, 0],
      out: perm8([TAG_EMPTY, 0, 0, 0, 1, 0, 0, 0]),
      d: ZERO4,
      h: ZERO4,
      u: ZERO4,
      sel_e: 1,
      sel_a: 0,
      sel_fa: 0,
      sel_b0: 0,
      sel_b1: 0,
      sel_b2: 0,
      sel_last: 0,
      sel_fin: fin_flag,
      pos: 0,
      cnt: 0,
      op: 0,
      param: 0,
      off: 0,
      k_vals: 0
    }
  }

  pure def make_pad_row(cnt_end: int): OnionRow = {
    {
      w: ZERO4,
      cap: ZERO4,
      out: ZERO8,
      d: ZERO4,
      h: ZERO4,
      u: ZERO4,
      sel_e: 0,
      sel_a: 0,
      sel_fa: 0,
      sel_b0: 0,
      sel_b1: 0,
      sel_b2: 0,
      sel_last: 0,
      sel_fin: 0,
      pos: 0,
      cnt: cnt_end,
      op: 0,
      param: 0,
      off: 0,
      k_vals: 0
    }
  }

  pure def make_a_row(
    w_in: List[int],
    cap_in: List[int],
    d_cur: List[int],
    u_in: List[int],
    fa_flag: int,
    last_flag: int,
    pos_in: int,
    cnt_in: int,
    e: LogEntryRow
  ): OnionRow = {
    {
      w: w_in,
      cap: cap_in,
      out: perm8(w_in.concat(cap_in)),
      d: d_cur,
      h: ZERO4,
      u: u_in,
      sel_e: 0,
      sel_a: 1,
      sel_fa: fa_flag,
      sel_b0: 0,
      sel_b1: 0,
      sel_b2: 0,
      sel_last: last_flag,
      sel_fin: 0,
      pos: pos_in,
      cnt: cnt_in,
      op: e.op,
      param: e.param,
      off: e.off,
      k_vals: e.k_vals
    }
  }

  pure def make_b_rows(
    d_cur: List[int],
    h_cur: List[int],
    cnt_in: int,
    e: LogEntryRow,
    fin_on_b2: int
  ): List[OnionRow] = {
    val b0_in = [TAG_STEP, d_cur.nth(0), d_cur.nth(1), d_cur.nth(2), STEP_LEN, 0, 0, 0]
    val b0_out = perm8(b0_in)
    val b1_cap = cap_part(b0_out)
    val b1_w = [d_cur.nth(3), h_cur.nth(0), h_cur.nth(1), h_cur.nth(2)]
    val b1_out = perm8(b1_w.concat(b1_cap))
    val b2_cap = cap_part(b1_out)
    val b2_w = [h_cur.nth(3), 0, 0, 0]
    val b2_out = perm8(b2_w.concat(b2_cap))
    [
      {
        w: [TAG_STEP, d_cur.nth(0), d_cur.nth(1), d_cur.nth(2)],
        cap: [STEP_LEN, 0, 0, 0],
        out: b0_out,
        d: d_cur,
        h: h_cur,
        u: ZERO4,
        sel_e: 0,
        sel_a: 0,
        sel_fa: 0,
        sel_b0: 1,
        sel_b1: 0,
        sel_b2: 0,
        sel_last: 0,
        sel_fin: 0,
        pos: 0,
        cnt: cnt_in,
        op: e.op,
        param: e.param,
        off: e.off,
        k_vals: e.k_vals
      },
      {
        w: b1_w,
        cap: b1_cap,
        out: b1_out,
        d: d_cur,
        h: h_cur,
        u: ZERO4,
        sel_e: 0,
        sel_a: 0,
        sel_fa: 0,
        sel_b0: 0,
        sel_b1: 1,
        sel_b2: 0,
        sel_last: 0,
        sel_fin: 0,
        pos: 0,
        cnt: cnt_in,
        op: e.op,
        param: e.param,
        off: e.off,
        k_vals: e.k_vals
      },
      {
        w: b2_w,
        cap: b2_cap,
        out: b2_out,
        d: d_cur,
        h: h_cur,
        u: ZERO4,
        sel_e: 0,
        sel_a: 0,
        sel_fa: 0,
        sel_b0: 0,
        sel_b1: 0,
        sel_b2: 1,
        sel_last: 0,
        sel_fin: fin_on_b2,
        pos: 0,
        cnt: cnt_in,
        op: e.op,
        param: e.param,
        off: e.off,
        k_vals: e.k_vals
      }
    ]
  }

  pure def honest_entry_rows(
    e: LogEntryRow,
    values: List[LogValueRow],
    d_cur: List[int],
    cnt_in: int,
    is_final_entry: int
  ): List[OnionRow] = {
    if (e.k_vals == 1) {
      val r0_out = perm8([TAG_ENT, e.op, e.param, values.nth(e.off).limb0, 5, 0, 0, 0])
      [
        make_a_row(
          [TAG_ENT, e.op, e.param, values.nth(e.off).limb0],
          [5, 0, 0, 0],
          d_cur,
          [1, 1, 1, 1],
          1, 0, 0, cnt_in, e
        ),
        make_a_row(
          [values.nth(e.off).limb1, 0, 0, 0],
          cap_part(r0_out),
          d_cur,
          [1, 0, 0, 0],
          0, 1, 4, cnt_in, e
        )
      ].concat(
        make_b_rows(
          d_cur,
          p2_native(TAG_ENT, entry_enc(e, values)),
          cnt_in,
          e,
          is_final_entry
        )
      )
    } else if (e.k_vals == 2) {
      val r0_out = perm8([TAG_ENT, e.op, e.param, values.nth(e.off).limb0, 7, 0, 0, 0])
      [
        make_a_row(
          [TAG_ENT, e.op, e.param, values.nth(e.off).limb0],
          [7, 0, 0, 0],
          d_cur,
          [1, 1, 1, 1],
          1, 0, 0, cnt_in, e
        ),
        make_a_row(
          [values.nth(e.off).limb1, values.nth(e.off + 1).limb0, values.nth(e.off + 1).limb1, 0],
          cap_part(r0_out),
          d_cur,
          [1, 1, 1, 0],
          0, 1, 4, cnt_in, e
        )
      ].concat(
        make_b_rows(
          d_cur,
          p2_native(TAG_ENT, entry_enc(e, values)),
          cnt_in,
          e,
          is_final_entry
        )
      )
    } else {
      val r0_out = perm8([TAG_ENT, e.op, e.param, values.nth(e.off).limb0, 9, 0, 0, 0])
      val r1_w = [values.nth(e.off).limb1, values.nth(e.off + 1).limb0, values.nth(e.off + 1).limb1, values.nth(e.off + 2).limb0]
      val r1_out = perm8(r1_w.concat(cap_part(r0_out)))
      [
        make_a_row(
          [TAG_ENT, e.op, e.param, values.nth(e.off).limb0],
          [9, 0, 0, 0],
          d_cur,
          [1, 1, 1, 1],
          1, 0, 0, cnt_in, e
        ),
        make_a_row(
          r1_w,
          cap_part(r0_out),
          d_cur,
          [1, 1, 1, 1],
          0, 0, 4, cnt_in, e
        ),
        make_a_row(
          [values.nth(e.off + 2).limb1, 0, 0, 0],
          cap_part(r1_out),
          d_cur,
          [1, 0, 0, 0],
          0, 1, 8, cnt_in, e
        )
      ].concat(
        make_b_rows(
          d_cur,
          p2_native(TAG_ENT, entry_enc(e, values)),
          cnt_in,
          e,
          is_final_entry
        )
      )
    }
  }

  pure def build_honest_system(entries: List[LogEntryRow], values: List[LogValueRow]): SystemInstance = {
    if (entries.nth(0).used == 0) {
      entries: entries,
      values: values,
      stmt_tie: native_onion(entries, values),
      trace: [
        make_e_row(1),
        make_pad_row(0),
        make_pad_row(0)
      ]
    } else if (entries.length() == 1 or entries.nth(1).used == 0) {
      entries: entries,
      values: values,
      stmt_tie: native_onion(entries, values),
      trace: [make_e_row(0)]
        .concat(honest_entry_rows(entries.nth(0), values, d_empty_native, 0, 1))
        .concat([make_pad_row(1), make_pad_row(1)])
    } else {
      val d1 = p2_native(
        TAG_STEP,
        d_empty_native.concat(p2_native(TAG_ENT, entry_enc(entries.nth(0), values)))
      )
      {
        entries: entries,
        values: values,
        stmt_tie: native_onion(entries, values),
        trace: [make_e_row(0)]
          .concat(honest_entry_rows(entries.nth(0), values, d_empty_native, 0, 0))
          .concat(honest_entry_rows(entries.nth(1), values, d1, 1, 1))
          .concat([make_pad_row(2)])
      }
    }
  }

  // ---- 9. Representative Tables & Honest Instances -----------------------------------------

  pure def sample_values_0: List[LogValueRow] = [
    { limb0: 0, limb1: 0, used: 0, cnt: 0 },
    { limb0: 0, limb1: 0, used: 0, cnt: 0 },
    { limb0: 0, limb1: 0, used: 0, cnt: 0 },
    { limb0: 0, limb1: 0, used: 0, cnt: 0 }
  ]

  pure def sample_values_1_seed(s: int): List[LogValueRow] = [
    { limb0: 11 + s, limb1: 12 + s, used: 1, cnt: 1 },
    { limb0: 0,      limb1: 0,      used: 0, cnt: 1 },
    { limb0: 0,      limb1: 0,      used: 0, cnt: 1 },
    { limb0: 0,      limb1: 0,      used: 0, cnt: 1 }
  ]

  pure def sample_values_2_seed(s: int): List[LogValueRow] = [
    { limb0: 11 + s, limb1: 12 + s, used: 1, cnt: 1 },
    { limb0: 21 + s, limb1: 22 + s, used: 1, cnt: 2 },
    { limb0: 0,      limb1: 0,      used: 0, cnt: 2 },
    { limb0: 0,      limb1: 0,      used: 0, cnt: 2 }
  ]

  pure def sample_values_3_seed(s: int): List[LogValueRow] = [
    { limb0: 11 + s, limb1: 12 + s, used: 1, cnt: 1 },
    { limb0: 21 + s, limb1: 22 + s, used: 1, cnt: 2 },
    { limb0: 31 + s, limb1: 32 + s, used: 1, cnt: 3 },
    { limb0: 0,      limb1: 0,      used: 0, cnt: 3 }
  ]

  pure def sample_values_1: List[LogValueRow] = sample_values_1_seed(0)
  pure def sample_values_2: List[LogValueRow] = sample_values_2_seed(0)
  pure def sample_values_3: List[LogValueRow] = sample_values_3_seed(0)

  pure def two_pub_entries: List[LogEntryRow] = [
    { op: OP_PUB, param: 1, off: 0, k_vals: 1, used: 1 },
    { op: OP_PUB, param: 2, off: 1, k_vals: 1, used: 1 }
  ]

  pure def three_pub_entries: List[LogEntryRow] = [
    { op: OP_PUB, param: 1, off: 0, k_vals: 1, used: 1 },
    { op: OP_PUB, param: 2, off: 1, k_vals: 1, used: 1 },
    { op: OP_PUB, param: 3, off: 2, k_vals: 1, used: 1 }
  ]

  pure def honest_empty_sys: SystemInstance = {
    build_honest_system(
      [
        { op: OP_NOP, param: 0, off: 0, k_vals: 0, used: 0 },
        { op: OP_NOP, param: 0, off: 0, k_vals: 0, used: 0 }
      ],
      sample_values_0
    )
  }

  pure def honest_pub_sys: SystemInstance = {
    build_honest_system(
      [
        { op: OP_PUB, param: 3, off: 0, k_vals: 1, used: 1 },
        { op: OP_NOP, param: 0, off: 1, k_vals: 0, used: 0 }
      ],
      sample_values_1
    )
  }

  pure def honest_digest_sys: SystemInstance = {
    build_honest_system(
      [
        { op: OP_DIGEST, param: 2, off: 0, k_vals: 2, used: 1 },
        { op: OP_NOP,    param: 0, off: 2, k_vals: 0, used: 0 }
      ],
      sample_values_2
    )
  }

  pure def honest_h2_sys: SystemInstance = {
    build_honest_system(
      [
        { op: OP_H2,  param: 0, off: 0, k_vals: 3, used: 1 },
        { op: OP_NOP, param: 0, off: 3, k_vals: 0, used: 0 }
      ],
      sample_values_3
    )
  }

  pure def honest_two_entries_sys: SystemInstance = {
    build_honest_system(
      [
        { op: OP_PUB,    param: 1, off: 0, k_vals: 1, used: 1 },
        { op: OP_DIGEST, param: 4, off: 1, k_vals: 2, used: 1 }
      ],
      sample_values_3
    )
  }

  // ---- 10. Non-Honest Adversarial Trace Generator (Task 2) ---------------------------------
  // Builds traces directly from raw selector schedules, entry field assignments, U masks, and
  // word choices without calling build_honest_system / honest_entry_rows.
  // Every active row computes out = perm8(w ++ cap) and claims the FIN row tie.

  pure def raw_row(
    sels: List[int],
    w_in: List[int],
    cap_in: List[int],
    d_in: List[int],
    h_in: List[int],
    u_in: List[int],
    pos_in: int,
    cnt_in: int,
    op_in: int,
    param_in: int,
    off_in: int,
    k_in: int
  ): OnionRow = {
    val act = sels.nth(0) + sels.nth(1) + sels.nth(3) + sels.nth(4) + sels.nth(5)
    {
      w: w_in,
      cap: cap_in,
      out: if (act == 1) perm8(w_in.concat(cap_in)) else ZERO8,
      d: d_in,
      h: h_in,
      u: u_in,
      sel_e: sels.nth(0),
      sel_a: sels.nth(1),
      sel_fa: sels.nth(2),
      sel_b0: sels.nth(3),
      sel_b1: sels.nth(4),
      sel_b2: sels.nth(5),
      sel_last: sels.nth(6),
      sel_fin: sels.nth(7),
      pos: pos_in,
      cnt: cnt_in,
      op: op_in,
      param: param_in,
      off: off_in,
      k_vals: k_in
    }
  }

  pure def claim_fin_tie(tr: List[OnionRow], cnt_has_b2: bool): TieMsg = {
    val fins = tr.indices().filter(r => tr.nth(r).sel_fin == 1)
    if (fins.size() >= 1)
      fins.fold({ d: ZERO4, n_used: 0 }, (_acc, r) => {
        d: rate_part(tr.nth(r).out),
        n_used: tie_row_count(tr.nth(r), cnt_has_b2)
      })
    else { d: ZERO4, n_used: 0 }
  }

  // Candidate U masks of width 4
  pure def u_choice(u_id: int): List[int] = {
    if (u_id == 0) [1, 0, 0, 0]
    else if (u_id == 1) [1, 1, 1, 0]
    else if (u_id == 2) [1, 1, 1, 1]
    else [0, 1, 0, 0]
  }

  // Raw B0..B2 synthesizer from arbitrary selector triples, d_vec, h_vec, cnt, and entry metadata
  pure def raw_b_segment(
    s_b0: List[int],
    s_b1: List[int],
    s_b2: List[int],
    d_vec: List[int],
    h_vec: List[int],
    cnt_v: int,
    op_v: int,
    param_v: int,
    off_v: int,
    k_v: int,
    include_b2: bool
  ): List[OnionRow] = {
    val r_b0 = raw_row(
      s_b0,
      [TAG_STEP, d_vec.nth(0), d_vec.nth(1), d_vec.nth(2)],
      [STEP_LEN, 0, 0, 0],
      d_vec, h_vec, ZERO4, 0, cnt_v, op_v, param_v, off_v, k_v
    )
    val r_b1 = raw_row(
      s_b1,
      [d_vec.nth(3), h_vec.nth(0), h_vec.nth(1), h_vec.nth(2)],
      cap_part(r_b0.out),
      d_vec, h_vec, ZERO4, 0, cnt_v, op_v, param_v, off_v, k_v
    )
    if (include_b2) {
      val r_b2 = raw_row(
        s_b2,
        [h_vec.nth(3), 0, 0, 0],
        cap_part(r_b1.out),
        d_vec, h_vec, ZERO4, 0, cnt_v, op_v, param_v, off_v, k_v
      )
      [r_b0, r_b1, r_b2]
    } else [r_b0, r_b1]
  }

  // Raw adversarial generator for 1-entry logs (k in {1, 2, 3}) and 2-entry / 3-entry logs.
  // Parameterized by nondeterministic choices of selector pattern, entry fields, U mask, and words.
  pure def gen_adv_1entry(
    op_c: int,
    param_c: int,
    limb_s: int,
    sel_pat: int,
    u_last_id: int,
    w0_tag: int,
    cap_chain: int,
    d_chain: int,
    cnt_step: int
  ): SystemInstance = {
    val kv = op_vals(op_c)
    val entries = [
      { op: op_c, param: param_c, off: 0, k_vals: kv, used: 1 },
      { op: OP_NOP, param: 0, off: kv, k_vals: 0, used: 0 }
    ]
    val vals =
      if (kv == 1) sample_values_1_seed(limb_s)
      else if (kv == 2) sample_values_2_seed(limb_s)
      else sample_values_3_seed(limb_s)
    val l0 = vals.nth(0).limb0
    val l1 = vals.nth(0).limb1
    val l2 = vals.nth(1).limb0
    val l3 = vals.nth(1).limb1
    val l4 = vals.nth(2).limb0
    val l5 = vals.nth(2).limb1
    val tot = 3 + 2 * kv
    val r_e = raw_row([1, 0, 0, 0, 0, 0, 0, 0], [TAG_EMPTY, 0, 0, 0], [1, 0, 0, 0], ZERO4, ZERO4, ZERO4, 0, 0, 0, 0, 0, 0)
    val d0 = if (d_chain == 1) rate_part(r_e.out) else ZERO4
    val u_last = u_choice(u_last_id)
    val r_a0 = raw_row(
      [0, 1, 1, 0, 0, 0, 0, 0],
      [w0_tag, op_c, param_c, l0],
      [tot, 0, 0, 0],
      d0, ZERO4, [1, 1, 1, 1], 0, 0, op_c, param_c, 0, kv
    )
    val cap1 = if (cap_chain == 1) cap_part(r_a0.out) else ZERO4
    val w_a1 =
      if (kv == 1) [l1, 0, 0, 0]
      else if (kv == 2) [l1, l2, l3, 0]
      else [l1, l2, l3, l4]
    val a_rows =
      if (sel_pat == 1) {
        // 3 A-blocks: FA -> mid A -> LAST A
        val r_a1 = raw_row(
          [0, 1, 0, 0, 0, 0, 0, 0],
          w_a1, cap1, d0, ZERO4, [1, 1, 1, 1], 4, 0, op_c, param_c, 0, kv
        )
        val cap2 = if (cap_chain == 1) cap_part(r_a1.out) else ZERO4
        val r_a2 = raw_row(
          [0, 1, 0, 0, 0, 0, 1, 0],
          [l5, 0, 0, 0], cap2, d0, ZERO4, u_last, 8, 0, op_c, param_c, 0, kv
        )
        [r_a0, r_a1, r_a2]
      } else {
        // 2 A-blocks: FA -> LAST A
        val r_a1 = raw_row(
          [0, 1, 0, 0, 0, 0, 1, 0],
          w_a1, cap1, d0, ZERO4, u_last, 4, 0, op_c, param_c, 0, kv
        )
        [r_a0, r_a1]
      }
    val h0 = rate_part(a_rows.nth(a_rows.length() - 1).out)
    val trunc_b1 = (sel_pat == 4)
    val b_rows = raw_b_segment(
      [0, 0, 0, 1, 0, 0, 0, 0],
      [0, 0, 0, 0, 1, 0, 0, if (trunc_b1) 1 else 0],
      [0, 0, 0, 0, 0, 1, 0, 1],
      d0, h0, 0, op_c, param_c, 0, kv, not(trunc_b1)
    )
    val pad_cnt = if (not(trunc_b1) and cnt_step == 1) 1 else 0
    val r_pad = raw_row([0, 0, 0, 0, 0, 0, 0, 0], ZERO4, ZERO4, ZERO4, ZERO4, ZERO4, 0, pad_cnt, 0, 0, 0, 0)
    val tr = [r_e].concat(a_rows).concat(b_rows).concat([r_pad])
    {
      entries: entries,
      values: vals,
      stmt_tie: claim_fin_tie(tr, cnt_step == 1),
      trace: tr
    }
  }

  // Raw adversarial generator for 2-entry logs (and 3-entry order-swap logs).
  pure def gen_adv_2entry(
    second_op: int,
    limb_s: int,
    adv_kind: int
  ): SystemInstance = {
    // adv_kind:
    //   0 -> valid 2-entry trace synthesized by raw_row (either PUB+PUB or PUB+DIGEST)
    //   1 -> swapped entry absorption order (catches mut_no_off_chain)
    //   2 -> LAST count mismatch: e0 has c=3, e1 has 1-row FA+LAST with c=3 (catches mut_no_last_count)
    //   3 -> FA pos != 0: e0 has 1-row FA+LAST with pos=2, c=3; e1 has pos=-2 -> 2, c=3 (catches mut_no_fa_pos_zero)
    //   4 -> non-FA off modified: e0 Row 2 sets off=1, e1 Row 2 sets off=0 (catches mut_no_same_off)
    //   5 -> early FIN on e0's B2 row while e1 continues with FIN=0 (catches mut_early_fin)
    //   6 -> 3-entry log [e0, e2, e1] with B2 k_vals modified (catches mut_no_same_k)
    //   7 -> 3-entry log [e0, e2, e1] with honest k_vals=1 on all rows (catches mut_no_b2_off_step)
    if (adv_kind == 6 or adv_kind == 7) {
      val vals = sample_values_3_seed(limb_s)
      val r_e = raw_row([1, 0, 0, 0, 0, 0, 0, 0], [TAG_EMPTY, 0, 0, 0], [1, 0, 0, 0], ZERO4, ZERO4, ZERO4, 0, 0, 0, 0, 0, 0)
      val d0 = rate_part(r_e.out)
      val k_b_e0 = if (adv_kind == 6) 2 else 1
      val k_b_e2 = if (adv_kind == 6) -1 else 1
      // Entry 0 (off=0, k=1 on FA/LAST; k_b_e0 on B0..B2)
      val e0_a0 = raw_row([0, 1, 1, 0, 0, 0, 0, 0], [TAG_ENT, OP_PUB, 1, vals.nth(0).limb0], [5, 0, 0, 0], d0, ZERO4, [1, 1, 1, 1], 0, 0, OP_PUB, 1, 0, 1)
      val e0_a1 = raw_row([0, 1, 0, 0, 0, 0, 1, 0], [vals.nth(0).limb1, 0, 0, 0], cap_part(e0_a0.out), d0, ZERO4, [1, 0, 0, 0], 4, 0, OP_PUB, 1, 0, 1)
      val h0 = rate_part(e0_a1.out)
      val e0_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0], d0, h0, 0, OP_PUB, 1, 0, k_b_e0, true)
      val d1 = rate_part(e0_b.nth(2).out)
      // Entry 2 absorbed second (off=2, k=1 on FA/LAST; k_b_e2 on B0..B2)
      val e2_a0 = raw_row([0, 1, 1, 0, 0, 0, 0, 0], [TAG_ENT, OP_PUB, 3, vals.nth(2).limb0], [5, 0, 0, 0], d1, ZERO4, [1, 1, 1, 1], 0, 1, OP_PUB, 3, 2, 1)
      val e2_a1 = raw_row([0, 1, 0, 0, 0, 0, 1, 0], [vals.nth(2).limb1, 0, 0, 0], cap_part(e2_a0.out), d1, ZERO4, [1, 0, 0, 0], 4, 1, OP_PUB, 3, 2, 1)
      val h2 = rate_part(e2_a1.out)
      val e2_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0], d1, h2, 1, OP_PUB, 3, 2, k_b_e2, true)
      val d2 = rate_part(e2_b.nth(2).out)
      // Entry 1 absorbed third (off=1, k=1)
      val e1_a0 = raw_row([0, 1, 1, 0, 0, 0, 0, 0], [TAG_ENT, OP_PUB, 2, vals.nth(1).limb0], [5, 0, 0, 0], d2, ZERO4, [1, 1, 1, 1], 0, 2, OP_PUB, 2, 1, 1)
      val e1_a1 = raw_row([0, 1, 0, 0, 0, 0, 1, 0], [vals.nth(1).limb1, 0, 0, 0], cap_part(e1_a0.out), d2, ZERO4, [1, 0, 0, 0], 4, 2, OP_PUB, 2, 1, 1)
      val h1 = rate_part(e1_a1.out)
      val e1_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 1], d2, h1, 2, OP_PUB, 2, 1, 1, true)
      val r_pad = raw_row([0, 0, 0, 0, 0, 0, 0, 0], ZERO4, ZERO4, ZERO4, ZERO4, ZERO4, 0, 3, 0, 0, 0, 0)
      val tr = [r_e, e0_a0, e0_a1].concat(e0_b).concat([e2_a0, e2_a1]).concat(e2_b).concat([e1_a0, e1_a1]).concat(e1_b).concat([r_pad])
      {
        entries: three_pub_entries,
        values: vals,
        stmt_tie: claim_fin_tie(tr, true),
        trace: tr
      }
    } else if (adv_kind == 2 or adv_kind == 3 or adv_kind == 4) {
      val vals = sample_values_2_seed(limb_s)
      val l0 = vals.nth(0).limb0
      val l1 = vals.nth(0).limb1
      val l2 = vals.nth(1).limb0
      val l3 = vals.nth(1).limb1
      val r_e = raw_row([1, 0, 0, 0, 0, 0, 0, 0], [TAG_EMPTY, 0, 0, 0], [1, 0, 0, 0], ZERO4, ZERO4, ZERO4, 0, 0, 0, 0, 0, 0)
      val d0 = rate_part(r_e.out)
      if (adv_kind == 2) {
        // e0 steals e1's limbs on LAST row (c=3 != 1); e1 uses 1-row FA+LAST with c=3 (reads 0 limbs)
        val e0_a0 = raw_row([0, 1, 1, 0, 0, 0, 0, 0], [TAG_ENT, OP_PUB, 1, l0], [5, 0, 0, 0], d0, ZERO4, [1, 1, 1, 1], 0, 0, OP_PUB, 1, 0, 1)
        val e0_a1 = raw_row([0, 1, 0, 0, 0, 0, 1, 0], [l1, l2, l3, 0], cap_part(e0_a0.out), d0, ZERO4, [1, 1, 1, 0], 4, 0, OP_PUB, 1, 0, 1)
        val h0 = rate_part(e0_a1.out)
        val e0_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0], d0, h0, 0, OP_PUB, 1, 0, 1, true)
        val d1 = rate_part(e0_b.nth(2).out)
        val e1_a0 = raw_row([0, 1, 1, 0, 0, 0, 1, 0], [TAG_ENT, OP_PUB, 2, 0], [5, 0, 0, 0], d1, ZERO4, [1, 1, 1, 0], 0, 1, OP_PUB, 2, 1, 1)
        val h1 = rate_part(e1_a0.out)
        val e1_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 1], d1, h1, 1, OP_PUB, 2, 1, 1, true)
        val r_pad = raw_row([0, 0, 0, 0, 0, 0, 0, 0], ZERO4, ZERO4, ZERO4, ZERO4, ZERO4, 0, 2, 0, 0, 0, 0)
        val tr = [r_e, e0_a0, e0_a1].concat(e0_b).concat([e1_a0]).concat(e1_b).concat([r_pad])
        { entries: two_pub_entries, values: vals, stmt_tie: claim_fin_tie(tr, true), trace: tr }
      } else if (adv_kind == 3) {
        // e0 has 1-row FA+LAST with pos=2, c=3 (pos+c=5==total, reads 0 limbs!);
        // e1 has Row 1 (FA, pos=-2, c=4, reads g=0) and Row 2 (LAST, pos=2, c=3, pos+c=5==total, reads g=1,2,3!)
        val e0_a0 = raw_row([0, 1, 1, 0, 0, 0, 1, 0], [TAG_ENT, OP_PUB, 1, 0], [5, 0, 0, 0], d0, ZERO4, [1, 1, 1, 0], 2, 0, OP_PUB, 1, 0, 1)
        val h0 = rate_part(e0_a0.out)
        val e0_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0], d0, h0, 0, OP_PUB, 1, 0, 1, true)
        val d1 = rate_part(e0_b.nth(2).out)
        val e1_a0 = raw_row([0, 1, 1, 0, 0, 0, 0, 0], [TAG_ENT, OP_PUB, 2, l0], [5, 0, 0, 0], d1, ZERO4, [1, 1, 1, 1], -2, 1, OP_PUB, 2, 1, 1)
        val e1_a1 = raw_row([0, 1, 0, 0, 0, 0, 1, 0], [l1, l2, l3, 0], cap_part(e1_a0.out), d1, ZERO4, [1, 1, 1, 0], 2, 1, OP_PUB, 2, 1, 1)
        val h1 = rate_part(e1_a1.out)
        val e1_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 1], d1, h1, 1, OP_PUB, 2, 1, 1, true)
        val r_pad = raw_row([0, 0, 0, 0, 0, 0, 0, 0], ZERO4, ZERO4, ZERO4, ZERO4, ZERO4, 0, 2, 0, 0, 0, 0)
        val tr = [r_e, e0_a0].concat(e0_b).concat([e1_a0, e1_a1]).concat(e1_b).concat([r_pad])
        { entries: two_pub_entries, values: vals, stmt_tie: claim_fin_tie(tr, true), trace: tr }
      } else {
        // adv_kind == 4: e0 Row 2 sets off=1 (reads g=3=l3), e1 Row 2 sets off=0 (reads g=1=l1); B0..B2 rows restore off=0,1
        val e0_a0 = raw_row([0, 1, 1, 0, 0, 0, 0, 0], [TAG_ENT, OP_PUB, 1, l0], [5, 0, 0, 0], d0, ZERO4, [1, 1, 1, 1], 0, 0, OP_PUB, 1, 0, 1)
        val e0_a1 = raw_row([0, 1, 0, 0, 0, 0, 1, 0], [l3, 0, 0, 0], cap_part(e0_a0.out), d0, ZERO4, [1, 0, 0, 0], 4, 0, OP_PUB, 1, 1, 1)
        val h0 = rate_part(e0_a1.out)
        val e0_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0], d0, h0, 0, OP_PUB, 1, 0, 1, true)
        val d1 = rate_part(e0_b.nth(2).out)
        val e1_a0 = raw_row([0, 1, 1, 0, 0, 0, 0, 0], [TAG_ENT, OP_PUB, 2, l2], [5, 0, 0, 0], d1, ZERO4, [1, 1, 1, 1], 0, 1, OP_PUB, 2, 1, 1)
        val e1_a1 = raw_row([0, 1, 0, 0, 0, 0, 1, 0], [l1, 0, 0, 0], cap_part(e1_a0.out), d1, ZERO4, [1, 0, 0, 0], 4, 1, OP_PUB, 2, 0, 1)
        val h1 = rate_part(e1_a1.out)
        val e1_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 1], d1, h1, 1, OP_PUB, 2, 1, 1, true)
        val r_pad = raw_row([0, 0, 0, 0, 0, 0, 0, 0], ZERO4, ZERO4, ZERO4, ZERO4, ZERO4, 0, 2, 0, 0, 0, 0)
        val tr = [r_e, e0_a0, e0_a1].concat(e0_b).concat([e1_a0, e1_a1]).concat(e1_b).concat([r_pad])
        { entries: two_pub_entries, values: vals, stmt_tie: claim_fin_tie(tr, true), trace: tr }
      }
    } else {
      // adv_kind in {0, 1, 5} on PUB (k=1) + second_op (PUB k=1 or DIGEST k=2)
      val k2 = op_vals(second_op)
      val entries = [
        { op: OP_PUB,    param: 1, off: 0, k_vals: 1,  used: 1 },
        { op: second_op, param: 4, off: 1, k_vals: k2, used: 1 }
      ]
      val vals = if (k2 == 1) sample_values_2_seed(limb_s) else sample_values_3_seed(limb_s)
      val r_e = raw_row([1, 0, 0, 0, 0, 0, 0, 0], [TAG_EMPTY, 0, 0, 0], [1, 0, 0, 0], ZERO4, ZERO4, ZERO4, 0, 0, 0, 0, 0, 0)
      val d0 = rate_part(r_e.out)
      val first_idx = if (adv_kind == 1) 1 else 0
      val second_idx = if (adv_kind == 1) 0 else 1
      val ef = entries.nth(first_idx)
      val es = entries.nth(second_idx)
      val tot_f = 3 + 2 * ef.k_vals
      val tot_s = 3 + 2 * es.k_vals
      val wf0 = [TAG_ENT, ef.op, ef.param, vals.nth(ef.off).limb0]
      val wf1 = if (ef.k_vals == 1) [vals.nth(ef.off).limb1, 0, 0, 0]
                else [vals.nth(ef.off).limb1, vals.nth(ef.off + 1).limb0, vals.nth(ef.off + 1).limb1, 0]
      val uf1 = if (ef.k_vals == 1) [1, 0, 0, 0] else [1, 1, 1, 0]
      val f_a0 = raw_row([0, 1, 1, 0, 0, 0, 0, 0], wf0, [tot_f, 0, 0, 0], d0, ZERO4, [1, 1, 1, 1], 0, 0, ef.op, ef.param, ef.off, ef.k_vals)
      val f_a1 = raw_row([0, 1, 0, 0, 0, 0, 1, 0], wf1, cap_part(f_a0.out), d0, ZERO4, uf1, 4, 0, ef.op, ef.param, ef.off, ef.k_vals)
      val hf = rate_part(f_a1.out)
      val fin_first = if (adv_kind == 5) 1 else 0
      val f_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, fin_first], d0, hf, 0, ef.op, ef.param, ef.off, ef.k_vals, true)
      val d1 = rate_part(f_b.nth(2).out)
      val ws0 = [TAG_ENT, es.op, es.param, vals.nth(es.off).limb0]
      val ws1 = if (es.k_vals == 1) [vals.nth(es.off).limb1, 0, 0, 0]
                else [vals.nth(es.off).limb1, vals.nth(es.off + 1).limb0, vals.nth(es.off + 1).limb1, 0]
      val us1 = if (es.k_vals == 1) [1, 0, 0, 0] else [1, 1, 1, 0]
      val s_a0 = raw_row([0, 1, 1, 0, 0, 0, 0, 0], ws0, [tot_s, 0, 0, 0], d1, ZERO4, [1, 1, 1, 1], 0, 1, es.op, es.param, es.off, es.k_vals)
      val s_a1 = raw_row([0, 1, 0, 0, 0, 0, 1, 0], ws1, cap_part(s_a0.out), d1, ZERO4, us1, 4, 1, es.op, es.param, es.off, es.k_vals)
      val hs = rate_part(s_a1.out)
      val fin_second = if (adv_kind == 5) 0 else 1
      val s_b = raw_b_segment([0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, fin_second], d1, hs, 1, es.op, es.param, es.off, es.k_vals, true)
      val r_pad = raw_row([0, 0, 0, 0, 0, 0, 0, 0], ZERO4, ZERO4, ZERO4, ZERO4, ZERO4, 0, 2, 0, 0, 0, 0)
      val tr = [r_e, f_a0, f_a1].concat(f_b).concat([s_a0, s_a1]).concat(s_b).concat([r_pad])
      {
        entries: entries,
        values: vals,
        stmt_tie: claim_fin_tie(tr, true),
        trace: tr
      }
    }
  }

  // ---- 11. Concrete Counterexamples for All 12 Mutants -------------------------------------

  // CEX 1 (mut_no_off_chain): absorbs entry 1 (DIGEST, off=1, k=2) before entry 0 (PUB, off=0, k=1)
  pure def cex_no_off_chain: SystemInstance = gen_adv_2entry(OP_DIGEST, 0, 1)

  // CEX 2 (mut_fa_reads_w0): replaces w[0] on FA row with 99 != TAG_ENT
  pure def cex_fa_reads_w0: SystemInstance = gen_adv_1entry(OP_PUB, 3, 0, 0, 0, 99, 1, 1, 1)

  // CEX 3 (mut_no_last_count): e0 has LAST c=3 (steals e1 limbs), e1 has 1-row FA+LAST with c=3 (reads 0 limbs)
  pure def cex_no_last_count: SystemInstance = gen_adv_2entry(OP_PUB, 0, 2)

  // CEX 4 (mut_fin_on_b1): sets FIN=1 on B1 and transitions directly to inactive pad row
  pure def cex_fin_on_b1: SystemInstance = gen_adv_1entry(OP_PUB, 3, 0, 4, 0, TAG_ENT, 1, 1, 1)

  // CEX 5 (mut_cap_not_chained): sets cap = [0,0,0,0] on continuation A-row
  pure def cex_cap_not_chained: SystemInstance = gen_adv_1entry(OP_PUB, 3, 0, 0, 0, TAG_ENT, 0, 1, 1)

  // CEX 6 (mut_cnt_no_b2): keeps cnt = 0 across B2 -> pad and reports n_used = 0
  pure def cex_cnt_no_b2: SystemInstance = gen_adv_1entry(OP_PUB, 3, 0, 0, 0, TAG_ENT, 1, 1, 0)

  // CEX 7 (mut_d_not_chained): uses d = [0,0,0,0] on entry 0 instead of D_EMPTY from E row
  pure def cex_d_not_chained: SystemInstance = gen_adv_1entry(OP_PUB, 3, 0, 0, 0, TAG_ENT, 1, 0, 1)

  // CEX 8 (mut_no_fa_pos_zero): e0 has FA+LAST with pos=2, c=3 (pos+c=5, reads 0 limbs); e1 has pos=-2 -> 2, c=3 (pos+c=5, reads g=0..3)
  pure def cex_no_fa_pos_zero: SystemInstance = gen_adv_2entry(OP_PUB, 0, 3)

  // CEX 9 (mut_no_same_off): e0 Row 2 sets off=1 (reads g=3) and e1 Row 2 sets off=0 (reads g=1), swapping second limbs
  pure def cex_no_same_off: SystemInstance = gen_adv_2entry(OP_PUB, 0, 4)

  // CEX 10 (mut_no_same_k): 3 PUB entries absorbed in order [e0, e2, e1] by setting k_vals=2 on e0.B0..B2 and k_vals=-1 on e2.B0..B2
  pure def cex_no_same_k: SystemInstance = gen_adv_2entry(OP_PUB, 0, 6)

  // CEX 11 (mut_no_b2_off_step): 3 PUB entries absorbed in order [e0, e2, e1] with honest k_vals=1 on all rows (violates only B2->FA off step, while x[O_E]*y[O_OFF]=0 holds!)
  pure def cex_no_b2_off_step: SystemInstance = gen_adv_2entry(OP_PUB, 0, 7)

  // CEX 12 (mut_early_fin): 2-entry trace where FIN=1 is placed on e0.B2 and FIN=0 on e1.B2
  pure def cex_early_fin: SystemInstance = gen_adv_2entry(OP_DIGEST, 0, 5)

  // ---- 12. Random One-Cell Tampering Helpers (Round 1 Lead Extension Preserved) ------------

  pure def fin_tie(tr: List[OnionRow], old: TieMsg): TieMsg = {
    val fins = tr.indices().filter(r => tr.nth(r).sel_fin == 1)
    if (fins.size() == 1)
      fins.fold(old, (_acc, r) => { d: rate_part(tr.nth(r).out), n_used: tr.nth(r).cnt + tr.nth(r).sel_b2 })
    else old
  }

  pure def bump_row(x: OnionRow, f: int, dl: int): OnionRow = {
    if (f == 0) { ...x, pos: x.pos + dl }
    else if (f == 1) { ...x, cnt: x.cnt + dl }
    else if (f == 2) { ...x, op: x.op + dl }
    else if (f == 3) { ...x, param: x.param + dl }
    else if (f == 4) { ...x, off: x.off + dl }
    else if (f == 5) { ...x, k_vals: x.k_vals + dl }
    else if (f == 6) { ...x, sel_a: x.sel_a + dl }
    else if (f == 7) { ...x, sel_fa: x.sel_fa + dl }
    else if (f == 8) { ...x, sel_last: x.sel_last + dl }
    else if (f == 9) { ...x, sel_b2: x.sel_b2 + dl }
    else if (f == 10) { ...x, sel_fin: x.sel_fin + dl }
    else if (f == 11) { ...x, w: x.w.replaceAt(1, x.w.nth(1) + dl) }
    else if (f == 12) { ...x, d: x.d.replaceAt(0, x.d.nth(0) + dl) }
    else if (f == 13) { ...x, h: x.h.replaceAt(3, x.h.nth(3) + dl) }
    else { ...x, u: x.u.replaceAt(3, x.u.nth(3) + dl) }
  }

  pure def tamper(sys: SystemInstance, kind: int, ri: int, f: int, dl: int): SystemInstance = {
    if (kind == 0 and sys.values.length() > 0) {
      val i = ri % sys.values.length()
      { ...sys, values: sys.values.replaceAt(i, { ...sys.values.nth(i), limb1: sys.values.nth(i).limb1 + dl }) }
    } else if (kind == 1) {
      { ...sys, stmt_tie: { ...sys.stmt_tie, d: sys.stmt_tie.d.replaceAt(f % 4, sys.stmt_tie.d.nth(f % 4) + dl) } }
    } else if (kind == 2) {
      { ...sys, stmt_tie: { ...sys.stmt_tie, n_used: sys.stmt_tie.n_used + dl } }
    } else {
      val i = ri % sys.trace.length()
      val tr2 = sys.trace.replaceAt(i, bump_row(sys.trace.nth(i), f, dl))
      { ...sys, trace: tr2, stmt_tie: fin_tie(tr2, sys.stmt_tie) }
    }
  }

  // ---- 13. State Machine & Guarded Actions -------------------------------------------------

  var state: ModelState

  pure def can_step_adv(s: ModelState, limb_s: int, cfg_id: int): bool = {
    s.adv_total >= 0 and limb_s >= 0 and cfg_id >= 0
  }

  pure def build_adv_from_choices(
    limb_s: int,
    cfg_id: int,
    free_op: int,
    free_sel: int,
    free_u: int,
    free_w0: int,
    free_cap: int,
    free_d: int,
    free_cnt: int
  ): SystemInstance = {
    // cfg_id 0..4: non-honest generator produces valid traces across all 5 log shapes (1-entry PUB, DIGEST, H2, 2-entry PUB+PUB, PUB+DIGEST)
    if (cfg_id == 0) gen_adv_1entry(OP_PUB, 3, limb_s, 0, 0, TAG_ENT, 1, 1, 1)
    else if (cfg_id == 1) gen_adv_1entry(OP_DIGEST, 2, limb_s, 0, 1, TAG_ENT, 1, 1, 1)
    else if (cfg_id == 2) gen_adv_1entry(OP_H2, 0, limb_s, 1, 0, TAG_ENT, 1, 1, 1)
    else if (cfg_id == 3) gen_adv_2entry(OP_PUB, limb_s, 0)
    else if (cfg_id == 4) gen_adv_2entry(OP_DIGEST, limb_s, 0)
    // cfg_id 5..16: non-honest generator explores all 12 mutant-breaking adversarial families with random limb seeds
    else if (cfg_id == 5) gen_adv_2entry(OP_DIGEST, limb_s, 1)
    else if (cfg_id == 6) gen_adv_1entry(OP_PUB, 3, limb_s, 0, 0, 99, 1, 1, 1)
    else if (cfg_id == 7) gen_adv_2entry(OP_PUB, limb_s, 2)
    else if (cfg_id == 8) gen_adv_1entry(OP_PUB, 3, limb_s, 4, 0, TAG_ENT, 1, 1, 1)
    else if (cfg_id == 9) gen_adv_1entry(OP_PUB, 3, limb_s, 0, 0, TAG_ENT, 0, 1, 1)
    else if (cfg_id == 10) gen_adv_1entry(OP_PUB, 3, limb_s, 0, 0, TAG_ENT, 1, 1, 0)
    else if (cfg_id == 11) gen_adv_1entry(OP_PUB, 3, limb_s, 0, 0, TAG_ENT, 1, 0, 1)
    else if (cfg_id == 12) gen_adv_2entry(OP_PUB, limb_s, 3)
    else if (cfg_id == 13) gen_adv_2entry(OP_PUB, limb_s, 4)
    else if (cfg_id == 14) gen_adv_2entry(OP_PUB, limb_s, 6)
    else if (cfg_id == 15) cex_no_b2_off_step
    else if (cfg_id == 16) gen_adv_2entry(OP_DIGEST, limb_s, 5)
    // cfg_id 17..19: unconstrained combinations of selector pattern, U mask, header word, cap/d/cnt chaining
    else gen_adv_1entry(free_op, 0, limb_s, free_sel, free_u, free_w0, free_cap, free_d, free_cnt)
  }

  pure def apply_adv_sys(s: ModelState, next_sys: SystemInstance): ModelState = {
    val accepted_inc = if (system_ok(next_sys)) 1 else 0
    {
      sys: next_sys,
      is_honest: false,
      step_kind: 2,
      adv_accepted: s.adv_accepted + accepted_inc,
      adv_total: s.adv_total + 1
    }
  }

  action init: bool = {
    nondet pick = Set(0, 1, 2, 3, 4).oneOf()
    val init_sys =
      if (pick == 0) honest_empty_sys
      else if (pick == 1) honest_pub_sys
      else if (pick == 2) honest_digest_sys
      else if (pick == 3) honest_h2_sys
      else honest_two_entries_sys
    all {
      state' = {
        sys: init_sys,
        is_honest: true,
        step_kind: 0,
        adv_accepted: 0,
        adv_total: 0
      }
    }
  }

  action step_honest: bool = {
    nondet pick = Set(0, 1, 2, 3, 4).oneOf()
    val next_sys =
      if (pick == 0) honest_empty_sys
      else if (pick == 1) honest_pub_sys
      else if (pick == 2) honest_digest_sys
      else if (pick == 3) honest_h2_sys
      else honest_two_entries_sys
    all {
      state.adv_total >= 0,
      state' = {
        ...state,
        sys: next_sys,
        is_honest: true,
        step_kind: 1
      }
    }
  }

  action step_adv: bool = {
    nondet limb_s = Set(0, 1, 2).oneOf()
    nondet cfg_id = 0.to(19).oneOf()
    nondet free_op = Set(OP_PUB, OP_DIGEST, OP_H2).oneOf()
    nondet free_sel = Set(0, 1, 4).oneOf()
    nondet free_u = Set(0, 1, 2, 3).oneOf()
    nondet free_w0 = Set(TAG_ENT, 99).oneOf()
    nondet free_cap = Set(0, 1).oneOf()
    nondet free_d = Set(0, 1).oneOf()
    nondet free_cnt = Set(0, 1).oneOf()
    val next_sys = build_adv_from_choices(
      limb_s, cfg_id, free_op, free_sel, free_u, free_w0, free_cap, free_d, free_cnt
    )
    all {
      can_step_adv(state, limb_s, cfg_id),
      state' = apply_adv_sys(state, next_sys)
    }
  }

  action step_tamper: bool = {
    nondet pick = Set(0, 1, 2, 3, 4).oneOf()
    nondet kind = Set(0, 1, 2, 3, 3, 3).oneOf()
    nondet ri = 0.to(40).oneOf()
    nondet f = 0.to(14).oneOf()
    nondet dl = Set(-2, -1, 1, 2).oneOf()
    val base =
      if (pick == 0) honest_empty_sys
      else if (pick == 1) honest_pub_sys
      else if (pick == 2) honest_digest_sys
      else if (pick == 3) honest_h2_sys
      else honest_two_entries_sys
    val next_sys = tamper(base, kind, ri, f, dl)
    val accepted_inc = if (system_ok(next_sys)) 1 else 0
    all {
      state.adv_total >= 0,
      state' = {
        sys: next_sys,
        is_honest: false,
        step_kind: 3,
        adv_accepted: state.adv_accepted + accepted_inc,
        adv_total: state.adv_total + 1
      }
    }
  }

  action step: bool = any {
    step_adv,
    step_honest,
    step_tamper
  }

  // ---- 14. Invariants & Witnesses (quint-modeling Step 5) ----------------------------------

  // Safety Invariant 1 (SOUND): Whenever every modelled constraint and bus balance holds,
  // the tie equals the native onion of the tables and n the used count.
  val SOUND: bool = {
    system_ok(state.sys) implies tie_is_sound(state.sys)
  }

  // Safety Invariant 2 (COMPLETE): Every honest trace is accepted and matches the native onion.
  val COMPLETE: bool = {
    state.is_honest implies (system_ok(state.sys) and tie_is_sound(state.sys))
  }

  val SOUND_AND_COMPLETE: bool = {
    SOUND and COMPLETE
  }

  val TAMPER_SOUND: bool = {
    system_ok(state.sys) implies tie_is_sound(state.sys)
  }

  // Positive witnesses for `quint run --witnesses`
  val witness_adv_accepted: bool = {
    state.step_kind == 2 and system_ok(state.sys)
  }

  val witness_adv_rejected: bool = {
    state.step_kind == 2 and not(system_ok(state.sys))
  }

  val witness_honest_step: bool = {
    state.step_kind == 1 and system_ok(state.sys)
  }

  val witness_tamper_step: bool = {
    state.step_kind == 3
  }

  // Negated-invariant witness (expected to be violated once the adversarial generator finds an accepted trace)
  val INV_WITNESS_NO_ADV_ACCEPTED: bool = {
    state.adv_accepted == 0
  }

  // Mutant invariants (each is VIOLATED when its mutant accepts an unsound adversarial trace)
  val INV_MUT_NO_OFF_CHAIN: bool = {
    mut_no_off_chain(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_FA_READS_W0: bool = {
    mut_fa_reads_w0(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_NO_LAST_COUNT: bool = {
    mut_no_last_count(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_FIN_ON_B1: bool = {
    mut_fin_on_b1(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_CAP_NOT_CHAINED: bool = {
    mut_cap_not_chained(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_CNT_NO_B2: bool = {
    mut_cnt_no_b2(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_D_NOT_CHAINED: bool = {
    mut_d_not_chained(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_NO_FA_POS_ZERO: bool = {
    mut_no_fa_pos_zero(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_NO_SAME_OFF: bool = {
    mut_no_same_off(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_NO_SAME_K: bool = {
    mut_no_same_k(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_NO_B2_OFF_STEP: bool = {
    mut_no_b2_off_step(state.sys) implies tie_is_sound(state.sys)
  }

  val INV_MUT_EARLY_FIN: bool = {
    mut_early_fin(state.sys) implies tie_is_sound(state.sys)
  }

  // Defense-in-depth invariant: dropping x[O_E] * y[O_OFF] == 0 alone while keeping b2 * y[O_FA] * (y[O_OFF] - x[O_OFF] - x[O_K]) == 0
  // remains sound because strictly increasing offsets force the identity permutation starting at off_0 = 0.
  val INV_DEFENSE_FIRST_OFF_ZERO: bool = {
    mut_no_first_off_zero_only(state.sys) implies tie_is_sound(state.sys)
  }

}
