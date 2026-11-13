// -*- mode: Bluespec; -*-
//
// Round 2 soundness model of the Sherlock E5 wrap in-circuit D-9 decoder
// (`lane-src/decode.rs`) vs stock reference (`lane-src/iface.rs`, e3-iface 9539418),
// modeled at reduced parameters over F_359.

module decode {

  // =========================================================================
  // 1. Reduced-Scale Parameters
  // =========================================================================

  // Prime scalar field modulus r = 359 (360 - 1).
  // R = r - 1 = 358 = 6 + 4*8 + 5*64 in base 8 (3 limbs of 3 bits).
  pure val FR_MOD: int = 359
  pure val R1_LIMBS: List[int] = [6, 4, 5]

  // Word packing: 2 words per packed Fr element, base 18, BabyBear modulus BB_P = 17.
  // Max packed value for 2 words is 17 + 17*18 = 323 < FR_MOD = 359 (no modular wrap).
  pure val WORD_BASE: int = 18
  pure val PACK_K: int = 2
  pure val BB_P: int = 17

  // Limbs: 3 limbs of 3 bits (base 8 < BB_P = 17 < WORD_BASE = 18).
  // Max 3-limb integer is 8^3 - 1 = 511 >= FR_MOD = 359 (Canon check is active).
  pure val LIMBS: int = 3
  pure val LIMB_BASE: int = 8

  // Statement header layout (13 words):
  // word 0:      mode (0 or 1)
  // words 1..2:  j (2 u16 limbs, low first)
  // words 3..4:  k (2 u16 limbs, low first)
  // word 5:      e_in (anchor in entry index, 4)
  // word 6:      e_out (anchor out entry index, 4 + mode)
  // words 7..12: lane_map (3 pairs of (lane, ent))
  pure val MODE_WORD: int = 0
  pure val J_WORDS: int = 1
  pure val K_WORDS: int = 3
  pure val E_IN_WORD: int = 5
  pure val E_OUT_WORD: int = 6
  pure val LANE_MAP: int = 7
  pure val LANE_PAIRS: int = 3
  pure val STMT: int = 13

  // Capacities: B = 7 entries (holds R of 5 or 6 entries + discretionary entries),
  // V = 16 values (holds R of 12 or 14 values + discretionary values),
  // P = 10 permutation slots, 4 call-vector lanes.
  pure val B_CAP: int = 7
  pure val V_CAP: int = 16
  pure val P_CAP: int = 10
  pure val LANES: int = 4

  pure val VALUES_AT: int = 27   // STMT + 2 * B_CAP = 13 + 14
  pure val CALL_AT: int = 75     // VALUES_AT + LIMBS * V_CAP = 27 + 48
  pure val N_WORDS: int = 87     // CALL_AT + LIMBS * LANES = 75 + 12
  pure val N_PUBLIC: int = 45    // 1 + ceil(87 / 2) = 1 + 44

  pure val PRE_HASH: int = 42

  // DIGEST tags (N_TAGS = 6):
  // 0: SHK_PROGC, 1: SHK_LOCK, 2: SHK_INPUT, 3: SHK_OUTCOME, 4: SHK_REG, 5: SHK_SET
  pure val SHK_PROGC: int = 0
  pure val SHK_LOCK: int = 1
  pure val SHK_INPUT: int = 2
  pure val SHK_OUTCOME: int = 3
  pure val SHK_REG: int = 4
  pure val SHK_SET: int = 5
  pure val N_TAGS: int = 6
  pure val TAG_FR_TABLE: List[int] = [101, 113, 127, 139, 151, 163]

  // Reservation R constants (plan D-10)
  pure val R_RUN: List[int] = [0, 2, 3]
  pure val R_OFF: List[int] = [0, 2, 4, 6, 10, 12]
  pure val ANCHOR_IN: int = 4

  pure val PATH_MAX: int = 4
  pure val PERM_VALS: int = 6

  // Range check bounds in the circuit (scaled analogs of 2^11, 2^16, 2^32, 2^17)
  pure val BOUND_LIM: int = 16
  pure val U16_LIM: int = 8
  pure val U32_LIM: int = 64
  pure val SPONGE_Q_LIM: int = 16
  pure val CAP_LIM: int = 32

  // Operation codes (matching iface.rs / decode.rs)
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

  type HashRow = {
    u_code: int,
    ins: List[int],
    out_v: int
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
    lm_x0: List[int],
    lm_x1: List[int],
    lm_x2: List[int],
    lm_tb: List[List[int]],
    pub_v: List[int],
    mv: List[int],
    ms: List[int]
  }

  type Flags = {
    check_sponge3_param: bool,
    use_sponge_idx: bool,
    check_mapped_twice: bool,
    check_u16: bool,
    check_mapped_replay: bool,
    gate_t_in_by_mode: bool,
    check_claim_pub: bool,
    check_chunk_range: bool
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
    check_sponge3_param: true,
    use_sponge_idx: true,
    check_mapped_twice: true,
    check_u16: true,
    check_mapped_replay: true,
    gate_t_in_by_mode: true,
    check_claim_pub: true,
    check_chunk_range: true
  }

  // =========================================================================
  // 3. Field, Hash Table, and Arithmetic Helpers
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

  // Inverse mod FR_MOD by Fermat: x^(FR_MOD - 2), square and multiply over 9 bits (FR_MOD < 512).
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

