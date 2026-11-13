// -*- mode: Bluespec; -*-
// Unit, exhaustive-enumeration, and mutation tests for qa/gemini-quint/e3-rreplay/r2/rreplay2.qnt.
// Covers Questions 1..6, SOUND, COMPLETE, log_merge, and all 5 mutants of the new j, k block.

module rreplay2_test {
  import rreplay2.* from "./rreplay2"

  // --------------------------------------------------------------------------
  // Question 1: BitsAir soundness for LowBits(x, n) and r16(x) on all 4 limbs
  // --------------------------------------------------------------------------

  run q1_r16_bits_air_exhaustiveTest = {
    val exp_mask = [1, 1, 0, 0, 0]

    // 1. For every boolean non-increasing mask s_bits of length 5 with sum == JK_BITS (2),
    //    s_bits is uniquely forced to [1, 1, 0, 0, 0].
    val mask_unique =
      0.to(1).forall(s0 =>
        0.to(1).forall(s1 =>
          0.to(1).forall(s2 =>
            0.to(1).forall(s3 =>
              0.to(1).forall(s4 => {
                val sb = [s0, s1, s2, s3, s4]
                val mono =
                  (s1 * (1 - s0) == 0) and
                  (s2 * (1 - s1) == 0) and
                  (s3 * (1 - s2) == 0) and
                  (s4 * (1 - s3) == 0)
                val sum_ok = (s0 + s1 + s2 + s3 + s4 == JK_BITS)
                (mono and sum_ok) iff (sb == exp_mask)
              })
            )
          )
        )
      )

    // 2. For every canonical BabyBear word x in [0, P):
    //    - honest_b_bits(x) and honest_s_bits(JK_BITS) satisfy bits_air_row_ok(x, ..., x % JK_BASE),
    //    - and (x % JK_BASE == x) holds iff 0 <= x < JK_BASE.
    val bits_sound_and_complete =
      0.to(P - 1).forall(x => {
        val bb = honest_b_bits(x)
        val sb = honest_s_bits(JK_BITS)
        val lo = low_bits_jk(x)
        and {
          bits_air_row_ok(x, bb, sb, JK_BITS, lo),
          r16_ok(x) iff ((0 <= x) and (x < JK_BASE)),
          (bits_air_row_ok(x, bb, sb, JK_BITS, x)) iff ((0 <= x) and (x < JK_BASE))
        }
      })

    // 3. r16 is enforced on all 4 limbs [j_lo, j_hi, k_lo, k_hi] in both mode 0 and mode 1.
    val base0 = honest_pub_words(0, 0, 0, 0, 0, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    val base1 = honest_pub_words(1, 1, 0, 2, 0, 1, 2, 3, 1, 2, DIGEST, SHK_LOCK, 0, 0)

    val bad0_jlo = { ...base0, j_lo: JK_BASE }
    val bad0_jhi = { ...base0, j_hi: JK_BASE }
    val bad0_klo = { ...base0, k_lo: JK_BASE }
    val bad0_khi = { ...base0, k_hi: JK_BASE }

    val bad1_jlo = { ...base1, j_lo: JK_BASE }
    val bad1_jhi = { ...base1, j_hi: JK_BASE }
    val bad1_klo = { ...base1, k_lo: JK_BASE }
    val bad1_khi = { ...base1, k_hi: JK_BASE }

    all {
      assert(mask_unique),
      assert(bits_sound_and_complete),
      assert(gen_r_check_ok(base0, honest_advice(base0))),
      assert(gen_r_check_ok(base1, honest_advice(base1))),
      assert(not(gen_r_check_ok(bad0_jlo, honest_advice(bad0_jlo)))),
      assert(not(gen_r_check_ok(bad0_jhi, honest_advice(bad0_jhi)))),
      assert(not(gen_r_check_ok(bad0_klo, honest_advice(bad0_klo)))),
      assert(not(gen_r_check_ok(bad0_khi, honest_advice(bad0_khi)))),
      assert(not(gen_r_check_ok(bad1_jlo, honest_advice(bad1_jlo)))),
      assert(not(gen_r_check_ok(bad1_jhi, honest_advice(bad1_jhi)))),
      assert(not(gen_r_check_ok(bad1_klo, honest_advice(bad1_klo)))),
      assert(not(gen_r_check_ok(bad1_khi, honest_advice(bad1_khi))))
    }
  }

  // --------------------------------------------------------------------------
  // Question 2: Exhaustive check over every limb tuple and both borrow values c
  //             (checked at both P = 17 and P_LARGE = 37 > 2^(2*2+1) = 32)
  // --------------------------------------------------------------------------

  run q2_order_and_wrap_exhaustiveTest = {
    // Verify F_17 and F_37 inverse tables first
    val inv17_ok = 0.to(P - 1).forall(a =>
      if (a == 0) bb_inv(a) == 0 else bb_mul(a, bb_inv(a)) == 1
    )
    val inv37_ok = 0.to(P_LARGE - 1).forall(a =>
      if (a == 0) bb_inv_37(a) == 0 else mod_mul(a, bb_inv_37(a), P_LARGE) == 1
    )

    // Exhaustive enumeration over all 4^4 = 256 limb tuples (j_lo, j_hi, k_lo, k_hi) in [0, JK_BASE)^4
    val exhaustive_p17 =
      0.to(JK_BASE - 1).forall(j_lo =>
        0.to(JK_BASE - 1).forall(j_hi =>
          0.to(JK_BASE - 1).forall(k_lo =>
            0.to(JK_BASE - 1).forall(k_hi => {
              val j_comb = j_lo + JK_BASE * j_hi
              val k_comb = k_lo + JK_BASE * k_hi
              val zj0 = honest_zero_adv(bb_add(j_lo, j_hi))
              val exp_c = if (k_lo < j_lo) 1 else 0
              val ok_c0_m1 = jk_block_ok_mod(1, j_lo, j_hi, k_lo, k_hi, zj0, 0, P)
              val ok_c1_m1 = jk_block_ok_mod(1, j_lo, j_hi, k_lo, k_hi, zj0, 1, P)
              val ok_c0_m0 = jk_block_ok_mod(0, j_lo, j_hi, k_lo, k_hi, zj0, 0, P)
              val ok_c1_m0 = jk_block_ok_mod(0, j_lo, j_hi, k_lo, k_hi, zj0, 1, P)
              and {
                // (j_lo + j_hi) == 0 mod P iff j == 0
                (bb_add(j_lo, j_hi) == 0) iff (j_comb == 0),
                // In mode 1, c = 0 passes iff 1 <= j <= k and k_lo >= j_lo
                ok_c0_m1 iff ((1 <= j_comb) and (j_comb <= k_comb) and (exp_c == 0)),
                // In mode 1, c = 1 passes iff 1 <= j <= k and k_lo < j_lo
                ok_c1_m1 iff ((1 <= j_comb) and (j_comb <= k_comb) and (exp_c == 1)),
                // In mode 1, exists c in {0, 1} passing iff 1 <= j <= k
                (ok_c0_m1 or ok_c1_m1) iff ((1 <= j_comb) and (j_comb <= k_comb)),
                // In mode 0, BOTH c = 0 and c = 1 satisfy all constraints for all 16-bit limbs
                ok_c0_m0 and ok_c1_m0
              }
            })
          )
        )
      )

    // Same exhaustive enumeration over P_LARGE = 37 (> 2^(2*2+1) = 32)
    val exhaustive_p37 =
      0.to(JK_BASE - 1).forall(j_lo =>
        0.to(JK_BASE - 1).forall(j_hi =>
          0.to(JK_BASE - 1).forall(k_lo =>
            0.to(JK_BASE - 1).forall(k_hi => {
              val j_comb = j_lo + JK_BASE * j_hi
              val k_comb = k_lo + JK_BASE * k_hi
              val zj0 = honest_zero_adv_37(mod_add(j_lo, j_hi, P_LARGE))
              val exp_c = if (k_lo < j_lo) 1 else 0
              val ok_c0_m1 = jk_block_ok_mod(1, j_lo, j_hi, k_lo, k_hi, zj0, 0, P_LARGE)
              val ok_c1_m1 = jk_block_ok_mod(1, j_lo, j_hi, k_lo, k_hi, zj0, 1, P_LARGE)
              val ok_c0_m0 = jk_block_ok_mod(0, j_lo, j_hi, k_lo, k_hi, zj0, 0, P_LARGE)
              val ok_c1_m0 = jk_block_ok_mod(0, j_lo, j_hi, k_lo, k_hi, zj0, 1, P_LARGE)
              and {
                (mod_add(j_lo, j_hi, P_LARGE) == 0) iff (j_comb == 0),
                ok_c0_m1 iff ((1 <= j_comb) and (j_comb <= k_comb) and (exp_c == 0)),
                ok_c1_m1 iff ((1 <= j_comb) and (j_comb <= k_comb) and (exp_c == 1)),
                (ok_c0_m1 or ok_c1_m1) iff ((1 <= j_comb) and (j_comb <= k_comb)),
                ok_c0_m0 and ok_c1_m0
              }
            })
          )
        )
      )

    all {
      assert(inv17_ok),
      assert(inv37_ok),
      assert(exhaustive_p17),
      assert(exhaustive_p37)
    }
  }

  // --------------------------------------------------------------------------
  // Question 3: Agreement between tape and reader_anchor_tags, and Round-1 counterexamples
  // --------------------------------------------------------------------------

  run q3_agreement_and_round1_counterexamplesTest = {
    // 1. Exhaustive agreement across mode in {0, 1, 2} and limbs in 0..JK_BASE (including JK_BASE = 4):
    //    jk_block_ok + mode_bit holds (for some valid advice) iff reader_anchor_tags returns ok,
    //    and whenever accepted, limbwise_tags == reader_anchor_tags.
    val agreement_ok =
      0.to(2).forall(mode =>
        0.to(JK_BASE).forall(j_lo =>
          0.to(JK_BASE).forall(j_hi =>
            0.to(JK_BASE).forall(k_lo =>
              0.to(JK_BASE).forall(k_hi => {
                val rt = reader_anchor_tags_words(mode, j_lo, j_hi, k_lo, k_hi)
                val lt = limbwise_tags(mode, j_lo, j_hi, k_lo, k_hi)
                val zj0 = honest_zero_adv(bb_add(j_lo, j_hi))
                val c = if ((mode == 1) and (k_lo < j_lo)) 1 else 0
                val mode_bit = (bb_mul(mode, bb_sub(mode, 1)) == 0)
                val tape_jk_ok = mode_bit and jk_block_ok_mod(mode, j_lo, j_hi, k_lo, k_hi, zj0, c, P)
                and {
                  tape_jk_ok iff rt.ok,
                  rt.ok implies (lt == rt)
                }
              })
            )
          )
        )
      )

    // 2. Concrete check on the 4 Round-1 Q6 counterexamples (all now refused by both tape and reader):
    //    (a) j = 0 in chunk mode: j = (0, 0), k = (3, 0)
    val pw_j0 = honest_pub_words(1, 0, 0, 3, 0, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    //    (b) j > k in low limb: j = (3, 0), k = (2, 0)
    val pw_j_gt_k_lo = honest_pub_words(1, 3, 0, 2, 0, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    //    (c) j > k across high limb: j = (0, 2) (8), k = (3, 1) (7)
    val pw_j_gt_k_hi = honest_pub_words(1, 0, 2, 3, 1, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    //    (d) Round-1 limb aliasing counterexample: j = (JK_BASE, 0) = (4, 0), k = (0, 1)
    val pw_alias_jk = honest_pub_words(1, JK_BASE, 0, 0, 1, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    //    (e) Limb >= JK_BASE on k_hi: j = (2, 0), k = (3, JK_BASE)
    val pw_k_limb = honest_pub_words(1, 2, 0, 3, JK_BASE, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)

    all {
      assert(agreement_ok),
      assert(not(reader_anchor_tags(pw_j0).ok)),
      assert(not(gen_r_check_ok(pw_j0, honest_advice(pw_j0)))),
      assert(not(reader_anchor_tags(pw_j_gt_k_lo).ok)),
      assert(not(gen_r_check_ok(pw_j_gt_k_lo, honest_advice(pw_j_gt_k_lo)))),
      assert(not(reader_anchor_tags(pw_j_gt_k_hi).ok)),
      assert(not(gen_r_check_ok(pw_j_gt_k_hi, honest_advice(pw_j_gt_k_hi)))),
      assert(not(reader_anchor_tags(pw_alias_jk).ok)),
      assert(not(gen_r_check_ok(pw_alias_jk, honest_advice(pw_alias_jk)))),
      // Even if a dishonest prover tries both c = 0 and c = 1 on pw_alias_jk, the tape refuses:
      assert(not(gen_r_check_ok(pw_alias_jk, { ...honest_advice(pw_alias_jk), c: 0 }))),
      assert(not(gen_r_check_ok(pw_alias_jk, { ...honest_advice(pw_alias_jk), c: 1 }))),
      assert(not(reader_anchor_tags(pw_k_limb).ok)),
      assert(not(gen_r_check_ok(pw_k_limb, honest_advice(pw_k_limb))))
    }
  }

  // --------------------------------------------------------------------------
  // Question 4: Completeness across all 1 <= j <= k < JK_BASE^2 and ordinary calls
  //             (including reduced-scale counterparts of j9_jk_range_on_words)
  // --------------------------------------------------------------------------

  run q4_completeness_edges_and_ordinaryTest = {
    // 1. Reduced-scale counterparts of the 5 positive cases in j9_jk_range_on_words:
    //    - chunk_1_of_1: j = 1 = (1, 0), k = 1 = (1, 0)
    val pw_1_of_1 = honest_pub_words(1, 1, 0, 1, 0, 1, 2, 3, 1, 1, DIGEST, SHK_PROGC, 0, 0)
    //    - chunk_ffff_of_10000: j = JK_BASE - 1 = (3, 0), k = JK_BASE = (0, 1) (borrow c = 1!)
    val pw_maxlo_of_base = honest_pub_words(1, 3, 0, 0, 1, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    //    - chunk_10000_of_10001: j = JK_BASE = (0, 1), k = JK_BASE + 1 = (1, 1) (c = 0)
    val pw_base_of_base1 = honest_pub_words(1, 0, 1, 1, 1, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    //    - chunk_2_of_20000: j = 2 = (2, 0), k = 2 * JK_BASE = (0, 2) (borrow c = 1!)
    val pw_2_of_2base = honest_pub_words(1, 2, 0, 0, 2, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    //    - chunk_full_of_full: j = k = JK_BASE^2 - 1 = 15 = (3, 3) (c = 0)
    val pw_full_of_full = honest_pub_words(1, 3, 3, 3, 3, 1, 2, 3, 4, 1, DIGEST, SHK_PROGC, 0, 0)

    // 2. Exhaustive completeness check over all 256 limb tuples in [0, JK_BASE)^4 for both modes:
    val all_valid_complete =
      0.to(1).forall(mode =>
        0.to(JK_BASE - 1).forall(j_lo =>
          0.to(JK_BASE - 1).forall(j_hi =>
            0.to(JK_BASE - 1).forall(k_lo =>
              0.to(JK_BASE - 1).forall(k_hi => {
                val pw = honest_pub_words(mode, j_lo, j_hi, k_lo, k_hi, 1, 2, 3, 1, 2, NOP, 0, 0, 0)
                val adv = honest_advice(pw)
                val j_comb = j_lo + JK_BASE * j_hi
                val k_comb = k_lo + JK_BASE * k_hi
                val valid = (mode == 0) or ((1 <= j_comb) and (j_comb <= k_comb))
                and {
                  (gen_r_check_ok(pw, adv) iff valid),
                  (reader_anchor_tags(pw).ok iff valid),
                  valid implies sound_at(pw, adv)
                }
              })
            )
          )
        )
      )

    all {
      assert(gen_r_check_ok(pw_1_of_1, honest_advice(pw_1_of_1))),
      assert(reader_anchor_tags(pw_1_of_1) == { ok: true, t_in: SHK_PROGC, t_out: SHK_PROGC }),
      assert(honest_borrow(pw_maxlo_of_base) == 1),
      assert(gen_r_check_ok(pw_maxlo_of_base, honest_advice(pw_maxlo_of_base))),
      assert(reader_anchor_tags(pw_maxlo_of_base) == { ok: true, t_in: SHK_LOCK, t_out: SHK_LOCK }),
      assert(honest_borrow(pw_base_of_base1) == 0),
      assert(gen_r_check_ok(pw_base_of_base1, honest_advice(pw_base_of_base1))),
      assert(reader_anchor_tags(pw_base_of_base1) == { ok: true, t_in: SHK_LOCK, t_out: SHK_LOCK }),
      assert(honest_borrow(pw_2_of_2base) == 1),
      assert(gen_r_check_ok(pw_2_of_2base, honest_advice(pw_2_of_2base))),
      assert(reader_anchor_tags(pw_2_of_2base) == { ok: true, t_in: SHK_LOCK, t_out: SHK_LOCK }),
      assert(honest_borrow(pw_full_of_full) == 0),
      assert(gen_r_check_ok(pw_full_of_full, honest_advice(pw_full_of_full))),
      assert(reader_anchor_tags(pw_full_of_full) == { ok: true, t_in: SHK_LOCK, t_out: SHK_PROGC }),
      assert(all_valid_complete)
    }
  }

  // --------------------------------------------------------------------------
  // Question 5: Native log_merge (identities, associativity in n, non-associativity in d,
  //             and the output range check n < LOG_N_LIMIT)
  // --------------------------------------------------------------------------

  run q5_log_merge_propertiesTest = {
    val empty_log = { d: D_EMPTY, n: 0 }
    val s0 = { d: 1, n: 1 }
    val s1 = { d: 2, n: 2 }
    val s2 = { d: 3, n: 3 }

    // 1. Injectivity of p2_tag_node over leaf digests (1..3) and depth-1 node digests
    val n12 = p2_tag_node(1, 2)
    val n23 = p2_tag_node(2, 3)
    val dom = Set(1, 2, 3, n12, n23)
    val hash_injective =
      dom.forall(a1 =>
        dom.forall(b1 =>
          dom.forall(a2 =>
            dom.forall(b2 =>
              (p2_tag_node(a1, b1) == p2_tag_node(a2, b2)) iff ((a1 == a2) and (b1 == b2))
            )
          )
        )
      )

    // 2. Empty identities (mirrors enc2_node_merge in lane-src-r2/iface.rs:984-989)
    val node01 = log_merge(s0, s1)
    val id_ok = and {
      node01.ok,
      log_merge(empty_log, empty_log) == { ok: true, log: empty_log },
      log_merge(empty_log, s0) == { ok: true, log: s0 },
      log_merge(s0, empty_log) == { ok: true, log: s0 },
      log_merge(empty_log, node01.log) == { ok: true, log: node01.log },
      log_merge(node01.log, empty_log) == { ok: true, log: node01.log }
    }

    // 3. Non-commutativity in d (segment order counts) and non-associativity in d (tree shape counts),
    //    while associative in n (mirrors enc2_node_merge in lane-src-r2/iface.rs:990-998)
    val node10 = log_merge(s1, s0)
    val node12 = log_merge(s1, s2)
    val left = log_merge(node01.log, s2)
    val right = log_merge(s0, node12.log)

    // Exhaustive check over all non-empty leaf triples (d0, d1, d2) in {1, 2, 3}^3
    // and counts (n0, n1, n2) in {1, 2, 3}^3 (where n0 + n1 + n2 <= 9 < LOG_N_LIMIT = 16):
    val shape_and_assoc_ok =
      1.to(3).forall(d0 =>
        1.to(3).forall(d1 =>
          1.to(3).forall(d2 =>
            1.to(3).forall(n0 =>
              1.to(3).forall(n1 =>
                1.to(3).forall(n2 => {
                  val l0 = { d: d0, n: n0 }
                  val l1 = { d: d1, n: n1 }
                  val l2 = { d: d2, n: n2 }
                  val m01 = log_merge(l0, l1)
                  val m12 = log_merge(l1, l2)
                  val l_tree = log_merge(m01.log, l2)
                  val r_tree = log_merge(l0, m12.log)
                  and {
                    l_tree.ok and r_tree.ok,
                    // Associative in n:
                    l_tree.log.n == r_tree.log.n,
                    l_tree.log.n == n0 + n1 + n2,
                    // Non-associative in d (tree shape always differs for non-empty segments):
                    l_tree.log.d != r_tree.log.d
                  }
                })
              )
            )
          )
        )
      )

    // 4. Range check n < LOG_N_LIMIT (mirrors enc2_node_merge lines 1000-1002):
    val node6 = left.log // n = 1 + 2 + 3 = 6
    val big_refused = log_merge({ d: node6.d, n: LOG_N_LIMIT - 6 }, node6)
    val edge_accepted = log_merge({ d: node6.d, n: LOG_N_LIMIT - 7 }, node6)
    val empty_over_limit = log_merge(empty_log, { d: 1, n: LOG_N_LIMIT })

    all {
      assert(hash_injective),
      assert(id_ok),
      assert(node01.log.n == node10.log.n),
      assert(node01.log.d != node10.log.d),
      assert(left.ok and right.ok),
      assert(left.log.n == 6 and right.log.n == 6),
      assert(left.log.d != right.log.d),
      assert(shape_and_assoc_ok),
      assert(not(big_refused.ok)),
      assert(edge_accepted.ok and (edge_accepted.log.n == LOG_N_LIMIT - 1)),
      assert(not(empty_over_limit.ok))
    }
  }

  // --------------------------------------------------------------------------
  // Mutation Tests for the 5 Mutants of the new j, k block
  // --------------------------------------------------------------------------

  // Mutant 1 (`mut_no_r16_jhi`):
  // Drops `r16(j_hi)`. Caught in BOTH mode 0 and mode 1:
  //   - In mode 1: j = (2, 16) (where j_hi = 16 = P - 1 >= JK_BASE = 4, j_lo + j_hi = 18 == 1 mod 17 != 0),
  //     k = (2, 0), c = 0 gives k_hi - j_hi - c = 0 - 16 - 0 == 1 mod 17 < 4!
  //   - In mode 0: j = (0, 4) (j_hi = 4 >= JK_BASE), k = (0, 0), c = 0.
  // Both are accepted by `mut_no_r16_jhi` and refused by `reader_anchor_tags`, `native_r_rule_holds`,
  // and `gen_r_check_ok`.
  run mut1_no_r16_jhi_caughtTest = {
    val pw_m1 = honest_pub_words(1, 2, 16, 2, 0, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    val adv_m1 = { ...honest_advice(pw_m1), c: 0 }

    val pw_m0 = honest_pub_words(0, 0, JK_BASE, 0, 0, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    val adv_m0 = honest_advice(pw_m0)

    all {
      assert(mut_no_r16_jhi(pw_m1, adv_m1)),
      assert(not(reader_anchor_tags(pw_m1).ok)),
      assert(not(native_r_rule_holds(pw_m1))),
      assert(not(gen_r_check_ok(pw_m1, adv_m1))),
      assert(mut_no_r16_jhi(pw_m0, adv_m0)),
      assert(not(reader_anchor_tags(pw_m0).ok)),
      assert(not(native_r_rule_holds(pw_m0))),
      assert(not(gen_r_check_ok(pw_m0, adv_m0)))
    }
  }

  // Mutant 2 (`mut_borrow_not_bit`):
  // Drops `c * (c - 1) == 0`.
  // In mode 1 with j = (2, 1) (j = 6) and k = (3, 0) (k = 3 < j = 6), setting non-boolean borrow
  // c = 13 (== -4 mod 17, so 4 * c == 1 mod 17; the reduced-scale counterpart of c = p - 30720
  // in BabyBear where 2^16 * c == 1 mod p) yields:
  //   k_lo - j_lo + 4 * c = 3 - 2 + 52 = 53 == 2 (mod 17) in [0, 4),
  //   k_hi - j_hi - c     = 0 - 1 - 13 = -14 == 3 (mod 17) in [0, 4).
  // Accepted by `mut_borrow_not_bit`, refused by `reader_anchor_tags`, `native_r_rule_holds`,
  // and `gen_r_check_ok`.
  run mut2_borrow_not_bit_caughtTest = {
    val pw_forged = honest_pub_words(1, 2, 1, 3, 0, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    val adv_forged = { ...honest_advice(pw_forged), c: 13 }

    all {
      assert(mut_borrow_not_bit(pw_forged, adv_forged)),
      assert(not(reader_anchor_tags(pw_forged).ok)),
      assert(not(native_r_rule_holds(pw_forged))),
      assert(not(gen_r_check_ok(pw_forged, adv_forged)))
    }
  }

  // Mutant 3 (`mut_order_unmasked`):
  // Leaves the order checks (`z_j0 == 0` and both borrow `r16` checks) unmasked by `mode`.
  // Caught on an honest ordinary call (`mode = 0, j = (0, 0), k = (0, 0)`), which is accepted
  // by `gen_r_check_ok`, `reader_anchor_tags`, and `native_r_rule_holds`, but falsely rejected
  // by `mut_order_unmasked` (completeness violation).
  // Also caught on `mode = 0, j = (3, 0), k = (0, 1)` where even `1 <= j <= k` holds, because
  // honest advice sets `c = 0` in `mode = 0`, so unmasked `k_lo - j_lo + 4*c = -3 == 14 mod 17`
  // fails `r16`.
  run mut3_order_unmasked_caughtTest = {
    val pw_ord0 = honest_pub_words(0, 0, 0, 0, 0, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    val adv_ord0 = honest_advice(pw_ord0)

    val pw_ord_borrow = honest_pub_words(0, 3, 0, 0, 1, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    val adv_ord_borrow = honest_advice(pw_ord_borrow)

    all {
      assert(gen_r_check_ok(pw_ord0, adv_ord0)),
      assert(reader_anchor_tags(pw_ord0).ok),
      assert(native_r_rule_holds(pw_ord0)),
      assert(not(mut_order_unmasked(pw_ord0, adv_ord0))),
      assert(gen_r_check_ok(pw_ord_borrow, adv_ord_borrow)),
      assert(reader_anchor_tags(pw_ord_borrow).ok),
      assert(native_r_rule_holds(pw_ord_borrow)),
      assert(not(mut_order_unmasked(pw_ord_borrow, adv_ord_borrow)))
    }
  }

  // Mutant 4 (`mut_j_ge_1_dropped`):
  // Drops `mode * z_j0 == 0` (the j >= 1 check).
  // Caught by `mode = 1, j = (0, 0), k = (1, 0)` (j = 0 <= k = 1): accepted by `mut_j_ge_1_dropped`,
  // refused by `reader_anchor_tags`, `native_r_rule_holds`, and `gen_r_check_ok`.
  run mut4_j_ge_1_dropped_caughtTest = {
    val pw_j0 = honest_pub_words(1, 0, 0, 1, 0, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    val adv_j0 = honest_advice(pw_j0)

    all {
      assert(mut_j_ge_1_dropped(pw_j0, adv_j0)),
      assert(not(reader_anchor_tags(pw_j0).ok)),
      assert(not(native_r_rule_holds(pw_j0))),
      assert(not(gen_r_check_ok(pw_j0, adv_j0)))
    }
  }

  // Mutant 5 (`mut_plus_c_in_hi`):
  // Uses `k_hi - j_hi + c` instead of `k_hi - j_hi - c` in the high-limb borrow equation.
  // Caught by `mode = 1, j = (2, 1) (6), k = (1, 1) (5)` with `c = 1`:
  //   k_lo - j_lo + 4 * c = 1 - 2 + 4 = 3 in [0, 4),
  //   k_hi - j_hi + c     = 1 - 1 + 1 = 1 in [0, 4) (whereas k_hi - j_hi - c = -1 == 16 mod 17 >= 4).
  // Accepted by `mut_plus_c_in_hi`, refused by `reader_anchor_tags`, `native_r_rule_holds`,
  // and `gen_r_check_ok`.
  run mut5_plus_c_in_hi_caughtTest = {
    val pw_forged = honest_pub_words(1, 2, 1, 1, 1, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    val adv_forged = honest_advice(pw_forged)

    all {
      assert(mut_plus_c_in_hi(pw_forged, adv_forged)),
      assert(not(reader_anchor_tags(pw_forged).ok)),
      assert(not(native_r_rule_holds(pw_forged))),
      assert(not(gen_r_check_ok(pw_forged, adv_forged)))
    }
  }

}
