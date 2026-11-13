// -*- mode: Bluespec; -*-
//
// Unit and property tests for `decode.qnt` (Round 2):
// - Verifies injectivity of the uninterpreted hash table.
// - Verifies SOUND, COMPLETE, and REPLAY_DIFF_EXACT across all corpus cases.
// - Verifies that the only divergence between `replay_all == true` (stock) and
//   `replay_all == false` (wrap) is a replay failure of an entry outside R and the lane map.
// - Verifies that each of the 8 mutants is caught by a concrete counterexample witness
//   that violates SOUND (and/or COMPLETE) under the mutant while passing the real constraints.

module decode_test {
  import decode.* from "./decode"

  // 1. Uninterpreted hash table is strictly injective on (u_code, ins) -> out_v.
  run hash_table_injective_test = {
    val n = HASH_TABLE.length()
    all {
      assert(0.to(n - 1).forall(i =>
        0.to(n - 1).forall(j => {
          val ri = HASH_TABLE.nth(i)
          val rj = HASH_TABLE.nth(j)
          (i == j) or (
            not(ri.u_code == rj.u_code and ri.ins == rj.ins) and
            ri.out_v != rj.out_v
          )
        })
      )),
      assert(0.to(n - 1).forall(i => HASH_TABLE.nth(i).out_v != 300)),
    }
  }

  // 2. Honest ordinary call (mode = 0, j = 0, k = 0)
  run honest_ord_test = {
    val ref_wrap = ref_decode(PW_HONEST_ORD, false)
    val ref_stock = ref_decode(PW_HONEST_ORD, true)
    val hw = honest_witness(PW_HONEST_ORD)
    all {
      assert(ref_wrap.ok),
      assert(ref_stock.ok),
      assert(ref_wrap.stmt == [77, 21, 50, 23]),
      assert(ref_stock.stmt == ref_wrap.stmt),
      assert(constraints_ok(PW_HONEST_ORD, hw)),
      assert(circuit_output(PW_HONEST_ORD, hw) == ref_wrap.stmt),
    }
  }

  // 3. Honest chunk call (mode = 1, j = 2, k = 3 -> t_in = LOCK, t_out = LOCK)
  run honest_chunk_mid_test = {
    val ref_wrap = ref_decode(PW_HONEST_CHUNK_MID, false)
    val ref_stock = ref_decode(PW_HONEST_CHUNK_MID, true)
    val hw = honest_witness(PW_HONEST_CHUNK_MID)
    all {
      assert(ref_wrap.ok),
      assert(ref_stock.ok),
      assert(ref_wrap.stmt == [15, 31, 50, 9]),
      assert(ref_stock.stmt == ref_wrap.stmt),
      assert(constraints_ok(PW_HONEST_CHUNK_MID, hw)),
      assert(circuit_output(PW_HONEST_CHUNK_MID, hw) == ref_wrap.stmt),
    }
  }

  // 4. Honest single-chunk call (mode = 1, j = 1, k = 1 -> t_in = PROGC, t_out = PROGC)
  run honest_chunk_single_test = {
    val ref_wrap = ref_decode(PW_HONEST_CHUNK_SINGLE, false)
    val ref_stock = ref_decode(PW_HONEST_CHUNK_SINGLE, true)
    val hw = honest_witness(PW_HONEST_CHUNK_SINGLE)
    all {
      assert(ref_wrap.ok),
      assert(ref_stock.ok),
      assert(ref_wrap.stmt == [8, 21, 50, 45]),
      assert(ref_stock.stmt == ref_wrap.stmt),
      assert(constraints_ok(PW_HONEST_CHUNK_SINGLE, hw)),
      assert(circuit_output(PW_HONEST_CHUNK_SINGLE, hw) == ref_wrap.stmt),
    }
  }

  // 5. Divergence between `replay_all == true` (stock) and `replay_all == false` (wrap):
  // An unmapped entry outside R with a forged output is rejected by `replay_all == true`
  // and accepted by `replay_all == false` and by the circuit.
  run replay_all_divergence_test = {
    val ref_stock = ref_decode(PW_UNMAPPED_BAD_REPLAY, true)
    val ref_wrap = ref_decode(PW_UNMAPPED_BAD_REPLAY, false)
    val hw = honest_witness(PW_UNMAPPED_BAD_REPLAY)
    all {
      assert(not(ref_stock.ok)),
      assert(ref_wrap.ok),
      assert(ref_wrap.stmt == [77, 21, 50, 9]),
      assert(not(unmapped_outside_r_replay_ok(PW_UNMAPPED_BAD_REPLAY))),
      assert(is_replay_diff_exact_on(PW_UNMAPPED_BAD_REPLAY)),
      assert(constraints_ok(PW_UNMAPPED_BAD_REPLAY, hw)),
      assert(circuit_output(PW_UNMAPPED_BAD_REPLAY, hw) == ref_wrap.stmt),
    }
  }

  // 6. Global SOUND, COMPLETE, and REPLAY_DIFF_EXACT across the entire corpus.
  run corpus_sound_and_complete_test = {
    all {
      assert(0.to(CORPUS.length() - 1).forall(i => {
        val c = CORPUS.nth(i)
        is_sound_on(c.pw, c.wit) and is_complete_on(c.pw) and is_replay_diff_exact_on(c.pw)
      })),
    }
  }