  // Uninterpreted injective hash table for sponge_u(u_code, ins).
  // All rows have pairwise distinct (u_code, ins) and pairwise distinct out_v in 0..250.
  pure val HASH_TABLE: List[HashRow] = [
    // Run DIGESTs: SHK_PROGC(0), SHK_INPUT(2), SHK_OUTCOME(3)
    { u_code: 4, ins: [101, 10], out_v: 21 },
    { u_code: 4, ins: [127, 11], out_v: 22 },
    { u_code: 4, ins: [139, 12], out_v: 23 },
    // Run SPONGE(3) claim_pub over [21, 22, 23]
    { u_code: 0, ins: [21, 22, 23], out_v: 50 },
    // Anchor DIGESTs under SHK_PROGC (tag 101) and SHK_LOCK (tag 113)
    { u_code: 4, ins: [101, 14], out_v: 24 },
    { u_code: 4, ins: [101, 42], out_v: 0 },
    { u_code: 4, ins: [113, 10], out_v: 31 },
    { u_code: 4, ins: [113, 14], out_v: 34 },
    // Discretionary DIGESTs under SHK_REG (tag 151) and SHK_SET (tag 163)
    { u_code: 4, ins: [151, 15], out_v: 45 },
    { u_code: 4, ins: [163, 16], out_v: 46 },
    // Discretionary SPONGE(2) / H2 and SPONGE(3) entries
    { u_code: 0, ins: [7, 8], out_v: 60 },
    { u_code: 0, ins: [7, 8, 60], out_v: 61 },
    { u_code: 0, ins: [1, 20, 2], out_v: 20 },
    { u_code: 0, ins: [99, 22, 23], out_v: 59 }
  ]

  pure def hash_slice_fn(u_code: int, ins: List[int]): int =
    range(0, HASH_TABLE.length()).foldl(300, (acc, i) => {
      val row = HASH_TABLE.nth(i)
      if (row.u_code == u_code and row.ins == ins) row.out_v else acc
    })

  pure def digest_fn(tag_v: int, d2: int): int =
    hash_slice_fn(4, [tag_v, d2])

  pure def anchor_tag_fn(t_bit: int): int =
    f_add(f_mul(f_sub(tag_fr(SHK_LOCK), tag_fr(SHK_PROGC)), t_bit), tag_fr(SHK_PROGC))

  // Encode an integer into 3 base-8 limbs (little-endian)
  pure def to_limbs(x: int): List[int] = [
    x % LIMB_BASE,
    (x / LIMB_BASE) % LIMB_BASE,
    (x / (LIMB_BASE * LIMB_BASE)) % LIMB_BASE
  ]

  // Reconstruct integer from 3 base-8 limbs
  pure def limbs_int(ls: List[int]): int =
    ls.nth(0) + ls.nth(1) * LIMB_BASE + ls.nth(2) * LIMB_BASE * LIMB_BASE

  // Pack 87 words into 45 public inputs (pw[0] = pre_hash, then 2 words per Fr)
  pure def pack_words(pre_hash: int, w: List[int]): List[int] = {
    val packed = range(0, 43 + 1).foldl([], (acc, i) => {
      val w0 = w.nth(2 * i)
      val w1 = if (2 * i + 1 < N_WORDS) w.nth(2 * i + 1) else 0
      acc.append(w0 + w1 * WORD_BASE)
    })
    List(pre_hash).concat(packed)
  }

  // =========================================================================
  // 4. Native Reference (`lane-src/iface.rs`)
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

