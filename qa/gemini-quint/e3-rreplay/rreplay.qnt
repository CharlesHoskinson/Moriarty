// -*- mode: Bluespec; -*-
// Soundness and completeness model of gen_r_check (E3 J7 reservation R replay in L_Fr).
// Reduced parameters:
//   - BabyBear prime P = 17
//   - LIMBS = 2 limbs of LIMB_BITS = 2 bits (LIMB_BASE = 4) per logged Fr value
//   - Scaled Fr prime order R_ORDER = 13 (13 < 4^2 = 16, so both limb >= 4 and Canon >= 13 exist)
//   - Bijective Poseidon-like permutation over (F_13)^3 with S-box x^5 mod 13 (gcd(5, 12) = 1)
//   - Full entry/offset structure of R (6 entries, R_OFF = [0, 2, 4, 6, 10, 12], 14 value slots)

module rreplay {

  // --------------------------------------------------------------------------
  // 1. Reduced field and layout constants (matching lane-src/iface.rs)
  // --------------------------------------------------------------------------

  pure val P: int = 17
  pure val R_ORDER: int = 13

  pure val LIMBS: int = 2
  pure val LIMB_BITS: int = 2
  pure val LIMB_BASE: int = 4

  // Reduced limb base for j, k (2-bit limbs < 4 in place of 16-bit limbs < 65536)
  pure val JK_BASE: int = 4

  // Word indices from lane-src/iface.rs
  pure val MODE_WORD: int = 74
  pure val J_WORDS: int = 75
  pure val K_WORDS: int = 77
  pure val E_IN_WORD: int = 79
  pure val E_OUT_WORD: int = 80
  pure val STMT: int = 114

  // Operation codes from lane-src/iface.rs
  pure val NOP: int = 0
  pure val PERM: int = 1
  pure val H2: int = 2
  pure val SPONGE: int = 3
  pure val PATH: int = 4
  pure val ECMUL: int = 5
  pure val ECMULG: int = 6
  pure val PUB: int = 7
  pure val DIGEST: int = 8

  // DIGEST tag indices from lane-src/iface.rs
  pure val SHK_PROGC: int = 0
  pure val SHK_LOCK: int = 1
  pure val SHK_INPUT: int = 2
  pure val SHK_OUTCOME: int = 3
  pure val SHK_REG: int = 4
  pure val SHK_SET: int = 5

  // Reservation R constants from lane-src/iface.rs
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
  // 2. Types for public words and prover advice
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
    zkb: ZeroAdv
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

  // --------------------------------------------------------------------------
  // 3. BabyBear (F_P) and Fr (F_{R_ORDER}) arithmetic and permutation
  // --------------------------------------------------------------------------

  pure def in_bb(x: int): bool =
    (0 <= x) and (x < P)

  pure def bb_add(a: int, b: int): int =
    (a + b) % P

  pure def bb_sub(a: int, b: int): int =
    ((a % P) + P - (b % P)) % P

  pure def bb_mul(a: int, b: int): int =
    (a * b) % P

  // Multiplicative inverse in F_17 (0 maps to 0)
  pure def bb_inv(a: int): int = {
    val m = ((a % P) + P) % P
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

  // Bijective Poseidon-like permutation on (F_13)^3:
  // Round constants (1, 2, 3), S-box x^5, and invertible MDS matrix
  //   [[1, 2, 3], [1, 3, 2], [2, 1, 2]] (det = -7 = 6 != 0 mod 13).
  // Moreover, every coordinate of out[0] has a non-zero weight (1, 2, 3),
  // so changing any single input word strictly changes out[0].
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
  // 4. Tape op semantics (lane-src/tape.rs, machine.rs, lverify_excerpt.rs)
  // --------------------------------------------------------------------------

  // InputAir: all public words and advice words are canonical BabyBear (< P)
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
    in_bb(adv.zkb.w), in_bb(adv.zkb.z)
  }

  // Zero test [a = 0] with advice (w, z): a * w = 1 - z, a * z = 0 in F_P
  pure def is_zero_ok(a: int, za: ZeroAdv): bool = {
    val aw = bb_mul(a, za.w)
    val one_z = bb_sub(1, za.z)
    val az = bb_mul(a, za.z)
    (aw == one_z) and (az == 0)
  }

  // Honest advice computation from lverify_excerpt.rs:182-196
  pure def honest_zero_adv(av: int): ZeroAdv = {
    val x = ((av % P) + P) % P
    if (x == 0) { w: 0, z: 1 }
    else { w: bb_inv(x), z: 0 }
  }

  pure def honest_advice(pw: PubWords): Advice = {
    z1a: honest_zero_adv(bb_sub(pw.j_lo, 1)),
    z1b: honest_zero_adv(pw.j_hi),
    zka: honest_zero_adv(bb_sub(pw.j_lo, pw.k_lo)),
    zkb: honest_zero_adv(bb_sub(pw.j_hi, pw.k_hi))
  }

  // Circuit-computed z_j1, z_jk, tag_in, tag_out from advice
  pure def circuit_z_j1(adv: Advice): int =
    bb_mul(adv.z1a.z, adv.z1b.z)

  pure def circuit_z_jk(adv: Advice): int =
    bb_mul(adv.zka.z, adv.zkb.z)

  pure def circuit_tag_in(pw: PubWords, adv: Advice): int =
    bb_mul(pw.mode, bb_sub(1, circuit_z_j1(adv)))

  pure def circuit_tag_out(pw: PubWords, adv: Advice): int =
    bb_mul(pw.mode, bb_sub(1, circuit_z_jk(adv)))

