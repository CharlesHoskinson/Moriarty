// -*- mode: Bluespec; -*-
//
// Unit and property tests for `decode.qnt`:
// - Verifies SOUND and COMPLETE across all corpus cases and additional negative cases.
// - Verifies that each of the 8 mutants is caught by a concrete counterexample witness
//   that violates SOUND (or COMPLETE) under the mutant while passing the real constraints.

module decode_test {
  import decode.* from "./decode"

  // 1. Honest cases: reference decode succeeds, honest witness satisfies constraints,
  // and circuit output matches reference statement.
  run honest_mixed_test = {
    val ref_r = ref_decode(PW_HONEST_MIXED)
    val hw = honest_witness(PW_HONEST_MIXED)
    all {
      assert(ref_r.ok),
      assert(ref_r.stmt == [25, 50, 99]),
      assert(constraints_ok(PW_HONEST_MIXED, hw)),
      assert(circuit_output(PW_HONEST_MIXED, hw) == ref_r.stmt),
    }
  }

  run honest_sponge_test = {
    val ref_r = ref_decode(PW_HONEST_SPONGE)
    val hw = honest_witness(PW_HONEST_SPONGE)
    all {
      assert(ref_r.ok),
      assert(ref_r.stmt == [5, 77, 12]),
      assert(constraints_ok(PW_HONEST_SPONGE, hw)),
      assert(circuit_output(PW_HONEST_SPONGE, hw) == ref_r.stmt),
    }
  }

  run honest_padded_test = {
    val ref_r = ref_decode(PW_HONEST_PADDED)
    val hw = honest_witness(PW_HONEST_PADDED)
    all {
      assert(ref_r.ok),
      assert(ref_r.stmt == [40, 120, 80]),
      assert(constraints_ok(PW_HONEST_PADDED, hw)),
      assert(circuit_output(PW_HONEST_PADDED, hw) == ref_r.stmt),
    }
  }

  // 2. Global SOUND and COMPLETE across the entire corpus on the real constraints.
  run corpus_sound_and_complete_test = {
    all {
      assert(0.to(CORPUS.length() - 1).forall(i => {
        val c = CORPUS.nth(i)
        is_sound_on(c.pw, c.wit) and is_complete_on(c.pw)
      })),
    }
  }

  // 3. Unpack and pre-hash negative tests.
  run unpack_refusals_test = {
    // Bad pre-hash pw[0]
    val pw_bad_pre = [PRE_HASH + 1].concat(PW_HONEST_MIXED.slice(1, N_PUBLIC))
    val hw_bad_pre = honest_witness(pw_bad_pre)

    // Non-canonical BabyBear word (>= BB_P = 13) in header word 0
    val w_bad_bb = [BB_P, 0].concat(WORDS_HONEST_MIXED.slice(2, N_WORDS))
    val pw_bad_bb = pack_words(PRE_HASH, w_bad_bb)
    val hw_bad_bb = honest_witness(pw_bad_bb)

    // Limb word >= LIMB_BASE (8) but < BB_P (13) at index VALUES_AT
    val w_bad_limb = WORDS_HONEST_MIXED.slice(0, VALUES_AT)
      .append(LIMB_BASE)
      .concat(WORDS_HONEST_MIXED.slice(VALUES_AT + 1, N_WORDS))
    val pw_bad_limb = pack_words(PRE_HASH, w_bad_limb)
    val hw_bad_limb = honest_witness(pw_bad_limb)

    all {
      assert(not(ref_decode(pw_bad_pre).ok)),
      assert(not(constraints_ok(pw_bad_pre, hw_bad_pre))),
      assert(not(ref_decode(pw_bad_bb).ok)),
      assert(not(constraints_ok(pw_bad_bb, hw_bad_bb))),
      assert(not(ref_decode(pw_bad_limb).ok)),
      assert(not(constraints_ok(pw_bad_limb, hw_bad_limb))),
      assert(not(ref_decode(PW_BAD_HIGH_BITS).ok)),
      assert(not(constraints_ok(PW_BAD_HIGH_BITS, WIT_BAD_HIGH_BITS))),
    }
  }

  // 4. Mutant 1 (`mut_no_canon_last_borrow`): caught by PW_BAD_CANON / WIT_BAD_CANON
  run catch_mut_no_canon_last_borrow_test = {
    all {
      assert(not(ref_decode(PW_BAD_CANON).ok)),
      assert(not(constraints_ok(PW_BAD_CANON, WIT_BAD_CANON))),
      assert(mut_no_canon_last_borrow(PW_BAD_CANON, WIT_BAD_CANON)),
    }
  }

  // 5. Mutant 2 (`mut_lo_only`): caught by PW_BAD_HI_BOUND / WIT_BAD_HI_BOUND
  run catch_mut_lo_only_test = {
    all {
      assert(not(ref_decode(PW_BAD_HI_BOUND).ok)),
      assert(not(constraints_ok(PW_BAD_HI_BOUND, WIT_BAD_HI_BOUND))),
      assert(mut_lo_only(PW_BAD_HI_BOUND, WIT_BAD_HI_BOUND)),
    }
  }

