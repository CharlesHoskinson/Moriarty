// -*- mode: Bluespec; -*-
// Unit and property tests for qa/gemini-quint/e3-rreplay/rreplay.qnt.
// Covers Questions 1..6, SOUND, COMPLETE, jk_range_ok, and all 6 mutants.

module rreplay_test {
  import rreplay.* from "./rreplay"

  // --------------------------------------------------------------------------
  // Question 1: Zero tests, mode bit, and forced tag_in / tag_out
  // --------------------------------------------------------------------------

  // Exhaustive check over F_17: mode * (mode - 1) == 0 forces mode in {0, 1},
  // and is_zero_ok(a, za) forces za.z == [a == 0] for every prover choice (w, z) in (F_17)^2.
  run q1_zero_test_and_mode_bit_exhaustiveTest = all {
    assert(
      0.to(P - 1).forall(m =>
        (bb_mul(m, bb_sub(m, 1)) == 0) iff ((m == 0) or (m == 1))
      )
    ),
    assert(
      0.to(P - 1).forall(a =>
        is_zero_ok(a, honest_zero_adv(a)) and
        0.to(P - 1).forall(w =>
          0.to(P - 1).forall(z =>
            is_zero_ok(a, { w: w, z: z }) implies (z == (if (a == 0) 1 else 0))
          )
        )
      )
    )
  }

  // Check that for all mode in {0, 1} and limb combinations, honest advice forces
  // circuit_tag_in == mode * (1 - [j = 1]) and circuit_tag_out == mode * (1 - [j = k]).
  run q1_tags_forcedTest = all {
    assert(
      0.to(1).forall(m =>
        0.to(3).forall(j_lo =>
          0.to(1).forall(j_hi =>
            0.to(3).forall(k_lo =>
              0.to(1).forall(k_hi => {
                val pw = honest_pub_words(m, j_lo, j_hi, k_lo, k_hi, 1, 2, 3, 1, 2, NOP, 0, 0, 0)
                val adv = honest_advice(pw)
                val tp = native_anchor_tags_limbwise(pw)
                and {
                  gen_r_check_ok(pw, adv),
                  circuit_tag_in(pw, adv) == tp.t_in,
                  circuit_tag_out(pw, adv) == tp.t_out
                }
              })
            )
          )
        )
      )
    )
  }

  // --------------------------------------------------------------------------
  // Question 2: Entry checks and R_OFF value offset alignment
  // --------------------------------------------------------------------------

