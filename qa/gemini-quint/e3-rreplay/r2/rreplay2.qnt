// -*- mode: Bluespec; -*-
// Round 2 soundness and completeness model of:
//   1. The built j, k range and order rule in gen_r_check (lane-src-r2/lverify_excerpt.rs, 95ed47b)
//      and anchor_tags (lane-src-r2/iface.rs), plus BitsAir (lane-src/machine.rs).
//   2. The native log merge rule log_merge and seg_log (lane-src-r2/iface.rs, ARCHITECTURE 1.11).
//
// Reduced parameters:
//   - BabyBear prime P = 17 (plus P_LARGE = 37 > 2^(2*2+1) = 32 for Q2 modular wrap checking)
//   - LIMBS = 2 limbs of LIMB_BITS = 2 bits (LIMB_BASE = 4) per logged Fr value
//   - JK_BITS = 2 bits (JK_BASE = 4) per j, k limb (in place of 16-bit limbs < 2^16)
//   - Scaled Fr prime order R_ORDER = 13 (13 < 4^2 = 16)
//   - Full entry/offset structure of R (6 entries, R_OFF = [0, 2, 4, 6, 10, 12], 14 value slots)

module rreplay2 {

  // --------------------------------------------------------------------------
  // 1. Reduced field and layout constants (matching lane-src-r2/iface.rs)
  // --------------------------------------------------------------------------

  pure val P: int = 17
  // Second prime P_LARGE = 37 > 2^(2*2+1) = 32 for Q2 modular wrap verification
  pure val P_LARGE: int = 37
  pure val R_ORDER: int = 13

  pure val LIMBS: int = 2
  pure val LIMB_BITS: int = 2
  pure val LIMB_BASE: int = 4

  // Reduced limb width and base for j, k (2-bit limbs < 4 in place of 16-bit limbs < 65536)
  pure val JK_BITS: int = 2
  pure val JK_BASE: int = 4

  // Reduced word bit width for BitsAir (5 bits since P = 17 < 2^5 = 32)
  pure val BITS_N: int = 5

  // Reduced log_merge bound (2^4 = 16 in place of 2^16 = 65536)
  pure val LOG_N_LIMIT: int = 16
  pure val D_EMPTY: int = 0

  // Word indices from lane-src-r2/iface.rs
  pure val MODE_WORD: int = 74
  pure val J_WORDS: int = 75
  pure val K_WORDS: int = 77
  pure val E_IN_WORD: int = 79
  pure val E_OUT_WORD: int = 80
  pure val STMT: int = 114

  // Operation codes from lane-src-r2/iface.rs
  pure val NOP: int = 0
  pure val PERM: int = 1
  pure val H2: int = 2
  pure val SPONGE: int = 3
  pure val PATH: int = 4
  pure val ECMUL: int = 5
  pure val ECMULG: int = 6
  pure val PUB: int = 7
  pure val DIGEST: int = 8

  // DIGEST tag indices from lane-src-r2/iface.rs
  pure val SHK_PROGC: int = 0
  pure val SHK_LOCK: int = 1
  pure val SHK_INPUT: int = 2
  pure val SHK_OUTCOME: int = 3
  pure val SHK_REG: int = 4
  pure val SHK_SET: int = 5

  // Reservation R constants from lane-src-r2/iface.rs
  pure val R_RUN: List[int] = [SHK_PROGC, SHK_INPUT, SHK_OUTCOME]
  pure val R_OFF: List[int] = [0, 2, 4, 6, 10, 12]
  pure val ANCHOR_IN: int = 4
  pure val N_R_ENTRIES: int = 6
  pure val N_R_VALS: int = 14

  // Scaled Fr constants for tag_fr and cap_digest
  pure val CAP_DIGEST: int = 9

  pure def tag_fr(k: int): int =
    if (k == SHK_PROGC) 1
    else if (k == SHK_LOCK) 2
    else if (k == SHK_INPUT) 3
    else if (k == SHK_OUTCOME) 4
    else if (k == SHK_REG) 5
    else 6

  // --------------------------------------------------------------------------
  // 2. Types for public words, prover advice, reader tags, and log merge
  // --------------------------------------------------------------------------

  type EntryPair = {
    op: int,
    param: int
  }

  type PubWords = {
    mode: int,
    j_lo: int,
    j_hi: int,
    k_lo: int,
    k_hi: int,
    e_in: int,
    e_out: int,
    entries: List[EntryPair],
    v_limbs: List[List[int]]
  }

  type ZeroAdv = {
    w: int,
    z: int
  }

  type Advice = {
    z1a: ZeroAdv,
    z1b: ZeroAdv,
    zka: ZeroAdv,
    zkb: ZeroAdv,
    zj0: ZeroAdv,
    c: int
  }

  type DecResult = {
    ok: bool,
    fv: int
  }

  type TagPair = {
    ok: bool,
    t_in: int,
    t_out: int
  }

  type LogVal = {
    d: int,
    n: int
  }

  type MergeResult = {
    ok: bool,
    log: LogVal
  }

  // --------------------------------------------------------------------------
  // 3. Modular arithmetic in F_p (for both P = 17 and P_LARGE = 37) and F_r
  // --------------------------------------------------------------------------

  pure def mod_in(x: int, p_mod: int): bool =
    (0 <= x) and (x < p_mod)

  pure def mod_norm(a: int, p_mod: int): int =
    ((a % p_mod) + p_mod) % p_mod

  pure def mod_add(a: int, b: int, p_mod: int): int =
    (mod_norm(a, p_mod) + mod_norm(b, p_mod)) % p_mod

  pure def mod_sub(a: int, b: int, p_mod: int): int =
    (mod_norm(a, p_mod) + p_mod - mod_norm(b, p_mod)) % p_mod