  // CSwapAir: asserts bit is boolean (0 or 1) and returns (l, r) = bit ? (b, a) : (a, b)
  pure def cswap_bool_ok(bit: int): bool =
    (bit == 0) or (bit == 1)

  pure def cswap_l(bit: int, a: int, b: int): int =
    if (bit == 1) b else a

  // fr_val(g, j) from lverify_excerpt.rs:251-261:
  // Horner evaluation from top limb LIMBS - 1 down to 0, doubling LIMB_BITS times via fr_add.
  pure def fr_val(limbs: List[int]): int = {
    val acc0 = pack1(limbs.nth(1))
    val d1 = fr_add(acc0, acc0)
    val d2 = fr_add(d1, d1)
    val l0 = pack1(limbs.nth(0))
    fr_add(d2, l0)
  }

  // SPONGE(3) limb-by-limb equality check (lverify_excerpt.rs:245-249)
  pure def sponge_in_limbs_ok(pw: PubWords): bool =
    0.to(2).forall(i =>
      0.to(LIMBS - 1).forall(t =>
        pw.v_limbs.nth(R_OFF.nth(3) + i).nth(t) == pw.v_limbs.nth(R_OFF.nth(i) + 1).nth(t)
      )
    )

  // --------------------------------------------------------------------------
  // 5. Full gen_r_check constraint predicate (lverify_excerpt.rs:165-292)
  // --------------------------------------------------------------------------