  // 7. Mutant 1 (`mut_no_sponge3_param`): caught by PW_BAD_SPONGE3_PARAM / WIT_BAD_SPONGE3_PARAM
  run catch_mut_no_sponge3_param_test = {
    all {
      assert(not(ref_decode(PW_BAD_SPONGE3_PARAM, false).ok)),
      assert(not(ref_decode(PW_BAD_SPONGE3_PARAM, true).ok)),
      assert(not(constraints_ok(PW_BAD_SPONGE3_PARAM, WIT_BAD_SPONGE3_PARAM))),
      assert(mut_no_sponge3_param(PW_BAD_SPONGE3_PARAM, WIT_BAD_SPONGE3_PARAM)),
    }
  }

  // 8. Mutant 2 (`mut_idx_plus1`): caught on both SOUND (PW_BAD_IDX_PLUS1) and COMPLETE (PW_HONEST_ORD)
  run catch_mut_idx_plus1_test = {
    val hw_honest = honest_witness(PW_HONEST_ORD)
    all {
      assert(not(ref_decode(PW_BAD_IDX_PLUS1, false).ok)),
      assert(not(constraints_ok(PW_BAD_IDX_PLUS1, WIT_BAD_IDX_PLUS1))),
      assert(mut_idx_plus1(PW_BAD_IDX_PLUS1, WIT_BAD_IDX_PLUS1)),
      assert(ref_decode(PW_HONEST_ORD, false).ok),
      assert(constraints_ok(PW_HONEST_ORD, hw_honest)),
      assert(not(mut_idx_plus1(PW_HONEST_ORD, hw_honest))),
    }
  }

  // 9. Mutant 3 (`mut_no_twice`): caught by PW_BAD_TWICE / WIT_BAD_TWICE
  // (where the first mapped entry outputs 0, so `s[lane] == 0` alone passes!)
  run catch_mut_no_twice_test = {
    all {
      assert(not(ref_decode(PW_BAD_TWICE, false).ok)),
      assert(not(constraints_ok(PW_BAD_TWICE, WIT_BAD_TWICE))),
      assert(mut_no_twice(PW_BAD_TWICE, WIT_BAD_TWICE)),
    }
  }

  // 10. Mutant 4 (`mut_no_u16`): caught by PW_BAD_U16 / WIT_BAD_U16
  run catch_mut_no_u16_test = {
    all {
      assert(not(ref_decode(PW_BAD_U16, false).ok)),
      assert(not(constraints_ok(PW_BAD_U16, WIT_BAD_U16))),
      assert(mut_no_u16(PW_BAD_U16, WIT_BAD_U16)),
    }
  }

  // 11. Mutant 5 (`mut_no_mapped_replay`): caught by PW_BAD_MAPPED_REPLAY / WIT_BAD_MAPPED_REPLAY
  run catch_mut_no_mapped_replay_test = {
    all {
      assert(not(ref_decode(PW_BAD_MAPPED_REPLAY, false).ok)),
      assert(not(constraints_ok(PW_BAD_MAPPED_REPLAY, WIT_BAD_MAPPED_REPLAY))),
      assert(mut_no_mapped_replay(PW_BAD_MAPPED_REPLAY, WIT_BAD_MAPPED_REPLAY)),
    }
  }

  // 12. Mutant 6 (`mut_t_in_wrong`): caught on both SOUND (PW_BAD_T_IN) and COMPLETE (PW_HONEST_ORD)
  run catch_mut_t_in_wrong_test = {
    val hw_honest = honest_witness(PW_HONEST_ORD)
    all {
      assert(not(ref_decode(PW_BAD_T_IN, false).ok)),
      assert(not(constraints_ok(PW_BAD_T_IN, WIT_BAD_T_IN))),
      assert(mut_t_in_wrong(PW_BAD_T_IN, WIT_BAD_T_IN)),
      assert(ref_decode(PW_HONEST_ORD, false).ok),
      assert(constraints_ok(PW_HONEST_ORD, hw_honest)),
      assert(not(mut_t_in_wrong(PW_HONEST_ORD, hw_honest))),
    }
  }

  // 13. Mutant 7 (`mut_no_claim_pub`): caught by PW_BAD_CLAIM_PUB / WIT_BAD_CLAIM_PUB
  run catch_mut_no_claim_pub_test = {
    all {
      assert(not(ref_decode(PW_BAD_CLAIM_PUB, false).ok)),
      assert(not(constraints_ok(PW_BAD_CLAIM_PUB, WIT_BAD_CLAIM_PUB))),
      assert(mut_no_claim_pub(PW_BAD_CLAIM_PUB, WIT_BAD_CLAIM_PUB)),
    }
  }

  // 14. Mutant 8 (`mut_no_chunk_range`): caught by PW_BAD_CHUNK_RANGE / WIT_BAD_CHUNK_RANGE
  run catch_mut_no_chunk_range_test = {
    // Also test j = 0 in mode = 1 (chunk j = 0 < 1)
    val w_j0 = WORDS_BAD_CHUNK_RANGE.replaceAt(J_WORDS, 0)
    val pw_j0 = pack_words(PRE_HASH, w_j0)
    val hw_j0 = honest_witness(pw_j0)
    all {
      assert(not(ref_decode(PW_BAD_CHUNK_RANGE, false).ok)),
      assert(not(constraints_ok(PW_BAD_CHUNK_RANGE, WIT_BAD_CHUNK_RANGE))),
      assert(mut_no_chunk_range(PW_BAD_CHUNK_RANGE, WIT_BAD_CHUNK_RANGE)),
      assert(not(ref_decode(pw_j0, false).ok)),
      assert(not(constraints_ok(pw_j0, hw_j0))),
      assert(mut_no_chunk_range(pw_j0, hw_j0)),
    }
  }

}