  pure def mod_mul(a: int, b: int, p_mod: int): int =
    (mod_norm(a, p_mod) * mod_norm(b, p_mod)) % p_mod

  pure def in_bb(x: int): bool =
    mod_in(x, P)

  pure def bb_add(a: int, b: int): int =
    mod_add(a, b, P)

  pure def bb_sub(a: int, b: int): int =
    mod_sub(a, b, P)

  pure def bb_mul(a: int, b: int): int =
    mod_mul(a, b, P)

  // Multiplicative inverse in F_17 (0 maps to 0)
  pure def bb_inv(a: int): int = {
    val m = mod_norm(a, P)
    if (m == 0) 0
    else if (m == 1) 1
    else if (m == 2) 9
    else if (m == 3) 6
    else if (m == 4) 13
    else if (m == 5) 7
    else if (m == 6) 3
    else if (m == 7) 5
    else if (m == 8) 15
    else if (m == 9) 2
    else if (m == 10) 12
    else if (m == 11) 14
    else if (m == 12) 10
    else if (m == 13) 4
    else if (m == 14) 11
    else if (m == 15) 8
    else 16
  }

  // Multiplicative inverse in F_37 via Fermat a^35 mod 37 (0 maps to 0)
  pure def bb_inv_37(a: int): int = {
    val a1 = mod_norm(a, P_LARGE)
    val a2 = (a1 * a1) % P_LARGE
    val a4 = (a2 * a2) % P_LARGE
    val a8 = (a4 * a4) % P_LARGE
    val a16 = (a8 * a8) % P_LARGE
    val a32 = (a16 * a16) % P_LARGE
    (((a32 * a2) % P_LARGE) * a1) % P_LARGE
  }

  pure def fr_add(a: int, b: int): int =
    (a + b) % R_ORDER

  pure def pack1(b0: int): int =
    b0 % R_ORDER

  // Bijective S-box x^5 mod 13 on F_13 (since gcd(5, 12) = 1)
  pure def sbox5(x: int): int = {
    val u = ((x % R_ORDER) + R_ORDER) % R_ORDER
    val u2 = (u * u) % R_ORDER
    val u4 = (u2 * u2) % R_ORDER
    (u4 * u) % R_ORDER
  }

  // Bijective Poseidon-like permutation on (F_13)^3
  pure def perm(st: List[int]): List[int] = {
    val s0 = sbox5(st.nth(0) + 1)
    val s1 = sbox5(st.nth(1) + 2)
    val s2 = sbox5(st.nth(2) + 3)
    val o0 = (1 * s0 + 2 * s1 + 3 * s2) % R_ORDER
    val o1 = (1 * s0 + 3 * s1 + 2 * s2) % R_ORDER
    val o2 = (2 * s0 + 1 * s1 + 2 * s2) % R_ORDER
    [o0, o1, o2]
  }

  // --------------------------------------------------------------------------
  // 4. BitsAir and LowBits / r16 semantics (lane-src/machine.rs:590-695)
  // --------------------------------------------------------------------------

  pure def is_bit(b: int): bool =
    (b == 0) or (b == 1)

  // BitsAir::eval at reduced width BITS_N = 5 (since P = 17 < 2^5 = 32 < r):
  //   - b_bits[0..4] boolean
  //   - s_bits[0..4] boolean and non-increasing: s_{i+1} * (1 - s_i) == 0
  //   - sum_i s_i == n
  //   - x == sum_i b_i 2^i
  //   - low_out == sum_i s_i b_i 2^i
  pure def bits_air_row_ok(
    x: int,
    b_bits: List[int],
    s_bits: List[int],
    n: int,
    low_out: int
  ): bool = {
    val len_ok = (b_bits.length() == BITS_N) and (s_bits.length() == BITS_N)
    if (not(len_ok)) false
    else {
      val b_bool = 0.to(BITS_N - 1).forall(i => is_bit(b_bits.nth(i)))
      val s_bool = 0.to(BITS_N - 1).forall(i => is_bit(s_bits.nth(i)))
      val s_monotone = 0.to(BITS_N - 2).forall(i =>
        (s_bits.nth(i + 1) * (1 - s_bits.nth(i))) == 0
      )
      val sum_s =
        s_bits.nth(0) + s_bits.nth(1) + s_bits.nth(2) + s_bits.nth(3) + s_bits.nth(4)
      val acc_x =
        b_bits.nth(0) * 1 + b_bits.nth(1) * 2 + b_bits.nth(2) * 4 +
        b_bits.nth(3) * 8 + b_bits.nth(4) * 16
      val acc_low =
        s_bits.nth(0) * b_bits.nth(0) * 1 +
        s_bits.nth(1) * b_bits.nth(1) * 2 +
        s_bits.nth(2) * b_bits.nth(2) * 4 +
        s_bits.nth(3) * b_bits.nth(3) * 8 +
        s_bits.nth(4) * b_bits.nth(4) * 16
      and {
        b_bool,
        s_bool,
        s_monotone,
        sum_s == n,
        x == acc_x,
        low_out == acc_low
      }
    }
  }

  // Honest BitsAir witness cells (machine.rs:618-627 bits_cells)
  pure def honest_b_bits(x: int): List[int] =
    [x % 2, (x / 2) % 2, (x / 4) % 2, (x / 8) % 2, (x / 16) % 2]

  pure def honest_s_bits(n: int): List[int] = [
    if (0 < n) 1 else 0,
    if (1 < n) 1 else 0,
    if (2 < n) 1 else 0,
    if (3 < n) 1 else 0,
    if (4 < n) 1 else 0
  ]

