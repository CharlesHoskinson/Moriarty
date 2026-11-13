// -*- mode: Bluespec; -*-
//
// Round 3 soundness model of the Sherlock E5 wrap in-circuit D-9 decoder
// (`lane-src/decode.rs`) vs stock reference (`lane-src/iface.rs`),
// including Step 7: native replay of every entry via a P-slot permutation tape
// tied to the entries by a fixed-multiplicity lookup.

module decode {

  // =========================================================================
  // 1. Reduced-Scale Parameters
  // =========================================================================

  pure val FR_MOD: int = 359
  pure val R1_LIMBS: List[int] = [6, 4, 5]

  pure val WORD_BASE: int = 18
  pure val PACK_K: int = 2
  pure val BB_P: int = 17

  pure val LIMBS: int = 3
  pure val LIMB_BASE: int = 8

  pure val MODE_WORD: int = 0
  pure val J_WORDS: int = 1
  pure val K_WORDS: int = 3
  pure val E_IN_WORD: int = 5
  pure val E_OUT_WORD: int = 6
  pure val LANE_MAP: int = 7
  pure val LANE_PAIRS: int = 3
  pure val STMT: int = 13

  // Capacities: B = 8 entries, V = 18 values, P = 8 permutation slots, 4 lanes
  pure val B_CAP: int = 8
  pure val V_CAP: int = 18
  pure val P_CAP: int = 8
  pure val LANES: int = 4

  pure val VALUES_AT: int = 29   // STMT + 2 * B_CAP = 13 + 16
  pure val CALL_AT: int = 83     // VALUES_AT + LIMBS * V_CAP = 29 + 54
  pure val N_WORDS: int = 95     // CALL_AT + LIMBS * LANES = 83 + 12
  pure val N_PUBLIC: int = 49    // 1 + ceil(95 / 2) = 1 + 48

  pure val PRE_HASH: int = 42

  pure val SHK_PROGC: int = 0
  pure val SHK_LOCK: int = 1
  pure val SHK_INPUT: int = 2
  pure val SHK_OUTCOME: int = 3
  pure val SHK_REG: int = 4
  pure val SHK_SET: int = 5
  pure val N_TAGS: int = 6
  pure val TAG_FR_TABLE: List[int] = [101, 113, 127, 139, 151, 163]

  pure val R_RUN: List[int] = [0, 2, 3]
  pure val R_OFF: List[int] = [0, 2, 4, 6, 10, 12]
  pure val ANCHOR_IN: int = 4

  pure val PATH_MAX: int = 4
  pure val PERM_VALS: int = 6

  pure val BOUND_LIM: int = 32
  pure val U16_LIM: int = 8
  pure val U32_LIM: int = 64
  pure val SPONGE_Q_LIM: int = 16
  pure val CAP_LIM: int = 32

  // Step 7 packed-tuple shift multipliers and slot range bounds (`decode.rs:188-191, 566-585`)
  pure val T_PARAM_SH: int = 32          // 2^5
  pure val T_O_SH: int = 8192            // 2^13
  pure val T_POFF_SH: int = 4194304      // 2^22
  pure val T_PERMS_SH: int = 2147483648  // 2^31
  pure val SLOT_PARAM_LIM: int = 256     // 2^8
  pure val SLOT_FIELD_LIM: int = 512     // 2^9

  // Fixed capacities `cap_fixed(0, 2)` and `cap_fixed(4, 2)` (`decode.rs:542`)
  pure val CAP_0_2: int = 2
  pure val CAP_4_2: int = 202

  pure val OP_NOP: int = 0
  pure val OP_PERM: int = 1
  pure val OP_H2: int = 2
  pure val OP_SPONGE: int = 3
  pure val OP_PATH: int = 4
  pure val OP_ECMUL: int = 5
  pure val OP_ECMULG: int = 6
  pure val OP_PUB: int = 7
  pure val OP_DIGEST: int = 8

  pure val ADMITTED: List[int] = [
    OP_NOP, OP_PERM, OP_H2, OP_SPONGE, OP_PATH, OP_PUB, OP_DIGEST
  ]

  pure val I_NOP: int = 0
  pure val I_PERM: int = 1
  pure val I_H2: int = 2
  pure val I_SPONGE: int = 3
  pure val I_PATH: int = 4
  pure val I_PUB: int = 5
  pure val I_DIGEST: int = 6

  // =========================================================================
  // 2. Types
  // =========================================================================

  type DecodeRes = {
    ok: bool,
    stmt: List[int],
    err: str
  }

  type PermRow = {
    in0: int,
    in1: int,
    in2: int,
    out0: int,
    out1: int,
    out2: int
  }

  type SlotWit = {
    act: int,
    bits: List[int],
    param: int,
    o: int,
    poff: int,
    prm: int,
    tm_inv: int,
    rem_inv: int,
    ys: List[int],
    tb: List[int]
  }

  type Witness = {
    w_words: List[int],
    lane_borrows: List[List[int]],
    vs_borrows: List[List[int]],
    op_bits: List[List[int]],
    sponge_q: List[int],
    sponge_e: List[int],
    g_bits: List[int],
    j1_inv: int,
    kj_inv: int,
    lm_inv: List[int],
    lm_h: List[List[int]],
    lm_e: List[List[int]],
    lm_diff_inv: List[List[int]],
    lm_val: List[int],
    pub_v: List[int],
    slots: List[SlotWit],
    slot_mult: List[int],
    mv: List[int],
    ms: List[int]
  }

  type Flags = {
    free_mult: bool,
    check_m_range: bool,
    check_inactive_zero: bool,
    use_has1: bool,
    use_chain: bool,
    use_first_cap: bool,
    gate_y1_in1: bool
  }

  type Query = {
    q_gate: int,
    q_key: int,
    q_v: int
  }

  type TableRow = {
    t_key: int,
    t_v: int,
    t_mult: int
  }

  pure val DEFAULT_FLAGS: Flags = {
    free_mult: false,
    check_m_range: true,
    check_inactive_zero: true,
    use_has1: true,
    use_chain: true,
    use_first_cap: true,
    gate_y1_in1: true
  }

  // =========================================================================
  // 3. Field, Uninterpreted 3-Tuple Permutation Table, and Helpers
  // =========================================================================

  pure def f_mod(x: int): int = {
    val r = x % FR_MOD
    if (r < 0) r + FR_MOD else r
  }

  pure def f_add(a: int, b: int): int = f_mod(a + b)
  pure def f_sub(a: int, b: int): int = f_mod(a - b)
  pure def f_mul(a: int, b: int): int = f_mod(a * b)

  pure def is_fr(x: int): bool = (x >= 0 and x < FR_MOD)
  pure def is_bit(b: int): bool = (b == 0 or b == 1)
  pure def in_bound(x: int, lim: int): bool = (x >= 0 and x < FR_MOD and x < lim)

  pure def f_sum_list(xs: List[int]): int =
    xs.foldl(0, (acc, x) => f_add(acc, x))

  pure def f_dot(xs: List[int], ys: List[int]): int =
    range(0, xs.length()).foldl(0, (acc, i) =>
      f_add(acc, f_mul(xs.nth(i), ys.nth(i)))
    )

  pure def pow_mod(b: int, e: int): int =
    range(0, 9).foldl({ acc: 1, sq: f_mod(b) }, (st, i) => {
      val bit = (e / (2 ^ i)) % 2
      { acc: if (bit == 1) f_mul(st.acc, st.sq) else st.acc, sq: f_mul(st.sq, st.sq) }
    }).acc

  pure def mod_inv(u: int): int = {
    val x = f_mod(u)
    if (x == 0) 0 else pow_mod(x, FR_MOD - 2)
  }

  pure def tag_fr(k: int): int =
    if (k >= 0 and k < N_TAGS) TAG_FR_TABLE.nth(k) else 0

  pure def cap_fixed_fn(u_code: int, len_v: int): int =
    if (u_code == 0) len_v
    else if (u_code == 4 and len_v == 2) CAP_4_2
    else f_mod(len_v + 50 * u_code)

  // Uninterpreted injective 3-tuple permutation table `perm([in0, in1, in2]) -> [out0, out1, out2]`.
  // All rows have pairwise distinct inputs and pairwise distinct outputs.
  pure val PERM_TABLE: List[PermRow] = [
    // Inactive / PATH slot input [0, 0, 0]
    { in0: 0, in1: 0, in2: 0, out0: 100, out1: 101, out2: 102 },
    // Run DIGESTs: SHK_PROGC(0), SHK_INPUT(2), SHK_OUTCOME(3)
    { in0: 101, in1: 10, in2: 202, out0: 21, out1: 111, out2: 112 },
    { in0: 127, in1: 11, in2: 202, out0: 22, out1: 113, out2: 114 },
    { in0: 139, in1: 12, in2: 202, out0: 23, out1: 115, out2: 116 },
    // Run SPONGE(3) over [21, 22, 23]: block 0 and block 1
    { in0: 21, in1: 22, in2: 3, out0: 40, out1: 41, out2: 42 },
    { in0: 63, in1: 41, in2: 42, out0: 50, out1: 117, out2: 118 },
    // Anchor DIGESTs under SHK_PROGC (101) and SHK_LOCK (113)
    { in0: 101, in1: 14, in2: 202, out0: 24, out1: 119, out2: 120 },
    { in0: 101, in1: 42, in2: 202, out0: 0, out1: 121, out2: 122 },
    { in0: 113, in1: 10, in2: 202, out0: 31, out1: 123, out2: 124 },
    { in0: 113, in1: 14, in2: 202, out0: 34, out1: 125, out2: 126 },
    // Discretionary DIGESTs under SHK_REG (151) and SHK_SET (163)
    { in0: 151, in1: 15, in2: 202, out0: 45, out1: 127, out2: 128 },
    { in0: 163, in1: 16, in2: 202, out0: 46, out1: 129, out2: 130 },
    // Discretionary PERM([5, 6, 7]) -> [65, 66, 67]
    { in0: 5, in1: 6, in2: 7, out0: 65, out1: 66, out2: 67 },
    // Discretionary H2(7, 8) / SPONGE(2, [7, 8]) -> 60
    { in0: 7, in1: 8, in2: 2, out0: 60, out1: 131, out2: 132 },
    // Discretionary SPONGE(1, [9]) -> 71
    { in0: 9, in1: 0, in2: 1, out0: 71, out1: 133, out2: 134 },
    // Discretionary SPONGE(4, [1, 2, 3, 4]): block 0 and block 1 -> 74
    { in0: 1, in1: 2, in2: 4, out0: 80, out1: 81, out2: 82 },
    { in0: 83, in1: 85, in2: 82, out0: 74, out1: 135, out2: 136 },
    // Mutant counterexample rows (forged sponge trajectories):
    // mut_no_has1: SPONGE(3) block 1 absorbing y1 = 55 -> in1 = 41 + 55 = 96
    { in0: 63, in1: 96, in2: 42, out0: 55, out1: 137, out2: 138 },
    // mut_no_chain: SPONGE(3) block 1 ignoring prev -> [23, 0, 0]
    { in0: 23, in1: 0, in2: 0, out0: 56, out1: 139, out2: 140 },
    // mut_first_cap_wrong: SPONGE(1, [9]) with cap 0 instead of 1 -> [9, 0, 0]
    { in0: 9, in1: 0, in2: 0, out0: 79, out1: 141, out2: 142 },
    // mut_ungated_y1: SPONGE(1, [9]) with ungated witness y1 = 5 -> [9, 5, 1]
    { in0: 9, in1: 5, in2: 1, out0: 78, out1: 143, out2: 144 }
  ]