  run q2_offsets_and_entriesTest = {
    val pw_ord = honest_pub_words(0, 0, 0, 0, 0, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    val adv_ord = honest_advice(pw_ord)
    // In mode 0, entry 5 can be a discretionary entry (e.g. PUB(2))
    val pw_ord_pub5 = honest_pub_words(0, 0, 0, 0, 0, 1, 2, 3, 1, 0, PUB, 2, 5, 6)
    // Tampering with entry 1 param (SHK_LOCK instead of SHK_INPUT) fails
    val pw_bad_e1 = { ...pw_ord, entries: pw_ord.entries.replaceAt(1, { op: DIGEST, param: SHK_LOCK }) }
    // Tampering with e_out in mode 0 (5 instead of 4) fails
    val pw_bad_eout = { ...pw_ord, e_out: 5 }
    // In mode 1, tampering with entry 5 op (NOP instead of DIGEST) fails
    val pw_chunk = honest_pub_words(1, 1, 0, 3, 0, 1, 2, 3, 1, 2, DIGEST, SHK_LOCK, 0, 0)
    val adv_chunk = honest_advice(pw_chunk)
    val pw_chunk_bad_e5 = { ...pw_chunk, entries: pw_chunk.entries.replaceAt(5, { op: NOP, param: 0 }) }

    all {
      assert(r_offsets_match_vals),
      assert(gen_r_check_ok(pw_ord, adv_ord)),
      assert(gen_r_check_ok(pw_ord_pub5, honest_advice(pw_ord_pub5))),
      assert(not(gen_r_check_ok(pw_bad_e1, adv_ord))),
      assert(not(gen_r_check_ok(pw_bad_eout, adv_ord))),
      assert(gen_r_check_ok(pw_chunk, adv_chunk)),
      assert(not(gen_r_check_ok(pw_chunk_bad_e5, adv_chunk)))
    }
  }

  // --------------------------------------------------------------------------
  // Question 3: Native replay equivalence (DIGEST, SPONGE(3), and mode cswap)
  // --------------------------------------------------------------------------

  run q3_replay_and_cswapTest = {
    val pw_ord = honest_pub_words(0, 0, 0, 0, 0, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    val adv_ord = honest_advice(pw_ord)
    // In mode 0, changing value slot 13 (entry 5 output y) still passes because cswap(0, y, o[0]) == y
    val pw_ord_v13 = { ...pw_ord, v_limbs: pw_ord.v_limbs.replaceAt(13, [3, 2]) }

    // In mode 1, changing value slot 13 fails because cswap(1, y, o[0]) == o[0] != y
    val pw_chunk = honest_pub_words(1, 1, 0, 2, 0, 1, 2, 3, 1, 2, DIGEST, SHK_LOCK, 0, 0)
    val adv_chunk = honest_advice(pw_chunk)
    val bad_y5 = (val_at(pw_chunk, 13) + 1) % R_ORDER
    val pw_chunk_bad_v13 = { ...pw_chunk, v_limbs: pw_chunk.v_limbs.replaceAt(13, to_limbs(bad_y5)) }

    // Breaking SPONGE(3) limb equality with DIGEST output 0 fails
    val pw_bad_sp_in = { ...pw_chunk, v_limbs: pw_chunk.v_limbs.replaceAt(6, [0, 0]) }

    all {
      assert(gen_r_check_ok(pw_ord_v13, adv_ord)),
      assert(not(gen_r_check_ok(pw_chunk_bad_v13, adv_chunk))),
      assert(not(gen_r_check_ok(pw_bad_sp_in, adv_chunk)))
    }
  }

  // --------------------------------------------------------------------------
  // Question 4: Non-canonical value limbs (limb >= LIMB_BASE or integer >= R_ORDER)
  // --------------------------------------------------------------------------

  run q4_value_limbs_and_decoderTest = {
    // Base honest call with d0 = 4 (canonical limbs [0, 1]: 0 + 4*1 = 4) and d1 = 1 (canonical limbs [1, 0])
    val pw = honest_pub_words(0, 0, 0, 0, 0, 4, 1, 3, 4, 0, NOP, 0, 0, 0)
    val adv = honest_advice(pw)

    // Case A: Non-canonical limb >= LIMB_BASE (4): limbs [4, 0] evaluate to 4 + 4*0 = 4 in fr_val.
    // Tape accepts, but dec_limbs (decode) refuses!
    val pw_noncanon_limb = { ...pw, v_limbs: pw.v_limbs.replaceAt(0, [4, 0]) }

    // Case B: Limbs < LIMB_BASE (4), but integer >= R_ORDER (13):
    // Limbs [2, 3] have 2 < 4 and 3 < 4, and integer 2 + 4*3 = 14 == 1 (mod 13).
    // Replacing d1 = 1 (slot 2) with [2, 3]: tape accepts, but dec_limbs (decode) refuses!
    val pw_noncanon_int = { ...pw, v_limbs: pw.v_limbs.replaceAt(2, [2, 3]) }

    // Case C: Exhaustive check over all (l0, l1) in (F_17)^2:
    // whenever dec_limbs([l0, l1]).ok holds, dec_limbs([l0, l1]).fv == fr_val([l0, l1]).
    val canon_bijection_ok = 0.to(P - 1).forall(l0 =>
      0.to(P - 1).forall(l1 => {
        val d = dec_limbs([l0, l1])
        d.ok implies ((d.fv == fr_val([l0, l1])) and (to_limbs(d.fv) == [l0, l1]))
      })
    )

    all {
      assert(gen_r_check_ok(pw, adv) and all_values_canonical(pw)),
      assert(gen_r_check_ok(pw_noncanon_limb, adv)),
      assert(not(dec_limbs([4, 0]).ok)),
      assert(not(all_values_canonical(pw_noncanon_limb))),
      assert(gen_r_check_ok(pw_noncanon_int, adv)),
      assert(not(dec_limbs([2, 3]).ok)),
      assert(not(all_values_canonical(pw_noncanon_int))),
      assert(canon_bijection_ok)
    }
  }

  // --------------------------------------------------------------------------
  // Question 5: Completeness across all 5 call/chunk regimes
  // --------------------------------------------------------------------------

  run q5_completeness_all_regimesTest = {
    // 1. Ordinary call (mode = 0, j = 0, k = 0)
    val pw_ord = honest_pub_words(0, 0, 0, 0, 0, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    // 2. Chunk j = 1 of k = 3 (mode = 1, j = 1 < k): tag_in = SHK_PROGC, tag_out = SHK_LOCK
    val pw_j1_k3 = honest_pub_words(1, 1, 0, 3, 0, 1, 2, 3, 1, 4, DIGEST, SHK_LOCK, 0, 0)
    // 3. Interior chunk 1 < j < k (mode = 1, j = 2, k = 3): tag_in = SHK_LOCK, tag_out = SHK_LOCK
    val pw_j2_k3 = honest_pub_words(1, 2, 0, 3, 0, 1, 2, 3, 5, 6, DIGEST, SHK_LOCK, 0, 0)
    // 4. Last chunk j = k > 1 (mode = 1, j = 3, k = 3): tag_in = SHK_LOCK, tag_out = SHK_PROGC
    val pw_j3_k3 = honest_pub_words(1, 3, 0, 3, 0, 1, 2, 3, 6, 1, DIGEST, SHK_PROGC, 0, 0)
    // 5. Single-chunk call j = k = 1 (mode = 1, j = 1, k = 1): tag_in = SHK_PROGC, tag_out = SHK_PROGC
    val pw_j1_k1 = honest_pub_words(1, 1, 0, 1, 0, 1, 2, 3, 1, 1, DIGEST, SHK_PROGC, 0, 0)

    all {
      assert(complete_at(pw_ord) and sound_at(pw_ord, honest_advice(pw_ord))),
      assert(complete_at(pw_j1_k3) and sound_at(pw_j1_k3, honest_advice(pw_j1_k3))),
      assert(complete_at(pw_j2_k3) and sound_at(pw_j2_k3, honest_advice(pw_j2_k3))),
      assert(complete_at(pw_j3_k3) and sound_at(pw_j3_k3, honest_advice(pw_j3_k3))),
      assert(complete_at(pw_j1_k1) and sound_at(pw_j1_k1, honest_advice(pw_j1_k1))),
      assert(native_anchor_tags_limbwise(pw_ord) == { ok: true, t_in: SHK_PROGC, t_out: SHK_PROGC }),
      assert(native_anchor_tags_limbwise(pw_j1_k3) == { ok: true, t_in: SHK_PROGC, t_out: SHK_LOCK }),
      assert(native_anchor_tags_limbwise(pw_j2_k3) == { ok: true, t_in: SHK_LOCK, t_out: SHK_LOCK }),
      assert(native_anchor_tags_limbwise(pw_j3_k3) == { ok: true, t_in: SHK_LOCK, t_out: SHK_PROGC }),
      assert(native_anchor_tags_limbwise(pw_j1_k1) == { ok: true, t_in: SHK_PROGC, t_out: SHK_PROGC })
    }
  }

  // --------------------------------------------------------------------------
  // Question 6: Missing range and order checks on j, k
  // --------------------------------------------------------------------------

  run q6_missing_jk_range_consequencesTest = {
    // Consequence 1: j = 0 in chunk mode (j_lo = 0, j_hi = 0, k_lo = 1, k_hi = 0)
    // Accepted by gen_r_check_ok with (tag_in, tag_out) = (SHK_LOCK, SHK_LOCK), rejected by jk_range_ok.
    val pw_j0 = honest_pub_words(1, 0, 0, 1, 0, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    val adv_j0 = honest_advice(pw_j0)

    // Consequence 2: j > k in chunk mode (j_lo = 2, j_hi = 0, k_lo = 1, k_hi = 0)
    // Accepted by gen_r_check_ok with (tag_in, tag_out) = (SHK_LOCK, SHK_LOCK), rejected by jk_range_ok.
    val pw_j_gt_k = honest_pub_words(1, 2, 0, 1, 0, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    val adv_j_gt_k = honest_advice(pw_j_gt_k)

    // Consequence 3: Limb >= JK_BASE (4) aliasing j == k in combined integer lo + JK_BASE * hi:
    // j = (4, 0) -> 4 + 4*0 = 4; k = (0, 1) -> 0 + 4*1 = 4.
    // gen_r_check_ok sees (4, 0) != (0, 1) so forces tag_out = SHK_LOCK (1),
    // whereas native_anchor_tags_combined (iface.rs u32w) sees j == k == 4 and expects tag_out = SHK_PROGC (0)!
    val pw_alias_jk = honest_pub_words(1, 4, 0, 0, 1, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    val adv_alias_jk = honest_advice(pw_alias_jk)

    // Consequence 4: Limb >= JK_BASE (4) aliasing j == 1 mod JK_BASE^2 (16):
    // j = (1, 4) -> 1 + 4*4 = 17 == 1 (mod 16), but j_hi = 4 != 0 so gen_r_check_ok forces tag_in = SHK_LOCK (1).
    val pw_alias_j1 = honest_pub_words(1, 1, 4, 2, 0, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    val adv_alias_j1 = honest_advice(pw_alias_j1)

    // Valid chunk (j = 2, k = 3) passes both gen_r_check_ok and jk_range_ok, and both tag definitions agree.
    val pw_valid = honest_pub_words(1, 2, 0, 3, 0, 1, 2, 3, 4, 5, DIGEST, SHK_LOCK, 0, 0)
    val adv_valid = honest_advice(pw_valid)

    all {
      assert(gen_r_check_ok(pw_j0, adv_j0) and not(gen_r_check_with_jk_range_ok(pw_j0, adv_j0))),
      assert(gen_r_check_ok(pw_j_gt_k, adv_j_gt_k) and not(gen_r_check_with_jk_range_ok(pw_j_gt_k, adv_j_gt_k))),
      assert(gen_r_check_ok(pw_alias_jk, adv_alias_jk) and not(gen_r_check_with_jk_range_ok(pw_alias_jk, adv_alias_jk))),
      assert(native_anchor_tags_limbwise(pw_alias_jk).t_out == SHK_LOCK),
      assert(native_anchor_tags_combined(pw_alias_jk).t_out == SHK_PROGC),
      assert(native_anchor_tags_limbwise(pw_alias_jk) != native_anchor_tags_combined(pw_alias_jk)),
      assert(gen_r_check_ok(pw_alias_j1, adv_alias_j1) and not(gen_r_check_with_jk_range_ok(pw_alias_j1, adv_alias_j1))),
      assert(gen_r_check_with_jk_range_ok(pw_valid, adv_valid)),
      assert(native_anchor_tags_limbwise(pw_valid) == native_anchor_tags_combined(pw_valid))
    }
  }

  // --------------------------------------------------------------------------
  // Mutation Tests: each mutant is caught by a concrete witness
  // --------------------------------------------------------------------------

  // Mutant 1 (`mut_no_az`):
  // For chunk j = 2, k = 3 (mode = 1, j_lo = 2, j_hi = 0), a1 = j_lo - 1 = 1 != 0.
  // Dishonest prover sets z1a = { w: 0, z: 1 } so a1 * w = 0 = 1 - z holds, forging z_j1 = 1
  // and tag_in = SHK_PROGC (0) instead of SHK_LOCK (1).
  run mut1_no_az_caughtTest = {
    val pw_honest = honest_pub_words(1, 2, 0, 3, 0, 1, 2, 3, 1, 5, DIGEST, SHK_LOCK, 0, 0)
    // Forged statement has entry 4 under SHK_PROGC instead of SHK_LOCK:
    val out_in_forged = native_digest(SHK_PROGC, 1)
    val pw_forged = {
      ...pw_honest,
      entries: pw_honest.entries.replaceAt(4, { op: DIGEST, param: SHK_PROGC }),
      v_limbs: pw_honest.v_limbs.replaceAt(11, to_limbs(out_in_forged))
    }
    val adv_forged = { ...honest_advice(pw_forged), z1a: { w: 0, z: 1 } }

    all {
      assert(mut_no_az(pw_forged, adv_forged)),
      assert(not(native_r_rule_holds(pw_forged))),
      assert(not(gen_r_check_ok(pw_forged, adv_forged)))
    }
  }

  // Mutant 2 (`mut_mode_not_bit`):
  // Dishonest prover sets mode = 2, e_out = 6 (with j = 1, k = 1 so 1 - z_j1 = 0 and 1 - z_jk = 0,
  // keeping tag_in = 0 and tag_out = 0).
  run mut2_mode_not_bit_caughtTest = {
    val pw_base = honest_pub_words(1, 1, 0, 1, 0, 1, 2, 3, 1, 1, DIGEST, SHK_PROGC, 0, 0)
    val pw_forged = { ...pw_base, mode: 2, e_out: 6 }
    val adv = honest_advice(pw_forged)

    all {
      assert(mut_mode_not_bit(pw_forged, adv)),
      assert(not(native_r_rule_holds(pw_forged))),
      assert(not(gen_r_check_ok(pw_forged, adv)))
    }
  }

  // Mutant 3 (`mut_tag_out_unmasked`):
  // An honest ordinary call (mode = 0, j = 0, k = 0) with NOP at entry 5 is accepted by
  // gen_r_check_ok, but rejected by mut_tag_out_unmasked (which requires entry 5 to be DIGEST(1)).
  run mut3_tag_out_unmasked_caughtTest = {
    val pw_ord = honest_pub_words(0, 0, 0, 0, 0, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    val adv_ord = honest_advice(pw_ord)

    all {
      assert(gen_r_check_ok(pw_ord, adv_ord)),
      assert(native_r_rule_holds(pw_ord)),
      assert(not(mut_tag_out_unmasked(pw_ord, adv_ord)))
    }
  }

  // Mutant 4 (`mut_sponge_swapped`):
  // Witness where SPONGE(3) output (slot 9) is computed from swapped inputs [out1, out0]
  // is accepted by mut_sponge_swapped, violates native_r_rule_holds, and is rejected by gen_r_check_ok.
  run mut4_sponge_swapped_caughtTest = {
    val pw_honest = honest_pub_words(0, 0, 0, 0, 0, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    val adv = honest_advice(pw_honest)
    val y0 = val_at(pw_honest, 1)
    val y1 = val_at(pw_honest, 3)
    val y2 = val_at(pw_honest, 5)
    val swapped_claim = native_sponge_u0_3(y1, y0, y2)
    val pw_forged = { ...pw_honest, v_limbs: pw_honest.v_limbs.replaceAt(9, to_limbs(swapped_claim)) }

    all {
      assert(mut_sponge_swapped(pw_forged, adv)),
      assert(not(native_r_rule_holds(pw_forged))),
      assert(not(gen_r_check_ok(pw_forged, adv)))
    }
  }

  // Mutant 5 (`mut_off_shift`):
  // Witness where entry 4 output is placed at shifted slot 10 (`native_digest(tag_in, claim)`)
  // and slot 11 is corrupted is accepted by mut_off_shift, violates native_r_rule_holds, and
  // is rejected by gen_r_check_ok.
  run mut5_off_shift_caughtTest = {
    val pw_honest = honest_pub_words(0, 0, 0, 0, 0, 1, 2, 3, 1, 0, NOP, 0, 0, 0)
    val adv = honest_advice(pw_honest)
    val claim = val_at(pw_honest, 9)
    val shifted_y4 = native_digest(SHK_PROGC, claim)
    val v_shifted1 = pw_honest.v_limbs.replaceAt(10, to_limbs(shifted_y4))
    val v_shifted2 = v_shifted1.replaceAt(11, [0, 0])
    val pw_forged = { ...pw_honest, v_limbs: v_shifted2 }

    all {
      assert(mut_off_shift(pw_forged, adv)),
      assert(not(native_r_rule_holds(pw_forged))),
      assert(not(gen_r_check_ok(pw_forged, adv)))
    }
  }

  // Mutant 6 (`mut_no_cswap`):
  // In chunk mode (mode = 1, j = 1, k = 2), forging entry 5's digest output at value slot 13
  // is accepted by mut_no_cswap, violates native_r_rule_holds, and is rejected by gen_r_check_ok.
  run mut6_no_cswap_caughtTest = {
    val pw_chunk = honest_pub_words(1, 1, 0, 2, 0, 1, 2, 3, 1, 2, DIGEST, SHK_LOCK, 0, 0)
    val adv = honest_advice(pw_chunk)
    val forged_y5 = (val_at(pw_chunk, 13) + 1) % R_ORDER
    val pw_forged = { ...pw_chunk, v_limbs: pw_chunk.v_limbs.replaceAt(13, to_limbs(forged_y5)) }

    all {
      assert(mut_no_cswap(pw_forged, adv)),
      assert(not(native_r_rule_holds(pw_forged))),
      assert(not(gen_r_check_ok(pw_forged, adv)))
    }
  }

}