  // 6. Mutant 3 (`mut_nop_chain_off`): caught by PW_BAD_NOP_CHAIN / WIT_BAD_NOP_CHAIN
  run catch_mut_nop_chain_off_test = {
    all {
      assert(not(ref_decode(PW_BAD_NOP_CHAIN).ok)),
      assert(not(constraints_ok(PW_BAD_NOP_CHAIN, WIT_BAD_NOP_CHAIN))),
      assert(mut_nop_chain_off(PW_BAD_NOP_CHAIN, WIT_BAD_NOP_CHAIN)),
    }
  }

  // 7. Mutant 4 (`mut_g_not_monotone`): caught by PW_BAD_ZERO_PAD / WIT_BAD_ZERO_PAD
  run catch_mut_g_not_monotone_test = {
    all {
      assert(not(ref_decode(PW_BAD_ZERO_PAD).ok)),
      assert(not(constraints_ok(PW_BAD_ZERO_PAD, WIT_BAD_ZERO_PAD))),
      assert(mut_g_not_monotone(PW_BAD_ZERO_PAD, WIT_BAD_ZERO_PAD)),
    }
  }

  // 8. Mutant 5 (`mut_skip_any_pair`): caught by PW_BAD_SKIP_PAIR / WIT_BAD_SKIP_PAIR
  run catch_mut_skip_any_pair_test = {
    all {
      assert(not(ref_decode(PW_BAD_SKIP_PAIR).ok)),
      assert(not(constraints_ok(PW_BAD_SKIP_PAIR, WIT_BAD_SKIP_PAIR))),
      // E5 lead: the lane's witness is refused by the mutant too (the gated one-hot).
      assert(not(mut_skip_any_pair(PW_BAD_SKIP_PAIR, WIT_BAD_SKIP_PAIR))),
      // The separating witness: nz = 2 writes one value into two lanes.
      assert(ref_decode(PW_BAD_SKIP2).ok),
      assert(ref_decode(PW_BAD_SKIP2).stmt == [0, 0, 9]),
      assert(not(constraints_ok(PW_BAD_SKIP2, WIT_BAD_SKIP2))),
      assert(mut_skip_any_pair(PW_BAD_SKIP2, WIT_BAD_SKIP2)),
      assert(circuit_output(PW_BAD_SKIP2, WIT_BAD_SKIP2) == [9, 0, 9]),
      assert(circuit_output(PW_BAD_SKIP2, WIT_BAD_SKIP2) != ref_decode(PW_BAD_SKIP2).stmt),
    }
  }

  // 9. Mutant 6 (`mut_no_taken_check`): caught by PW_BAD_TAKEN / WIT_BAD_TAKEN
  run catch_mut_no_taken_check_test = {
    all {
      assert(not(ref_decode(PW_BAD_TAKEN).ok)),
      assert(not(constraints_ok(PW_BAD_TAKEN, WIT_BAD_TAKEN))),
      assert(mut_no_taken_check(PW_BAD_TAKEN, WIT_BAD_TAKEN)),
    }
  }

  // 10. Mutant 7 (`mut_s_before_lanemap`): caught on both SOUND (accepts PW_BAD_S_BEFORE_LM)
  // and COMPLETE (rejects honest PW_HONEST_MIXED where PUB(2) reads lane 2 after lane map).
  run catch_mut_s_before_lanemap_test = {
    val hw_honest = honest_witness(PW_HONEST_MIXED)
    all {
      assert(not(ref_decode(PW_BAD_S_BEFORE_LM).ok)),
      assert(not(constraints_ok(PW_BAD_S_BEFORE_LM, WIT_BAD_S_BEFORE_LM))),
      assert(mut_s_before_lanemap(PW_BAD_S_BEFORE_LM, WIT_BAD_S_BEFORE_LM)),
      assert(ref_decode(PW_HONEST_MIXED).ok),
      assert(constraints_ok(PW_HONEST_MIXED, hw_honest)),
      assert(not(mut_s_before_lanemap(PW_HONEST_MIXED, hw_honest))),
    }
  }

  // 11. Mutant 8 (`mut_v_s_keys_overlap`): caught by PW_BAD_KEY_OVERLAP / WIT_BAD_KEY_OVERLAP
  run catch_mut_v_s_keys_overlap_test = {
    all {
      assert(not(ref_decode(PW_BAD_KEY_OVERLAP).ok)),
      assert(not(constraints_ok(PW_BAD_KEY_OVERLAP, WIT_BAD_KEY_OVERLAP))),
      assert(mut_v_s_keys_overlap(PW_BAD_KEY_OVERLAP, WIT_BAD_KEY_OVERLAP)),
    }
  }

}