  pure def perm_fn(x0: int, x1: int, x2: int): List[int] =
    range(0, PERM_TABLE.length()).foldl([300, 301, 302], (acc, i) => {
      val row = PERM_TABLE.nth(i)
      if (row.in0 == x0 and row.in1 == x1 and row.in2 == x2)
        [row.out0, row.out1, row.out2]
      else acc
    })

  // Reference sponge_u (`iface.rs:192-206`): chunks of 2 over `perm_fn`
  pure def sponge_u_fn(u_code: int, xs: List[int]): int = {
    val n = xs.length()
    val n_blocks = (n + 1) / 2
    val init_s = [0, 0, cap_fixed_fn(u_code, n)]
    val final_s = range(0, n_blocks).foldl(init_s, (s, b) => {
      val s0 = f_add(s.nth(0), xs.nth(2 * b))
      val s1 = if (2 * b + 1 < n) f_add(s.nth(1), xs.nth(2 * b + 1)) else s.nth(1)
      perm_fn(s0, s1, s.nth(2))
    })
    final_s.nth(0)
  }

  pure def h2_fn(a: int, b: int): int =
    sponge_u_fn(0, [a, b])

  pure def digest_fn(tag_v: int, d2: int): int =
    sponge_u_fn(4, [tag_v, d2])

  // Packed replay tuple (`decode.rs:518-525`):
  // bits[0..5] + param * 2^5 + o * 2^13 + poff * 2^22 + prm * 2^31
  pure def pack_tup(bits: List[int], param: int, o: int, poff: int, prm: int): int =
    bits.nth(0) +
    2 * bits.nth(1) +
    4 * bits.nth(2) +
    8 * bits.nth(3) +
    16 * bits.nth(4) +
    T_PARAM_SH * param +
    T_O_SH * o +
    T_POFF_SH * poff +
    T_PERMS_SH * prm

  pure def to_limbs(x: int): List[int] = [
    x % LIMB_BASE,
    (x / LIMB_BASE) % LIMB_BASE,
    (x / (LIMB_BASE * LIMB_BASE)) % LIMB_BASE
  ]

  pure def limbs_int(ls: List[int]): int =
    ls.nth(0) + ls.nth(1) * LIMB_BASE + ls.nth(2) * LIMB_BASE * LIMB_BASE

  pure def pack_words(pre_hash: int, w: List[int]): List[int] = {
    val packed = range(0, 47 + 1).foldl([], (acc, i) => {
      val w0 = w.nth(2 * i)
      val w1 = if (2 * i + 1 < N_WORDS) w.nth(2 * i + 1) else 0
      acc.append(w0 + w1 * WORD_BASE)
    })
    List(pre_hash).concat(packed)
  }

  // =========================================================================
  // 4. Native Stock Reference (`lane-src/iface.rs` with full replay)
  // =========================================================================

  pure def op_admitted(op: int): bool =
    (op == OP_NOP or op == OP_PERM or op == OP_H2 or op == OP_SPONGE or
     op == OP_PATH or op == OP_PUB or op == OP_DIGEST)

  pure def op_lo(op: int): int =
    if (op == OP_SPONGE or op == OP_PATH) 1 else 0

  pure def op_hi(op: int): int =
    if (op == OP_NOP or op == OP_PERM or op == OP_H2) 0
    else if (op == OP_SPONGE) V_CAP - 1
    else if (op == OP_PATH) PATH_MAX
    else if (op == OP_PUB) LANES - 1
    else if (op == OP_DIGEST) N_TAGS - 1
    else -1

  pure def op_vals(op: int, param: int): int =
    if (op == OP_NOP) 0
    else if (op == OP_PERM) PERM_VALS
    else if (op == OP_H2) 3
    else if (op == OP_SPONGE) param + 1
    else if (op == OP_PATH) 3
    else if (op == OP_PUB) 1
    else if (op == OP_DIGEST) 2
    else 0

  pure def op_perms(op: int, param: int): int =
    if (op == OP_PERM or op == OP_H2 or op == OP_DIGEST) 1
    else if (op == OP_SPONGE) (param + 1) / 2
    else if (op == OP_PATH) param
    else 0

  pure def ref_unpack(pw: List[int]): { ok: bool, words: List[int], err: str } = {
    if (pw.length() != N_PUBLIC) {
      { ok: false, words: [], err: "word count" }
    } else if (pw.nth(0) != PRE_HASH) {
      { ok: false, words: [], err: "pre-hash" }
    } else {
      range(0, 47 + 1).foldl({ ok: true, words: [], err: "" }, (st, i) => {
        if (not(st.ok)) st
        else {
          val x = pw.nth(i + 1)
          if (not(is_fr(x))) {
            { ok: false, words: [], err: "pw not canonical Fr" }
          } else {
            val k = if (N_WORDS - 2 * i < 2) N_WORDS - 2 * i else 2
            val w0 = x % WORD_BASE
            val b1 = x / WORD_BASE
            if (k == 1) {
              if (w0 >= BB_P) {
                { ok: false, words: [], err: "word not BabyBear" }
              } else if (b1 != 0) {
                { ok: false, words: [], err: "high bits set" }
              } else {
                { ok: true, words: st.words.append(w0), err: "" }
              }
            } else {
              val w1 = b1 % WORD_BASE
              val b2 = b1 / WORD_BASE
              if (w0 >= BB_P or w1 >= BB_P) {
                { ok: false, words: [], err: "word not BabyBear" }
              } else if (b2 != 0) {
                { ok: false, words: [], err: "high bits set" }
              } else {
                { ok: true, words: st.words.append(w0).append(w1), err: "" }
              }
            }
          }
        }
      })
    }
  }

  pure def ref_dec(ls: List[int]): { ok: bool, v_out: int } = {
    val limbs_ok = (ls.length() == LIMBS and
      ls.nth(0) >= 0 and ls.nth(0) < LIMB_BASE and
      ls.nth(1) >= 0 and ls.nth(1) < LIMB_BASE and
      ls.nth(2) >= 0 and ls.nth(2) < LIMB_BASE)
    val n = limbs_int(ls)
    if (limbs_ok and n < FR_MOD) {
      { ok: true, v_out: n }
    } else {
      { ok: false, v_out: 0 }
    }
  }

  pure def ref_anchor_tags(w: List[int]): { ok: bool, tag_in: int, tag_out: int } = {
    val mode = w.nth(MODE_WORD)
    val limbs_u16 = 0.to(3).forall(i =>
      w.nth(J_WORDS + i) >= 0 and w.nth(J_WORDS + i) < U16_LIM
    )
    if (not(limbs_u16)) {
      { ok: false, tag_in: 0, tag_out: 0 }
    } else {
      val j = w.nth(J_WORDS) + w.nth(J_WORDS + 1) * U16_LIM
      val k = w.nth(K_WORDS) + w.nth(K_WORDS + 1) * U16_LIM
      if (mode == 0) {
        { ok: true, tag_in: SHK_PROGC, tag_out: SHK_PROGC }
      } else if (mode == 1) {
        if (j < 1 or j > k) {
          { ok: false, tag_in: 0, tag_out: 0 }
        } else {
          val ti = if (j == 1) SHK_PROGC else SHK_LOCK
          val to_tag = if (j == k) SHK_PROGC else SHK_LOCK
          { ok: true, tag_in: ti, tag_out: to_tag }
        }
      } else {
        { ok: false, tag_in: 0, tag_out: 0 }
      }
    }
  }

  pure def ref_r_entries(w: List[int]): { ok: bool, r_list: List[{ op: int, param: int }] } = {
    val at = ref_anchor_tags(w)
    if (not(at.ok)) {
      { ok: false, r_list: [] }
    } else {
      val base = [
        { op: OP_DIGEST, param: R_RUN.nth(0) },
        { op: OP_DIGEST, param: R_RUN.nth(1) },
        { op: OP_DIGEST, param: R_RUN.nth(2) },
        { op: OP_SPONGE, param: 3 },
        { op: OP_DIGEST, param: at.tag_in }
      ]
      val full = if (w.nth(MODE_WORD) == 1)
        base.append({ op: OP_DIGEST, param: at.tag_out })
      else base
      { ok: true, r_list: full }
    }
  }

  // Stock native replay of one entry (`iface.rs:653-665`):
  // PERM, H2, SPONGE, DIGEST are replayed; PATH, PUB, NOP are not.
  pure def entry_replay_ok(op: int, param: int, o: int, vs: List[int]): bool = {
    val needed = op_vals(op, param)
    if (o < 0 or needed < 0 or o + needed > vs.length()) false
    else if (op == OP_PERM) {
      val p_out = perm_fn(vs.nth(o), vs.nth(o + 1), vs.nth(o + 2))
      p_out.nth(0) == vs.nth(o + 3) and
      p_out.nth(1) == vs.nth(o + 4) and
      p_out.nth(2) == vs.nth(o + 5)
    } else if (op == OP_H2) {
      h2_fn(vs.nth(o), vs.nth(o + 1)) == vs.nth(o + 2)
    } else if (op == OP_SPONGE) {
      if (param < 1) false
      else sponge_u_fn(0, vs.slice(o, o + param)) == vs.nth(o + param)
    } else if (op == OP_DIGEST) {
      digest_fn(tag_fr(param), vs.nth(o)) == vs.nth(o + 1)
    } else {
      true
    }
  }