  // Reference unpack (`iface::unpack` + pinned pre_hash check on pw[0])
  pure def ref_unpack(pw: List[int]): { ok: bool, words: List[int], err: str } = {
    if (pw.length() != N_PUBLIC) {
      { ok: false, words: [], err: "word count" }
    } else if (pw.nth(0) != PRE_HASH) {
      { ok: false, words: [], err: "pre-hash" }
    } else {
      range(0, 43 + 1).foldl({ ok: true, words: [], err: "" }, (st, i) => {
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

  // Reference dec (`iface::dec`): every limb < LIMB_BASE and integer < FR_MOD
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

  // Reference anchor_tags (`iface::anchor_tags`)
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

  // Reference r_entries (`iface::r_entries`)
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

  // Native replay of one entry (op, param) at offset o in vs (`iface::decode` lines 653-665)
  pure def entry_replay_ok(op: int, param: int, o: int, vs: List[int]): bool = {
    val needed = op_vals(op, param)
    if (o < 0 or needed < 0 or o + needed > vs.length()) false
    else if (op == OP_DIGEST) {
      digest_fn(tag_fr(param), vs.nth(o)) == vs.nth(o + 1)
    } else if (op == OP_SPONGE) {
      if (param < 1) false
      else {
        val inputs = vs.slice(o, o + param)
        hash_slice_fn(0, inputs) == vs.nth(o + param)
      }
    } else if (op == OP_H2) {
      hash_slice_fn(0, [vs.nth(o), vs.nth(o + 1)]) == vs.nth(o + 2)
    } else if (op == OP_PERM) {
      val y0 = hash_slice_fn(1, [vs.nth(o), vs.nth(o + 1), vs.nth(o + 2)])
      vs.nth(o + 3) == y0 and vs.nth(o + 4) == 0 and vs.nth(o + 5) == 0
    } else {
      true
    }
  }

  // Reference decode (`iface::decode` with flag `replay_all`:
  // when `replay_all` is true, stock replays every entry;
  // when `replay_all` is false, the wrap replays only R and the mapped entries).
  pure def ref_decode(pw: List[int], replay_all: bool): DecodeRes = {
    val u = ref_unpack(pw)
    if (not(u.ok)) {
      { ok: false, stmt: [], err: u.err }
    } else {
      val w = u.words
      // 1. Decode lanes
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
        // 2. Decode logged values vs
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
          // 3. Entries loop
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
            // Zero padding of vs after off_n
            val pad_ok = 0.to(V_CAP - 1).forall(j =>
              (j < ent_res.off) or (vs.nth(j) == 0)
            )
            if (not(pad_ok)) {
              { ok: false, stmt: [], err: "value row after off_n not zero" }
            } else {
              // 4. R and the anchor rule
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
                      // Replay of R (and all entries when replay_all is true)
                      val r_len = r_list.length()
                      val r_replay_ok = 0.to(B_CAP - 1).forall(i => {
                        val op = w.nth(STMT + 2 * i)
                        val param = w.nth(STMT + 2 * i + 1)
                        val o = ent_res.offs.nth(i)
                        if (replay_all or i < r_len) entry_replay_ok(op, param, o, vs)
                        else true
                      })
                      if (not(r_replay_ok)) {
                        { ok: false, stmt: [], err: "entry: native replay" }
                      } else {
                        // 5. Lane map (`iface::apply_lane_map`) + replay of mapped entries
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
                                } else if (not(entry_replay_ok(op_e, param_e, o_e, vs))) {
                                  { ...st, ok: false, err: "lane map entry: native replay" }
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
                          // 6. PUB entries check against s after lane map
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

  // Predicate checking whether all entries outside R and outside the active lane map
  // pass native replay (used to verify that this is the ONLY difference between
  // `replay_all == true` and `replay_all == false`).
  pure def unmapped_outside_r_replay_ok(pw: List[int]): bool = {
    val u = ref_unpack(pw)
    if (not(u.ok)) true
    else {
      val w = u.words
      val vs = range(0, V_CAP).foldl([], (acc, j) => {
        val at = VALUES_AT + LIMBS * j
        acc.append(limbs_int(w.slice(at, at + LIMBS)))
      })
      val offs = range(0, B_CAP).foldl({ off: 0, xs: [] }, (st, i) => {
        val op = w.nth(STMT + 2 * i)
        val param = w.nth(STMT + 2 * i + 1)
        { off: st.off + op_vals(op, param), xs: st.xs.append(st.off) }
      }).xs
      val r_len = 5 + w.nth(MODE_WORD)
      0.to(B_CAP - 1).forall(i => {
        val in_r = (i < r_len)
        val in_lm = 0.to(LANE_PAIRS - 1).exists(k => {
          val lane = w.nth(LANE_MAP + 2 * k)
          val ent = w.nth(LANE_MAP + 2 * k + 1)
          not(lane == 0 and ent == 0) and ent == i
        })
        if (in_r or in_lm) true
        else {
          val op = w.nth(STMT + 2 * i)
          val param = w.nth(STMT + 2 * i + 1)
          entry_replay_ok(op, param, offs.nth(i), vs)
        }
      })
    }
  }

  // =========================================================================
  // 5. In-Circuit Constraint System (`lane-src/decode.rs`)
  // =========================================================================

  // `[x == 0]` gadget (`decode::is_zero`): `z = 1 - x * inv`, `x * z == 0`.
  pure def check_is_zero(x: int, inv: int): { ok: bool, z_bit: int } = {
    val xi = f_mul(x, inv)
    val z = f_sub(1, xi)
    val xz = f_mul(x, z)
    { ok: (xz == 0), z_bit: z }
  }

  // Gated one-hot constraint (`decode::one_hot`):
  // bs are bits, sum(bs) == gate (mod r), sum_i (i * bs[i]) == idx (mod r).
  pure def check_one_hot(n: int, bs: List[int], idx: int, gate: int): bool = {
    bs.length() == n and
    0.to(n - 1).forall(i => is_bit(bs.nth(i))) and
    f_sum_list(bs) == f_mod(gate) and
    range(0, n).foldl(0, (acc, i) => f_add(acc, f_mul(i, bs.nth(i)))) == f_mod(idx)
  }

  // Borrow chain constraint (`decode::dec`):
  // d_k = R_k - l_k - c_{k-1} + LIMB_BASE * c_k (mod r), d_k in [0, LIMB_BASE), c_{LIMBS-1} == 0.
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

  // Reconstruct Fr value from limbs inside the circuit (`decode::dec` linear combination)
  pure def circuit_dec_v(ls: List[int]): int =
    f_mod(limbs_int(ls))

  // Compute circuit output vector `s` from witness
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

  // Symbolic log-derivative lookup check
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
      wit.lm_x0.length() == LANE_PAIRS and
      wit.lm_x1.length() == LANE_PAIRS and
      wit.lm_x2.length() == LANE_PAIRS and
      wit.lm_tb.length() == LANE_PAIRS and
      wit.pub_v.length() == B_CAP and
      wit.mv.length() == V_CAP and
      wit.ms.length() == LANES and
      0.to(LANE_PAIRS - 1).forall(k =>
        is_fr(wit.lm_inv.nth(k)) and
        is_fr(wit.lm_val.nth(k)) and
        is_fr(wit.lm_x0.nth(k)) and
        is_fr(wit.lm_x1.nth(k)) and
        is_fr(wit.lm_x2.nth(k)) and
        wit.lm_diff_inv.nth(k).length() == LANE_PAIRS and
        0.to(LANE_PAIRS - 1).forall(i => is_fr(wit.lm_diff_inv.nth(k).nth(i)))
      ) and
      0.to(B_CAP - 1).forall(i => is_fr(wit.sponge_q.nth(i)) and is_fr(wit.pub_v.nth(i))) and
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
      val unpack_lin_ok = 0.to(43).forall(i => {
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

      // 3. Entries constraints
      val entries_step = range(0, B_CAP).foldl(
        { ok: true, off: 0, np: 0, offs: [] },
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
              offs: st.offs.append(st.off)
            }
          }
        }
      )

      val cap_v = f_sub(V_CAP, entries_step.off)
      val cap_p = f_sub(P_CAP, entries_step.np)
      val cap_ok = (in_bound(cap_v, CAP_LIM) and in_bound(cap_p, CAP_LIM))

      // Zero padding after off_n: monotone g_bits with sum(g) == cap_v and g[j] * vs[j] == 0
      val g = wit.g_bits
      val g_bits_ok = 0.to(V_CAP - 1).forall(j => is_bit(g.nth(j)))
      val g_mono_ok = 1.to(V_CAP - 1).forall(j => f_mul(g.nth(j - 1), g.nth(j)) == g.nth(j - 1))
      val g_zero_ok = 0.to(V_CAP - 1).forall(j => f_mul(g.nth(j), vs.nth(j)) == 0)
      val g_sum_ok = (f_sum_list(g) == cap_v)

      // 4. R and the anchor rule at static positions (`decode.rs:387-450`)
      val mode = w.nth(MODE_WORD)
      val mode_bit_ok = (f_sub(f_mul(mode, mode), mode) == 0)
      val jk_u16_ok = (
        not(fl.check_u16) or
        0.to(3).forall(i => in_bound(w.nth(J_WORDS + i), U16_LIM))
      )
      val j_val = f_add(w.nth(J_WORDS), f_mul(U16_LIM, w.nth(J_WORDS + 1)))
      val k_val = f_add(w.nth(K_WORDS), f_mul(U16_LIM, w.nth(K_WORDS + 1)))
      val j1 = f_sub(j_val, 1)
      val kj = f_sub(k_val, j_val)
      val chunk_range_ok = (
        not(fl.check_chunk_range) or
        (in_bound(f_mul(mode, j1), U32_LIM) and in_bound(f_mul(mode, kj), U32_LIM))
      )
      val iz_j1 = check_is_zero(j1, wit.j1_inv)
      val iz_jk = check_is_zero(kj, wit.kj_inv)
      val eq_j1 = iz_j1.z_bit
      val eq_jk = iz_jk.z_bit
      val me = f_mul(mode, eq_j1)
      val t_in = if (fl.gate_t_in_by_mode) f_sub(mode, me) else f_sub(1, eq_j1)
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
      val claim_pub_ok = (
        not(fl.check_claim_pub) or
        0.to(2).forall(i => vs.nth(sp_off + i) == vs.nth(R_OFF.nth(i) + 1))
      )

      val r_replay_ok = (
        0.to(2).forall(i => {
          val t = R_RUN.nth(i)
          val y = digest_fn(tag_fr(t), vs.nth(R_OFF.nth(i)))
          y == vs.nth(R_OFF.nth(i) + 1)
        }) and
        hash_slice_fn(0, [vs.nth(sp_off), vs.nth(sp_off + 1), vs.nth(sp_off + 2)]) == vs.nth(sp_off + 3) and
        digest_fn(anchor_tag_fn(t_in), vs.nth(R_OFF.nth(4))) == vs.nth(R_OFF.nth(4) + 1) and
        f_mul(mode, f_sub(digest_fn(anchor_tag_fn(t_out), vs.nth(R_OFF.nth(5))), vs.nth(R_OFF.nth(5) + 1))) == 0
      )

      val step4_ok = (
        mode_bit_ok and jk_u16_ok and chunk_range_ok and
        iz_j1.ok and iz_jk.ok and r_ops_ok and
        anchor_words_ok and claim_pub_ok and r_replay_ok
      )

      // 5. Lane map, in stock order (`decode.rs:451-530`)
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
          val u16_ok = (not(fl.check_u16) or (in_bound(lane, U16_LIM) and in_bound(ent, U16_LIM)))
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
          val sp3_ok = (not(fl.check_sponge3_param) or f_mul(sel_sp, f_sub(p, 3)) == 0)
          val taken_ok = (h_ok and f_dot(hk, st.s_vec) == 0)
          val twice_ok = (
            not(fl.check_mapped_twice) or
            0.to(LANE_PAIRS - 1).forall(i => {
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
          )
          val o = if (e_ok and entries_step.offs.length() == B_CAP) f_dot(ek, entries_step.offs) else 0
          val idx = if (fl.use_sponge_idx) f_add(o, f_add(1, f_mul(2, sel_sp))) else f_add(o, 1)
          val vk = wit.lm_val.nth(k)
          val x0 = wit.lm_x0.nth(k)
          val i1 = f_add(o, 1)
          val i2 = f_add(o, 2)
          val x1 = wit.lm_x1.nth(k)
          val x2 = wit.lm_x2.nth(k)
          val pd = f_mul(sel_dig, p)
          val tb = wit.lm_tb.nth(k)
          val tb_ok = check_one_hot(N_TAGS, tb, pd, sel_dig)
          val tag_v = if (tb_ok) f_dot(TAG_FR_TABLE, tb) else 0
          val yd = digest_fn(tag_v, x0)
          val ys = hash_slice_fn(0, [x0, x1, x2])
          val replay_ok = (
            not(fl.check_mapped_replay) or
            (f_mul(sel_dig, f_sub(yd, vk)) == 0 and f_mul(sel_sp, f_sub(ys, vk)) == 0)
          )
          val updated_s = if (h_ok) {
            range(0, LANES).foldl([], (acc, l) =>
              acc.append(f_add(st.s_vec.nth(l), f_mul(hk.nth(l), vk)))
            )
          } else st.s_vec
          val new_qv = st.qv
            .append({ q_gate: nz, q_key: idx, q_v: vk })
            .append({ q_gate: nz, q_key: o, q_v: x0 })
            .append({ q_gate: sel_sp, q_key: i1, q_v: x1 })
            .append({ q_gate: sel_sp, q_key: i2, q_v: x2 })
          {
            ok: (st.ok and u16_ok and zu_ok and h_ok and e_ok and ds_ok and
                 sp3_ok and taken_ok and twice_ok and tb_ok and replay_ok),
            s_vec: updated_s,
            qv: new_qv
          }
        }
      )

      // 6. PUB entries and log-derivative lookup
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

      val table_v = range(0, V_CAP).foldl([], (acc, j) =>
        acc.append({ t_key: j, t_v: vs.nth(j), t_mult: wit.mv.nth(j) })
      )
      val table_s = range(0, LANES).foldl([], (acc, l) =>
        acc.append({ t_key: f_add(V_CAP, l), t_v: lm_step.s_vec.nth(l), t_mult: wit.ms.nth(l) })
      )
      val all_queries = pub_queries.qv.concat(pub_queries.qs)
      val all_table = table_v.concat(table_s)
      val lookup_ok = check_lookup(all_queries, all_table)

      pre_hash_ok and unpack_range_ok and unpack_lin_ok and
      lanes_canon_ok and vs_canon_ok and entries_step.ok and
      cap_ok and g_bits_ok and g_mono_ok and g_zero_ok and g_sum_ok and
      step4_ok and lm_step.ok and lookup_ok
    }
  }

  // Real constraint system predicate
  pure def constraints_ok(pw: List[int], wit: Witness): bool =
    constraints_with_flags(DEFAULT_FLAGS, pw, wit)

  // =========================================================================
  // 6. Mutant Constraint Predicates (8 Mutants)
  // =========================================================================

  // Mutant 1: drops `sel_sp * (p - 3) == 0` in lane map (`decode.rs:485-487`)
  pure def mut_no_sponge3_param(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, check_sponge3_param: false }, pw, wit)

  // Mutant 2: uses output index `o + 1` for every op instead of `o + 1 + 2 * sel_sp` (`decode.rs:503`)
  pure def mut_idx_plus1(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, use_sponge_idx: false }, pw, wit)

  // Mutant 3: drops pairwise duplicate-lane check `nz_i * nz_j * ((lane_i - lane_j) * inv - 1) == 0` (`decode.rs:492-499`)
  pure def mut_no_twice(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, check_mapped_twice: false }, pw, wit)

  // Mutant 4: drops u16 range checks (`< U16_LIM`) on j, k and lane-map words (`decode.rs:393-395, 469-470`)
  pure def mut_no_u16(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, check_u16: false }, pw, wit)

  // Mutant 5: drops native replay of mapped entries `sel_dig * (yd - val) == 0` and `sel_sp * (ys - val) == 0` (`decode.rs:517-522`)
  pure def mut_no_mapped_replay(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, check_mapped_replay: false }, pw, wit)