  // Functional semantics of LowBits(x, JK_BITS) and r16(x) (LowBits(x, 16) == x)
  pure def low_bits_jk(x: int): int =
    x % JK_BASE

  pure def r16_ok(x: int): bool =
    in_bb(x) and (low_bits_jk(x) == x)

  pure def r16_ok_mod(x: int, p_mod: int): bool =
    mod_in(x, p_mod) and ((x % JK_BASE) == x)

  // --------------------------------------------------------------------------
  // 5. Tape op semantics and the new j, k block (lverify_excerpt.rs:165-325)
  // --------------------------------------------------------------------------

  pure def pub_in_bb(pw: PubWords): bool = and {
    in_bb(pw.mode),
    in_bb(pw.j_lo),
    in_bb(pw.j_hi),
    in_bb(pw.k_lo),
    in_bb(pw.k_hi),
    in_bb(pw.e_in),
    in_bb(pw.e_out),
    pw.entries.length() == N_R_ENTRIES,
    0.to(N_R_ENTRIES - 1).forall(i =>
      in_bb(pw.entries.nth(i).op) and in_bb(pw.entries.nth(i).param)
    ),
    pw.v_limbs.length() == N_R_VALS,
    0.to(N_R_VALS - 1).forall(j =>
      pw.v_limbs.nth(j).length() == LIMBS and
      0.to(LIMBS - 1).forall(t => in_bb(pw.v_limbs.nth(j).nth(t)))
    )
  }

  pure def advice_in_bb(adv: Advice): bool = and {
    in_bb(adv.z1a.w), in_bb(adv.z1a.z),
    in_bb(adv.z1b.w), in_bb(adv.z1b.z),
    in_bb(adv.zka.w), in_bb(adv.zka.z),
    in_bb(adv.zkb.w), in_bb(adv.zkb.z),
    in_bb(adv.zj0.w), in_bb(adv.zj0.z),
    in_bb(adv.c)
  }

  // Zero test [a = 0] with advice (w, z): a * w = 1 - z, a * z = 0 in F_p
  pure def is_zero_ok_mod(a: int, za: ZeroAdv, p_mod: int): bool = {
    val aw = mod_mul(a, za.w, p_mod)
    val one_z = mod_sub(1, za.z, p_mod)
    val az = mod_mul(a, za.z, p_mod)
    (aw == one_z) and (az == 0)
  }

  pure def is_zero_ok(a: int, za: ZeroAdv): bool =
    is_zero_ok_mod(a, za, P)

  pure def honest_zero_adv(av: int): ZeroAdv = {
    val x = mod_norm(av, P)
    if (x == 0) { w: 0, z: 1 }
    else { w: bb_inv(x), z: 0 }
  }

  pure def honest_zero_adv_37(av: int): ZeroAdv = {
    val x = mod_norm(av, P_LARGE)
    if (x == 0) { w: 0, z: 1 }
    else { w: bb_inv_37(x), z: 0 }
  }

  // Honest borrow advice: c = [mode == 1 && k_lo < j_lo] (lverify_excerpt.rs:231-233)
  pure def honest_borrow(pw: PubWords): int =
    if ((pw.mode == 1) and (pw.k_lo < pw.j_lo)) 1 else 0

  pure def honest_advice(pw: PubWords): Advice = {
    z1a: honest_zero_adv(bb_sub(pw.j_lo, 1)),
    z1b: honest_zero_adv(pw.j_hi),
    zka: honest_zero_adv(bb_sub(pw.j_lo, pw.k_lo)),
    zkb: honest_zero_adv(bb_sub(pw.j_hi, pw.k_hi)),
    zj0: honest_zero_adv(bb_add(pw.j_lo, pw.j_hi)),
    c: honest_borrow(pw)
  }

  // Standalone predicate for the new j, k range and order block parameterized by prime p_mod
  // (lverify_excerpt.rs:220-246)
  pure def jk_block_ok_mod(
    mode: int,
    j_lo: int,
    j_hi: int,
    k_lo: int,
    k_hi: int,
    zj0: ZeroAdv,
    c: int,
    p_mod: int
  ): bool = {
    val limbs_r16 =
      r16_ok_mod(j_lo, p_mod) and r16_ok_mod(j_hi, p_mod) and
      r16_ok_mod(k_lo, p_mod) and r16_ok_mod(k_hi, p_mod)

    val s = mod_add(j_lo, j_hi, p_mod)
    val zj0_ok = is_zero_ok_mod(s, zj0, p_mod)
    val j_ge_1_ok = (mod_mul(mode, zj0.z, p_mod) == 0)

    val c1 = mod_sub(c, 1, p_mod)
    val cc = mod_mul(c, c1, p_mod)
    val c_bit_ok = (cc == 0)

    val x_lo = mod_add(mod_sub(k_lo, j_lo, p_mod), mod_mul(JK_BASE, c, p_mod), p_mod)
    val m_lo = mod_mul(mode, x_lo, p_mod)
    val r_lo_ok = r16_ok_mod(m_lo, p_mod)

    val x_hi = mod_sub(mod_sub(k_hi, j_hi, p_mod), c, p_mod)
    val m_hi = mod_mul(mode, x_hi, p_mod)
    val r_hi_ok = r16_ok_mod(m_hi, p_mod)

    and {
      mod_in(zj0.w, p_mod),
      mod_in(zj0.z, p_mod),
      mod_in(c, p_mod),
      limbs_r16,
      zj0_ok,
      j_ge_1_ok,
      c_bit_ok,
      r_lo_ok,
      r_hi_ok
    }
  }

  pure def jk_block_ok(pw: PubWords, adv: Advice): bool =
    jk_block_ok_mod(pw.mode, pw.j_lo, pw.j_hi, pw.k_lo, pw.k_hi, adv.zj0, adv.c, P)