  // Stock `iface::decode` with full replay of every entry (`iface.rs:604-676`)
  pure def ref_decode(pw: List[int]): DecodeRes = {
    val u = ref_unpack(pw)
    if (not(u.ok)) {
      { ok: false, stmt: [], err: u.err }
    } else {
      val w = u.words
      val lanes_res = range(0, LANES).foldl({ ok: true, xs: [] }, (st, i) => {
        if (not(st.ok)) st
        else {
          val at = CALL_AT + LIMBS * i
          val d = ref_dec(w.slice(at, at + LIMBS))
          if (not(d.ok)) { ok: false, xs: [] }
          else { ok: true, xs: st.xs.append(d.v_out) }
        }
      })
      if (not(lanes_res.ok)) {
        { ok: false, stmt: [], err: "lane range or Canon" }
      } else {
        val vs_res = range(0, V_CAP).foldl({ ok: true, xs: [] }, (st, j) => {
          if (not(st.ok)) st
          else {
            val at = VALUES_AT + LIMBS * j
            val d = ref_dec(w.slice(at, at + LIMBS))
            if (not(d.ok)) { ok: false, xs: [] }
            else { ok: true, xs: st.xs.append(d.v_out) }
          }
        })
        if (not(vs_res.ok)) {
          { ok: false, stmt: [], err: "value range or Canon" }
        } else {
          val vs = vs_res.xs
          val ent_res = range(0, B_CAP).foldl(
            { ok: true, off: 0, np: 0, ended: false, offs: [], err: "" },
            (st, i) => {
              if (not(st.ok)) st
              else {
                val op = w.nth(STMT + 2 * i)
                val param = w.nth(STMT + 2 * i + 1)
                if (not(op_admitted(op))) {
                  { ...st, ok: false, err: "op not admitted" }
                } else if (param < op_lo(op) or param > op_hi(op)) {
                  { ...st, ok: false, err: "param out of bound" }
                } else if (op != OP_NOP and st.ended) {
                  { ...st, ok: false, err: "used entry after NOP" }
                } else {
                  val now_ended = (st.ended or op == OP_NOP)
                  {
                    ok: true,
                    off: st.off + op_vals(op, param),
                    np: st.np + op_perms(op, param),
                    ended: now_ended,
                    offs: st.offs.append(st.off),
                    err: ""
                  }
                }
              }
            }
          )
          if (not(ent_res.ok)) {
            { ok: false, stmt: [], err: ent_res.err }
          } else if (ent_res.off > V_CAP or ent_res.np > P_CAP) {
            { ok: false, stmt: [], err: "capacity" }
          } else {
            val pad_ok = 0.to(V_CAP - 1).forall(j =>
              (j < ent_res.off) or (vs.nth(j) == 0)
            )
            if (not(pad_ok)) {
              { ok: false, stmt: [], err: "value row after off_n not zero" }
            } else {
              val r_res = ref_r_entries(w)
              if (not(r_res.ok)) {
                { ok: false, stmt: [], err: "mode" }
              } else {
                val r_list = r_res.r_list
                val r_match = 0.to(r_list.length() - 1).forall(i => {
                  val want = r_list.nth(i)
                  w.nth(STMT + 2 * i) == want.op and w.nth(STMT + 2 * i + 1) == want.param
                })
                if (not(r_match)) {
                  { ok: false, stmt: [], err: "R entry" }
                } else {
                  val mode = w.nth(MODE_WORD)
                  val anchor_words_ok = (
                    w.nth(E_IN_WORD) == ANCHOR_IN and
                    w.nth(E_OUT_WORD) == ANCHOR_IN + mode
                  )
                  if (not(anchor_words_ok)) {
                    { ok: false, stmt: [], err: "anchor entry words" }
                  } else {
                    val sp = R_OFF.nth(3)
                    val claim_inputs_ok = (
                      vs.nth(sp) == vs.nth(R_OFF.nth(0) + 1) and
                      vs.nth(sp + 1) == vs.nth(R_OFF.nth(1) + 1) and
                      vs.nth(sp + 2) == vs.nth(R_OFF.nth(2) + 1)
                    )
                    if (not(claim_inputs_ok)) {
                      { ok: false, stmt: [], err: "claim_pub inputs" }
                    } else {
                      // Full native replay of EVERY entry (`iface.rs:653-665`)
                      val all_replay_ok = 0.to(B_CAP - 1).forall(i => {
                        val op = w.nth(STMT + 2 * i)
                        val param = w.nth(STMT + 2 * i + 1)
                        val o = ent_res.offs.nth(i)
                        entry_replay_ok(op, param, o, vs)
                      })
                      if (not(all_replay_ok)) {
                        { ok: false, stmt: [], err: "entry: native replay" }
                      } else {
                        val lm_res = range(0, LANE_PAIRS).foldl(
                          { ok: true, s_vec: lanes_res.xs, mapped: [false, false, false, false], err: "" },
                          (st, k) => {
                            if (not(st.ok)) st
                            else {
                              val lane = w.nth(LANE_MAP + 2 * k)
                              val ent = w.nth(LANE_MAP + 2 * k + 1)
                              if (lane == 0 and ent == 0) st
                              else if (lane >= U16_LIM or ent >= U16_LIM) {
                                { ...st, ok: false, err: "lane map pair: not u16" }
                              } else if (ent < 0 or ent >= B_CAP) {
                                { ...st, ok: false, err: "lane map entry" }
                              } else {
                                val op_e = w.nth(STMT + 2 * ent)
                                val param_e = w.nth(STMT + 2 * ent + 1)
                                val o_e = ent_res.offs.nth(ent)
                                val op_allowed = (op_e == OP_DIGEST or (op_e == OP_SPONGE and param_e == 3))
                                if (not(op_allowed) or lane < 0 or lane >= LANES or
                                    st.mapped.nth(lane) or st.s_vec.nth(lane) != 0) {
                                  { ...st, ok: false, err: "lane map pair" }
                                } else {
                                  val out_idx = o_e + op_vals(op_e, param_e) - 1
                                  val updated_s = st.s_vec.replaceAt(lane, vs.nth(out_idx))
                                  val updated_m = st.mapped.replaceAt(lane, true)
                                  { ok: true, s_vec: updated_s, mapped: updated_m, err: "" }
                                }
                              }
                            }
                          }
                        )
                        if (not(lm_res.ok)) {
                          { ok: false, stmt: [], err: lm_res.err }
                        } else {
                          val pub_ok = 0.to(B_CAP - 1).forall(i => {
                            val op = w.nth(STMT + 2 * i)
                            val param = w.nth(STMT + 2 * i + 1)
                            val o = ent_res.offs.nth(i)
                            (op != OP_PUB) or (lm_res.s_vec.nth(param) == vs.nth(o))
                          })
                          if (not(pub_ok)) {
                            { ok: false, stmt: [], err: "PUB differs from its lane" }
                          } else {
                            { ok: true, stmt: lm_res.s_vec, err: "" }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  // =========================================================================
  // 5. In-Circuit Constraint System (`lane-src/decode.rs`)
  // =========================================================================

  pure def check_is_zero(x: int, inv: int): { ok: bool, z_bit: int } = {
    val xi = f_mul(x, inv)
    val z = f_sub(1, xi)
    val xz = f_mul(x, z)
    { ok: (xz == 0), z_bit: z }
  }

  pure def check_one_hot(n: int, bs: List[int], idx: int, gate: int): bool = {
    bs.length() == n and
    0.to(n - 1).forall(i => is_bit(bs.nth(i))) and
    f_sum_list(bs) == f_mod(gate) and
    range(0, n).foldl(0, (acc, i) => f_add(acc, f_mul(i, bs.nth(i)))) == f_mod(idx)
  }

  pure def check_dec_canon(ls: List[int], borrows: List[int]): bool = {
    ls.length() == LIMBS and
    borrows.length() == LIMBS and
    0.to(LIMBS - 1).forall(k => is_bit(borrows.nth(k))) and
    borrows.nth(LIMBS - 1) == 0 and
    0.to(LIMBS - 1).forall(k => {
      val prev_c = if (k == 0) 0 else borrows.nth(k - 1)
      val cur_c = borrows.nth(k)
      val d_k = f_mod(R1_LIMBS.nth(k) - ls.nth(k) - prev_c + LIMB_BASE * cur_c)
      in_bound(d_k, LIMB_BASE)
    })
  }

  pure def circuit_dec_v(ls: List[int]): int =
    f_mod(limbs_int(ls))

  pure def circuit_output(pw: List[int], wit: Witness): List[int] = {
    val w = wit.w_words
    val init_lanes = range(0, LANES).foldl([], (acc, l) => {
      val at = CALL_AT + LIMBS * l
      acc.append(circuit_dec_v(w.slice(at, at + LIMBS)))
    })
    range(0, LANE_PAIRS).foldl(init_lanes, (s_cur, k) => {
      val hk = wit.lm_h.nth(k)
      val vk = wit.lm_val.nth(k)
      range(0, LANES).foldl([], (acc, l) =>
        acc.append(f_add(s_cur.nth(l), f_mul(hk.nth(l), vk)))
      )
    })
  }

  // Symbolic log-derivative lookup check for (V, S) table (`decode.rs:702-723`)
  pure def check_lookup(queries: List[Query], table: List[TableRow]): bool = {
    val q_ok = 0.to(queries.length() - 1).forall(qi => {
      val q = queries.nth(qi)
      val lhs_w = range(0, queries.length()).foldl(0, (acc, j) => {
        val qj = queries.nth(j)
        if (qj.q_key == q.q_key and qj.q_v == q.q_v) f_add(acc, qj.q_gate) else acc
      })
      val rhs_w = range(0, table.length()).foldl(0, (acc, j) => {
        val tj = table.nth(j)
        if (tj.t_key == q.q_key and tj.t_v == q.q_v) f_add(acc, tj.t_mult) else acc
      })
      lhs_w == rhs_w
    })
    val t_ok = 0.to(table.length() - 1).forall(ti => {
      val t = table.nth(ti)
      val lhs_w = range(0, queries.length()).foldl(0, (acc, j) => {
        val qj = queries.nth(j)
        if (qj.q_key == t.t_key and qj.q_v == t.t_v) f_add(acc, qj.q_gate) else acc
      })
      val rhs_w = range(0, table.length()).foldl(0, (acc, j) => {
        val tj = table.nth(j)
        if (tj.t_key == t.t_key and tj.t_v == t.t_v) f_add(acc, tj.t_mult) else acc
      })
      lhs_w == rhs_w
    })
    q_ok and t_ok
  }

  // Symbolic slot-against-entries lookup check (`decode.rs:724-742`):
  // `sum_t act_t / (alpha - pk_t) == sum_i mult_i / (alpha - row_i)`
  pure def check_slot_lookup(
    slot_pks: List[{ act: int, pk: int }],
    entry_rows: List[{ mult: int, row: int }]
  ): bool = {
    val s_ok = 0.to(slot_pks.length() - 1).forall(ti => {
      val sp = slot_pks.nth(ti)
      val lhs_w = range(0, slot_pks.length()).foldl(0, (acc, j) => {
        val sj = slot_pks.nth(j)
        if (sj.pk == sp.pk) f_add(acc, sj.act) else acc
      })
      val rhs_w = range(0, entry_rows.length()).foldl(0, (acc, i) => {
        val ei = entry_rows.nth(i)
        if (ei.row == sp.pk) f_add(acc, ei.mult) else acc
      })
      lhs_w == rhs_w
    })
    val e_ok = 0.to(entry_rows.length() - 1).forall(ei => {
      val er = entry_rows.nth(ei)
      val lhs_w = range(0, slot_pks.length()).foldl(0, (acc, j) => {
        val sj = slot_pks.nth(j)
        if (sj.pk == er.row) f_add(acc, sj.act) else acc
      })
      val rhs_w = range(0, entry_rows.length()).foldl(0, (acc, i) => {
        val ej = entry_rows.nth(i)
        if (ej.row == er.row) f_add(acc, ej.mult) else acc
      })
      lhs_w == rhs_w
    })
    s_ok and e_ok
  }

  // Full circuit constraint predicate parameterized by mutant flags
  pure def constraints_with_flags(fl: Flags, pw: List[int], wit: Witness): bool = {
    val shapes_ok = (
      pw.length() == N_PUBLIC and
      0.to(N_PUBLIC - 1).forall(i => is_fr(pw.nth(i))) and
      wit.w_words.length() == N_WORDS and
      0.to(N_WORDS - 1).forall(i => is_fr(wit.w_words.nth(i))) and
      wit.lane_borrows.length() == LANES and
      wit.vs_borrows.length() == V_CAP and
      wit.op_bits.length() == B_CAP and
      wit.sponge_q.length() == B_CAP and
      wit.sponge_e.length() == B_CAP and
      wit.g_bits.length() == V_CAP and
      is_fr(wit.j1_inv) and
      is_fr(wit.kj_inv) and
      wit.lm_inv.length() == LANE_PAIRS and
      wit.lm_h.length() == LANE_PAIRS and
      wit.lm_e.length() == LANE_PAIRS and
      wit.lm_diff_inv.length() == LANE_PAIRS and
      wit.lm_val.length() == LANE_PAIRS and
      wit.pub_v.length() == B_CAP and
      wit.slots.length() == P_CAP and
      wit.slot_mult.length() == B_CAP and
      wit.mv.length() == V_CAP and
      wit.ms.length() == LANES and
      0.to(LANE_PAIRS - 1).forall(k =>
        is_fr(wit.lm_inv.nth(k)) and
        is_fr(wit.lm_val.nth(k)) and
        wit.lm_diff_inv.nth(k).length() == LANE_PAIRS and
        0.to(LANE_PAIRS - 1).forall(i => is_fr(wit.lm_diff_inv.nth(k).nth(i)))
      ) and
      0.to(B_CAP - 1).forall(i =>
        is_fr(wit.sponge_q.nth(i)) and
        is_fr(wit.pub_v.nth(i)) and
        is_fr(wit.slot_mult.nth(i))
      ) and
      0.to(P_CAP - 1).forall(ti => {
        val sw = wit.slots.nth(ti)
        is_bit(sw.act) and
        sw.bits.length() == 5 and
        0.to(4).forall(b => is_bit(sw.bits.nth(b))) and
        is_fr(sw.param) and
        is_fr(sw.o) and
        is_fr(sw.poff) and
        is_fr(sw.prm) and
        is_fr(sw.tm_inv) and
        is_fr(sw.rem_inv) and
        sw.ys.length() == 6 and
        0.to(5).forall(k => is_fr(sw.ys.nth(k))) and
        sw.tb.length() == N_TAGS
      }) and
      0.to(V_CAP - 1).forall(j => is_fr(wit.mv.nth(j))) and
      0.to(LANES - 1).forall(l => is_fr(wit.ms.nth(l)))
    )
    if (not(shapes_ok)) false
    else {
      val w = wit.w_words
      val pre_hash_ok = (pw.nth(0) == PRE_HASH)

      // 1. Unpack constraints
      val unpack_range_ok = 0.to(N_WORDS - 1).forall(idx =>
        if (idx >= VALUES_AT) in_bound(w.nth(idx), LIMB_BASE)
        else in_bound(w.nth(idx), BB_P)
      )
      val unpack_lin_ok = 0.to(47).forall(i => {
        val k = if (N_WORDS - 2 * i < 2) N_WORDS - 2 * i else 2
        val s = if (k == 1) w.nth(2 * i)
                else f_add(w.nth(2 * i), f_mul(WORD_BASE, w.nth(2 * i + 1)))
        s == pw.nth(i + 1)
      })

      // 2. Lanes and logged values Canon borrow chains
      val lanes_canon_ok = 0.to(LANES - 1).forall(l => {
        val at = CALL_AT + LIMBS * l
        check_dec_canon(w.slice(at, at + LIMBS), wit.lane_borrows.nth(l))
      })
      val vs_canon_ok = 0.to(V_CAP - 1).forall(j => {
        val at = VALUES_AT + LIMBS * j
        check_dec_canon(w.slice(at, at + LIMBS), wit.vs_borrows.nth(j))
      })

      val init_lanes = range(0, LANES).foldl([], (acc, l) => {
        val at = CALL_AT + LIMBS * l
        acc.append(circuit_dec_v(w.slice(at, at + LIMBS)))
      })
      val vs = range(0, V_CAP).foldl([], (acc, j) => {
        val at = VALUES_AT + LIMBS * j
        acc.append(circuit_dec_v(w.slice(at, at + LIMBS)))
      })

      // 3. Entries constraints (`decode.rs:310-400`)
      val entries_step = range(0, B_CAP).foldl(
        { ok: true, off: 0, np: 0, offs: [], poffs: [], permss: [], pars: [] },
        (st, i) => {
          val op = w.nth(STMT + 2 * i)
          val param = w.nth(STMT + 2 * i + 1)
          val c = wit.op_bits.nth(i)
          val c_len_ok = (c.length() == 7 and 0.to(6).forall(k => is_bit(c.nth(k))))
          if (not(st.ok) or not(c_len_ok)) {
            { ...st, ok: false }
          } else {
            val one_ok = (f_sum_list(c) == 1)
            val opc = f_dot(ADMITTED, c)
            val op_ok = (opc == op)
            val lo_coeffs = [0, 0, 0, 1, 1, 0, 0]
            val hi_coeffs = [0, 0, 0, V_CAP - 1, PATH_MAX, LANES - 1, N_TAGS - 1]
            val lo = f_dot(lo_coeffs, c)
            val hi = f_dot(hi_coeffs, c)
            val x = f_sub(param, lo)
            val y = f_sub(hi, param)
            val lo_ok = in_bound(x, BOUND_LIM)
            val hi_ok = in_bound(y, BOUND_LIM)
            val nop_ok = (
              i == 0 or
              f_mul(wit.op_bits.nth(i - 1).nth(I_NOP), c.nth(I_NOP)) == wit.op_bits.nth(i - 1).nth(I_NOP)
            )
            val sp = f_add(f_mul(c.nth(I_SPONGE), param), c.nth(I_SPONGE))
            val dv = f_mod(
              PERM_VALS * c.nth(I_PERM) +
              3 * c.nth(I_H2) +
              3 * c.nth(I_PATH) +
              1 * c.nth(I_PUB) +
              2 * c.nth(I_DIGEST) +
              sp
            )
            val q = wit.sponge_q.nth(i)
            val e = wit.sponge_e.nth(i)
            val sp_slots_ok = (
              is_bit(e) and
              f_mod(2 * q - param - e) == 0 and
              in_bound(q, SPONGE_Q_LIM)
            )
            val spq = f_mul(c.nth(I_SPONGE), q)
            val pp = f_mul(c.nth(I_PATH), param)
            val dp = f_mod(c.nth(I_PERM) + c.nth(I_H2) + c.nth(I_DIGEST) + spq + pp)
            {
              ok: (st.ok and one_ok and op_ok and lo_ok and hi_ok and nop_ok and sp_slots_ok),
              off: f_add(st.off, dv),
              np: f_add(st.np, dp),
              offs: st.offs.append(st.off),
              poffs: st.poffs.append(st.np),
              permss: st.permss.append(dp),
              pars: st.pars.append(e)
            }
          }
        }
      )

      val cap_v = f_sub(V_CAP, entries_step.off)
      val cap_p = f_sub(P_CAP, entries_step.np)
      val cap_ok = (in_bound(cap_v, CAP_LIM) and in_bound(cap_p, CAP_LIM))

      val g = wit.g_bits
      val g_bits_ok = 0.to(V_CAP - 1).forall(j => is_bit(g.nth(j)))
      val g_mono_ok = 1.to(V_CAP - 1).forall(j => f_mul(g.nth(j - 1), g.nth(j)) == g.nth(j - 1))
      val g_zero_ok = 0.to(V_CAP - 1).forall(j => f_mul(g.nth(j), vs.nth(j)) == 0)
      val g_sum_ok = (f_sum_list(g) == cap_v)

      // 4. R and the anchor rule at static positions (`decode.rs:401-445`)
      val mode = w.nth(MODE_WORD)
      val mode_bit_ok = (f_sub(f_mul(mode, mode), mode) == 0)
      val jk_u16_ok = 0.to(3).forall(i => in_bound(w.nth(J_WORDS + i), U16_LIM))
      val j_val = f_add(w.nth(J_WORDS), f_mul(U16_LIM, w.nth(J_WORDS + 1)))
      val k_val = f_add(w.nth(K_WORDS), f_mul(U16_LIM, w.nth(K_WORDS + 1)))
      val j1 = f_sub(j_val, 1)
      val kj = f_sub(k_val, j_val)
      val chunk_range_ok = (
        in_bound(f_mul(mode, j1), U32_LIM) and
        in_bound(f_mul(mode, kj), U32_LIM)
      )
      val iz_j1 = check_is_zero(j1, wit.j1_inv)
      val iz_jk = check_is_zero(kj, wit.kj_inv)
      val eq_j1 = iz_j1.z_bit
      val eq_jk = iz_jk.z_bit
      val me = f_mul(mode, eq_j1)
      val t_in = f_sub(mode, me)
      val t_out = f_sub(1, eq_jk)

      val r_ops_ok = (
        entries_step.ok and
        0.to(2).forall(i =>
          wit.op_bits.nth(i).nth(I_DIGEST) == 1 and
          w.nth(STMT + 2 * i + 1) == R_RUN.nth(i)
        ) and
        wit.op_bits.nth(3).nth(I_SPONGE) == 1 and
        w.nth(STMT + 2 * 3 + 1) == 3 and
        wit.op_bits.nth(4).nth(I_DIGEST) == 1 and
        w.nth(STMT + 2 * 4 + 1) == t_in and
        f_mul(mode, f_sub(wit.op_bits.nth(5).nth(I_DIGEST), 1)) == 0 and
        f_mul(mode, f_sub(w.nth(STMT + 2 * 5 + 1), t_out)) == 0
      )

      val anchor_words_ok = (
        w.nth(E_IN_WORD) == ANCHOR_IN and
        f_sub(w.nth(E_OUT_WORD), mode) == ANCHOR_IN
      )

      val sp_off = R_OFF.nth(3)
      val claim_pub_ok = 0.to(2).forall(i => vs.nth(sp_off + i) == vs.nth(R_OFF.nth(i) + 1))

      val step4_ok = (
        mode_bit_ok and jk_u16_ok and chunk_range_ok and
        iz_j1.ok and iz_jk.ok and r_ops_ok and
        anchor_words_ok and claim_pub_ok
      )

      // 5. Lane map, in stock order (`decode.rs:446-500`)
      val digs = range(0, B_CAP).foldl([], (acc, i) =>
        acc.append(if (entries_step.ok) wit.op_bits.nth(i).nth(I_DIGEST) else 0)
      )
      val sps = range(0, B_CAP).foldl([], (acc, i) =>
        acc.append(if (entries_step.ok) wit.op_bits.nth(i).nth(I_SPONGE) else 0)
      )
      val params = range(0, B_CAP).foldl([], (acc, i) =>
        acc.append(w.nth(STMT + 2 * i + 1))
      )

      val lm_step = range(0, LANE_PAIRS).foldl(
        { ok: true, s_vec: init_lanes, qv: [] },
        (st, k) => {
          val lane = w.nth(LANE_MAP + 2 * k)
          val ent = w.nth(LANE_MAP + 2 * k + 1)
          val u16_ok = (in_bound(lane, U16_LIM) and in_bound(ent, U16_LIM))
          val u = f_add(lane, f_mul(LANES, ent))
          val inv = wit.lm_inv.nth(k)
          val nz = f_mul(u, inv)
          val z = f_sub(1, nz)
          val zu_ok = (f_mul(z, u) == 0)
          val hk = wit.lm_h.nth(k)
          val ek = wit.lm_e.nth(k)
          val h_ok = check_one_hot(LANES, hk, lane, nz)
          val e_ok = check_one_hot(B_CAP, ek, ent, nz)
          val sel_dig = if (e_ok) f_dot(ek, digs) else 0
          val sel_sp = if (e_ok) f_dot(ek, sps) else 0
          val p = if (e_ok) f_dot(ek, params) else 0
          val ds_ok = (f_add(sel_dig, sel_sp) == nz)
          val sp3_ok = (f_mul(sel_sp, f_sub(p, 3)) == 0)
          val taken_ok = (h_ok and f_dot(hk, st.s_vec) == 0)
          val twice_ok = 0.to(LANE_PAIRS - 1).forall(i => {
            if (i >= k) true
            else {
              val prev_lane = w.nth(LANE_MAP + 2 * i)
              val prev_ent = w.nth(LANE_MAP + 2 * i + 1)
              val prev_u = f_add(prev_lane, f_mul(LANES, prev_ent))
              val prev_nz = f_mul(prev_u, wit.lm_inv.nth(i))
              val dl = f_sub(lane, prev_lane)
              val dinv = wit.lm_diff_inv.nth(k).nth(i)
              val m = f_mul(dl, dinv)
              val gate_pair = f_mul(nz, prev_nz)
              f_mul(gate_pair, m) == gate_pair
            }
          })
          val o = if (e_ok and entries_step.offs.length() == B_CAP) f_dot(ek, entries_step.offs) else 0
          val idx = f_add(o, f_add(1, f_mul(2, sel_sp)))
          val vk = wit.lm_val.nth(k)
          val updated_s = if (h_ok) {
            range(0, LANES).foldl([], (acc, l) =>
              acc.append(f_add(st.s_vec.nth(l), f_mul(hk.nth(l), vk)))
            )
          } else st.s_vec
          val new_qv = st.qv.append({ q_gate: nz, q_key: idx, q_v: vk })
          {
            ok: (st.ok and u16_ok and zu_ok and h_ok and e_ok and ds_ok and
                 sp3_ok and taken_ok and twice_ok),
            s_vec: updated_s,
            qv: new_qv
          }
        }
      )

      // 6. PUB entries queries (`decode.rs:501-510`)
      val pub_queries = range(0, B_CAP).foldl(
        { qv: lm_step.qv, qs: [] },
        (st, i) => {
          val gate = if (entries_step.ok) wit.op_bits.nth(i).nth(I_PUB) else 0
          val off_i = if (entries_step.offs.length() == B_CAP) entries_step.offs.nth(i) else 0
          val param_i = w.nth(STMT + 2 * i + 1)
          val vi = wit.pub_v.nth(i)
          {
            qv: st.qv.append({ q_gate: gate, q_key: off_i, q_v: vi }),
            qs: st.qs.append({ q_gate: gate, q_key: f_add(V_CAP, param_i), q_v: vi })
          }
        }
      )

      // 7. Native replay of every entry via P permutation slots (`decode.rs:512-655, 724-742`)
      val entry_rows = range(0, B_CAP).foldl([], (acc, i) => {
        if (not(entries_step.ok)) acc
        else {
          val c = wit.op_bits.nth(i)
          val row_bits = [
            c.nth(I_PERM),
            c.nth(I_H2),
            c.nth(I_SPONGE),
            c.nth(I_DIGEST),
            entries_step.pars.nth(i)
          ]
          val r_val = pack_tup(
            row_bits,
            params.nth(i),
            entries_step.offs.nth(i),
            entries_step.poffs.nth(i),
            entries_step.permss.nth(i)
          )
          val mult_i = if (fl.free_mult) wit.slot_mult.nth(i) else entries_step.permss.nth(i)
          acc.append({ mult: mult_i, row: r_val })
        }
      })

      val slot_step = range(0, P_CAP).foldl(
        {
          ok: true,
          prev: [0, 0, 0],
          qv: pub_queries.qv,
          slot_pks: [],
          act_sum: 0
        },
        (st, ti) => {
          val sw = wit.slots.nth(ti)
          val act = sw.act
          val mono_ok = (ti == 0 or f_mul(wit.slots.nth(ti - 1).act, act) == act)
          val fld_range_ok = (
            in_bound(sw.param, SLOT_PARAM_LIM) and
            in_bound(sw.o, SLOT_FIELD_LIM) and
            in_bound(sw.poff, SLOT_FIELD_LIM) and
            in_bound(sw.prm, SLOT_FIELD_LIM)
          )
          val pk = pack_tup(sw.bits, sw.param, sw.o, sw.poff, sw.prm)
          val na = f_sub(1, act)
          val inact_ok = (not(fl.check_inactive_zero) or (na == 0 or pk == 0))

          val c_perm = sw.bits.nth(0)
          val c_h2 = sw.bits.nth(1)
          val c_sp = sw.bits.nth(2)
          val c_dig = sw.bits.nth(3)
          val par = sw.bits.nth(4)

          val tm = f_sub(ti, sw.poff)
          val d1 = f_mul(act, tm)
          val rem = f_sub(f_sub(sw.prm, tm), 1)
          val d2 = f_mul(act, rem)
          val m_range_ok = (
            not(fl.check_m_range) or
            (in_bound(d1, SLOT_FIELD_LIM) and in_bound(d2, SLOT_FIELD_LIM))
          )

          val iz_tm = check_is_zero(tm, sw.tm_inv)
          val iz_rem = check_is_zero(rem, sw.rem_inv)
          val first = iz_tm.z_bit
          val last = iz_rem.z_bit
          val nf = f_sub(1, first)
          val lp = f_mul(last, par)
          val has1 = if (fl.use_has1) f_sub(1, lp) else 1
          val sp1 = f_mul(c_sp, has1)
          val spl = f_mul(c_sp, last)

          val g0 = f_mod(c_perm + c_h2 + c_sp + c_dig)
          val g1 = f_mod(c_perm + c_h2 + c_dig + sp1)
          val g2 = f_add(c_perm, c_h2)
          val g3 = f_add(c_perm, spl)
          val i0 = f_add(sw.o, f_mul(2, tm))
          val i1 = f_add(i0, 1)
          val i2 = f_add(i0, 2)
          val op3 = f_add(sw.o, 3)
          val pm3 = f_mul(c_perm, op3)
          val opp = f_add(sw.o, sw.param)
          val i3 = f_add(pm3, f_mul(c_sp, opp))
          val i4 = f_add(sw.o, 4)
          val i5 = f_add(sw.o, 5)

          val y0 = sw.ys.nth(0)
          val y1 = sw.ys.nth(1)
          val y2 = sw.ys.nth(2)
          val y3 = sw.ys.nth(3)
          val y4 = sw.ys.nth(4)
          val y5 = sw.ys.nth(5)

          val new_qv = st.qv
            .append({ q_gate: g0, q_key: i0, q_v: y0 })
            .append({ q_gate: g1, q_key: i1, q_v: y1 })
            .append({ q_gate: g2, q_key: i2, q_v: y2 })
            .append({ q_gate: g3, q_key: i3, q_v: y3 })
            .append({ q_gate: c_perm, q_key: i4, q_v: y4 })
            .append({ q_gate: c_perm, q_key: i5, q_v: y5 })

          val pd = f_mul(c_dig, sw.param)
          val tb_ok = check_one_hot(N_TAGS, sw.tb, pd, c_dig)
          val tag_v = if (tb_ok) f_dot(TAG_FR_TABLE, sw.tb) else 0

          val pse = f_mod(c_perm + c_h2 + c_sp)
          val spn = if (fl.use_chain) f_mul(c_sp, nf) else 0
          val in0 = f_mod(f_mul(pse, y0) + f_mul(c_dig, tag_v) + f_mul(spn, st.prev.nth(0)))

          val ph = f_add(c_perm, c_h2)
          val y1_sp_gate = if (fl.gate_y1_in1) sp1 else c_sp
          val in1 = f_mod(
            f_mul(ph, y1) +
            f_mul(c_dig, y0) +
            f_mul(y1_sp_gate, y1) +
            f_mul(spn, st.prev.nth(1))
          )

          val sf = if (fl.use_first_cap) f_mul(c_sp, first) else 0
          val in2 = f_mod(
            f_mul(c_perm, y2) +
            f_mul(c_h2, CAP_0_2) +
            f_mul(c_dig, CAP_4_2) +
            f_mul(sf, sw.param) +
            f_mul(spn, st.prev.nth(2))
          )

          val out = perm_fn(in0, in1, in2)

          val out_ok = (
            f_mul(c_perm, f_sub(out.nth(0), y3)) == 0 and
            f_mul(c_perm, f_sub(out.nth(1), y4)) == 0 and
            f_mul(c_perm, f_sub(out.nth(2), y5)) == 0 and
            f_mul(c_h2, f_sub(out.nth(0), y2)) == 0 and
            f_mul(c_dig, f_sub(out.nth(0), y1)) == 0 and
            f_mul(spl, f_sub(out.nth(0), y3)) == 0
          )

          {
            ok: (st.ok and mono_ok and fld_range_ok and inact_ok and
                 m_range_ok and iz_tm.ok and iz_rem.ok and tb_ok and out_ok),
            prev: out,
            qv: new_qv,
            slot_pks: st.slot_pks.append({ act: act, pk: pk }),
            act_sum: f_add(st.act_sum, act)
          }
        }
      )

      val act_sum_ok = (slot_step.act_sum == entries_step.np)
      val slot_lookup_ok = (
        entries_step.ok and
        check_slot_lookup(slot_step.slot_pks, entry_rows)
      )

      val table_v = range(0, V_CAP).foldl([], (acc, j) =>
        acc.append({ t_key: j, t_v: vs.nth(j), t_mult: wit.mv.nth(j) })
      )
      val table_s = range(0, LANES).foldl([], (acc, l) =>
        acc.append({ t_key: f_add(V_CAP, l), t_v: lm_step.s_vec.nth(l), t_mult: wit.ms.nth(l) })
      )
      val all_queries = slot_step.qv.concat(pub_queries.qs)
      val all_table = table_v.concat(table_s)
      val lookup_ok = check_lookup(all_queries, all_table)

      pre_hash_ok and unpack_range_ok and unpack_lin_ok and
      lanes_canon_ok and vs_canon_ok and entries_step.ok and
      cap_ok and g_bits_ok and g_mono_ok and g_zero_ok and g_sum_ok and
      step4_ok and lm_step.ok and slot_step.ok and act_sum_ok and
      slot_lookup_ok and lookup_ok
    }
  }

  pure def constraints_ok(pw: List[int], wit: Witness): bool =
    constraints_with_flags(DEFAULT_FLAGS, pw, wit)

  // =========================================================================
  // 6. Mutant Constraint Predicates (7 Required Mutants)
  // =========================================================================

  // Mutant 1: free multiplicities in the slot-to-entry lookup (`decode.rs:736-739`)
  pure def mut_free_mult(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, free_mult: true }, pw, wit)

  // Mutant 2: drop `0 <= m < perms` on active slots (`decode.rs:581-585`)
  pure def mut_no_m_range(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, check_m_range: false }, pw, wit)

  // Mutant 3: drop `(1 - act) * pk == 0` on inactive slots (`decode.rs:575-577`)
  pure def mut_no_inactive_zero(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, check_inactive_zero: false }, pw, wit)

  // Mutant 4: always absorb `y1` in SPONGE (`has1 = 1` instead of `1 - last * par`, `decode.rs:589-591`)
  pure def mut_no_has1(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, use_has1: false }, pw, wit)

  // Mutant 5: ignore `prev` in SPONGE (`spn = 0`, `decode.rs:621-635`)
  pure def mut_no_chain(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, use_chain: false }, pw, wit)

  // Mutant 6: capacity 0 instead of `param` on first SPONGE block (`sf = 0`, `decode.rs:633-634`)
  pure def mut_first_cap_wrong(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, use_first_cap: false }, pw, wit)

  // Mutant 7: `y1` enters `in1` without `has1` gate (`c_sp * y1` instead of `sp1 * y1`, `decode.rs:628-629`)
  pure def mut_ungated_y1(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, gate_y1_in1: false }, pw, wit)

  // =========================================================================
  // 7. Honest Witness Generator (`Native` closures in `lane-src/decode.rs`)
  // =========================================================================

  pure def honest_borrows(ls: List[int]): List[int] = {
    val d0 = R1_LIMBS.nth(0) - ls.nth(0)
    val b0 = if (d0 < 0) 1 else 0
    val d1 = R1_LIMBS.nth(1) - ls.nth(1) - b0
    val b1 = if (d1 < 0) 1 else 0
    [b0, b1, 0]
  }

  pure def honest_witness(pw: List[int]): Witness = {
    val w = range(0, 47 + 1).foldl([], (acc, i) => {
      val x = pw.nth(i + 1)
      val w0 = x % WORD_BASE
      val w1 = (x / WORD_BASE) % WORD_BASE
      if (2 * i + 1 < N_WORDS) acc.append(w0).append(w1)
      else acc.append(w0)
    })

    val l_borrows = range(0, LANES).foldl([], (acc, l) => {
      val at = CALL_AT + LIMBS * l
      acc.append(honest_borrows(w.slice(at, at + LIMBS)))
    })
    val v_borrows = range(0, V_CAP).foldl([], (acc, j) => {
      val at = VALUES_AT + LIMBS * j
      acc.append(honest_borrows(w.slice(at, at + LIMBS)))
    })
    val vs = range(0, V_CAP).foldl([], (acc, j) => {
      val at = VALUES_AT + LIMBS * j
      acc.append(circuit_dec_v(w.slice(at, at + LIMBS)))
    })

    val ent_st = range(0, B_CAP).foldl(
      { op_bits: [], sq: [], se: [], off: 0, np: 0, offs: [], poffs: [], permss: [] },
      (st, i) => {
        val op = w.nth(STMT + 2 * i)
        val param = w.nth(STMT + 2 * i + 1)
        val c = range(0, 6 + 1).foldl([], (acc, k) =>
          acc.append(if (op == ADMITTED.nth(k)) 1 else 0)
        )
        val q = (param + 1) / 2
        val e = param % 2
        val dv = op_vals(op, param)
        val dp = op_perms(op, param)
        {
          op_bits: st.op_bits.append(c),
          sq: st.sq.append(q),
          se: st.se.append(e),
          off: st.off + dv,
          np: st.np + dp,
          offs: st.offs.append(st.off),
          poffs: st.poffs.append(st.np),
          permss: st.permss.append(dp)
        }
      }
    )

    val g = range(0, V_CAP).foldl([], (acc, j) =>
      acc.append(if (j >= ent_st.off) 1 else 0)
    )

    val j_val = f_add(w.nth(J_WORDS), f_mul(U16_LIM, w.nth(J_WORDS + 1)))
    val k_val = f_add(w.nth(K_WORDS), f_mul(U16_LIM, w.nth(K_WORDS + 1)))
    val j1_inv = mod_inv(f_sub(j_val, 1))
    val kj_inv = mod_inv(f_sub(k_val, j_val))

    val digs = range(0, B_CAP).foldl([], (acc, i) =>
      acc.append(ent_st.op_bits.nth(i).nth(I_DIGEST))
    )
    val sps = range(0, B_CAP).foldl([], (acc, i) =>
      acc.append(ent_st.op_bits.nth(i).nth(I_SPONGE))
    )
    val params = range(0, B_CAP).foldl([], (acc, i) =>
      acc.append(w.nth(STMT + 2 * i + 1))
    )

    val lm_st = range(0, LANE_PAIRS).foldl(
      { invs: [], hs: [], es: [], diff_invs: [], vals: [], qv: [] },
      (st, k) => {
        val lane = w.nth(LANE_MAP + 2 * k)
        val ent = w.nth(LANE_MAP + 2 * k + 1)
        val u = f_add(lane, f_mul(LANES, ent))
        val inv = mod_inv(u)
        val nz = f_mul(u, inv)
        val hk = range(0, LANES).foldl([], (acc, l) =>
          acc.append(if (nz == 1 and lane == l) 1 else 0)
        )
        val ek = range(0, B_CAP).foldl([], (acc, i) =>
          acc.append(if (nz == 1 and ent == i) 1 else 0)
        )
        val sel_sp = f_dot(ek, sps)
        val diff_row = range(0, LANE_PAIRS).foldl([], (acc, i) => {
          val prev_lane = w.nth(LANE_MAP + 2 * i)
          acc.append(if (i < k) mod_inv(f_sub(lane, prev_lane)) else 0)
        })
        val o = f_dot(ek, ent_st.offs)
        val idx = f_add(o, f_add(1, f_mul(2, sel_sp)))
        val vk = if (nz == 1 and idx >= 0 and idx < V_CAP) vs.nth(idx) else 0
        {
          invs: st.invs.append(inv),
          hs: st.hs.append(hk),
          es: st.es.append(ek),
          diff_invs: st.diff_invs.append(diff_row),
          vals: st.vals.append(vk),
          qv: st.qv.append({ q_gate: nz, q_key: idx, q_v: vk })
        }
      }
    )

    val pub_st = range(0, B_CAP).foldl(
      { pvs: [], qv: lm_st.qv, qs: [] },
      (st, i) => {
        val gate = ent_st.op_bits.nth(i).nth(I_PUB)
        val off_i = ent_st.offs.nth(i)
        val param_i = w.nth(STMT + 2 * i + 1)
        val vi = if (gate == 1 and off_i >= 0 and off_i < V_CAP) vs.nth(off_i) else 0
        {
          pvs: st.pvs.append(vi),
          qv: st.qv.append({ q_gate: gate, q_key: off_i, q_v: vi }),
          qs: st.qs.append({ q_gate: gate, q_key: param_i, q_v: vi })
        }
      }
    )

    // Step 7 honest slot witnesses (`decode.rs:534-652`)
    val slot_st = range(0, P_CAP).foldl(
      { slots: [], qv: pub_st.qv },
      (st, ti) => {
        val active = (ti < ent_st.np)
        val match_idx = range(0, B_CAP).foldl(-1, (acc, i) => {
          val p = ent_st.poffs.nth(i)
          val q = ent_st.permss.nth(i)
          if (acc == -1 and p <= ti and ti < p + q) i else acc
        })
        val act_b = if (active and match_idx >= 0) 1 else 0
        val bits = if (act_b == 1) {
          val c = ent_st.op_bits.nth(match_idx)
          [
            c.nth(I_PERM),
            c.nth(I_H2),
            c.nth(I_SPONGE),
            c.nth(I_DIGEST),
            ent_st.se.nth(match_idx)
          ]
        } else [0, 0, 0, 0, 0]
        val s_param = if (act_b == 1) params.nth(match_idx) else 0
        val s_o = if (act_b == 1) ent_st.offs.nth(match_idx) else 0
        val s_poff = if (act_b == 1) ent_st.poffs.nth(match_idx) else 0
        val s_prm = if (act_b == 1) ent_st.permss.nth(match_idx) else 0

        val c_perm = bits.nth(0)
        val c_h2 = bits.nth(1)
        val c_sp = bits.nth(2)
        val c_dig = bits.nth(3)
        val par = bits.nth(4)

        val tm = f_sub(ti, s_poff)
        val rem = f_sub(f_sub(s_prm, tm), 1)
        val tm_inv = mod_inv(tm)
        val rem_inv = mod_inv(rem)
        val last = if (rem == 0) 1 else 0
        val has1 = f_sub(1, f_mul(last, par))
        val sp1 = f_mul(c_sp, has1)
        val spl = f_mul(c_sp, last)

        val g0 = f_mod(c_perm + c_h2 + c_sp + c_dig)
        val g1 = f_mod(c_perm + c_h2 + c_dig + sp1)
        val g2 = f_add(c_perm, c_h2)
        val g3 = f_add(c_perm, spl)
        val i0 = f_add(s_o, f_mul(2, tm))
        val i1 = f_add(i0, 1)
        val i2 = f_add(i0, 2)
        val op3 = f_add(s_o, 3)
        val pm3 = f_mul(c_perm, op3)
        val opp = f_add(s_o, s_param)
        val i3 = f_add(pm3, f_mul(c_sp, opp))
        val i4 = f_add(s_o, 4)
        val i5 = f_add(s_o, 5)

        val y0 = if (g0 == 1 and i0 >= 0 and i0 < V_CAP) vs.nth(i0) else 0
        val y1 = if (g1 == 1 and i1 >= 0 and i1 < V_CAP) vs.nth(i1) else 0
        val y2 = if (g2 == 1 and i2 >= 0 and i2 < V_CAP) vs.nth(i2) else 0
        val y3 = if (g3 == 1 and i3 >= 0 and i3 < V_CAP) vs.nth(i3) else 0
        val y4 = if (c_perm == 1 and i4 >= 0 and i4 < V_CAP) vs.nth(i4) else 0
        val y5 = if (c_perm == 1 and i5 >= 0 and i5 < V_CAP) vs.nth(i5) else 0

        val pd = f_mul(c_dig, s_param)
        val tb = range(0, N_TAGS).foldl([], (acc, k) =>
          acc.append(if (c_dig == 1 and pd == k) 1 else 0)
        )

        val sw: SlotWit = {
          act: act_b,
          bits: bits,
          param: s_param,
          o: s_o,
          poff: s_poff,
          prm: s_prm,
          tm_inv: tm_inv,
          rem_inv: rem_inv,
          ys: [y0, y1, y2, y3, y4, y5],
          tb: tb
        }

        val new_qv = st.qv
          .append({ q_gate: g0, q_key: i0, q_v: y0 })
          .append({ q_gate: g1, q_key: i1, q_v: y1 })
          .append({ q_gate: g2, q_key: i2, q_v: y2 })
          .append({ q_gate: g3, q_key: i3, q_v: y3 })
          .append({ q_gate: c_perm, q_key: i4, q_v: y4 })
          .append({ q_gate: c_perm, q_key: i5, q_v: y5 })

        { slots: st.slots.append(sw), qv: new_qv }
      }
    )

    val mv = range(0, V_CAP).foldl([], (acc, j) => {
      val cnt = range(0, slot_st.qv.length()).foldl(0, (c_acc, qi) => {
        val q = slot_st.qv.nth(qi)
        if (q.q_gate == 1 and q.q_key == j) c_acc + 1 else c_acc
      })
      acc.append(cnt)
    })

    val ms = range(0, LANES).foldl([], (acc, l) => {
      val cnt = range(0, pub_st.qs.length()).foldl(0, (c_acc, qi) => {
        val q = pub_st.qs.nth(qi)
        if (q.q_gate == 1 and q.q_key == l) c_acc + 1 else c_acc
      })
      acc.append(cnt)
    })

    {
      w_words: w,
      lane_borrows: l_borrows,
      vs_borrows: v_borrows,
      op_bits: ent_st.op_bits,
      sponge_q: ent_st.sq,
      sponge_e: ent_st.se,
      g_bits: g,
      j1_inv: j1_inv,
      kj_inv: kj_inv,
      lm_inv: lm_st.invs,
      lm_h: lm_st.hs,
      lm_e: lm_st.es,
      lm_diff_inv: lm_st.diff_invs,
      lm_val: lm_st.vals,
      pub_v: pub_st.pvs,
      slots: slot_st.slots,
      slot_mult: ent_st.permss,
      mv: mv,
      ms: ms
    }
  }

  // =========================================================================
  // 8. Word Vector Builders and Concrete Test Corpus
  // =========================================================================

  pure def build_words(
    mode: int,
    j_limbs: List[int],
    k_limbs: List[int],
    e_in: int,
    e_out: int,
    lm: List[int],
    ents: List[int],
    vs_vals: List[int],
    lane_vals: List[int]
  ): List[int] = {
    val hdr = List(mode)
      .concat(j_limbs)
      .concat(k_limbs)
      .append(e_in)
      .append(e_out)
      .concat(lm)
    val vs_words = range(0, V_CAP).foldl([], (acc, j) =>
      acc.concat(to_limbs(vs_vals.nth(j)))
    )
    val lane_words = range(0, LANES).foldl([], (acc, l) =>
      acc.concat(to_limbs(lane_vals.nth(l)))
    )
    hdr.concat(ents).concat(vs_words).concat(lane_words)
  }

  // Common R entries for mode 0 (entries 0..4, 12 values: vs[0..12], 6 perms: slots[0..6])
  pure val R_ENTS_ORD: List[int] = [
    OP_DIGEST, 0,
    OP_DIGEST, 2,
    OP_DIGEST, 3,
    OP_SPONGE, 3,
    OP_DIGEST, 0
  ]
  pure val R_VS_ORD: List[int] = [
    10, 21,          // entry 0: DIGEST(PROGC, 10) -> 21
    11, 22,          // entry 1: DIGEST(INPUT, 11) -> 22
    12, 23,          // entry 2: DIGEST(OUTCOME, 12) -> 23
    21, 22, 23, 50,  // entry 3: SPONGE(3, [21, 22, 23]) -> 50
    10, 21           // entry 4: DIGEST(PROGC, 10) -> 21
  ]

  // Case 1: Honest ordinary call (mode = 0) with H2, PUB, NOP
  // Entry 5: H2(0) on [7, 8] -> 60 (vs[12..15]); Entry 6: PUB(2) (vs[15] = 50); Entry 7: NOP(0).
  pure val WORDS_HONEST_ORD: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 3, 2],
    R_ENTS_ORD.concat([OP_H2, 0, OP_PUB, 2, OP_NOP, 0]),
    R_VS_ORD.concat([7, 8, 60, 50, 0, 0]),
    [77, 0, 0, 0]
  )
  pure val PW_HONEST_ORD: List[int] = pack_words(PRE_HASH, WORDS_HONEST_ORD)

  // Case 2: Honest PERM + PATH call (mode = 0)
  // Entry 5: PERM(0) on [5, 6, 7] -> [65, 66, 67] (vs[12..18], 1 perm at slot 6).
  // Entry 6..7: NOP(0).
  pure val WORDS_HONEST_PERM: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 0, 0],
    R_ENTS_ORD.concat([OP_PERM, 0, OP_NOP, 0, OP_NOP, 0]),
    R_VS_ORD.concat([5, 6, 7, 65, 66, 67]),
    [77, 0, 0, 9]
  )
  pure val PW_HONEST_PERM: List[int] = pack_words(PRE_HASH, WORDS_HONEST_PERM)

  // Case 3: Honest SPONGE(1), SPONGE(2), PATH(1) call (mode = 0)
  // Entry 5: SPONGE(1) on [9] -> 71 (vs[12..14], slot 6)
  // Entry 6: SPONGE(2) on [7, 8] -> 60 (vs[14..17], slot 7)
  // Entry 7: NOP(0)
  pure val WORDS_HONEST_SPONGE_1_2: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 0, 0],
    R_ENTS_ORD.concat([OP_SPONGE, 1, OP_SPONGE, 2, OP_NOP, 0]),
    R_VS_ORD.concat([9, 71, 7, 8, 60, 0]),
    [77, 0, 0, 9]
  )
  pure val PW_HONEST_SPONGE_1_2: List[int] = pack_words(PRE_HASH, WORDS_HONEST_SPONGE_1_2)