  pure def gen_r_check_ok(pw: PubWords, adv: Advice): bool = {
    // mode is a bit: mode * (mode - 1) == 0
    val m1 = bb_sub(pw.mode, 1)
    val mm = bb_mul(pw.mode, m1)
    val mode_ok = (mm == 0)

    // Zero tests for [j = 1] and [j = k]
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

    // Entries 0..2: DIGEST(R_RUN[i])
    val run_ent_ok = 0.to(2).forall(i =>
      pw.entries.nth(i).op == DIGEST and pw.entries.nth(i).param == R_RUN.nth(i)
    )
    // Entry 3: SPONGE(3)
    val sp_ent_ok = (pw.entries.nth(3).op == SPONGE) and (pw.entries.nth(3).param == 3)
    // Entry 4 (ANCHOR_IN): DIGEST(tag_in)
    val e4_ent_ok = (pw.entries.nth(ANCHOR_IN).op == DIGEST) and (pw.entries.nth(ANCHOR_IN).param == tag_in)
    // Entry 5 (ANCHOR_IN + 1): masked by mode
    val e5_op_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(ANCHOR_IN + 1).op, DIGEST)) == 0)
    val e5_param_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(ANCHOR_IN + 1).param, tag_out)) == 0)

    // e_in = 4, e_out = 4 + mode
    val ein_ok = (pw.e_in == ANCHOR_IN)
    val eout_ok = (pw.e_out == bb_add(ANCHOR_IN, pw.mode))

    // SPONGE(3) inputs = run DIGEST outputs limb by limb
    val sp_limbs_ok = sponge_in_limbs_ok(pw)

    // Native replay of run DIGEST entries 0..2
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

    // SPONGE(3), u = 0: state (0, 0, 3); absorb (y0, y1), permute, absorb y2, permute
    val s_first = perm([y0, y1, 3])
    val s0_abs = fr_add(s_first.nth(0), y2)
    val s_second = perm([s0_abs, s_first.nth(1), s_first.nth(2)])
    val claim = fr_val(pw.v_limbs.nth(R_OFF.nth(3) + 3))
    val sp_replay_ok = (s_second.nth(0) == claim)

    // Anchor in (entry 4)
    val x4 = fr_val(pw.v_limbs.nth(R_OFF.nth(4)))
    val y4 = fr_val(pw.v_limbs.nth(R_OFF.nth(4) + 1))
    val tk4 = cswap_l(tag_in, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val o4 = perm([tk4, x4, CAP_DIGEST])
    val e4_replay_ok = cswap_bool_ok(tag_in) and (o4.nth(0) == y4)

    // Anchor out (entry 5, gated by cswap on mode)
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

  // --------------------------------------------------------------------------
  // 6. Native reference functions (lane-src/iface.rs)
  // --------------------------------------------------------------------------

  // Value count of an entry (iface.rs:128-140)
  pure def vals_count(op: int, param: int): int =
    if (op == NOP) 0
    else if (op == PERM) 6
    else if (op == H2) 3
    else if (op == SPONGE) param + 1
    else if (op == PATH) 3
    else if (op == ECMUL) 5
    else if (op == ECMULG) 3
    else if (op == PUB) 1
    else if (op == DIGEST) 2
    else -1

  // Native sponge_u(u, xs) for 2 or 3 elements (iface.rs:192-206)
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

  // Native limb decomposition of a canonical Fr element (iface.rs:380-387)
  pure def to_limbs(x: int): List[int] = {
    val c = ((x % R_ORDER) + R_ORDER) % R_ORDER
    [c % LIMB_BASE, (c / LIMB_BASE) % LIMB_BASE]
  }

  // Native dec(l): checks limb range (< LIMB_BASE) and Canon (< R_ORDER) (iface.rs:391-404)
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

  // Limb-wise native anchor tags (as checked by gen_r_check)
  pure def native_anchor_tags_limbwise(pw: PubWords): TagPair = {
    val j_is_1 = (pw.j_lo == 1) and (pw.j_hi == 0)
    val j_eq_k = (pw.j_lo == pw.k_lo) and (pw.j_hi == pw.k_hi)
    if (pw.mode == 0) {
      { ok: true, t_in: SHK_PROGC, t_out: SHK_PROGC }
    } else if (pw.mode == 1) {
      {
        ok: true,
        t_in: if (j_is_1) SHK_PROGC else SHK_LOCK,
        t_out: if (j_eq_k) SHK_PROGC else SHK_LOCK
      }
    } else {
      { ok: false, t_in: 0, t_out: 0 }
    }
  }

  // Combined-integer native anchor tags (iface.rs:510-522: u32w = lo + (hi << 16))
  pure def native_anchor_tags_combined(pw: PubWords): TagPair = {
    val j_comb = pw.j_lo + JK_BASE * pw.j_hi
    val k_comb = pw.k_lo + JK_BASE * pw.k_hi
    if (pw.mode == 0) {
      { ok: true, t_in: SHK_PROGC, t_out: SHK_PROGC }
    } else if (pw.mode == 1) {
      {
        ok: true,
        t_in: if (j_comb == 1) SHK_PROGC else SHK_LOCK,
        t_out: if (j_comb == k_comb) SHK_PROGC else SHK_LOCK
      }
    } else {
      { ok: false, t_in: 0, t_out: 0 }
    }
  }

  // Expected entries of R (iface.rs:525-534)
  pure def native_r_entries(pw: PubWords): List[EntryPair] = {
    val tp = native_anchor_tags_limbwise(pw)
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

  // Check that cumulative value offsets of R match R_OFF = [0, 2, 4, 6, 10, 12]
  pure val r_offsets_match_vals: bool = {
    val e0 = vals_count(DIGEST, SHK_PROGC)
    val e1 = vals_count(DIGEST, SHK_INPUT)
    val e2 = vals_count(DIGEST, SHK_OUTCOME)
    val e3 = vals_count(SPONGE, 3)
    val e4 = vals_count(DIGEST, SHK_PROGC)
    and {
      R_OFF.nth(0) == 0,
      R_OFF.nth(1) == R_OFF.nth(0) + e0,
      R_OFF.nth(2) == R_OFF.nth(1) + e1,
      R_OFF.nth(3) == R_OFF.nth(2) + e2,
      R_OFF.nth(4) == R_OFF.nth(3) + e3,
      R_OFF.nth(5) == R_OFF.nth(4) + e4
    }
  }

  // Check whether all 14 value slots in R pass dec_limbs (limb < LIMB_BASE and integer < R_ORDER)
  pure def all_values_canonical(pw: PubWords): bool =
    0.to(N_R_VALS - 1).forall(j => dec_limbs(pw.v_limbs.nth(j)).ok)

  pure def val_at(pw: PubWords, j: int): int =
    fr_val(pw.v_limbs.nth(j))

  // Native verification of the R entries and replay on decoded values (iface.rs:577-603)
  pure def native_r_rule_holds(pw: PubWords): bool = {
    val tp = native_anchor_tags_limbwise(pw)
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

  // Construct an honest PubWords instance for given parameters (iface.rs:657-700)
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
  // 7. Proposed j, k range and order check (Question 6)
  // --------------------------------------------------------------------------

  // Proposed constraint: j_lo, j_hi, k_lo, k_hi in [0, JK_BASE) and (when mode == 1) 1 <= j <= k
  pure def jk_range_ok(pw: PubWords): bool = {
    val limbs_ok = and {
      0 <= pw.j_lo, pw.j_lo < JK_BASE,
      0 <= pw.j_hi, pw.j_hi < JK_BASE,
      0 <= pw.k_lo, pw.k_lo < JK_BASE,
      0 <= pw.k_hi, pw.k_hi < JK_BASE
    }
    val j_comb = pw.j_lo + JK_BASE * pw.j_hi
    val k_comb = pw.k_lo + JK_BASE * pw.k_hi
    val order_ok = (pw.mode == 0) or ((1 <= j_comb) and (j_comb <= k_comb))
    limbs_ok and order_ok
  }

  pure def gen_r_check_with_jk_range_ok(pw: PubWords, adv: Advice): bool =
    gen_r_check_ok(pw, adv) and jk_range_ok(pw)

  // --------------------------------------------------------------------------
  // 8. Six Mutants of gen_r_check (for mutation testing)
  // --------------------------------------------------------------------------

  // Mutant 1: mut_no_az — drops `a * z = 0` from is_zero(a), keeping only `a * w = 1 - z`.
  pure def is_zero_mut_no_az(a: int, za: ZeroAdv): bool = {
    val aw = bb_mul(a, za.w)
    val one_z = bb_sub(1, za.z)
    aw == one_z
  }

  pure def mut_no_az(pw: PubWords, adv: Advice): bool = {
    val m1 = bb_sub(pw.mode, 1)
    val mode_ok = (bb_mul(pw.mode, m1) == 0)
    val z1a_ok = is_zero_mut_no_az(bb_sub(pw.j_lo, 1), adv.z1a)
    val z1b_ok = is_zero_mut_no_az(pw.j_hi, adv.z1b)
    val z_j1 = bb_mul(adv.z1a.z, adv.z1b.z)
    val zka_ok = is_zero_mut_no_az(bb_sub(pw.j_lo, pw.k_lo), adv.zka)
    val zkb_ok = is_zero_mut_no_az(bb_sub(pw.j_hi, pw.k_hi), adv.zkb)
    val z_jk = bb_mul(adv.zka.z, adv.zkb.z)
    val tag_in = bb_mul(pw.mode, bb_sub(1, z_j1))
    val tag_out = bb_mul(pw.mode, bb_sub(1, z_jk))

    val run_ent_ok = 0.to(2).forall(i =>
      pw.entries.nth(i).op == DIGEST and pw.entries.nth(i).param == R_RUN.nth(i)
    )
    val sp_ent_ok = (pw.entries.nth(3).op == SPONGE) and (pw.entries.nth(3).param == 3)
    val e4_ent_ok = (pw.entries.nth(4).op == DIGEST) and (pw.entries.nth(4).param == tag_in)
    val e5_op_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(5).op, DIGEST)) == 0)
    val e5_param_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(5).param, tag_out)) == 0)
    val ein_ok = (pw.e_in == ANCHOR_IN)
    val eout_ok = (pw.e_out == bb_add(ANCHOR_IN, pw.mode))
    val sp_limbs_ok = sponge_in_limbs_ok(pw)

    val y0 = fr_val(pw.v_limbs.nth(1))
    val y1 = fr_val(pw.v_limbs.nth(3))
    val y2 = fr_val(pw.v_limbs.nth(5))
    val run_replay_ok =
      (perm([tag_fr(R_RUN.nth(0)), fr_val(pw.v_limbs.nth(0)), CAP_DIGEST]).nth(0) == y0) and
      (perm([tag_fr(R_RUN.nth(1)), fr_val(pw.v_limbs.nth(2)), CAP_DIGEST]).nth(0) == y1) and
      (perm([tag_fr(R_RUN.nth(2)), fr_val(pw.v_limbs.nth(4)), CAP_DIGEST]).nth(0) == y2)

    val s_first = perm([y0, y1, 3])
    val s_second = perm([fr_add(s_first.nth(0), y2), s_first.nth(1), s_first.nth(2)])
    val sp_replay_ok = (s_second.nth(0) == fr_val(pw.v_limbs.nth(9)))

    val tk4 = cswap_l(tag_in, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val e4_replay_ok =
      cswap_bool_ok(tag_in) and
      (perm([tk4, fr_val(pw.v_limbs.nth(10)), CAP_DIGEST]).nth(0) == fr_val(pw.v_limbs.nth(11)))

    val y5 = fr_val(pw.v_limbs.nth(13))
    val tk5 = cswap_l(tag_out, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val o5 = perm([tk5, fr_val(pw.v_limbs.nth(12)), CAP_DIGEST])
    val e5_replay_ok =
      cswap_bool_ok(tag_out) and cswap_bool_ok(pw.mode) and (cswap_l(pw.mode, y5, o5.nth(0)) == y5)

    and {
      pub_in_bb(pw), advice_in_bb(adv), mode_ok,
      z1a_ok, z1b_ok, zka_ok, zkb_ok,
      run_ent_ok, sp_ent_ok, e4_ent_ok, e5_op_ok, e5_param_ok,
      ein_ok, eout_ok, sp_limbs_ok,
      run_replay_ok, sp_replay_ok, e4_replay_ok, e5_replay_ok
    }
  }

  // Mutant 2: mut_mode_not_bit — omits `mode * (mode - 1) == 0` and the boolean check on mode.
  pure def mut_mode_not_bit(pw: PubWords, adv: Advice): bool = {
    val z1a_ok = is_zero_ok(bb_sub(pw.j_lo, 1), adv.z1a)
    val z1b_ok = is_zero_ok(pw.j_hi, adv.z1b)
    val z_j1 = bb_mul(adv.z1a.z, adv.z1b.z)
    val zka_ok = is_zero_ok(bb_sub(pw.j_lo, pw.k_lo), adv.zka)
    val zkb_ok = is_zero_ok(bb_sub(pw.j_hi, pw.k_hi), adv.zkb)
    val z_jk = bb_mul(adv.zka.z, adv.zkb.z)
    val tag_in = bb_mul(pw.mode, bb_sub(1, z_j1))
    val tag_out = bb_mul(pw.mode, bb_sub(1, z_jk))

    val run_ent_ok = 0.to(2).forall(i =>
      pw.entries.nth(i).op == DIGEST and pw.entries.nth(i).param == R_RUN.nth(i)
    )
    val sp_ent_ok = (pw.entries.nth(3).op == SPONGE) and (pw.entries.nth(3).param == 3)
    val e4_ent_ok = (pw.entries.nth(4).op == DIGEST) and (pw.entries.nth(4).param == tag_in)
    val e5_op_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(5).op, DIGEST)) == 0)
    val e5_param_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(5).param, tag_out)) == 0)
    val ein_ok = (pw.e_in == ANCHOR_IN)
    val eout_ok = (pw.e_out == bb_add(ANCHOR_IN, pw.mode))
    val sp_limbs_ok = sponge_in_limbs_ok(pw)

    val y0 = fr_val(pw.v_limbs.nth(1))
    val y1 = fr_val(pw.v_limbs.nth(3))
    val y2 = fr_val(pw.v_limbs.nth(5))
    val run_replay_ok =
      (perm([tag_fr(R_RUN.nth(0)), fr_val(pw.v_limbs.nth(0)), CAP_DIGEST]).nth(0) == y0) and
      (perm([tag_fr(R_RUN.nth(1)), fr_val(pw.v_limbs.nth(2)), CAP_DIGEST]).nth(0) == y1) and
      (perm([tag_fr(R_RUN.nth(2)), fr_val(pw.v_limbs.nth(4)), CAP_DIGEST]).nth(0) == y2)

    val s_first = perm([y0, y1, 3])
    val s_second = perm([fr_add(s_first.nth(0), y2), s_first.nth(1), s_first.nth(2)])
    val sp_replay_ok = (s_second.nth(0) == fr_val(pw.v_limbs.nth(9)))

    val tk4 = if (tag_in == 0) tag_fr(SHK_PROGC) else tag_fr(SHK_LOCK)
    val e4_replay_ok =
      (perm([tk4, fr_val(pw.v_limbs.nth(10)), CAP_DIGEST]).nth(0) == fr_val(pw.v_limbs.nth(11)))

    val y5 = fr_val(pw.v_limbs.nth(13))
    val tk5 = if (tag_out == 0) tag_fr(SHK_PROGC) else tag_fr(SHK_LOCK)
    val o5 = perm([tk5, fr_val(pw.v_limbs.nth(12)), CAP_DIGEST])
    val l5 = if (pw.mode == 0) y5 else o5.nth(0)
    val e5_replay_ok = (l5 == y5)

    and {
      pub_in_bb(pw), advice_in_bb(adv),
      z1a_ok, z1b_ok, zka_ok, zkb_ok,
      run_ent_ok, sp_ent_ok, e4_ent_ok, e5_op_ok, e5_param_ok,
      ein_ok, eout_ok, sp_limbs_ok,
      run_replay_ok, sp_replay_ok, e4_replay_ok, e5_replay_ok
    }
  }

  // Mutant 3: mut_tag_out_unmasked — leaves tag_out unmasked by mode (`tag_out = 1 - z_jk`)
  // and leaves entry 5's op/param check unmasked by mode.
  pure def mut_tag_out_unmasked(pw: PubWords, adv: Advice): bool = {
    val mode_ok = (bb_mul(pw.mode, bb_sub(pw.mode, 1)) == 0)
    val z1a_ok = is_zero_ok(bb_sub(pw.j_lo, 1), adv.z1a)
    val z1b_ok = is_zero_ok(pw.j_hi, adv.z1b)
    val z_j1 = bb_mul(adv.z1a.z, adv.z1b.z)
    val zka_ok = is_zero_ok(bb_sub(pw.j_lo, pw.k_lo), adv.zka)
    val zkb_ok = is_zero_ok(bb_sub(pw.j_hi, pw.k_hi), adv.zkb)
    val z_jk = bb_mul(adv.zka.z, adv.zkb.z)

    val tag_in = bb_mul(pw.mode, bb_sub(1, z_j1))
    val tag_out = bb_sub(1, z_jk)

    val run_ent_ok = 0.to(2).forall(i =>
      pw.entries.nth(i).op == DIGEST and pw.entries.nth(i).param == R_RUN.nth(i)
    )
    val sp_ent_ok = (pw.entries.nth(3).op == SPONGE) and (pw.entries.nth(3).param == 3)
    val e4_ent_ok = (pw.entries.nth(4).op == DIGEST) and (pw.entries.nth(4).param == tag_in)
    val e5_op_ok = (pw.entries.nth(5).op == DIGEST)
    val e5_param_ok = (pw.entries.nth(5).param == tag_out)
    val ein_ok = (pw.e_in == ANCHOR_IN)
    val eout_ok = (pw.e_out == bb_add(ANCHOR_IN, pw.mode))
    val sp_limbs_ok = sponge_in_limbs_ok(pw)

    val y0 = fr_val(pw.v_limbs.nth(1))
    val y1 = fr_val(pw.v_limbs.nth(3))
    val y2 = fr_val(pw.v_limbs.nth(5))
    val run_replay_ok =
      (perm([tag_fr(R_RUN.nth(0)), fr_val(pw.v_limbs.nth(0)), CAP_DIGEST]).nth(0) == y0) and
      (perm([tag_fr(R_RUN.nth(1)), fr_val(pw.v_limbs.nth(2)), CAP_DIGEST]).nth(0) == y1) and
      (perm([tag_fr(R_RUN.nth(2)), fr_val(pw.v_limbs.nth(4)), CAP_DIGEST]).nth(0) == y2)

    val s_first = perm([y0, y1, 3])
    val s_second = perm([fr_add(s_first.nth(0), y2), s_first.nth(1), s_first.nth(2)])
    val sp_replay_ok = (s_second.nth(0) == fr_val(pw.v_limbs.nth(9)))

    val tk4 = cswap_l(tag_in, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val e4_replay_ok =
      cswap_bool_ok(tag_in) and
      (perm([tk4, fr_val(pw.v_limbs.nth(10)), CAP_DIGEST]).nth(0) == fr_val(pw.v_limbs.nth(11)))

    val y5 = fr_val(pw.v_limbs.nth(13))
    val tk5 = cswap_l(tag_out, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val o5 = perm([tk5, fr_val(pw.v_limbs.nth(12)), CAP_DIGEST])
    val e5_replay_ok =
      cswap_bool_ok(tag_out) and cswap_bool_ok(pw.mode) and (cswap_l(pw.mode, y5, o5.nth(0)) == y5)

    and {
      pub_in_bb(pw), advice_in_bb(adv), mode_ok,
      z1a_ok, z1b_ok, zka_ok, zkb_ok,
      run_ent_ok, sp_ent_ok, e4_ent_ok, e5_op_ok, e5_param_ok,
      ein_ok, eout_ok, sp_limbs_ok,
      run_replay_ok, sp_replay_ok, e4_replay_ok, e5_replay_ok
    }
  }

  // Mutant 4: mut_sponge_swapped — absorbs (outs[1], outs[0]) in swapped order in SPONGE(3).
  pure def mut_sponge_swapped(pw: PubWords, adv: Advice): bool = {
    val mode_ok = (bb_mul(pw.mode, bb_sub(pw.mode, 1)) == 0)
    val z1a_ok = is_zero_ok(bb_sub(pw.j_lo, 1), adv.z1a)
    val z1b_ok = is_zero_ok(pw.j_hi, adv.z1b)
    val z_j1 = bb_mul(adv.z1a.z, adv.z1b.z)
    val zka_ok = is_zero_ok(bb_sub(pw.j_lo, pw.k_lo), adv.zka)
    val zkb_ok = is_zero_ok(bb_sub(pw.j_hi, pw.k_hi), adv.zkb)
    val z_jk = bb_mul(adv.zka.z, adv.zkb.z)
    val tag_in = bb_mul(pw.mode, bb_sub(1, z_j1))
    val tag_out = bb_mul(pw.mode, bb_sub(1, z_jk))

    val run_ent_ok = 0.to(2).forall(i =>
      pw.entries.nth(i).op == DIGEST and pw.entries.nth(i).param == R_RUN.nth(i)
    )
    val sp_ent_ok = (pw.entries.nth(3).op == SPONGE) and (pw.entries.nth(3).param == 3)
    val e4_ent_ok = (pw.entries.nth(4).op == DIGEST) and (pw.entries.nth(4).param == tag_in)
    val e5_op_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(5).op, DIGEST)) == 0)
    val e5_param_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(5).param, tag_out)) == 0)
    val ein_ok = (pw.e_in == ANCHOR_IN)
    val eout_ok = (pw.e_out == bb_add(ANCHOR_IN, pw.mode))
    val sp_limbs_ok = sponge_in_limbs_ok(pw)

    val y0 = fr_val(pw.v_limbs.nth(1))
    val y1 = fr_val(pw.v_limbs.nth(3))
    val y2 = fr_val(pw.v_limbs.nth(5))
    val run_replay_ok =
      (perm([tag_fr(R_RUN.nth(0)), fr_val(pw.v_limbs.nth(0)), CAP_DIGEST]).nth(0) == y0) and
      (perm([tag_fr(R_RUN.nth(1)), fr_val(pw.v_limbs.nth(2)), CAP_DIGEST]).nth(0) == y1) and
      (perm([tag_fr(R_RUN.nth(2)), fr_val(pw.v_limbs.nth(4)), CAP_DIGEST]).nth(0) == y2)

    // Swapped first absorb: [y1, y0, 3] instead of [y0, y1, 3]
    val s_first = perm([y1, y0, 3])
    val s_second = perm([fr_add(s_first.nth(0), y2), s_first.nth(1), s_first.nth(2)])
    val sp_replay_ok = (s_second.nth(0) == fr_val(pw.v_limbs.nth(9)))

    val tk4 = cswap_l(tag_in, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val e4_replay_ok =
      cswap_bool_ok(tag_in) and
      (perm([tk4, fr_val(pw.v_limbs.nth(10)), CAP_DIGEST]).nth(0) == fr_val(pw.v_limbs.nth(11)))

    val y5 = fr_val(pw.v_limbs.nth(13))
    val tk5 = cswap_l(tag_out, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val o5 = perm([tk5, fr_val(pw.v_limbs.nth(12)), CAP_DIGEST])
    val e5_replay_ok =
      cswap_bool_ok(tag_out) and cswap_bool_ok(pw.mode) and (cswap_l(pw.mode, y5, o5.nth(0)) == y5)

    and {
      pub_in_bb(pw), advice_in_bb(adv), mode_ok,
      z1a_ok, z1b_ok, zka_ok, zkb_ok,
      run_ent_ok, sp_ent_ok, e4_ent_ok, e5_op_ok, e5_param_ok,
      ein_ok, eout_ok, sp_limbs_ok,
      run_replay_ok, sp_replay_ok, e4_replay_ok, e5_replay_ok
    }
  }

  // Mutant 5: mut_off_shift — uses R_OFF_SHIFTED = [0, 2, 4, 6, 9, 11] (treating SPONGE(3) as 3 values instead of 4).
  pure def mut_off_shift(pw: PubWords, adv: Advice): bool = {
    val mode_ok = (bb_mul(pw.mode, bb_sub(pw.mode, 1)) == 0)
    val z1a_ok = is_zero_ok(bb_sub(pw.j_lo, 1), adv.z1a)
    val z1b_ok = is_zero_ok(pw.j_hi, adv.z1b)
    val z_j1 = bb_mul(adv.z1a.z, adv.z1b.z)
    val zka_ok = is_zero_ok(bb_sub(pw.j_lo, pw.k_lo), adv.zka)
    val zkb_ok = is_zero_ok(bb_sub(pw.j_hi, pw.k_hi), adv.zkb)
    val z_jk = bb_mul(adv.zka.z, adv.zkb.z)
    val tag_in = bb_mul(pw.mode, bb_sub(1, z_j1))
    val tag_out = bb_mul(pw.mode, bb_sub(1, z_jk))

    val run_ent_ok = 0.to(2).forall(i =>
      pw.entries.nth(i).op == DIGEST and pw.entries.nth(i).param == R_RUN.nth(i)
    )
    val sp_ent_ok = (pw.entries.nth(3).op == SPONGE) and (pw.entries.nth(3).param == 3)
    val e4_ent_ok = (pw.entries.nth(4).op == DIGEST) and (pw.entries.nth(4).param == tag_in)
    val e5_op_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(5).op, DIGEST)) == 0)
    val e5_param_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(5).param, tag_out)) == 0)
    val ein_ok = (pw.e_in == ANCHOR_IN)
    val eout_ok = (pw.e_out == bb_add(ANCHOR_IN, pw.mode))
    val sp_limbs_ok = sponge_in_limbs_ok(pw)

    val y0 = fr_val(pw.v_limbs.nth(1))
    val y1 = fr_val(pw.v_limbs.nth(3))
    val y2 = fr_val(pw.v_limbs.nth(5))
    val run_replay_ok =
      (perm([tag_fr(R_RUN.nth(0)), fr_val(pw.v_limbs.nth(0)), CAP_DIGEST]).nth(0) == y0) and
      (perm([tag_fr(R_RUN.nth(1)), fr_val(pw.v_limbs.nth(2)), CAP_DIGEST]).nth(0) == y1) and
      (perm([tag_fr(R_RUN.nth(2)), fr_val(pw.v_limbs.nth(4)), CAP_DIGEST]).nth(0) == y2)

    val s_first = perm([y0, y1, 3])
    val s_second = perm([fr_add(s_first.nth(0), y2), s_first.nth(1), s_first.nth(2)])
    val sp_replay_ok = (s_second.nth(0) == fr_val(pw.v_limbs.nth(9)))

    // Shifted offsets: entry 4 at 9..10 instead of 10..11, entry 5 at 11..12 instead of 12..13
    val tk4 = cswap_l(tag_in, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val e4_replay_ok =
      cswap_bool_ok(tag_in) and
      (perm([tk4, fr_val(pw.v_limbs.nth(9)), CAP_DIGEST]).nth(0) == fr_val(pw.v_limbs.nth(10)))

    val y5 = fr_val(pw.v_limbs.nth(12))
    val tk5 = cswap_l(tag_out, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val o5 = perm([tk5, fr_val(pw.v_limbs.nth(11)), CAP_DIGEST])
    val e5_replay_ok =
      cswap_bool_ok(tag_out) and cswap_bool_ok(pw.mode) and (cswap_l(pw.mode, y5, o5.nth(0)) == y5)

    and {
      pub_in_bb(pw), advice_in_bb(adv), mode_ok,
      z1a_ok, z1b_ok, zka_ok, zkb_ok,
      run_ent_ok, sp_ent_ok, e4_ent_ok, e5_op_ok, e5_param_ok,
      ein_ok, eout_ok, sp_limbs_ok,
      run_replay_ok, sp_replay_ok, e4_replay_ok, e5_replay_ok
    }
  }

  // Mutant 6: mut_no_cswap — omits the final cswap(mode, y, o[0]) on entry 5 (leaves y5 == y5 tautology).
  pure def mut_no_cswap(pw: PubWords, adv: Advice): bool = {
    val mode_ok = (bb_mul(pw.mode, bb_sub(pw.mode, 1)) == 0)
    val z1a_ok = is_zero_ok(bb_sub(pw.j_lo, 1), adv.z1a)
    val z1b_ok = is_zero_ok(pw.j_hi, adv.z1b)
    val z_j1 = bb_mul(adv.z1a.z, adv.z1b.z)
    val zka_ok = is_zero_ok(bb_sub(pw.j_lo, pw.k_lo), adv.zka)
    val zkb_ok = is_zero_ok(bb_sub(pw.j_hi, pw.k_hi), adv.zkb)
    val z_jk = bb_mul(adv.zka.z, adv.zkb.z)
    val tag_in = bb_mul(pw.mode, bb_sub(1, z_j1))
    val tag_out = bb_mul(pw.mode, bb_sub(1, z_jk))

    val run_ent_ok = 0.to(2).forall(i =>
      pw.entries.nth(i).op == DIGEST and pw.entries.nth(i).param == R_RUN.nth(i)
    )
    val sp_ent_ok = (pw.entries.nth(3).op == SPONGE) and (pw.entries.nth(3).param == 3)
    val e4_ent_ok = (pw.entries.nth(4).op == DIGEST) and (pw.entries.nth(4).param == tag_in)
    val e5_op_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(5).op, DIGEST)) == 0)
    val e5_param_ok = (bb_mul(pw.mode, bb_sub(pw.entries.nth(5).param, tag_out)) == 0)
    val ein_ok = (pw.e_in == ANCHOR_IN)
    val eout_ok = (pw.e_out == bb_add(ANCHOR_IN, pw.mode))
    val sp_limbs_ok = sponge_in_limbs_ok(pw)

    val y0 = fr_val(pw.v_limbs.nth(1))
    val y1 = fr_val(pw.v_limbs.nth(3))
    val y2 = fr_val(pw.v_limbs.nth(5))
    val run_replay_ok =
      (perm([tag_fr(R_RUN.nth(0)), fr_val(pw.v_limbs.nth(0)), CAP_DIGEST]).nth(0) == y0) and
      (perm([tag_fr(R_RUN.nth(1)), fr_val(pw.v_limbs.nth(2)), CAP_DIGEST]).nth(0) == y1) and
      (perm([tag_fr(R_RUN.nth(2)), fr_val(pw.v_limbs.nth(4)), CAP_DIGEST]).nth(0) == y2)

    val s_first = perm([y0, y1, 3])
    val s_second = perm([fr_add(s_first.nth(0), y2), s_first.nth(1), s_first.nth(2)])
    val sp_replay_ok = (s_second.nth(0) == fr_val(pw.v_limbs.nth(9)))

    val tk4 = cswap_l(tag_in, tag_fr(SHK_PROGC), tag_fr(SHK_LOCK))
    val e4_replay_ok =
      cswap_bool_ok(tag_in) and
      (perm([tk4, fr_val(pw.v_limbs.nth(10)), CAP_DIGEST]).nth(0) == fr_val(pw.v_limbs.nth(11)))

    // Missing cswap(mode, y5, o5[0]): asserts y5 == y5 instead of selecting o5[0] when mode == 1
    val y5 = fr_val(pw.v_limbs.nth(13))
    val e5_replay_ok = cswap_bool_ok(tag_out) and (y5 == y5)

    and {
      pub_in_bb(pw), advice_in_bb(adv), mode_ok,
      z1a_ok, z1b_ok, zka_ok, zkb_ok,
      run_ent_ok, sp_ent_ok, e4_ent_ok, e5_op_ok, e5_param_ok,
      ein_ok, eout_ok, sp_limbs_ok,
      run_replay_ok, sp_replay_ok, e4_replay_ok, e5_replay_ok
    }
  }

  // --------------------------------------------------------------------------
  // 9. Pure soundness & completeness predicates and state machine
  // --------------------------------------------------------------------------

  pure def sound_at(pw: PubWords, adv: Advice): bool =
    gen_r_check_ok(pw, adv) implies (
      native_r_rule_holds(pw) and
      (circuit_tag_in(pw, adv) == native_anchor_tags_limbwise(pw).t_in) and
      (circuit_tag_out(pw, adv) == native_anchor_tags_limbwise(pw).t_out)
    )

  pure def complete_at(pw: PubWords): bool =
    gen_r_check_ok(pw, honest_advice(pw))

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
    nondet branch = Set(0, 1, 2, 3).oneOf()
    nondet mode = Set(0, 1, 2).oneOf()
    nondet j_lo = Set(0, 1, 2, 3, 4).oneOf()
    nondet j_hi = Set(0, 1, 4).oneOf()
    nondet k_lo = Set(0, 1, 2, 3).oneOf()
    nondet k_hi = Set(0, 1).oneOf()
    nondet d0 = Set(0, 1, 2, 5).oneOf()
    nondet d1 = Set(0, 2, 4).oneOf()
    nondet d2 = Set(1, 3, 6).oneOf()
    nondet d_in = Set(0, 1, 7).oneOf()
    nondet d_out = Set(0, 2, 8).oneOf()
    nondet e5_op = Set(NOP, PUB, DIGEST).oneOf()
    nondet e5_param = Set(0, 1, 2).oneOf()
    nondet bad_w = Set(0, 1, 2, 9).oneOf()
    nondet bad_z = Set(0, 1, 2).oneOf()
    nondet bad_tag = Set(SHK_PROGC, SHK_LOCK, SHK_INPUT).oneOf()
    nondet bad_limb = Set(0, 1, 2, 3, 4, 5).oneOf()

    val base_mode = if (mode == 2) 1 else mode
    val base_pw = honest_pub_words(base_mode, j_lo, j_hi, k_lo, k_hi, d0, d1, d2, d_in, d_out, e5_op, e5_param, 0, 0)
    val base_adv = honest_advice(base_pw)

    val next_pw =
      if (branch == 0) base_pw
      else if (branch == 1) { ...base_pw, mode: mode, e_out: bb_add(ANCHOR_IN, mode) }
      else if (branch == 2) { ...base_pw, entries: base_pw.entries.replaceAt(4, { op: DIGEST, param: bad_tag }) }
      else { ...base_pw, v_limbs: base_pw.v_limbs.replaceAt(13, [bad_limb, 0]) }

    val next_adv =
      if (branch == 1) { ...base_adv, z1a: { w: bad_w, z: bad_z } }
      else base_adv

    val next_honest = (branch == 0)

    all {
      st_pw' = next_pw,
      st_adv' = next_adv,
      st_honest' = next_honest
    }
  }

  // --------------------------------------------------------------------------
  // 10. Invariants: SOUND and COMPLETE
  // --------------------------------------------------------------------------

  // SOUND: every assignment (st_pw, st_adv) that satisfies gen_r_check_ok has the
  // entry structure and tags of the native rule and replay outputs equal to the
  // native functions of the decoded values.
  val SOUND: bool =
    sound_at(st_pw, st_adv)

  // COMPLETE: every honest witness (constructed from l_vals with honest_advice)
  // satisfies every assertion of gen_r_check_ok.
  val COMPLETE: bool =
    st_honest implies gen_r_check_ok(st_pw, st_adv)

  // Combined invariant for quint run
  val inv_sound_and_complete: bool =
    SOUND and COMPLETE

  // Extra invariant showing what jk_range_ok adds: when gen_r_check_with_jk_range_ok holds,
  // the limbwise and combined-integer anchor tags agree AND (for mode == 1) 1 <= j <= k.
  val inv_jk_range_strengthened: bool =
    gen_r_check_with_jk_range_ok(st_pw, st_adv) implies (
      (native_anchor_tags_limbwise(st_pw) == native_anchor_tags_combined(st_pw)) and
      ((st_pw.mode == 0) or (
        (1 <= st_pw.j_lo + JK_BASE * st_pw.j_hi) and
        (st_pw.j_lo + JK_BASE * st_pw.j_hi <= st_pw.k_lo + JK_BASE * st_pw.k_hi)
      ))
    )


  // Lead addition (2026-11-13): the same property on the check WITHOUT the j, k rule (f725b15).
  // Expected to be VIOLATED (Q6): the old check accepts j = 0, j > k and aliased limbs.
  val inv_jk_unchecked_expect_violation: bool =
    gen_r_check_ok(st_pw, st_adv) implies (
      (native_anchor_tags_limbwise(st_pw) == native_anchor_tags_combined(st_pw)) and
      ((st_pw.mode == 0) or (
        (1 <= st_pw.j_lo + JK_BASE * st_pw.j_hi) and
        (st_pw.j_lo + JK_BASE * st_pw.j_hi <= st_pw.k_lo + JK_BASE * st_pw.k_hi)
      ))
    )
}
