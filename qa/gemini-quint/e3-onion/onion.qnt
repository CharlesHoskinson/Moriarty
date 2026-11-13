// -*- mode: Bluespec; -*-
// E3 J8 Soundness Model: OnionAir (lane-src/iface_l.rs)
// Reduced-scale formal specification of the BabyBear L_iface log onion AIR.
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

module onion {

  // ---- Constants ---------------------------------------------------------------------------

  pure def TAG_EMPTY: int = 1
  pure def TAG_STEP: int = 2
  pure def TAG_NODE: int = 3
  pure def TAG_ENT: int = 4

  pure def OP_NOP: int = 0
  pure def OP_H2: int = 2
  pure def OP_PUB: int = 7
  pure def OP_DIGEST: int = 8

  pure def W_BLK: int = 4
  pure def LIMBS: int = 2
  pure def STEP_LEN: int = 9
  pure def H_MAX: int = 16

  // ---- Types -------------------------------------------------------------------------------

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

  type EntMsg = {
    op: int,
    param: int,
    off: int,
    k_vals: int
  }

  type ValMsg = {
    idx: int,
    limb: int
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

  // ---- Helpers -----------------------------------------------------------------------------

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

  // Lead fix (2026-11-13): the lane's Cantor pairing overflowed the Rust backend's integers on
  // chained digests. Ideal permutation replaced by a bijection of Z_M^8, M = 1000003 prime:
  // out = V in + c (mod M) with V the Vandermonde matrix on nodes 2..9 (invertible mod M), so
  // every output word mixes every input word and the map is injective on reduced inputs.
  pure val PM: int = 1000003
  pure def vpow(b: int, e: int): int = 0.to(e - 1).fold(1, (acc, _k) => (acc * b) % PM)
  pure def perm8(st_in: List[int]): List[int] = {
    range(0, 8).foldl([], (outs, i) => outs.append(
      (range(0, 8).foldl(0, (acc, jj) =>
        (acc + vpow(i + 2, jj) * (((st_in.nth(jj) % PM) + PM) % PM)) % PM) + i + 1) % PM
    ))
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

  // ---- Native Reference (lane-src/iface.rs) ------------------------------------------------

  pure def p2_block_indices(total_len: int): List[int] = {
    if (total_len <= 4) [0]
    else if (total_len <= 8) [0, 1]
    else [0, 1, 2]
  }

  pure def p2_native(tag: int, xs: List[int]): List[int] = {
    p2_words([tag].concat(xs))
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

  pure def flat_limbs(values: List[LogValueRow], off: int, k_vals: int): List[int] = {
    if (k_vals == 1) [
      values.nth(off).limb0,
      values.nth(off).limb1
    ] else if (k_vals == 2) [
      values.nth(off).limb0,
      values.nth(off).limb1,
      values.nth(off + 1).limb0,
      values.nth(off + 1).limb1
    ] else if (k_vals == 3) [
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

  // ---- Bus Balance & Table Constraints (IfaceAir) ------------------------------------------

  // LogEntry + LogValue + LOG_BUS constraints
  pure def tables_valid(entries: List[LogEntryRow], values: List[LogValueRow]): bool = {
    entries.length() > 0 and
    values.length() > 0 and
    // Entry 0 starts at off = 0
    entries.nth(0).off == 0 and
    // Each entry has valid op, used flag, and k_vals = op_vals(op)
    entries.indices().forall(i => {
      (entries.nth(i).op == OP_NOP and entries.nth(i).used == 0 and entries.nth(i).k_vals == 0) or
      ((entries.nth(i).op == OP_PUB or entries.nth(i).op == OP_DIGEST or entries.nth(i).op == OP_H2) and
       entries.nth(i).used == 1 and
       entries.nth(i).k_vals == op_vals(entries.nth(i).op))
    }) and
    // No used entry after NOP, and running sum off_{i+1} = off_i + k_vals_i
    entries.indices().forall(i =>
      if (i + 1 < entries.length()) (
        ((1 - entries.nth(i).used) * entries.nth(i + 1).used == 0) and
        (entries.nth(i + 1).off == entries.nth(i).off + entries.nth(i).k_vals)
      ) else true
    ) and
    // Value table: boolean used, non-increasing, unused limbs zero, running cnt
    values.nth(0).cnt == values.nth(0).used and
    values.indices().forall(r => {
      is_bit(values.nth(r).used) and
      ((1 - values.nth(r).used) * values.nth(r).limb0 == 0) and
      ((1 - values.nth(r).used) * values.nth(r).limb1 == 0) and
      (if (r + 1 < values.length()) (
        ((1 - values.nth(r).used) * values.nth(r + 1).used == 0) and
        (values.nth(r + 1).cnt == values.nth(r).cnt + values.nth(r + 1).used)
      ) else true)
    }) and
    // LOG_BUS: off_end on last entry row equals cnt on last value row
    (entries.nth(entries.length() - 1).off + entries.nth(entries.length() - 1).k_vals ==
     values.nth(values.length() - 1).cnt)
  }

  // ENT_BUS multiset balance: LogEntry sends with +used, OnionAir receives with -sel_fa
  pure def ent_bus_balanced(entries: List[LogEntryRow], tr: List[OnionRow]): bool = {
    // Every FA row has sel_fa in {0,1}; count of matching messages must equal on both sides
    tr.indices().forall(r =>
      if (tr.nth(r).sel_fa == 1) (
        entries.foldl(0, (acc, e) =>
          acc + (if (
            e.used == 1 and
            e.op == tr.nth(r).op and
            e.param == tr.nth(r).param and
            e.off == tr.nth(r).off and
            e.k_vals == tr.nth(r).k_vals
          ) 1 else 0)
        ) ==
        tr.foldl(0, (acc, r2) =>
          acc + (if (
            r2.sel_fa == 1 and
            r2.op == tr.nth(r).op and
            r2.param == tr.nth(r).param and
            r2.off == tr.nth(r).off and
            r2.k_vals == tr.nth(r).k_vals
          ) 1 else 0)
        )
      ) else true
    ) and
    entries.foldl(0, (acc, e) => acc + (if (e.used == 1) 1 else 0)) ==
    tr.foldl(0, (acc, r) => acc + (if (r.sel_fa == 1) 1 else 0))
  }

  // Multiplicity m on VAL_BUS for row x and slot j in 0..3
  pure def val_mult(x: OnionRow, j: int, fa_skips_header: bool): int = {
    x.sel_a * x.u.nth(j) - (if (fa_skips_header and j < 3) x.sel_fa else 0)
  }

  pure def val_key(x: OnionRow, j: int): int = {
    x.off * LIMBS + x.pos + j - 3
  }

  // VAL_BUS multiset balance:
  // LogValue sends (2*(cnt-1) + t, limb_t) with multiplicity used;
  // OnionAir receives (off*2 + pos + j - 3, w[j]) with multiplicity m(x, j).
  pure def val_bus_balanced_cfg(values: List[LogValueRow], tr: List[OnionRow], fa_skips_header: bool): bool = {
    // 1. No negative multiplicities allowed on OnionAir side, and every active lookup matches a used value limb
    tr.indices().forall(r =>
      Set(0, 1, 2, 3).forall(j => {
        val_mult(tr.nth(r), j, fa_skips_header) >= 0 and
        (if (val_mult(tr.nth(r), j, fa_skips_header) == 1) (
          val_key(tr.nth(r), j) >= 0 and
          (val_key(tr.nth(r), j) / 2) < values.length() and
          values.nth(val_key(tr.nth(r), j) / 2).used == 1 and
          (if (val_key(tr.nth(r), j) % 2 == 0)
            tr.nth(r).w.nth(j) == values.nth(val_key(tr.nth(r), j) / 2).limb0
           else
            tr.nth(r).w.nth(j) == values.nth(val_key(tr.nth(r), j) / 2).limb1)
        ) else true)
      })
    ) and
    // 2. Every used value limb (2*v_idx + t) is looked up exactly once across all (r, j)
    values.indices().forall(v_idx =>
      if (values.nth(v_idx).used == 1) (
        Set(0, 1).forall(t =>
          tr.foldl(0, (acc, row) =>
            acc + [0, 1, 2, 3].foldl(0, (inner, j) =>
              inner + (if (val_mult(row, j, fa_skips_header) == 1 and val_key(row, j) == 2 * v_idx + t) 1 else 0)
            )
          ) == 1
        )
      ) else true
    )
  }

  // TIE_BUS balance: Stmt receives stmt_tie once; OnionAir sends (out[0..4], cnt + (b2 if cnt_has_b2 else 0)) on FIN rows
  pure def tie_bus_balanced_cfg(stmt_tie: TieMsg, tr: List[OnionRow], cnt_has_b2: bool): bool = {
    tr.foldl(0, (acc, r) => acc + (if (r.sel_fin == 1) 1 else 0)) == 1 and
    tr.indices().forall(r =>
      if (tr.nth(r).sel_fin == 1) (
        list_eq(rate_part(tr.nth(r).out), stmt_tie.d) and
        (tr.nth(r).cnt + (if (cnt_has_b2) tr.nth(r).sel_b2 else 0) == stmt_tie.n_used)
      ) else true
    )
  }

  // Poseidon2PermuteBus lookup: on every active row, out == perm8(w ++ cap)
  pure def perm_bus_ok(tr: List[OnionRow]): bool = {
    tr.indices().forall(r =>
      if (row_act(tr.nth(r)) == 1)
        list_eq(tr.nth(r).out, perm8(tr.nth(r).w.concat(tr.nth(r).cap)))
      else true
    )
  }

  // ---- OnionAir Constraint Predicate (lane-src/iface_l.rs lines 538-678) -------------------

  type ConstraintFlags = {
    enforce_off_chain: bool,
    enforce_fa_w0: bool,
    enforce_last_count: bool,
    enforce_fin_no_b1: bool,
    enforce_cap_chain: bool,
    enforce_cnt_b2: bool,
    enforce_d_chain: bool
  }

  pure def REAL_FLAGS: ConstraintFlags = {
    enforce_off_chain: true,
    enforce_fa_w0: true,
    enforce_last_count: true,
    enforce_fin_no_b1: true,
    enforce_cap_chain: true,
    enforce_cnt_b2: true,
    enforce_d_chain: true
  }

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
    // Selector booleans
    is_bit(x.sel_e) and is_bit(x.sel_a) and is_bit(x.sel_fa) and
    is_bit(x.sel_b0) and is_bit(x.sel_b1) and is_bit(x.sel_b2) and
    is_bit(x.sel_last) and is_bit(x.sel_fin) and
    is_bit(x.u.nth(0)) and is_bit(x.u.nth(1)) and is_bit(x.u.nth(2)) and is_bit(x.u.nth(3)) and
    // ax boolean and FA => A
    is_bit(row_act(x)) and
    (x.sel_fa * (1 - x.sel_a) == 0) and
    // FIN exclusion: x[O_FIN] * (x[O_A] + x[O_B0] + x[O_B1]) == 0
    (if (cfg.enforce_fin_no_b1)
      (x.sel_fin * (x.sel_a + x.sel_b0 + x.sel_b1) == 0)
     else
      (x.sel_fin * (x.sel_a + x.sel_b0) == 0)) and
    // Capacities and fixed words on E, FA, B0
    (x.sel_e * (x.w.nth(0) - TAG_EMPTY) == 0) and
    (x.sel_e * (x.cap.nth(0) - 1) == 0) and
    (x.sel_fa * (x.cap.nth(0) - row_total(x)) == 0) and
    (x.sel_b0 * (x.cap.nth(0) - STEP_LEN) == 0) and
    Set(1, 2, 3).forall(m => row_first_blk(x) * x.cap.nth(m) == 0) and
    Set(1, 2, 3).forall(j => x.sel_e * x.w.nth(j) == 0) and
    // A blocks: [tagEnt, op, param, limbs...], zero padded
    (if (cfg.enforce_fa_w0) (x.sel_fa * (x.w.nth(0) - TAG_ENT) == 0) else true) and
    (x.sel_fa * (x.w.nth(1) - x.op) == 0) and
    (x.sel_fa * (x.w.nth(2) - x.param) == 0) and
    (x.sel_fa * x.pos == 0) and
    (x.sel_a * (1 - x.u.nth(0)) == 0) and
    Set(0, 1, 2).forall(j => x.u.nth(j + 1) * (1 - x.u.nth(j)) == 0) and
    Set(0, 1, 2, 3).forall(j => x.sel_a * (1 - x.u.nth(j)) * x.w.nth(j) == 0) and
    (x.sel_a * (1 - x.sel_last) * (W_BLK - row_u_sum(x)) == 0) and
    (if (cfg.enforce_last_count)
      (x.sel_a * x.sel_last * (x.pos + row_u_sum(x) - row_total(x)) == 0)
     else true) and
    // B blocks: [tagStep, d0, d1, d2], [d3, h0, h1, h2], [h3, 0, 0, 0]
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
    (x.sel_fin - row_act(x) * (1 - row_act(y)) == 0) and
    (if (cfg.enforce_cnt_b2)
      (y.cnt - x.cnt - x.sel_b2 == 0)
     else
      (y.cnt - x.cnt == 0)) and
    // Capacity chaining on continuation blocks (cont_y)
    (if (cfg.enforce_cap_chain)
      Set(0, 1, 2, 3).forall(m => row_cont(y) * (y.cap.nth(m) - x.out.nth(4 + m)) == 0)
     else true) and
    // POS step by W_BLK = 4 on continuation A rows
    ((y.sel_a - y.sel_fa) * (y.pos - x.pos - W_BLK) == 0) and
    // Carried entry fields across same-entry rows
    (row_same_entry(y) * (y.op - x.op) == 0) and
    (row_same_entry(y) * (y.param - x.param) == 0) and
    (row_same_entry(y) * (y.off - x.off) == 0) and
    (row_same_entry(y) * (y.k_vals - x.k_vals) == 0) and
    // Chaining d and h
    Set(0, 1, 2, 3).forall(m =>
      (row_same_entry(y) * (y.d.nth(m) - x.d.nth(m)) == 0) and
      (if (cfg.enforce_d_chain) (y.sel_fa * (y.d.nth(m) - x.out.nth(m)) == 0) else true) and
      (y.sel_b0 * (y.h.nth(m) - x.out.nth(m)) == 0) and
      ((y.sel_b1 + y.sel_b2) * (y.h.nth(m) - x.h.nth(m)) == 0)
    ) and
    // OFF chain across entries
    (if (cfg.enforce_off_chain) (
      (x.sel_e * y.off == 0) and
      (x.sel_b2 * y.sel_fa * (y.off - x.off - x.k_vals) == 0)
    ) else true)
  }

  // Full system check parameterized by ConstraintFlags
  pure def system_ok_cfg(sys: SystemInstance, cfg: ConstraintFlags): bool = {
    tables_valid(sys.entries, sys.values) and
    sys.trace.length() >= 1 and
    // First row constraints
    (sys.trace.nth(0).sel_e == 1) and
    (sys.trace.nth(0).cnt == 0) and
    // Last row constraint
    (sys.trace.nth(sys.trace.length() - 1).sel_fin - row_act(sys.trace.nth(sys.trace.length() - 1)) == 0) and
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

  // Soundness check: does the statement tie match the native onion of the tables?
  pure def tie_is_sound(sys: SystemInstance): bool = {
    list_eq(sys.stmt_tie.d, native_onion(sys.entries, sys.values).d) and
    sys.stmt_tie.n_used == native_onion(sys.entries, sys.values).n_used
  }

  // ---- Mutants (each relaxes one constraint family) ----------------------------------------

  // Mutant 1: omits OFF chain in OnionAir (allows swapping two entries)
  pure def mut_no_off_chain(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_off_chain: false })
  }

  // Mutant 2: omits FA w[0] == TAG_ENT check (since VAL_BUS skips j < 3 on FA, w[0] would be free)
  pure def mut_fa_reads_w0(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_fa_w0: false })
  }

  // Mutant 3: omits last-block count pos + c == total on LAST A-rows
  pure def mut_no_last_count(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_last_count: false })
  }

  // Mutant 4: allows FIN = 1 on B1 row and transition from B1 to inactive
  pure def mut_fin_on_b1(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_fin_no_b1: false })
  }

  // Mutant 5: omits capacity chaining from previous out[4..8] on continuation blocks
  pure def mut_cap_not_chained(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_cap_chain: false })
  }

  // Mutant 6: omits B2 increment in CNT transition and TIE_BUS n_used expression
  pure def mut_cnt_no_b2(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_cnt_b2: false })
  }

  // Mutant 7: omits chaining of d from previous E/B2 output onto FA row
  pure def mut_d_not_chained(sys: SystemInstance): bool = {
    system_ok_cfg(sys, { ...REAL_FLAGS, enforce_d_chain: false })
  }

  // ---- Honest Trace Builders (lane-src/iface_l.rs OnionAir::trace) -------------------------

  pure def zero4: List[int] = [0, 0, 0, 0]

  pure def make_e_row(fin_flag: int): OnionRow = {
    {
      w: [TAG_EMPTY, 0, 0, 0],
      cap: [1, 0, 0, 0],
      out: perm8([TAG_EMPTY, 0, 0, 0, 1, 0, 0, 0]),
      d: zero4,
      h: zero4,
      u: zero4,
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
      w: zero4,
      cap: zero4,
      out: [0, 0, 0, 0, 0, 0, 0, 0],
      d: zero4,
      h: zero4,
      u: zero4,
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

  pure def u_mask_for_len(rem_len: int): List[int] = {
    [
      if (rem_len >= 1) 1 else 0,
      if (rem_len >= 2) 1 else 0,
      if (rem_len >= 3) 1 else 0,
      if (rem_len >= 4) 1 else 0
    ]
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
      h: zero4,
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
    // B0: [TAG_STEP, d0, d1, d2], cap = [9, 0, 0, 0]
    [
      {
        w: [TAG_STEP, d_cur.nth(0), d_cur.nth(1), d_cur.nth(2)],
        cap: [STEP_LEN, 0, 0, 0],
        out: perm8([TAG_STEP, d_cur.nth(0), d_cur.nth(1), d_cur.nth(2), STEP_LEN, 0, 0, 0]),
        d: d_cur,
        h: h_cur,
        u: zero4,
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
      // B1: [d3, h0, h1, h2], cap = B0.out[4..8]
      {
        w: [d_cur.nth(3), h_cur.nth(0), h_cur.nth(1), h_cur.nth(2)],
        cap: cap_part(perm8([TAG_STEP, d_cur.nth(0), d_cur.nth(1), d_cur.nth(2), STEP_LEN, 0, 0, 0])),
        out: perm8(
          [d_cur.nth(3), h_cur.nth(0), h_cur.nth(1), h_cur.nth(2)].concat(
            cap_part(perm8([TAG_STEP, d_cur.nth(0), d_cur.nth(1), d_cur.nth(2), STEP_LEN, 0, 0, 0]))
          )
        ),
        d: d_cur,
        h: h_cur,
        u: zero4,
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
      // B2: [h3, 0, 0, 0], cap = B1.out[4..8]
      {
        w: [h_cur.nth(3), 0, 0, 0],
        cap: cap_part(
          perm8(
            [d_cur.nth(3), h_cur.nth(0), h_cur.nth(1), h_cur.nth(2)].concat(
              cap_part(perm8([TAG_STEP, d_cur.nth(0), d_cur.nth(1), d_cur.nth(2), STEP_LEN, 0, 0, 0]))
            )
          )
        ),
        out: perm8(
          [h_cur.nth(3), 0, 0, 0].concat(
            cap_part(
              perm8(
                [d_cur.nth(3), h_cur.nth(0), h_cur.nth(1), h_cur.nth(2)].concat(
                  cap_part(perm8([TAG_STEP, d_cur.nth(0), d_cur.nth(1), d_cur.nth(2), STEP_LEN, 0, 0, 0]))
                )
              )
            )
          )
        ),
        d: d_cur,
        h: h_cur,
        u: zero4,
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

  // Honest rows for a single entry (A blocks + B0, B1, B2)
  pure def honest_entry_rows(
    e: LogEntryRow,
    values: List[LogValueRow],
    d_cur: List[int],
    cnt_in: int,
    is_final_entry: int
  ): List[OnionRow] = {
    if (e.k_vals == 1) (
      // total = 5: 2 A-blocks (blk0 len 4, blk1 len 1)
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
          cap_part(perm8([TAG_ENT, e.op, e.param, values.nth(e.off).limb0, 5, 0, 0, 0])),
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
    ) else if (e.k_vals == 2) (
      // total = 7: 2 A-blocks (blk0 len 4, blk1 len 3)
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
          cap_part(perm8([TAG_ENT, e.op, e.param, values.nth(e.off).limb0, 7, 0, 0, 0])),
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
    ) else (
      // k_vals == 3, total = 9: 3 A-blocks (blk0 len 4, blk1 len 4, blk2 len 1)
      [
        make_a_row(
          [TAG_ENT, e.op, e.param, values.nth(e.off).limb0],
          [9, 0, 0, 0],
          d_cur,
          [1, 1, 1, 1],
          1, 0, 0, cnt_in, e
        ),
        make_a_row(
          [values.nth(e.off).limb1, values.nth(e.off + 1).limb0, values.nth(e.off + 1).limb1, values.nth(e.off + 2).limb0],
          cap_part(perm8([TAG_ENT, e.op, e.param, values.nth(e.off).limb0, 9, 0, 0, 0])),
          d_cur,
          [1, 1, 1, 1],
          0, 0, 4, cnt_in, e
        ),
        make_a_row(
          [values.nth(e.off + 2).limb1, 0, 0, 0],
          cap_part(
            perm8(
              [values.nth(e.off).limb1, values.nth(e.off + 1).limb0, values.nth(e.off + 1).limb1, values.nth(e.off + 2).limb0].concat(
                cap_part(perm8([TAG_ENT, e.op, e.param, values.nth(e.off).limb0, 9, 0, 0, 0]))
              )
            )
          ),
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
    )
  }

  // Honest full system generator (with inactive padding rows at the end)
  pure def build_honest_system(entries: List[LogEntryRow], values: List[LogValueRow]): SystemInstance = {
    if (entries.nth(0).used == 0) {
      // Empty log: E row has FIN = 1, followed by inactive padding rows with cnt = 0
      entries: entries,
      values: values,
      stmt_tie: native_onion(entries, values),
      trace: [
        make_e_row(1),
        make_pad_row(0),
        make_pad_row(0)
      ]
    } else if (entries.length() == 1 or entries.nth(1).used == 0) {
      // 1 used entry + padding
      entries: entries,
      values: values,
      stmt_tie: native_onion(entries, values),
      trace: [make_e_row(0)]
        .concat(honest_entry_rows(entries.nth(0), values, d_empty_native, 0, 1))
        .concat([make_pad_row(1), make_pad_row(1)])
    } else {
      // 2 used entries + padding
      entries: entries,
      values: values,
      stmt_tie: native_onion(entries, values),
      trace: [make_e_row(0)]
        .concat(honest_entry_rows(entries.nth(0), values, d_empty_native, 0, 0))
        .concat(
          honest_entry_rows(
            entries.nth(1),
            values,
            p2_native(
              TAG_STEP,
              d_empty_native.concat(p2_native(TAG_ENT, entry_enc(entries.nth(0), values)))
            ),
            1,
            1
          )
        )
        .concat([make_pad_row(2)])
    }
  }

  // ---- Representative Test Logs & Mutant Counterexamples -----------------------------------

  // Sample value table (4 value slots)
  pure def sample_values_0: List[LogValueRow] = [
    { limb0: 0, limb1: 0, used: 0, cnt: 0 },
    { limb0: 0, limb1: 0, used: 0, cnt: 0 },
    { limb0: 0, limb1: 0, used: 0, cnt: 0 },
    { limb0: 0, limb1: 0, used: 0, cnt: 0 }
  ]

  pure def sample_values_1: List[LogValueRow] = [
    { limb0: 11, limb1: 12, used: 1, cnt: 1 },
    { limb0: 0,  limb1: 0,  used: 0, cnt: 1 },
    { limb0: 0,  limb1: 0,  used: 0, cnt: 1 },
    { limb0: 0,  limb1: 0,  used: 0, cnt: 1 }
  ]

  pure def sample_values_2: List[LogValueRow] = [
    { limb0: 11, limb1: 12, used: 1, cnt: 1 },
    { limb0: 21, limb1: 22, used: 1, cnt: 2 },
    { limb0: 0,  limb1: 0,  used: 0, cnt: 2 },
    { limb0: 0,  limb1: 0,  used: 0, cnt: 2 }
  ]

  pure def sample_values_3: List[LogValueRow] = [
    { limb0: 11, limb1: 12, used: 1, cnt: 1 },
    { limb0: 21, limb1: 22, used: 1, cnt: 2 },
    { limb0: 31, limb1: 32, used: 1, cnt: 3 },
    { limb0: 0,  limb1: 0,  used: 0, cnt: 3 }
  ]

  // Honest instance 0: Empty log (n = 0, FIN on E row)
  pure def honest_empty_sys: SystemInstance = {
    build_honest_system(
      [
        { op: OP_NOP, param: 0, off: 0, k_vals: 0, used: 0 },
        { op: OP_NOP, param: 0, off: 0, k_vals: 0, used: 0 }
      ],
      sample_values_0
    )
  }

  // Honest instance 1: Single PUB entry (k = 1, 2 limbs, 2 A-blocks)
  pure def honest_pub_sys: SystemInstance = {
    build_honest_system(
      [
        { op: OP_PUB, param: 3, off: 0, k_vals: 1, used: 1 },
        { op: OP_NOP, param: 0, off: 1, k_vals: 0, used: 0 }
      ],
      sample_values_1
    )
  }

  // Honest instance 2: Single DIGEST entry (k = 2, 4 limbs, 2 A-blocks)
  pure def honest_digest_sys: SystemInstance = {
    build_honest_system(
      [
        { op: OP_DIGEST, param: 2, off: 0, k_vals: 2, used: 1 },
        { op: OP_NOP,    param: 0, off: 2, k_vals: 0, used: 0 }
      ],
      sample_values_2
    )
  }

  // Honest instance 3: Single H2 entry (k = 3, 6 limbs, 3 A-blocks including middle A row)
  pure def honest_h2_sys: SystemInstance = {
    build_honest_system(
      [
        { op: OP_H2,  param: 0, off: 0, k_vals: 3, used: 1 },
        { op: OP_NOP, param: 0, off: 3, k_vals: 0, used: 0 }
      ],
      sample_values_3
    )
  }

  // Honest instance 4: Two entries (PUB then DIGEST, total 3 values = 6 limbs)
  pure def honest_two_entries_sys: SystemInstance = {
    build_honest_system(
      [
        { op: OP_PUB,    param: 1, off: 0, k_vals: 1, used: 1 },
        { op: OP_DIGEST, param: 4, off: 1, k_vals: 2, used: 1 }
      ],
      sample_values_3
    )
  }

  // ---- Concrete Counterexamples for the 7 Mutants ------------------------------------------

  // Counterexample 1 (catches mut_no_off_chain):
  // Absorbs entry 1 (off=1, k=2) BEFORE entry 0 (off=0, k=1).
  // ENT_BUS and VAL_BUS both balance, but the onion order is swapped!
  pure def cex_no_off_chain: SystemInstance = {
    entries: honest_two_entries_sys.entries,
    values: sample_values_3,
    stmt_tie: {
      d: p2_native(
        TAG_STEP,
        p2_native(
          TAG_STEP,
          d_empty_native.concat(p2_native(TAG_ENT, entry_enc(honest_two_entries_sys.entries.nth(1), sample_values_3)))
        ).concat(p2_native(TAG_ENT, entry_enc(honest_two_entries_sys.entries.nth(0), sample_values_3)))
      ),
      n_used: 2
    },
    trace: [make_e_row(0)]
      .concat(honest_entry_rows(honest_two_entries_sys.entries.nth(1), sample_values_3, d_empty_native, 0, 0))
      .concat(
        honest_entry_rows(
          honest_two_entries_sys.entries.nth(0),
          sample_values_3,
          p2_native(
            TAG_STEP,
            d_empty_native.concat(p2_native(TAG_ENT, entry_enc(honest_two_entries_sys.entries.nth(1), sample_values_3)))
          ),
          1,
          1
        )
      )
      .concat([make_pad_row(2)])
  }

  // Counterexample 2 (catches mut_fa_reads_w0):
  // Replaces w[0] on the FA row with 99 (instead of TAG_ENT = 4).
  // Since VAL_BUS skips j < 3 on FA, VAL_BUS still balances, forging h and log_d.
  pure def cex_fa_reads_w0: SystemInstance = {
    entries: honest_pub_sys.entries,
    values: sample_values_1,
    stmt_tie: {
      d: p2_native(
        TAG_STEP,
        d_empty_native.concat(p2_words([99, OP_PUB, 3, 11, 12]))
      ),
      n_used: 1
    },
    trace: [
      make_e_row(0),
      make_a_row(
        [99, OP_PUB, 3, 11],
        [5, 0, 0, 0],
        d_empty_native,
        [1, 1, 1, 1],
        1, 0, 0, 0, honest_pub_sys.entries.nth(0)
      ),
      make_a_row(
        [12, 0, 0, 0],
        cap_part(perm8([99, OP_PUB, 3, 11, 5, 0, 0, 0])),
        d_empty_native,
        [1, 0, 0, 0],
        0, 1, 4, 0, honest_pub_sys.entries.nth(0)
      )
    ].concat(
      make_b_rows(
        d_empty_native,
        p2_words([99, OP_PUB, 3, 11, 12]),
        0,
        honest_pub_sys.entries.nth(0),
        1
      )
    ).concat([make_pad_row(1)])
  }

  // Counterexample 3 (catches mut_no_last_count):
  // Two PUB entries: e0 (off=0, k=1, total=5) and e1 (off=1, k=1, total=5).
  // Without pos + c == total on LAST A-rows:
  // - e0's second A-row (pos=4, last=1) sets u=[1,1,1,0] (c=3, pos+c=7!=5), stealing e1's limbs (g=2,3)!
  // - e1 has a single A-row (FA=1, last=1, pos=0) with u=[1,1,1,0] (c=3, pos+c=3!=5), which queries
  //   0 limbs on VAL_BUS because j < 3 is skipped on FA and u[3] == 0!
  // VAL_BUS and ENT_BUS are 100% balanced, yet both entries' hashes and log_d are forged!
  pure def two_pub_entries: List[LogEntryRow] = [
    { op: OP_PUB, param: 1, off: 0, k_vals: 1, used: 1 },
    { op: OP_PUB, param: 2, off: 1, k_vals: 1, used: 1 }
  ]

  pure def forged_h0_no_last: List[int] = {
    rate_part(
      perm8(
        [12, 21, 22, 0].concat(
          cap_part(perm8([TAG_ENT, OP_PUB, 1, 11, 5, 0, 0, 0]))
        )
      )
    )
  }

  pure def forged_d1_no_last: List[int] = {
    p2_native(TAG_STEP, d_empty_native.concat(forged_h0_no_last))
  }

  pure def forged_h1_no_last: List[int] = {
    rate_part(perm8([TAG_ENT, OP_PUB, 2, 0, 5, 0, 0, 0]))
  }

  pure def cex_no_last_count: SystemInstance = {
    entries: two_pub_entries,
    values: sample_values_2,
    stmt_tie: {
      d: p2_native(TAG_STEP, forged_d1_no_last.concat(forged_h1_no_last)),
      n_used: 2
    },
    trace: [
      make_e_row(0),
      // Entry 0: FA (pos=0, c=4) + LAST (pos=4, c=3 != 1, reads g=1,2,3)
      make_a_row(
        [TAG_ENT, OP_PUB, 1, 11],
        [5, 0, 0, 0],
        d_empty_native,
        [1, 1, 1, 1],
        1, 0, 0, 0, two_pub_entries.nth(0)
      ),
      make_a_row(
        [12, 21, 22, 0],
        cap_part(perm8([TAG_ENT, OP_PUB, 1, 11, 5, 0, 0, 0])),
        d_empty_native,
        [1, 1, 1, 0],
        0, 1, 4, 0, two_pub_entries.nth(0)
      )
    ].concat(
      make_b_rows(d_empty_native, forged_h0_no_last, 0, two_pub_entries.nth(0), 0)
    ).concat([
      // Entry 1: single row with FA=1, LAST=1, pos=0, u=[1,1,1,0] (c=3 != 5, reads 0 limbs!)
      make_a_row(
        [TAG_ENT, OP_PUB, 2, 0],
        [5, 0, 0, 0],
        forged_d1_no_last,
        [1, 1, 1, 0],
        1, 1, 0, 1, two_pub_entries.nth(1)
      )
    ]).concat(
      make_b_rows(forged_d1_no_last, forged_h1_no_last, 1, two_pub_entries.nth(1), 1)
    ).concat([make_pad_row(2)])
  }

  // Counterexample 4 (catches mut_fin_on_b1):
  // Sets FIN = 1 on B1 of entry 0 and transitions directly to an inactive row (dropping B2).
  pure def cex_fin_on_b1: SystemInstance = {
    entries: honest_pub_sys.entries,
    values: sample_values_1,
    stmt_tie: {
      d: rate_part(honest_pub_sys.trace.nth(4).out),
      n_used: 0
    },
    trace: [
      honest_pub_sys.trace.nth(0),
      honest_pub_sys.trace.nth(1),
      honest_pub_sys.trace.nth(2),
      honest_pub_sys.trace.nth(3),
      { ...honest_pub_sys.trace.nth(4), sel_fin: 1 },
      make_pad_row(0)
    ]
  }

  // Counterexample 5 (catches mut_cap_not_chained):
  // Sets cap = [0, 0, 0, 0] on the second A-row of entry 0 instead of chaining FA.out[4..8].
  pure def forged_h_cap_zero: List[int] = {
    rate_part(perm8([12, 0, 0, 0, 0, 0, 0, 0]))
  }

  pure def cex_cap_not_chained: SystemInstance = {
    entries: honest_pub_sys.entries,
    values: sample_values_1,
    stmt_tie: {
      d: p2_native(TAG_STEP, d_empty_native.concat(forged_h_cap_zero)),
      n_used: 1
    },
    trace: [
      make_e_row(0),
      honest_pub_sys.trace.nth(1),
      make_a_row(
        [12, 0, 0, 0],
        zero4,
        d_empty_native,
        [1, 0, 0, 0],
        0, 1, 4, 0, honest_pub_sys.entries.nth(0)
      )
    ].concat(
      make_b_rows(d_empty_native, forged_h_cap_zero, 0, honest_pub_sys.entries.nth(0), 1)
    ).concat([make_pad_row(1)])
  }

  // Counterexample 6 (catches mut_cnt_no_b2):
  // Keeps cnt = 0 across B2 -> pad and reports n_used = 0 on TIE_BUS for a 1-entry log.
  pure def cex_cnt_no_b2: SystemInstance = {
    entries: honest_pub_sys.entries,
    values: sample_values_1,
    stmt_tie: {
      d: honest_pub_sys.stmt_tie.d,
      n_used: 0
    },
    trace: [
      honest_pub_sys.trace.nth(0),
      honest_pub_sys.trace.nth(1),
      honest_pub_sys.trace.nth(2),
      honest_pub_sys.trace.nth(3),
      honest_pub_sys.trace.nth(4),
      honest_pub_sys.trace.nth(5),
      make_pad_row(0),
      make_pad_row(0)
    ]
  }

  // Counterexample 7 (catches mut_d_not_chained):
  // Uses d = [0, 0, 0, 0] on entry 0 instead of D_EMPTY from the E row.
  pure def cex_d_not_chained: SystemInstance = {
    entries: honest_pub_sys.entries,
    values: sample_values_1,
    stmt_tie: {
      d: p2_native(
        TAG_STEP,
        zero4.concat(p2_native(TAG_ENT, entry_enc(honest_pub_sys.entries.nth(0), sample_values_1)))
      ),
      n_used: 1
    },
    trace: [make_e_row(0)]
      .concat(honest_entry_rows(honest_pub_sys.entries.nth(0), sample_values_1, zero4, 0, 1))
      .concat([make_pad_row(1)])
  }

  // ---- State Machine & Invariants for `quint run` ------------------------------------------

  var cur_sys: SystemInstance
  var cur_is_honest: bool

  action init = {
    nondet pick = oneOf(Set(0, 1, 2, 3, 4))
    all {
      cur_is_honest' = true,
      cur_sys' =
        if (pick == 0) honest_empty_sys
        else if (pick == 1) honest_pub_sys
        else if (pick == 2) honest_digest_sys
        else if (pick == 3) honest_h2_sys
        else honest_two_entries_sys
    }
  }

  action step = {
    nondet pick = oneOf(Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11))
    all {
      cur_is_honest' = (pick <= 4),
      cur_sys' =
        if (pick == 0) honest_empty_sys
        else if (pick == 1) honest_pub_sys
        else if (pick == 2) honest_digest_sys
        else if (pick == 3) honest_h2_sys
        else if (pick == 4) honest_two_entries_sys
        else if (pick == 5) cex_no_off_chain
        else if (pick == 6) cex_fa_reads_w0
        else if (pick == 7) cex_no_last_count
        else if (pick == 8) cex_fin_on_b1
        else if (pick == 9) cex_cap_not_chained
        else if (pick == 10) cex_cnt_no_b2
        else cex_d_not_chained
    }
  }

  // Invariant 1 (SOUND): Every trace that satisfies all real constraints and bus balances
  // yields a tie equal to the native onion of the table and n = used entries.
  def SOUND: bool = {
    system_ok(cur_sys) implies tie_is_sound(cur_sys)
  }

  // Invariant 2 (COMPLETE): Every honest trace satisfies all real constraints, bus balances,
  // and matches the native onion.
  def COMPLETE: bool = {
    cur_is_honest implies (system_ok(cur_sys) and tie_is_sound(cur_sys))
  }

  def SOUND_AND_COMPLETE: bool = {
    SOUND and COMPLETE
  }

  // Mutant invariants (each FAILS on its corresponding mutant when `quint run` samples that cex)
  def INV_MUT_NO_OFF_CHAIN: bool = {
    mut_no_off_chain(cur_sys) implies tie_is_sound(cur_sys)
  }

  def INV_MUT_FA_READS_W0: bool = {
    mut_fa_reads_w0(cur_sys) implies tie_is_sound(cur_sys)
  }

  def INV_MUT_NO_LAST_COUNT: bool = {
    mut_no_last_count(cur_sys) implies tie_is_sound(cur_sys)
  }

  def INV_MUT_FIN_ON_B1: bool = {
    mut_fin_on_b1(cur_sys) implies tie_is_sound(cur_sys)
  }

  def INV_MUT_CAP_NOT_CHAINED: bool = {
    mut_cap_not_chained(cur_sys) implies tie_is_sound(cur_sys)
  }

  def INV_MUT_CNT_NO_B2: bool = {
    mut_cnt_no_b2(cur_sys) implies tie_is_sound(cur_sys)
  }

  def INV_MUT_D_NOT_CHAINED: bool = {
    mut_d_not_chained(cur_sys) implies tie_is_sound(cur_sys)
  }


  // ---- Lead extension (2026-11-13): random one-cell tampering of an honest instance ---------
  // The adversary changes one value limb, one tie word, or one scalar or selector cell of one
  // trace row by delta in {-2, -1, 1, 2}, and then claims the tie that the (tampered) trace's FIN
  // rows send when there is one FIN row (else the old tie). TAMPER_SOUND: system_ok implies the
  // tie equals the native onion of the (tampered) tables.
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
  def TAMPER_SOUND: bool = system_ok(cur_sys) implies tie_is_sound(cur_sys)
  action step_tamper = {
    nondet pick = oneOf(Set(0, 1, 2, 3, 4))
    nondet kind = oneOf(Set(0, 1, 2, 3, 3, 3))
    nondet ri = oneOf(0.to(40))
    nondet f = oneOf(0.to(14))
    nondet dl = oneOf(Set(-2, -1, 1, 2))
    val base =
      if (pick == 0) honest_empty_sys
      else if (pick == 1) honest_pub_sys
      else if (pick == 2) honest_digest_sys
      else if (pick == 3) honest_h2_sys
      else honest_two_entries_sys
    all {
      cur_is_honest' = false,
      cur_sys' = tamper(base, kind, ri, f, dl)
    }
  }
}