  // Case 4: Honest SPONGE(4) + PUB call (mode = 0, fills all P = 8 slots and V = 18 values)
  // Entry 5: SPONGE(4) on [1, 2, 3, 4] -> 74 (vs[12..17], 2 perms at slots 6..7)
  // Entry 6: PUB(0) on 77 (vs[17] = 77, 0 perms)
  // Entry 7: NOP(0)
  pure val WORDS_HONEST_SPONGE_4: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 0, 0],
    R_ENTS_ORD.concat([OP_SPONGE, 4, OP_PUB, 0, OP_NOP, 0]),
    R_VS_ORD.concat([1, 2, 3, 4, 74, 77]),
    [77, 0, 0, 9]
  )
  pure val PW_HONEST_SPONGE_4: List[int] = pack_words(PRE_HASH, WORDS_HONEST_SPONGE_4)

  // Case 5: Honest chunk call with PATH(1) (mode = 1, j = 2, k = 3)
  // Entries 0..5: R (14 values, 7 perms at slots 0..6)
  // Entry 6: PATH(1) on [10, 1, 99] (vs[14..17], 1 perm at slot 7, not replayed by stock or wrap)
  // Entry 7: PUB(1) on 31 (vs[17] = 31)
  pure val WORDS_HONEST_CHUNK_PATH: List[int] = build_words(
    1, [2, 0], [3, 0], 4, 5,
    [1, 4, 2, 3, 0, 0],
    [
      OP_DIGEST, 0,
      OP_DIGEST, 2,
      OP_DIGEST, 3,
      OP_SPONGE, 3,
      OP_DIGEST, 1,
      OP_DIGEST, 1,
      OP_PATH, 1,
      OP_PUB, 1
    ],
    [10, 21, 11, 22, 12, 23, 21, 22, 23, 50, 10, 31, 14, 34, 10, 1, 99, 31],
    [15, 0, 0, 9]
  )
  pure val PW_HONEST_CHUNK_PATH: List[int] = pack_words(PRE_HASH, WORDS_HONEST_CHUNK_PATH)

  // Case 6: Counterexample for Mutant 1 (`mut_free_mult`)
  // Entry 5 is DIGEST(4) with input 15 and FORGED output vs[13] = 199 != 45 (poff = 6, perms = 1).
  // Entry 6 is PATH(1) at vs[14..17] = [10, 1, 99] (poff = 7, perms = 1).
  // With free multiplicities, the prover sets slot_mult[5] = 0 and slot_mult[6] = 2,
  // placing row_6 (PATH(1), which has no output check) into both slot 6 and slot 7!
  pure val WORDS_BAD_FREE_MULT: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 0, 0],
    R_ENTS_ORD.concat([OP_DIGEST, 4, OP_PATH, 1, OP_NOP, 0]),
    R_VS_ORD.concat([15, 199, 10, 1, 99, 0]),
    [77, 0, 0, 9]
  )
  pure val PW_BAD_FREE_MULT: List[int] = pack_words(PRE_HASH, WORDS_BAD_FREE_MULT)
  pure val WIT_BAD_FREE_MULT: Witness = {
    val hw = honest_witness(PW_BAD_FREE_MULT)
    val path_sw = hw.slots.nth(7)
    val slot6_as_path: SlotWit = {
      ...path_sw,
      tm_inv: mod_inv(f_sub(6, 7)),
      rem_inv: mod_inv(f_sub(f_sub(1, f_sub(6, 7)), 1))
    }
    {
      ...hw,
      slots: hw.slots.replaceAt(6, slot6_as_path),
      slot_mult: [1, 1, 1, 2, 1, 0, 2, 0],
      mv: [1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 0, 0, 0, 0, 0, 0]
    }
  }

  // Case 7: Counterexample for Mutant 2 (`mut_no_m_range`)
  // Entry 5 is SPONGE(1) at o = 12, poff = 6, perms = 1 with input 9 and FORGED output 199 != 71.
  // Entry 6 is PATH(1) at o = 14, poff = 7, perms = 1 with vs[14..17] = [10, 1, 99].
  // The dishonest witness swaps slots 6 and 7: slot 6 holds PATH(1) (row_6) and slot 7 holds SPONGE(1) (row_5).
  // At slot 7, tm = 7 - 6 = 1 and rem = 1 - 1 - 1 = 358 != 0, so `last = 0` and `spl = 0`,
  // gating off the SPONGE output check and accepting the forged SPONGE(1) output 199!
  pure val WORDS_BAD_NO_M_RANGE: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 0, 0],
    R_ENTS_ORD.concat([OP_SPONGE, 1, OP_PATH, 1, OP_NOP, 0]),
    R_VS_ORD.concat([9, 199, 10, 1, 99, 0]),
    [77, 0, 0, 9]
  )
  pure val PW_BAD_NO_M_RANGE: List[int] = pack_words(PRE_HASH, WORDS_BAD_NO_M_RANGE)
  pure val WIT_BAD_NO_M_RANGE: Witness = {
    val hw = honest_witness(PW_BAD_NO_M_RANGE)
    val sw5 = hw.slots.nth(6)
    val sw6 = hw.slots.nth(7)
    // Place sw6 (PATH, poff=7, prm=1) at ti=6: tm = -1 (358), rem = 1
    val slot6_swapped: SlotWit = {
      ...sw6,
      tm_inv: mod_inv(f_sub(6, 7)),
      rem_inv: mod_inv(1)
    }
    // Place sw5 (SPONGE(1), o=12, poff=6, prm=1, par=1) at ti=7:
    // tm = 1, rem = -1 (358) -> first = 0, last = 0 -> has1 = 1 -> g0 = 1 (i0 = 14), g1 = 1 (i1 = 15), g3 = 0!
    val slot7_swapped: SlotWit = {
      ...sw5,
      tm_inv: mod_inv(1),
      rem_inv: mod_inv(f_sub(0, 1)),
      ys: [10, 1, 0, 0, 0, 0]
    }
    {
      ...hw,
      slots: hw.slots.replaceAt(6, slot6_swapped).replaceAt(7, slot7_swapped),
      mv: [1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 0, 0, 1, 1, 0, 0]
    }
  }

  // Case 8: Counterexample for Mutant 3 (`mut_no_inactive_zero`)
  // On PW_HONEST_ORD (np = 7, so slot 7 is inactive with act = 0),
  // the dishonest witness sets pk_7 != 0 on inactive slot 7 with c_sp = 1, o = V_CAP = 18
  // (reading S-table key 18 = V_CAP + 0, where s[0] = 77!), violating both
  // inactive-slot zeroness (Question 1 & 5) and V-read index bounds < off_n <= V_CAP (Question 4).
  pure val WIT_BAD_INACTIVE_ZERO: Witness = {
    val hw = honest_witness(PW_HONEST_ORD)
    // At ti = 7 (act = 0): set bits = [0, 0, 0, 0, 1] (par = 1, all op bits 0) and o = 18, poff = 7, prm = 1:
    // then g0 = g1 = g2 = g3 = 0, no output check, and pk_7 != 0 on inactive slot 7!
    val bad_slot7: SlotWit = {
      act: 0,
      bits: [0, 0, 0, 0, 1],
      param: 5,
      o: 18,
      poff: 7,
      prm: 1,
      tm_inv: 0,
      rem_inv: 0,
      ys: [0, 0, 0, 0, 0, 0],
      tb: [0, 0, 0, 0, 0, 0]
    }
    {
      ...hw,
      slots: hw.slots.replaceAt(7, bad_slot7)
    }
  }

  // Case 9: Counterexample for Mutant 4 (`mut_no_has1`)
  // Entry 3 in R is SPONGE(3) on [21, 22, 23] with FORGED output vs[9] = 55 != 50.
  // Under `mut_no_has1` (has1 = 1 always), block 1 of SPONGE(3) absorbs y1 = vs[9] = 55 into in1:
  // perm([63, 41 + 55, 42]) = perm([63, 96, 42]) = [55, 137, 138], matching forged vs[9] = 55!
  pure val WORDS_BAD_NO_HAS1: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 3, 2],
    R_ENTS_ORD.concat([OP_H2, 0, OP_PUB, 2, OP_NOP, 0]),
    [10, 21, 11, 22, 12, 23, 21, 22, 23, 55, 10, 21, 7, 8, 60, 55, 0, 0],
    [77, 0, 0, 0]
  )
  pure val PW_BAD_NO_HAS1: List[int] = pack_words(PRE_HASH, WORDS_BAD_NO_HAS1)
  pure val WIT_BAD_NO_HAS1: Witness = {
    val hw = honest_witness(PW_BAD_NO_HAS1)
    // At slot 4 (second block of SPONGE(3)), under `mut_no_has1`, g1 = 1 at i1 = 9 (vs[9] = 55)
    val s4 = hw.slots.nth(4)
    val s4_has1: SlotWit = { ...s4, ys: [23, 55, 0, 55, 0, 0] }
    {
      ...hw,
      slots: hw.slots.replaceAt(4, s4_has1),
      mv: hw.mv.replaceAt(9, hw.mv.nth(9) + 1)
    }
  }

  // Case 10: Counterexample for Mutant 5 (`mut_no_chain`)
  // Entry 3 in R is SPONGE(3) on [21, 22, 23] with FORGED output vs[9] = 56 != 50.
  // Under `mut_no_chain` (prev ignored), block 1 computes perm([23, 0, 0]) = [56, 139, 140],
  // matching forged vs[9] = 56!
  pure val WORDS_BAD_NO_CHAIN: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 3, 2],
    R_ENTS_ORD.concat([OP_H2, 0, OP_PUB, 2, OP_NOP, 0]),
    [10, 21, 11, 22, 12, 23, 21, 22, 23, 56, 10, 21, 7, 8, 60, 56, 0, 0],
    [77, 0, 0, 0]
  )
  pure val PW_BAD_NO_CHAIN: List[int] = pack_words(PRE_HASH, WORDS_BAD_NO_CHAIN)
  pure val WIT_BAD_NO_CHAIN: Witness = honest_witness(PW_BAD_NO_CHAIN)

  // Case 11: Counterexample for Mutant 6 (`mut_first_cap_wrong`)
  // Entry 3 in R is SPONGE(3), so to isolate a 1-block sponge where only `first` cap is wrong,
  // we use `mut_first_cap_wrong` on PW_BAD_FIRST_CAP where SPONGE(3) has output 50 under cap 0?
  // Wait: under `mut_first_cap_wrong`, EVERY sponge has cap 0 on block 0!
  // On PW_HONEST_ORD, `mut_first_cap_wrong` rejects honest R entry 3 (violating COMPLETE).
  // And if PERM_TABLE or a 1-block sponge has cap 0: for Entry 3 SPONGE(3), if block 0 has cap 0,
  // `[21, 22, 0] -> [40, 41, 42]` or for Entry 5 SPONGE(1) on `[9]`, `[9, 0, 0] -> [79, 141, 142]`.
  // Even better: in PW_BAD_FIRST_CAP, Entry 3 has inputs [21, 22, 23] and Entry 5 is SPONGE(1) on [9]
  // with forged output 79 != 71! Wait: for Entry 3 to also pass under `mut_first_cap_wrong`,
  // block 0 of Entry 3 has input `[21, 22, 0]`. In `PERM_TABLE`, `[21, 22, 0]` is not listed so it maps to `[300, 301, 302]`,
  // and block 1 maps `[300 + 23, 301, 302]` to `[300, 301, 302]`, so setting `vs[9] = 300` makes Entry 3 pass under `mut_first_cap_wrong`!
  pure val WORDS_BAD_FIRST_CAP: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 3, 2],
    R_ENTS_ORD.concat([OP_SPONGE, 1, OP_PUB, 2, OP_NOP, 0]),
    [10, 21, 11, 22, 12, 23, 21, 22, 23, 300, 10, 21, 9, 79, 300, 0, 0, 0],
    [77, 0, 0, 0]
  )
  pure val PW_BAD_FIRST_CAP: List[int] = pack_words(PRE_HASH, WORDS_BAD_FIRST_CAP)
  pure val WIT_BAD_FIRST_CAP: Witness = honest_witness(PW_BAD_FIRST_CAP)

  // Case 12: Counterexample for Mutant 7 (`mut_ungated_y1`)
  // Entry 5 is SPONGE(1) on [9] with FORGED output vs[13] = 78 != 71.
  // On slot 6 (SPONGE(1): first = 1, last = 1, par = 1 -> has1 = 0 -> g1 = 0),
  // the query `(g1 = 0, i1 = 13, y1)` has gate 0, so witness `y1` is unconstrained by the V lookup!
  // The dishonest prover sets `ys[1] = 5` on slot 6; under `mut_ungated_y1`, `y1 = 5` enters `in1`,
  // giving `perm([9, 5, 1]) = [78, 143, 144]`, matching the forged output 78!
  pure val WORDS_BAD_UNGATED_Y1: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 0, 0],
    R_ENTS_ORD.concat([OP_SPONGE, 1, OP_NOP, 0, OP_NOP, 0]),
    R_VS_ORD.concat([9, 78, 0, 0, 0, 0]),
    [77, 0, 0, 9]
  )
  pure val PW_BAD_UNGATED_Y1: List[int] = pack_words(PRE_HASH, WORDS_BAD_UNGATED_Y1)
  pure val WIT_BAD_UNGATED_Y1: Witness = {
    val hw = honest_witness(PW_BAD_UNGATED_Y1)
    val s6 = hw.slots.nth(6)
    val s6_ungated: SlotWit = { ...s6, ys: [9, 5, 0, 78, 0, 0] }
    {
      ...hw,
      slots: hw.slots.replaceAt(6, s6_ungated)
    }
  }

  // Catalog of (pw, wit) pairs for state exploration
  pure val CORPUS: List[{ pw: List[int], wit: Witness }] = [
    { pw: PW_HONEST_ORD, wit: honest_witness(PW_HONEST_ORD) },
    { pw: PW_HONEST_PERM, wit: honest_witness(PW_HONEST_PERM) },
    { pw: PW_HONEST_SPONGE_1_2, wit: honest_witness(PW_HONEST_SPONGE_1_2) },
    { pw: PW_HONEST_SPONGE_4, wit: honest_witness(PW_HONEST_SPONGE_4) },
    { pw: PW_HONEST_CHUNK_PATH, wit: honest_witness(PW_HONEST_CHUNK_PATH) },
    { pw: PW_BAD_FREE_MULT, wit: WIT_BAD_FREE_MULT },
    { pw: PW_BAD_NO_M_RANGE, wit: WIT_BAD_NO_M_RANGE },
    { pw: PW_HONEST_ORD, wit: WIT_BAD_INACTIVE_ZERO },
    { pw: PW_BAD_NO_HAS1, wit: WIT_BAD_NO_HAS1 },
    { pw: PW_BAD_NO_CHAIN, wit: WIT_BAD_NO_CHAIN },
    { pw: PW_BAD_FIRST_CAP, wit: WIT_BAD_FIRST_CAP },
    { pw: PW_BAD_UNGATED_Y1, wit: WIT_BAD_UNGATED_Y1 }
  ]

  // =========================================================================
  // 9. Soundness & Completeness Predicates, State Machine, and Invariants
  // =========================================================================

  // Checks Question 1 & Question 5: every active slot t < np holds the exact tuple of its
  // unique entry i with poff_i <= t < poff_i + perms_i, and every inactive slot t >= np holds pk_t == 0.
  pure def slots_placed_exact(pw: List[int], wit: Witness): bool = {
    val hw = honest_witness(pw)
    0.to(P_CAP - 1).forall(ti => {
      val sw = wit.slots.nth(ti)
      val hsw = hw.slots.nth(ti)
      val pk = pack_tup(sw.bits, sw.param, sw.o, sw.poff, sw.prm)
      val hpk = pack_tup(hsw.bits, hsw.param, hsw.o, hsw.poff, hsw.prm)
      sw.act == hsw.act and pk == hpk and (sw.act == 1 or pk == 0)
    })
  }

  // Pure soundness predicate on a (pw, wit) pair against stock reference decode:
  pure def is_sound_on(pw: List[int], wit: Witness): bool = {
    val ref_r = ref_decode(pw)
    constraints_ok(pw, wit) implies (
      ref_r.ok and
      circuit_output(pw, wit) == ref_r.stmt and
      slots_placed_exact(pw, wit)
    )
  }

  // Pure completeness predicate on a pw vector against stock reference decode:
  pure def is_complete_on(pw: List[int]): bool = {
    val ref_r = ref_decode(pw)
    val hw = honest_witness(pw)
    ref_r.ok implies (constraints_ok(pw, hw) and circuit_output(pw, hw) == ref_r.stmt)
  }

  var cur_pw: List[int]
  var cur_wit: Witness

  action init = {
    nondet idx = oneOf(0.to(CORPUS.length() - 1))
    val c = CORPUS.nth(idx)
    all {
      cur_pw' = c.pw,
      cur_wit' = c.wit,
    }
  }

  action step_corpus = {
    nondet idx = oneOf(0.to(CORPUS.length() - 1))
    val c = CORPUS.nth(idx)
    all {
      cur_pw' = c.pw,
      cur_wit' = c.wit,
    }
  }

  pure def perturb(hw: Witness, which: int, k: int, x: int): Witness =
    if (which == 1) { ...hw, lm_inv: hw.lm_inv.replaceAt(k % LANE_PAIRS, x) }
    else if (which == 2) { ...hw, mv: hw.mv.replaceAt(k % V_CAP, x) }
    else if (which == 3) { ...hw, ms: hw.ms.replaceAt(k % LANES, x) }
    else if (which == 4) { ...hw, pub_v: hw.pub_v.replaceAt(k % B_CAP, x) }
    else if (which == 5) { ...hw, lm_val: hw.lm_val.replaceAt(k % LANE_PAIRS, x) }
    else if (which == 6) { ...hw, j1_inv: x }
    else if (which == 7) { ...hw, kj_inv: x }
    else hw

  action step_fuzz = {
    nondet idx = oneOf(0.to(CORPUS.length() - 1))
    nondet j = oneOf(0.to(N_WORDS - 1))
    nondet v = oneOf(0.to(BB_P - 1))
    nondet which = oneOf(0.to(7))
    nondet k = oneOf(0.to(7))
    nondet x = oneOf(0.to(FR_MOD - 1))
    val w0 = honest_witness(CORPUS.nth(idx).pw).w_words
    val pw1 = pack_words(PRE_HASH, w0.replaceAt(j, v))
    all {
      cur_pw' = pw1,
      cur_wit' = perturb(honest_witness(pw1), which, k, x),
    }
  }

  action step = any { step_corpus, step_fuzz }

  // State invariants for `quint run`
  val SOUND: bool = is_sound_on(cur_pw, cur_wit)
  val COMPLETE: bool = is_complete_on(cur_pw)
  val SOUND_AND_COMPLETE: bool = (SOUND and COMPLETE)

  // Mutant soundness invariants (each fails on its corresponding mutant in `quint run`)
  val SOUND_mut_free_mult: bool = {
    val ref_r = ref_decode(cur_pw)
    mut_free_mult(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_no_m_range: bool = {
    val ref_r = ref_decode(cur_pw)
    mut_no_m_range(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_no_inactive_zero: bool = {
    val ref_r = ref_decode(cur_pw)
    mut_no_inactive_zero(cur_pw, cur_wit) implies (
      ref_r.ok and
      circuit_output(cur_pw, cur_wit) == ref_r.stmt and
      slots_placed_exact(cur_pw, cur_wit)
    )
  }
  val SOUND_mut_no_has1: bool = {
    val ref_r = ref_decode(cur_pw)
    mut_no_has1(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_no_chain: bool = {
    val ref_r = ref_decode(cur_pw)
    mut_no_chain(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_first_cap_wrong: bool = {
    val ref_r = ref_decode(cur_pw)
    mut_first_cap_wrong(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_ungated_y1: bool = {
    val ref_r = ref_decode(cur_pw)
    mut_ungated_y1(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }

}