  // Circuit-computed z_j1, z_jk, tag_in, tag_out from advice
  pure def circuit_z_j1(adv: Advice): int =
    bb_mul(adv.z1a.z, adv.z1b.z)

  pure def circuit_z_jk(adv: Advice): int =
    bb_mul(adv.zka.z, adv.zkb.z)

  pure def circuit_tag_in(pw: PubWords, adv: Advice): int =
    bb_mul(pw.mode, bb_sub(1, circuit_z_j1(adv)))

  pure def circuit_tag_out(pw: PubWords, adv: Advice): int =
    bb_mul(pw.mode, bb_sub(1, circuit_z_jk(adv)))

  pure def cswap_bool_ok(bit: int): bool =
    (bit == 0) or (bit == 1)

  pure def cswap_l(bit: int, a: int, b: int): int =
    if (bit == 1) b else a

  pure def fr_val(limbs: List[int]): int = {
    val acc0 = pack1(limbs.nth(1))
    val d1 = fr_add(acc0, acc0)
    val d2 = fr_add(d1, d1)
    val l0 = pack1(limbs.nth(0))
    fr_add(d2, l0)
  }

  pure def sponge_in_limbs_ok(pw: PubWords): bool =
    0.to(2).forall(i =>
      0.to(LIMBS - 1).forall(t =>
        pw.v_limbs.nth(R_OFF.nth(3) + i).nth(t) == pw.v_limbs.nth(R_OFF.nth(i) + 1).nth(t)
      )
    )