  // Mutant 6: computes `t_in = 1 - [j == 1]` without gating by `mode` (`decode.rs:408-409`)
  pure def mut_t_in_wrong(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, gate_t_in_by_mode: false }, pw, wit)

  // Mutant 7: drops `claim_pub` input checks `vs[sp + i] == vs[R_OFF[i] + 1]` (`decode.rs:428-431`)
  pure def mut_no_claim_pub(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, check_claim_pub: false }, pw, wit)

  // Mutant 8: drops gated chunk range checks `mode * (j - 1) < 2^32` and `mode * (k - j) < 2^32` (`decode.rs:400-404`)
  pure def mut_no_chunk_range(pw: List[int], wit: Witness): bool =
    constraints_with_flags({ ...DEFAULT_FLAGS, check_chunk_range: false }, pw, wit)

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
    // 1. Unpack words from pw[1..44]
    val w = range(0, 43 + 1).foldl([], (acc, i) => {
      val x = pw.nth(i + 1)
      val w0 = x % WORD_BASE
      val w1 = (x / WORD_BASE) % WORD_BASE
      if (2 * i + 1 < N_WORDS) acc.append(w0).append(w1)
      else acc.append(w0)
    })

    // 2. Canon borrows and values for lanes and vs
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

    // 3. Entries witnesses and offsets
    val ent_st = range(0, B_CAP).foldl(
      { op_bits: [], sq: [], se: [], off: 0, offs: [] },
      (st, i) => {
        val op = w.nth(STMT + 2 * i)
        val param = w.nth(STMT + 2 * i + 1)
        val c = range(0, 6 + 1).foldl([], (acc, k) =>
          acc.append(if (op == ADMITTED.nth(k)) 1 else 0)
        )
        val q = (param + 1) / 2
        val e = param % 2
        val dv = op_vals(op, param)
        {
          op_bits: st.op_bits.append(c),
          sq: st.sq.append(q),
          se: st.se.append(e),
          off: st.off + dv,
          offs: st.offs.append(st.off)
        }
      }
    )

    val g = range(0, V_CAP).foldl([], (acc, j) =>
      acc.append(if (j >= ent_st.off) 1 else 0)
    )

    // 4. Step 4 `is_zero` inverses on j - 1 and k - j
    val j_val = f_add(w.nth(J_WORDS), f_mul(U16_LIM, w.nth(J_WORDS + 1)))
    val k_val = f_add(w.nth(K_WORDS), f_mul(U16_LIM, w.nth(K_WORDS + 1)))
    val j1_inv = mod_inv(f_sub(j_val, 1))
    val kj_inv = mod_inv(f_sub(k_val, j_val))

    // 5. Lane-map witnesses
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
      { invs: [], hs: [], es: [], diff_invs: [], vals: [], x0s: [], x1s: [], x2s: [], tbs: [], qv: [] },
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
        val sel_dig = f_dot(ek, digs)
        val sel_sp = f_dot(ek, sps)
        val p = f_dot(ek, params)
        val pd = f_mul(sel_dig, p)
        val tb = range(0, N_TAGS).foldl([], (acc, t) =>
          acc.append(if (sel_dig == 1 and pd == t) 1 else 0)
        )
        val diff_row = range(0, LANE_PAIRS).foldl([], (acc, i) => {
          val prev_lane = w.nth(LANE_MAP + 2 * i)
          acc.append(if (i < k) mod_inv(f_sub(lane, prev_lane)) else 0)
        })
        val o = f_dot(ek, ent_st.offs)
        val idx = f_add(o, f_add(1, f_mul(2, sel_sp)))
        val i1 = f_add(o, 1)
        val i2 = f_add(o, 2)
        val vk = if (nz == 1 and idx >= 0 and idx < V_CAP) vs.nth(idx) else 0
        val x0 = if (nz == 1 and o >= 0 and o < V_CAP) vs.nth(o) else 0
        val x1 = if (sel_sp == 1 and i1 >= 0 and i1 < V_CAP) vs.nth(i1) else 0
        val x2 = if (sel_sp == 1 and i2 >= 0 and i2 < V_CAP) vs.nth(i2) else 0
        val new_qv = st.qv
          .append({ q_gate: nz, q_key: idx, q_v: vk })
          .append({ q_gate: nz, q_key: o, q_v: x0 })
          .append({ q_gate: sel_sp, q_key: i1, q_v: x1 })
          .append({ q_gate: sel_sp, q_key: i2, q_v: x2 })
        {
          invs: st.invs.append(inv),
          hs: st.hs.append(hk),
          es: st.es.append(ek),
          diff_invs: st.diff_invs.append(diff_row),
          vals: st.vals.append(vk),
          x0s: st.x0s.append(x0),
          x1s: st.x1s.append(x1),
          x2s: st.x2s.append(x2),
          tbs: st.tbs.append(tb),
          qv: new_qv
        }
      }
    )

    // 6. PUB query values and multiplicities
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

    val mv = range(0, V_CAP).foldl([], (acc, j) => {
      val cnt = range(0, pub_st.qv.length()).foldl(0, (c_acc, qi) => {
        val q = pub_st.qv.nth(qi)
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
      lm_x0: lm_st.x0s,
      lm_x1: lm_st.x1s,
      lm_x2: lm_st.x2s,
      lm_tb: lm_st.tbs,
      pub_v: pub_st.pvs,
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

  // Common R entries for mode 0 (entries 0..4, 12 values: vs[0..12])
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

  // Case 1: Honest ordinary call (mode = 0, j = 0, k = 0)
  // Entry 5: PUB(2) (checks s[2] == 50 after lane map maps SPONGE(3) output 50 to lane 2)
  // Entry 6: NOP(0)
  // Lane map: (1, 4) -> 21, (2, 3) -> 50, (3, 2) -> 23
  pure val WORDS_HONEST_ORD: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 3, 2],
    R_ENTS_ORD.concat([OP_PUB, 2, OP_NOP, 0]),
    R_VS_ORD.concat([50, 0, 0, 0]),
    [77, 0, 0, 0]
  )
  pure val PW_HONEST_ORD: List[int] = pack_words(PRE_HASH, WORDS_HONEST_ORD)

  // Case 2: Honest chunk call (mode = 1, j = 2, k = 3: 1 < j < k -> t_in = LOCK, t_out = LOCK)
  // Entries 0..3: run entries; Entry 4: DIGEST(LOCK, 10) -> 31; Entry 5: DIGEST(LOCK, 14) -> 34;
  // Entry 6: PUB(1) (checks s[1] == 31).
  // Lane map: (1, 4) -> 31, (2, 3) -> 50, (0, 0) skipped.
  pure val WORDS_HONEST_CHUNK_MID: List[int] = build_words(
    1, [2, 0], [3, 0], 4, 5,
    [1, 4, 2, 3, 0, 0],
    [
      OP_DIGEST, 0,
      OP_DIGEST, 2,
      OP_DIGEST, 3,
      OP_SPONGE, 3,
      OP_DIGEST, 1,
      OP_DIGEST, 1,
      OP_PUB, 1
    ],
    [10, 21, 11, 22, 12, 23, 21, 22, 23, 50, 10, 31, 14, 34, 31, 0],
    [15, 0, 0, 9]
  )
  pure val PW_HONEST_CHUNK_MID: List[int] = pack_words(PRE_HASH, WORDS_HONEST_CHUNK_MID)

  // Case 3: Honest single-chunk call (mode = 1, j = 1, k = 1 -> t_in = PROGC, t_out = PROGC)
  // Entry 6 is discretionary DIGEST(SHK_REG = 4, 15) -> 45, mapped to lane 3.
  pure val WORDS_HONEST_CHUNK_SINGLE: List[int] = build_words(
    1, [1, 0], [1, 0], 4, 5,
    [1, 4, 2, 3, 3, 6],
    [
      OP_DIGEST, 0,
      OP_DIGEST, 2,
      OP_DIGEST, 3,
      OP_SPONGE, 3,
      OP_DIGEST, 0,
      OP_DIGEST, 0,
      OP_DIGEST, 4
    ],
    [10, 21, 11, 22, 12, 23, 21, 22, 23, 50, 10, 21, 14, 24, 15, 45],
    [8, 0, 0, 0]
  )
  pure val PW_HONEST_CHUNK_SINGLE: List[int] = pack_words(PRE_HASH, WORDS_HONEST_CHUNK_SINGLE)

  // Case 4: Divergence case between `replay_all == true` (stock) and `replay_all == false` (wrap):
  // Entry 5 is an unmapped discretionary DIGEST(4) outside R with forged output vs[13] = 199 != 45.
  pure val WORDS_UNMAPPED_BAD_REPLAY: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 0, 0],
    R_ENTS_ORD.concat([OP_DIGEST, 4, OP_NOP, 0]),
    R_VS_ORD.concat([15, 199, 0, 0]),
    [77, 0, 0, 9]
  )
  pure val PW_UNMAPPED_BAD_REPLAY: List[int] = pack_words(PRE_HASH, WORDS_UNMAPPED_BAD_REPLAY)

  // Case 5: Counterexample for `mut_no_sponge3_param`
  // Entry 5 is SPONGE(2) (param = 2 != 3, 3 values at vs[12..15] = [7, 8, 60]),
  // Entry 6 is PUB(0) (1 value at vs[15] = 61 == hash_slice(0, [7, 8, 60]) == lanes[0]).
  // Pair 2 maps (lane 3, ent 5). Without `sel_sp * (p - 3) == 0`, the circuit reads
  // idx = 12 + 3 = 15 (value 61) and replays hash_slice(0, [7, 8, 60]) == 61, accepting!
  pure val WORDS_BAD_SPONGE3_PARAM: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 3, 5],
    R_ENTS_ORD.concat([OP_SPONGE, 2, OP_PUB, 0]),
    R_VS_ORD.concat([7, 8, 60, 61]),
    [61, 0, 0, 0]
  )
  pure val PW_BAD_SPONGE3_PARAM: List[int] = pack_words(PRE_HASH, WORDS_BAD_SPONGE3_PARAM)
  pure val WIT_BAD_SPONGE3_PARAM: Witness = honest_witness(PW_BAD_SPONGE3_PARAM)

  // Case 6: Counterexample for `mut_idx_plus1`
  // Entry 5 is SPONGE(3) at o = 12 with vs[12..16] = [1, 20, 2, 99].
  // Note hash_slice(0, [1, 20, 2]) == 20 == vs[13], whereas vs[15] == 99 != 20.
  // Pair 0 maps (lane 3, ent 5); pairs 1 and 2 are (0, 0).
  // Under `mut_idx_plus1`, idx = o + 1 = 13, so val = vs[13] = 20 == ys, accepting a forged SPONGE(3)
  // and setting s[3] = 20 instead of 99!
  pure val WORDS_BAD_IDX_PLUS1: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [3, 5, 0, 0, 0, 0],
    R_ENTS_ORD.concat([OP_SPONGE, 3, OP_NOP, 0]),
    R_VS_ORD.concat([1, 20, 2, 99]),
    [77, 0, 0, 0]
  )
  pure val PW_BAD_IDX_PLUS1: List[int] = pack_words(PRE_HASH, WORDS_BAD_IDX_PLUS1)
  pure val WIT_BAD_IDX_PLUS1: Witness = {
    val hw = honest_witness(PW_BAD_IDX_PLUS1)
    {
      ...hw,
      lm_val: [20, 0, 0],
      mv: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 0]
    }
  }

  // Case 7: Counterexample for `mut_no_twice` (including a first mapped output of 0!)
  // Entry 4 in R is DIGEST(PROGC, 42) -> 0 (vs[10] = 42, vs[11] = 0).
  // Pair 0 maps (lane 2, ent 4) -> writes 0 to s[2], so s[2] is STILL 0!
  // Pair 1 maps (lane 2, ent 3) -> writes 50 to s[2]. Since s[2] == 0 before pair 1,
  // `taken = dot(h, s) == 0` PASSES on pair 1; only the pairwise check refuses it!
  pure val WORDS_BAD_TWICE: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [2, 4, 2, 3, 0, 0],
    R_ENTS_ORD.concat([OP_NOP, 0, OP_NOP, 0]),
    [10, 21, 11, 22, 12, 23, 21, 22, 23, 50, 42, 0, 0, 0, 0, 0],
    [77, 0, 0, 0]
  )
  pure val PW_BAD_TWICE: List[int] = pack_words(PRE_HASH, WORDS_BAD_TWICE)
  pure val WIT_BAD_TWICE: Witness = honest_witness(PW_BAD_TWICE)

  // Case 8: Counterexample for `mut_no_u16`
  // Mode 0 with j_lo = U16_LIM = 8 (< BB_P = 17, so unpack passes, but >= U16_LIM).
  pure val WORDS_BAD_U16: List[int] = build_words(
    0, [U16_LIM, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 3, 2],
    R_ENTS_ORD.concat([OP_PUB, 2, OP_NOP, 0]),
    R_VS_ORD.concat([50, 0, 0, 0]),
    [77, 0, 0, 0]
  )
  pure val PW_BAD_U16: List[int] = pack_words(PRE_HASH, WORDS_BAD_U16)
  pure val WIT_BAD_U16: Witness = honest_witness(PW_BAD_U16)

  // Case 9: Counterexample for `mut_no_mapped_replay`
  // Entry 5 is discretionary DIGEST(4) at o = 12 with vs[12] = 15 and forged output vs[13] = 199 != 45.
  // Pair 2 maps (lane 3, ent 5). Without mapped-entry replay, the forged output 199 is written into s[3].
  pure val WORDS_BAD_MAPPED_REPLAY: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 3, 5],
    R_ENTS_ORD.concat([OP_DIGEST, 4, OP_NOP, 0]),
    R_VS_ORD.concat([15, 199, 0, 0]),
    [77, 0, 0, 0]
  )
  pure val PW_BAD_MAPPED_REPLAY: List[int] = pack_words(PRE_HASH, WORDS_BAD_MAPPED_REPLAY)
  pure val WIT_BAD_MAPPED_REPLAY: Witness = honest_witness(PW_BAD_MAPPED_REPLAY)

  // Case 10: Counterexample for `mut_t_in_wrong` (`t_in = 1 - [j == 1]` without `mode`)
  // Mode 0 with j = 0, k = 0: since j != 1, `1 - [j == 1]` is 1 (SHK_LOCK) instead of 0 (SHK_PROGC).
  // Entry 4 is set to DIGEST(SHK_LOCK = 1) with vs[10] = 10, vs[11] = 31.
  pure val WORDS_BAD_T_IN: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 3, 2],
    [
      OP_DIGEST, 0,
      OP_DIGEST, 2,
      OP_DIGEST, 3,
      OP_SPONGE, 3,
      OP_DIGEST, 1,
      OP_PUB, 2,
      OP_NOP, 0
    ],
    [10, 21, 11, 22, 12, 23, 21, 22, 23, 50, 10, 31, 50, 0, 0, 0],
    [77, 0, 0, 0]
  )
  pure val PW_BAD_T_IN: List[int] = pack_words(PRE_HASH, WORDS_BAD_T_IN)
  pure val WIT_BAD_T_IN: Witness = honest_witness(PW_BAD_T_IN)

  // Case 11: Counterexample for `mut_no_claim_pub`
  // Entry 3 (SPONGE(3) of claim_pub) has inputs [99, 22, 23] (vs[6] = 99 != vs[1] = 21)
  // and valid sponge output vs[9] = hash_slice(0, [99, 22, 23]) = 59.
  pure val WORDS_BAD_CLAIM_PUB: List[int] = build_words(
    0, [0, 0], [0, 0], 4, 4,
    [1, 4, 2, 3, 3, 2],
    R_ENTS_ORD.concat([OP_PUB, 2, OP_NOP, 0]),
    [10, 21, 11, 22, 12, 23, 99, 22, 23, 59, 10, 21, 59, 0, 0, 0],
    [77, 0, 0, 0]
  )
  pure val PW_BAD_CLAIM_PUB: List[int] = pack_words(PRE_HASH, WORDS_BAD_CLAIM_PUB)
  pure val WIT_BAD_CLAIM_PUB: Witness = honest_witness(PW_BAD_CLAIM_PUB)

  // Case 12: Counterexample for `mut_no_chunk_range`
  // Mode 1 (chunk) with j = 3 > k = 2 (invalid chunk ordering j > k, so t_in = 1, t_out = 1).
  pure val WORDS_BAD_CHUNK_RANGE: List[int] = build_words(
    1, [3, 0], [2, 0], 4, 5,
    [1, 4, 2, 3, 0, 0],
    [
      OP_DIGEST, 0,
      OP_DIGEST, 2,
      OP_DIGEST, 3,
      OP_SPONGE, 3,
      OP_DIGEST, 1,
      OP_DIGEST, 1,
      OP_PUB, 1
    ],
    [10, 21, 11, 22, 12, 23, 21, 22, 23, 50, 10, 31, 14, 34, 31, 0],
    [15, 0, 0, 9]
  )
  pure val PW_BAD_CHUNK_RANGE: List[int] = pack_words(PRE_HASH, WORDS_BAD_CHUNK_RANGE)
  pure val WIT_BAD_CHUNK_RANGE: Witness = honest_witness(PW_BAD_CHUNK_RANGE)

  // Catalog of (pw, wit) pairs for state exploration
  pure val CORPUS: List[{ pw: List[int], wit: Witness }] = [
    { pw: PW_HONEST_ORD, wit: honest_witness(PW_HONEST_ORD) },
    { pw: PW_HONEST_CHUNK_MID, wit: honest_witness(PW_HONEST_CHUNK_MID) },
    { pw: PW_HONEST_CHUNK_SINGLE, wit: honest_witness(PW_HONEST_CHUNK_SINGLE) },
    { pw: PW_UNMAPPED_BAD_REPLAY, wit: honest_witness(PW_UNMAPPED_BAD_REPLAY) },
    { pw: PW_BAD_SPONGE3_PARAM, wit: WIT_BAD_SPONGE3_PARAM },
    { pw: PW_BAD_IDX_PLUS1, wit: WIT_BAD_IDX_PLUS1 },
    { pw: PW_BAD_TWICE, wit: WIT_BAD_TWICE },
    { pw: PW_BAD_U16, wit: WIT_BAD_U16 },
    { pw: PW_BAD_MAPPED_REPLAY, wit: WIT_BAD_MAPPED_REPLAY },
    { pw: PW_BAD_T_IN, wit: WIT_BAD_T_IN },
    { pw: PW_BAD_CLAIM_PUB, wit: WIT_BAD_CLAIM_PUB },
    { pw: PW_BAD_CHUNK_RANGE, wit: WIT_BAD_CHUNK_RANGE }
  ]

  // =========================================================================
  // 9. State Machine & Invariants (SOUND, COMPLETE, and Replay Divergence)
  // =========================================================================

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

  // Pure soundness predicate on a (pw, wit) pair against reference decode with `replay_all = false`:
  pure def is_sound_on(pw: List[int], wit: Witness): bool = {
    val ref_r = ref_decode(pw, false)
    constraints_ok(pw, wit) implies (ref_r.ok and circuit_output(pw, wit) == ref_r.stmt)
  }

  // Pure completeness predicate on a pw vector against reference decode with `replay_all = false`:
  pure def is_complete_on(pw: List[int]): bool = {
    val ref_r = ref_decode(pw, false)
    val hw = honest_witness(pw)
    ref_r.ok implies (constraints_ok(pw, hw) and circuit_output(pw, hw) == ref_r.stmt)
  }

  // Pure predicate checking that the ONLY difference between `replay_all = true` and
  // `replay_all = false` is a replay failure of an entry outside R and outside the lane map:
  pure def is_replay_diff_exact_on(pw: List[int]): bool = {
    val r_all = ref_decode(pw, true)
    val r_wrap = ref_decode(pw, false)
    (r_all.ok implies (r_wrap.ok and r_all.stmt == r_wrap.stmt)) and
    (r_wrap.ok implies (r_all.ok == unmapped_outside_r_replay_ok(pw)))
  }

  // State invariants for `quint run`
  val SOUND: bool = is_sound_on(cur_pw, cur_wit)
  val COMPLETE: bool = is_complete_on(cur_pw)
  val REPLAY_DIFF_EXACT: bool = is_replay_diff_exact_on(cur_pw)
  val SOUND_AND_COMPLETE: bool = (SOUND and COMPLETE and REPLAY_DIFF_EXACT)

  // Mutant soundness invariants (each fails on its corresponding mutant in `quint run`)
  val SOUND_mut_no_sponge3_param: bool = {
    val ref_r = ref_decode(cur_pw, false)
    mut_no_sponge3_param(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_idx_plus1: bool = {
    val ref_r = ref_decode(cur_pw, false)
    mut_idx_plus1(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_no_twice: bool = {
    val ref_r = ref_decode(cur_pw, false)
    mut_no_twice(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_no_u16: bool = {
    val ref_r = ref_decode(cur_pw, false)
    mut_no_u16(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_no_mapped_replay: bool = {
    val ref_r = ref_decode(cur_pw, false)
    mut_no_mapped_replay(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_t_in_wrong: bool = {
    val ref_r = ref_decode(cur_pw, false)
    mut_t_in_wrong(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_no_claim_pub: bool = {
    val ref_r = ref_decode(cur_pw, false)
    mut_no_claim_pub(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }
  val SOUND_mut_no_chunk_range: bool = {
    val ref_r = ref_decode(cur_pw, false)
    mut_no_chunk_range(cur_pw, cur_wit) implies (ref_r.ok and circuit_output(cur_pw, cur_wit) == ref_r.stmt)
  }

}