  // Core R check without the new j, k block (round-1 check, used as helper)
  pure def gen_r_core_ok(pw: PubWords, adv: Advice): bool = {
    val m1 = bb_sub(pw.mode, 1)
    val mm = bb_mul(pw.mode, m1)
    val mode_ok = (mm == 0)

    val a1 = bb_sub(pw.j_lo, 1)
    val z1a_ok = is_zero_ok(a1, adv.z1a)
    val z1b_ok = is_zero_ok(pw.j_hi, adv.z1b)
    val z_j1 = bb_mul(adv.z1a.z, adv.z1b.z)

    val aka = bb_sub(pw.j_lo, pw.k_lo)
    val zka_ok = is_zero_ok(aka, adv.zka)
    val akb = bb_sub(pw.j_hi, pw.k_hi)
    val zkb_ok = is_zero_ok(akb, adv.zkb)
    val z_jk = bb_mul(adv.zka.z, adv.zkb.z)

    val tag_in = bb_mul(pw.mode, bb_sub(1, z_j1))
    val tag_out = bb_mul(pw.mode, bb_sub(1, z_jk))

    val run_ent_ok = 0.to(2).forall(i =>
      pw.entries.nth(i).op == DIGEST and pw.entries.nth(i).param == R_RUN.nth(i)
    )
    val sp_ent_ok = (pw.entries.nth(3).op == SPONGE) and (pw.entries.nth(3).param == 3)
    val e4_ent_ok = (pw.entries.nth(ANCHOR_IN).op == DIGEST) and (pw.entries.nth(ANCHOR_IN).param == tag_in)
    val e5_op_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(ANCHOR_IN + 1).op, DIGEST)) == 0)
    val e5_param_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(ANCHOR_IN + 1).param, tag_out)) == 0)

    val ein_ok = (pw.e_in == ANCHOR_IN)
    val eout_ok = (pw.e_out == bb_add(ANCHOR_IN, pw.mode))
    val sp_limbs_ok = sponge_in_limbs_ok(pw)

    val x0 = fr_val(pw.v_limbs.nth(R_OFF.nth(0)))
    val y0 = fr_val(pw.v_limbs.nth(R_OFF.nth(0) + 1))
    val o0 = perm([tag_fr(R_RUN.nth(0)), x0, CAP_DIGEST])

    val x1 = fr_val(pw.v_limbs.nth(R_OFF.nth(1)))
    val y1 = fr_val(pw.v_limbs.nth(R_OFF.nth(1) + 1))
    val o1 = perm([tag_fr(R_RUN.nth(1)), x1, CAP_DIGEST])

    val x2 = fr_val(pw.v_limbs.nth(R_OFF.nth(2)))
    val y2 = fr_val(pw.v_limbs.nth(R_OFF.nth(2) + 1))
    val o2 = perm([tag_fr(R_RUN.nth(2)), x2, CAP_DIGEST])

    val run_replay_ok = (o0.nth(0) == y0) and (o1.nth(0) == y1) and (o2.nth(0) == y2)

    val s_first = perm([y0, y1, 3])
    val s0_abs = fr_add(s_first.nth(0), y2)
    val s_second = perm([s0_abs, s_first.nth(1), s_first.nth(2)])
    val claim = fr_val(pw.v_limbs.nth(R_OFF.nth(3) + 3))
    val sp_replay_ok = (s_second.nth(0) == claim)

    val x4 = fr_val(pw.v_limbs.nth(R_OFF.nth(4)))
    val y4 = fr_val(pw.v_limbs.nth(R_OFF.nth(4) + 1))
    val tk4 = cswap_l(tag_in, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val o4 = perm([tk4, x4, CAP_DIGEST])
    val e4_replay_ok = cswap_bool_ok(tag_in) and (o4.nth(0) == y4)

    val x5 = fr_val(pw.v_limbs.nth(R_OFF.nth(5)))
    val y5 = fr_val(pw.v_limbs.nth(R_OFF.nth(5) + 1))
    val tk5 = cswap_l(tag_out, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val o5 = perm([tk5, x5, CAP_DIGEST])
    val l5 = cswap_l(pw.mode, y5, o5.nth(0))
    val e5_replay_ok = cswap_bool_ok(tag_out) and cswap_bool_ok(pw.mode) and (l5 == y5)

    and {
      pub_in_bb(pw),
      advice_in_bb(adv),
      mode_ok,
      z1a_ok,
      z1b_ok,
      zka_ok,
      zkb_ok,
      run_ent_ok,
      sp_ent_ok,
      e4_ent_ok,
      e5_op_ok,
      e5_param_ok,
      ein_ok,
      eout_ok,
      sp_limbs_ok,
      run_replay_ok,
      sp_replay_ok,
      e4_replay_ok,
      e5_replay_ok
    }
  }

  // Full 95ed47b gen_r_check predicate (core + built j, k range and order block)
  pure def gen_r_check_ok(pw: PubWords, adv: Advice): bool =
    gen_r_core_ok(pw, adv) and jk_block_ok(pw, adv)

  // --------------------------------------------------------------------------
  // 6. Native reader rule (anchor_tags, decode) and log_merge (lane-src-r2/iface.rs)
  // --------------------------------------------------------------------------

  pure def j_int(pw: PubWords): int =
    pw.j_lo + JK_BASE * pw.j_hi

  pure def k_int(pw: PubWords): int =
    pw.k_lo + JK_BASE * pw.k_hi

  // Reader anchor_tags from lane-src-r2/iface.rs:533-549:
  // None unless mode in {0, 1}, all 4 limbs in [0, JK_BASE), and for mode 1 also 1 <= j <= k.
  pure def reader_anchor_tags_words(
    mode: int,
    j_lo: int,
    j_hi: int,
    k_lo: int,
    k_hi: int
  ): TagPair = {
    val limbs_in_range = and {
      0 <= j_lo, j_lo < JK_BASE,
      0 <= j_hi, j_hi < JK_BASE,
      0 <= k_lo, k_lo < JK_BASE,
      0 <= k_hi, k_hi < JK_BASE
    }
    if (not(limbs_in_range)) {
      { ok: false, t_in: 0, t_out: 0 }
    } else {
      val j_comb = j_lo + JK_BASE * j_hi
      val k_comb = k_lo + JK_BASE * k_hi
      if (mode == 0) {
        { ok: true, t_in: SHK_PROGC, t_out: SHK_PROGC }
      } else if (mode == 1) {
        if ((j_comb < 1) or (j_comb > k_comb)) {
          { ok: false, t_in: 0, t_out: 0 }
        } else {
          {
            ok: true,
            t_in: if (j_comb == 1) SHK_PROGC else SHK_LOCK,
            t_out: if (j_comb == k_comb) SHK_PROGC else SHK_LOCK
          }
        }
      } else {
        { ok: false, t_in: 0, t_out: 0 }
      }
    }
  }

  pure def reader_anchor_tags(pw: PubWords): TagPair =
    reader_anchor_tags_words(pw.mode, pw.j_lo, pw.j_hi, pw.k_lo, pw.k_hi)

  // Tape's limb-wise tag computation (for agreement checks)
  pure def limbwise_tags(mode: int, j_lo: int, j_hi: int, k_lo: int, k_hi: int): TagPair = {
    val j_is_1 = (j_lo == 1) and (j_hi == 0)
    val j_eq_k = (j_lo == k_lo) and (j_hi == k_hi)
    if (mode == 0) {
      { ok: true, t_in: SHK_PROGC, t_out: SHK_PROGC }
    } else if (mode == 1) {
      {
        ok: true,
        t_in: if (j_is_1) SHK_PROGC else SHK_LOCK,
        t_out: if (j_eq_k) SHK_PROGC else SHK_LOCK
      }
    } else {
      { ok: false, t_in: 0, t_out: 0 }
    }
  }

  pure def native_sponge_u0_3(a: int, b: int, c: int): int = {
    val s0 = [0, 0, 3]
    val s1 = perm([fr_add(s0.nth(0), a), fr_add(s0.nth(1), b), s0.nth(2)])
    val s2 = perm([fr_add(s1.nth(0), c), s1.nth(1), s1.nth(2)])
    s2.nth(0)
  }

  pure def native_digest(tag: int, d2: int): int = {
    val s0 = [0, 0, CAP_DIGEST]
    val s1 = perm([fr_add(s0.nth(0), tag_fr(tag)), fr_add(s0.nth(1), d2), s0.nth(2)])
    s1.nth(0)
  }

  pure def to_limbs(x: int): List[int] = {
    val c = ((x % R_ORDER) + R_ORDER) % R_ORDER
    [c % LIMB_BASE, (c / LIMB_BASE) % LIMB_BASE]
  }

  pure def dec_limbs(l: List[int]): DecResult = {
    if (l.length() != LIMBS) { ok: false, fv: 0 }
    else {
      val l0 = l.nth(0)
      val l1 = l.nth(1)
      val limb_ok = (0 <= l0) and (l0 < LIMB_BASE) and (0 <= l1) and (l1 < LIMB_BASE)
      val big = l0 + LIMB_BASE * l1
      if (limb_ok and (big < R_ORDER)) { ok: true, fv: big }
      else { ok: false, fv: 0 }
    }
  }

  pure def val_at(pw: PubWords, j: int): int =
    fr_val(pw.v_limbs.nth(j))

  pure def native_r_entries(pw: PubWords): List[EntryPair] = {
    val tp = reader_anchor_tags(pw)
    val base = [
      { op: DIGEST, param: SHK_PROGC },
      { op: DIGEST, param: SHK_INPUT },
      { op: DIGEST, param: SHK_OUTCOME },
      { op: SPONGE, param: 3 },
      { op: DIGEST, param: tp.t_in }
    ]
    if (pw.mode == 1) base.append({ op: DIGEST, param: tp.t_out })
    else base
  }

  // Native verification of the R entries (using the 95ed47b reader_anchor_tags rule) and replay
  pure def native_r_rule_holds(pw: PubWords): bool = {
    val tp = reader_anchor_tags(pw)
    val exp_r = native_r_entries(pw)
    val entries_match = 0.to(exp_r.length() - 1).forall(i =>
      pw.entries.nth(i) == exp_r.nth(i)
    )
    val anchor_words_ok = (pw.e_in == ANCHOR_IN) and (pw.e_out == ANCHOR_IN + pw.mode)

    val sp = R_OFF.nth(3)
    val sp_inputs_ok = and {
      val_at(pw, sp + 0) == val_at(pw, R_OFF.nth(0) + 1),
      val_at(pw, sp + 1) == val_at(pw, R_OFF.nth(1) + 1),
      val_at(pw, sp + 2) == val_at(pw, R_OFF.nth(2) + 1)
    }

    val replay_0 = (native_digest(SHK_PROGC, val_at(pw, 0)) == val_at(pw, 1))
    val replay_1 = (native_digest(SHK_INPUT, val_at(pw, 2)) == val_at(pw, 3))
    val replay_2 = (native_digest(SHK_OUTCOME, val_at(pw, 4)) == val_at(pw, 5))
    val replay_3 = (native_sponge_u0_3(val_at(pw, 6), val_at(pw, 7), val_at(pw, 8)) == val_at(pw, 9))
    val replay_4 = (native_digest(tp.t_in, val_at(pw, 10)) == val_at(pw, 11))
    val replay_5 = (pw.mode == 0) or (native_digest(tp.t_out, val_at(pw, 12)) == val_at(pw, 13))

    and {
      tp.ok,
      entries_match,
      anchor_words_ok,
      sp_inputs_ok,
      replay_0,
      replay_1,
      replay_2,
      replay_3,
      replay_4,
      replay_5
    }
  }

  // Abstract injective node hash P2(TAG_NODE, d_a, d_b) with disjoint small integer ranges:
  //   - leaf digests are in 1..7 (D_EMPTY = 0)
  //   - (leaf, leaf) -> 8 + 8 * d_a + d_b       in [17, 63]
  //   - (node, leaf) -> 100 + 8 * d_a + d_b     in [237, 611]
  //   - (leaf, node) -> 1000 + 64 * d_a + d_b   in [1081, 1511]
  //   - (node, node) -> 2000 + 64 * d_a + d_b   in [3097, 6095]
  pure def p2_tag_node(d_a: int, d_b: int): int =
    if ((d_a < 8) and (d_b < 8)) 8 + 8 * d_a + d_b
    else if ((d_a >= 8) and (d_b < 8)) 100 + 8 * d_a + d_b
    else if ((d_a < 8) and (d_b >= 8)) 1000 + 64 * d_a + d_b
    else 2000 + 64 * d_a + d_b

  // Native log_merge (lane-src-r2/iface.rs:297-307, ARCHITECTURE 1.11)
  pure def log_merge(a: LogVal, b: LogVal): MergeResult = {
    val r =
      if (a.n == 0) b
      else if (b.n == 0) a
      else { d: p2_tag_node(a.d, b.d), n: a.n + b.n }
    if (r.n < LOG_N_LIMIT) { ok: true, log: r }
    else { ok: false, log: { d: D_EMPTY, n: 0 } }
  }

  // Construct an honest PubWords instance for given parameters
  pure def honest_pub_words(
    mode: int,
    j_lo: int,
    j_hi: int,
    k_lo: int,
    k_hi: int,
    d0: int,
    d1: int,
    d2: int,
    d_in: int,
    d_out: int,
    e5_op_ord: int,
    e5_param_ord: int,
    v12_ord: int,
    v13_ord: int
  ): PubWords = {
    val j_is_1 = (j_lo == 1) and (j_hi == 0)
    val j_eq_k = (j_lo == k_lo) and (j_hi == k_hi)
    val t_in = if (mode == 0) SHK_PROGC else (if (j_is_1) SHK_PROGC else SHK_LOCK)
    val t_out = if (mode == 0) SHK_PROGC else (if (j_eq_k) SHK_PROGC else SHK_LOCK)

    val out0 = native_digest(SHK_PROGC, d0)
    val out1 = native_digest(SHK_INPUT, d1)
    val out2 = native_digest(SHK_OUTCOME, d2)
    val claim = native_sponge_u0_3(out0, out1, out2)
    val out_in = native_digest(t_in, d_in)
    val out_out = native_digest(t_out, d_out)

    val ent5 = if (mode == 1) { op: DIGEST, param: t_out } else { op: e5_op_ord, param: e5_param_ord }
    val v12 = if (mode == 1) d_out else v12_ord
    val v13 = if (mode == 1) out_out else v13_ord

    {
      mode: mode,
      j_lo: j_lo,
      j_hi: j_hi,
      k_lo: k_lo,
      k_hi: k_hi,
      e_in: ANCHOR_IN,
      e_out: ANCHOR_IN + mode,
      entries: [
        { op: DIGEST, param: SHK_PROGC },
        { op: DIGEST, param: SHK_INPUT },
        { op: DIGEST, param: SHK_OUTCOME },
        { op: SPONGE, param: 3 },
        { op: DIGEST, param: t_in },
        ent5
      ],
      v_limbs: [
        to_limbs(d0),
        to_limbs(out0),
        to_limbs(d1),
        to_limbs(out1),
        to_limbs(d2),
        to_limbs(out2),
        to_limbs(out0),
        to_limbs(out1),
        to_limbs(out2),
        to_limbs(claim),
        to_limbs(d_in),
        to_limbs(out_in),
        to_limbs(v12),
        to_limbs(v13)
      ]
    }
  }

  // --------------------------------------------------------------------------
  // 7. Five Named Mutants of the new j, k block
  // --------------------------------------------------------------------------

  // Mutant 1: mut_no_r16_jhi — drops r16(j_hi) from the 4 limb range checks.
  pure def mut_no_r16_jhi(pw: PubWords, adv: Advice): bool = {
    val limbs_r16_no_jhi =
      r16_ok(pw.j_lo) and r16_ok(pw.k_lo) and r16_ok(pw.k_hi)
    val s = bb_add(pw.j_lo, pw.j_hi)
    val zj0_ok = is_zero_ok(s, adv.zj0)
    val j_ge_1_ok = (bb_mul(pw.mode, adv.zj0.z) == 0)
    val c_bit_ok = (bb_mul(adv.c, bb_sub(adv.c, 1)) == 0)
    val m_lo = bb_mul(pw.mode, bb_add(bb_sub(pw.k_lo, pw.j_lo), bb_mul(JK_BASE, adv.c)))
    val m_hi = bb_mul(pw.mode, bb_sub(bb_sub(pw.k_hi, pw.j_hi), adv.c))
    and {
      gen_r_core_ok(pw, adv),
      limbs_r16_no_jhi,
      zj0_ok,
      j_ge_1_ok,
      c_bit_ok,
      r16_ok(m_lo),
      r16_ok(m_hi)
    }
  }

  // Mutant 2: mut_borrow_not_bit — drops `c * (c - 1) == 0` (the boolean check on borrow `c`).
  pure def mut_borrow_not_bit(pw: PubWords, adv: Advice): bool = {
    val limbs_r16 =
      r16_ok(pw.j_lo) and r16_ok(pw.j_hi) and r16_ok(pw.k_lo) and r16_ok(pw.k_hi)
    val s = bb_add(pw.j_lo, pw.j_hi)
    val zj0_ok = is_zero_ok(s, adv.zj0)
    val j_ge_1_ok = (bb_mul(pw.mode, adv.zj0.z) == 0)
    val m_lo = bb_mul(pw.mode, bb_add(bb_sub(pw.k_lo, pw.j_lo), bb_mul(JK_BASE, adv.c)))
    val m_hi = bb_mul(pw.mode, bb_sub(bb_sub(pw.k_hi, pw.j_hi), adv.c))
    and {
      gen_r_core_ok(pw, adv),
      limbs_r16,
      zj0_ok,
      j_ge_1_ok,
      r16_ok(m_lo),
      r16_ok(m_hi)
    }
  }

  // Mutant 3: mut_order_unmasked — leaves the j >= 1 check and both borrow differences
  // unmasked by `mode` (requiring 1 <= j <= k even when mode == 0).
  pure def mut_order_unmasked(pw: PubWords, adv: Advice): bool = {
    val limbs_r16 =
      r16_ok(pw.j_lo) and r16_ok(pw.j_hi) and r16_ok(pw.k_lo) and r16_ok(pw.k_hi)
    val s = bb_add(pw.j_lo, pw.j_hi)
    val zj0_ok = is_zero_ok(s, adv.zj0)
    val j_ge_1_unmasked = (adv.zj0.z == 0)
    val c_bit_ok = (bb_mul(adv.c, bb_sub(adv.c, 1)) == 0)
    val x_lo = bb_add(bb_sub(pw.k_lo, pw.j_lo), bb_mul(JK_BASE, adv.c))
    val x_hi = bb_sub(bb_sub(pw.k_hi, pw.j_hi), adv.c)
    and {
      gen_r_core_ok(pw, adv),
      limbs_r16,
      zj0_ok,
      j_ge_1_unmasked,
      c_bit_ok,
      r16_ok(x_lo),
      r16_ok(x_hi)
    }
  }

  // Mutant 4: mut_j_ge_1_dropped — omits `mode * z_j0 == 0` (the j >= 1 check for chunks).
  pure def mut_j_ge_1_dropped(pw: PubWords, adv: Advice): bool = {
    val limbs_r16 =
      r16_ok(pw.j_lo) and r16_ok(pw.j_hi) and r16_ok(pw.k_lo) and r16_ok(pw.k_hi)
    val s = bb_add(pw.j_lo, pw.j_hi)
    val zj0_ok = is_zero_ok(s, adv.zj0)
    val c_bit_ok = (bb_mul(adv.c, bb_sub(adv.c, 1)) == 0)
    val m_lo = bb_mul(pw.mode, bb_add(bb_sub(pw.k_lo, pw.j_lo), bb_mul(JK_BASE, adv.c)))
    val m_hi = bb_mul(pw.mode, bb_sub(bb_sub(pw.k_hi, pw.j_hi), adv.c))
    and {
      gen_r_core_ok(pw, adv),
      limbs_r16,
      zj0_ok,
      c_bit_ok,
      r16_ok(m_lo),
      r16_ok(m_hi)
    }
  }

  // Mutant 5: mut_plus_c_in_hi — adds `c` instead of subtracting `c` in `k_hi - j_hi - c`.
  pure def mut_plus_c_in_hi(pw: PubWords, adv: Advice): bool = {
    val limbs_r16 =
      r16_ok(pw.j_lo) and r16_ok(pw.j_hi) and r16_ok(pw.k_lo) and r16_ok(pw.k_hi)
    val s = bb_add(pw.j_lo, pw.j_hi)
    val zj0_ok = is_zero_ok(s, adv.zj0)
    val j_ge_1_ok = (bb_mul(pw.mode, adv.zj0.z) == 0)
    val c_bit_ok = (bb_mul(adv.c, bb_sub(adv.c, 1)) == 0)
    val m_lo = bb_mul(pw.mode, bb_add(bb_sub(pw.k_lo, pw.j_lo), bb_mul(JK_BASE, adv.c)))
    val m_hi_bad = bb_mul(pw.mode, bb_add(bb_sub(pw.k_hi, pw.j_hi), adv.c))
    and {
      gen_r_core_ok(pw, adv),
      limbs_r16,
      zj0_ok,
      j_ge_1_ok,
      c_bit_ok,
      r16_ok(m_lo),
      r16_ok(m_hi_bad)
    }
  }

  // --------------------------------------------------------------------------
  // 8. Soundness & Completeness predicates and state machine
  // --------------------------------------------------------------------------

  // SOUND: whenever the tape accepts (gen_r_check_ok holds):
  //   1. reader_anchor_tags returns Some (rt.ok) with the exact same tags as the tape,
  //   2. all 4 limbs are in [0, JK_BASE),
  //   3. for chunks (mode == 1), 1 <= j <= k as combined integers,
  //   4. the full native R rule and replay hold.
  pure def sound_at(pw: PubWords, adv: Advice): bool = {
    val rt = reader_anchor_tags(pw)
    val j_comb = j_int(pw)
    val k_comb = k_int(pw)
    gen_r_check_ok(pw, adv) implies and {
      rt.ok,
      circuit_tag_in(pw, adv) == rt.t_in,
      circuit_tag_out(pw, adv) == rt.t_out,
      0 <= pw.j_lo,
      pw.j_lo < JK_BASE,
      0 <= pw.j_hi,
      pw.j_hi < JK_BASE,
      0 <= pw.k_lo,
      pw.k_lo < JK_BASE,
      0 <= pw.k_hi,
      pw.k_hi < JK_BASE,
      (pw.mode == 1) implies ((1 <= j_comb) and (j_comb <= k_comb)),
      native_r_rule_holds(pw)
    }
  }

  // COMPLETE: every honest instance (valid mode, 4 limbs in [0, JK_BASE), and 1 <= j <= k when mode == 1)
  // is accepted by both the tape (with honest_advice) and the reader.
  pure def valid_jk_for_mode(pw: PubWords): bool = {
    val rt = reader_anchor_tags(pw)
    rt.ok
  }

  pure def complete_at(pw: PubWords): bool =
    valid_jk_for_mode(pw) implies (
      gen_r_check_ok(pw, honest_advice(pw)) and reader_anchor_tags(pw).ok
    )

  var st_pw: PubWords
  var st_adv: Advice
  var st_honest: bool

  action init = {
    val pw0 = honest_pub_words(0, 0, 0, 0, 0, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    all {
      st_pw' = pw0,
      st_adv' = honest_advice(pw0),
      st_honest' = true
    }
  }

  action step = {
    nondet branch = Set(0, 1, 2, 3, 4).oneOf()
    nondet mode = Set(0, 1, 2).oneOf()
    nondet j_lo = Set(0, 1, 2, 3, 4).oneOf()
    nondet j_hi = Set(0, 1, 2, 3, 4, 16).oneOf()
    nondet k_lo = Set(0, 1, 2, 3, 4).oneOf()
    nondet k_hi = Set(0, 1, 2, 3, 4).oneOf()
    nondet d0 = Set(0, 1, 2, 5).oneOf()
    nondet d1 = Set(0, 2, 4).oneOf()
    nondet d2 = Set(1, 3, 6).oneOf()
    nondet d_in = Set(0, 1, 7).oneOf()
    nondet d_out = Set(0, 2, 8).oneOf()
    nondet e5_op = Set(NOP, PUB, DIGEST).oneOf()
    nondet e5_param = Set(0, 1, 2).oneOf()
    nondet bad_c = Set(0, 1, 2, 13, 16).oneOf()
    nondet bad_w = Set(0, 1, 2, 9).oneOf()
    nondet bad_z = Set(0, 1, 2).oneOf()
    nondet bad_tag = Set(SHK_PROGC, SHK_LOCK, SHK_INPUT).oneOf()

    val base_mode = if (mode == 2) 1 else mode
    val base_pw = honest_pub_words(base_mode, j_lo, j_hi, k_lo, k_hi, d0, d1, d2, d_in, d_out, e5_op, e5_param, 0, 0)
    val base_adv = honest_advice(base_pw)

    val next_pw =
      if (branch == 0) base_pw
      else if (branch == 1) { ...base_pw, mode: mode, e_out: bb_add(ANCHOR_IN, mode) }
      else if (branch == 2) { ...base_pw, entries: base_pw.entries.replaceAt(4, { op: DIGEST, param: bad_tag }) }
      else base_pw

    val next_adv =
      if (branch == 0) base_adv
      else if (branch == 3) { ...base_adv, c: bad_c }
      else if (branch == 4) { ...base_adv, zj0: { w: bad_w, z: bad_z }, c: bad_c }
      else base_adv

    val next_honest = (branch == 0) and valid_jk_for_mode(next_pw)

    all {
      st_pw' = next_pw,
      st_adv' = next_adv,
      st_honest' = next_honest
    }
  }

  // --------------------------------------------------------------------------
  // 9. Invariants: SOUND and COMPLETE
  // --------------------------------------------------------------------------

  val SOUND: bool =
    sound_at(st_pw, st_adv)

  val COMPLETE: bool =
    st_honest implies (gen_r_check_ok(st_pw, st_adv) and reader_anchor_tags(st_pw).ok)

  val inv_sound_and_complete: bool =
    SOUND and COMPLETE

}
