// -*- mode: Bluespec; -*-
//
// Unit and property tests for `decode.qnt` (Round 4, over repaired field FR_MOD = 2053):
// - Verifies injectivity of the uninterpreted 3-tuple permutation table `PERM_TABLE`.
// - Verifies SOUND and COMPLETE across honest calls covering PERM, H2, DIGEST,
//   SPONGE(1), SPONGE(2), SPONGE(3), SPONGE(4), PATH(1), PUB, and NOP.
// - Verifies that `mut_free_mult` is an equivalent mutant when `check_m_range` and all
//   other constraints are kept (via `slot_entry_m_range_unique` across the corpus and an
//   exhaustive pure check over all 64 `(i6, i7)` slot assignments on `PW_BAD_FREE_MULT`),
//   and that disabling `check_m_range` as well makes `WIT_BAD_FREE_MULT` pass.
// - Verifies that each of the other 6 mutants (`mut_no_m_range`, `mut_no_inactive_zero`,
//   `mut_no_has1`, `mut_no_chain`, `mut_first_cap_wrong`, `mut_ungated_y1`) is caught by
//   a concrete counterexample witness in the repaired field.

module decode_test {
  import decode.* from "./decode"

  // 1. Uninterpreted 3-tuple permutation table is strictly injective.
  run perm_table_injective_test = {
    val n = PERM_TABLE.length()
    all {
      assert(0.to(n - 1).forall(i =>
        0.to(n - 1).forall(j => {
          val ri = PERM_TABLE.nth(i)
          val rj = PERM_TABLE.nth(j)
          (i == j) or (
            not(ri.in0 == rj.in0 and ri.in1 == rj.in1 and ri.in2 == rj.in2) and
            not(ri.out0 == rj.out0 and ri.out1 == rj.out1 and ri.out2 == rj.out2) and
            ri.out0 != rj.out0
          )
        })
      )),
      assert(0.to(n - 1).forall(i => PERM_TABLE.nth(i).out0 != 300)),
    }
  }

  // 2. Honest ordinary call (mode = 0) with H2, PUB, NOP
  run honest_ord_test = {
    val ref_r = ref_decode(PW_HONEST_ORD)
    val hw = honest_witness(PW_HONEST_ORD)
    all {
      assert(ref_r.ok),
      assert(ref_r.stmt == [77, 21, 50, 23]),
      assert(constraints_ok(PW_HONEST_ORD, hw)),
      assert(circuit_output(PW_HONEST_ORD, hw) == ref_r.stmt),
      assert(slots_placed_exact(PW_HONEST_ORD, hw)),
    }
  }

  // 3. Honest PERM call (mode = 0, PERM([5, 6, 7]) -> [65, 66, 67])
  run honest_perm_test = {
    val ref_r = ref_decode(PW_HONEST_PERM)
    val hw = honest_witness(PW_HONEST_PERM)
    all {
      assert(ref_r.ok),
      assert(ref_r.stmt == [77, 21, 50, 9]),
      assert(constraints_ok(PW_HONEST_PERM, hw)),
      assert(circuit_output(PW_HONEST_PERM, hw) == ref_r.stmt),
      assert(slots_placed_exact(PW_HONEST_PERM, hw)),
    }
  }

  // 4. Honest SPONGE(1) and SPONGE(2) call
  run honest_sponge_1_2_test = {
    val ref_r = ref_decode(PW_HONEST_SPONGE_1_2)
    val hw = honest_witness(PW_HONEST_SPONGE_1_2)
    all {
      assert(ref_r.ok),
      assert(ref_r.stmt == [77, 21, 50, 9]),
      assert(constraints_ok(PW_HONEST_SPONGE_1_2, hw)),
      assert(circuit_output(PW_HONEST_SPONGE_1_2, hw) == ref_r.stmt),
      assert(slots_placed_exact(PW_HONEST_SPONGE_1_2, hw)),
    }
  }

  // 5. Honest SPONGE(4) call (multi-block even sponge, fills all 8 slots and 18 values)
  run honest_sponge_4_test = {
    val ref_r = ref_decode(PW_HONEST_SPONGE_4)
    val hw = honest_witness(PW_HONEST_SPONGE_4)
    all {
      assert(ref_r.ok),
      assert(ref_r.stmt == [77, 21, 50, 9]),
      assert(constraints_ok(PW_HONEST_SPONGE_4, hw)),
      assert(circuit_output(PW_HONEST_SPONGE_4, hw) == ref_r.stmt),
      assert(slots_placed_exact(PW_HONEST_SPONGE_4, hw)),
    }
  }

  // 6. Honest chunk call with PATH(1) (mode = 1, j = 2, k = 3)
  run honest_chunk_path_test = {
    val ref_r = ref_decode(PW_HONEST_CHUNK_PATH)
    val hw = honest_witness(PW_HONEST_CHUNK_PATH)
    all {
      assert(ref_r.ok),
      assert(ref_r.stmt == [15, 31, 50, 9]),
      assert(constraints_ok(PW_HONEST_CHUNK_PATH, hw)),
      assert(circuit_output(PW_HONEST_CHUNK_PATH, hw) == ref_r.stmt),
      assert(slots_placed_exact(PW_HONEST_CHUNK_PATH, hw)),
    }
  }

  // 7. Global SOUND and COMPLETE across the entire corpus.
  run corpus_sound_and_complete_test = {
    all {
      assert(0.to(CORPUS.length() - 1).forall(i => {
        val c = CORPUS.nth(i)
        is_sound_on(c.pw, c.wit) and is_complete_on(c.pw)
      })),
    }
  }

  // 8. Mutant 1 (`mut_free_mult`): equivalent mutant when `check_m_range` and all other
  // constraints are kept. Verifies:
  // - Round 3's `WIT_BAD_FREE_MULT` is refused by `mut_free_mult` in `FR_MOD = 2053` and only
  //   passes when `check_m_range` is also disabled (`free_mult: true, check_m_range: false`);
  // - Across all corpus entry tables, `check_m_range` uniquely pins each active slot `t < np`
  //   to the single entry `i` with `poff_i <= t < poff_i + perms_i`;
  // - Exhaustively across all 64 `(i6, i7) in (0..B_CAP-1)^2` slot assignments on `PW_BAD_FREE_MULT`,
  //   `mut_free_mult` refuses every attack witness.
  run catch_mut_free_mult_test = {
    all {
      assert(not(ref_decode(PW_BAD_FREE_MULT).ok)),
      assert(not(constraints_ok(PW_BAD_FREE_MULT, WIT_BAD_FREE_MULT))),
      assert(not(mut_free_mult(PW_BAD_FREE_MULT, WIT_BAD_FREE_MULT))),
      assert(constraints_with_flags(
        { ...DEFAULT_FLAGS, free_mult: true, check_m_range: false },
        PW_BAD_FREE_MULT,
        WIT_BAD_FREE_MULT
      )),
      assert(0.to(CORPUS.length() - 1).forall(k =>
        slot_entry_m_range_unique(CORPUS.nth(k).pw)
      )),
      assert(0.to(B_CAP - 1).forall(i6 =>
        0.to(B_CAP - 1).forall(i7 =>
          not(mut_free_mult(PW_BAD_FREE_MULT, free_mult_attack_wit(i6, i7)))
        )
      )),
    }
  }

  // 9. Mutant 2 (`mut_no_m_range`): caught by PW_BAD_NO_M_RANGE / WIT_BAD_NO_M_RANGE
  run catch_mut_no_m_range_test = {
    all {
      assert(not(ref_decode(PW_BAD_NO_M_RANGE).ok)),
      assert(not(constraints_ok(PW_BAD_NO_M_RANGE, WIT_BAD_NO_M_RANGE))),
      assert(mut_no_m_range(PW_BAD_NO_M_RANGE, WIT_BAD_NO_M_RANGE)),
    }
  }

  // 10. Mutant 3 (`mut_no_inactive_zero`): caught by PW_HONEST_ORD / WIT_BAD_INACTIVE_ZERO
  run catch_mut_no_inactive_zero_test = {
    all {
      assert(not(slots_placed_exact(PW_HONEST_ORD, WIT_BAD_INACTIVE_ZERO))),
      assert(not(constraints_ok(PW_HONEST_ORD, WIT_BAD_INACTIVE_ZERO))),
      assert(mut_no_inactive_zero(PW_HONEST_ORD, WIT_BAD_INACTIVE_ZERO)),
    }
  }

  // 11. Mutant 4 (`mut_no_has1`): caught on both SOUND (PW_BAD_NO_HAS1) and COMPLETE (PW_HONEST_ORD)
  run catch_mut_no_has1_test = {
    val hw_honest = honest_witness(PW_HONEST_ORD)
    all {
      assert(not(ref_decode(PW_BAD_NO_HAS1).ok)),
      assert(not(constraints_ok(PW_BAD_NO_HAS1, WIT_BAD_NO_HAS1))),
      assert(mut_no_has1(PW_BAD_NO_HAS1, WIT_BAD_NO_HAS1)),
      assert(ref_decode(PW_HONEST_ORD).ok),
      assert(constraints_ok(PW_HONEST_ORD, hw_honest)),
      assert(not(mut_no_has1(PW_HONEST_ORD, hw_honest))),
    }
  }

  // 12. Mutant 5 (`mut_no_chain`): caught on both SOUND (PW_BAD_NO_CHAIN) and COMPLETE (PW_HONEST_ORD)
  run catch_mut_no_chain_test = {
    val hw_honest = honest_witness(PW_HONEST_ORD)
    all {
      assert(not(ref_decode(PW_BAD_NO_CHAIN).ok)),
      assert(not(constraints_ok(PW_BAD_NO_CHAIN, WIT_BAD_NO_CHAIN))),
      assert(mut_no_chain(PW_BAD_NO_CHAIN, WIT_BAD_NO_CHAIN)),
      assert(ref_decode(PW_HONEST_ORD).ok),
      assert(constraints_ok(PW_HONEST_ORD, hw_honest)),
      assert(not(mut_no_chain(PW_HONEST_ORD, hw_honest))),
    }
  }

  // 13. Mutant 6 (`mut_first_cap_wrong`): caught on both SOUND (PW_BAD_FIRST_CAP) and COMPLETE (PW_HONEST_ORD)
  run catch_mut_first_cap_wrong_test = {
    val hw_honest = honest_witness(PW_HONEST_ORD)
    all {
      assert(not(ref_decode(PW_BAD_FIRST_CAP).ok)),
      assert(not(constraints_ok(PW_BAD_FIRST_CAP, WIT_BAD_FIRST_CAP))),
      assert(mut_first_cap_wrong(PW_BAD_FIRST_CAP, WIT_BAD_FIRST_CAP)),
      assert(ref_decode(PW_HONEST_ORD).ok),
      assert(constraints_ok(PW_HONEST_ORD, hw_honest)),
      assert(not(mut_first_cap_wrong(PW_HONEST_ORD, hw_honest))),
    }
  }

  // 14. Mutant 7 (`mut_ungated_y1`): caught by PW_BAD_UNGATED_Y1 / WIT_BAD_UNGATED_Y1
  run catch_mut_ungated_y1_test = {
    all {
      assert(not(ref_decode(PW_BAD_UNGATED_Y1).ok)),
      assert(not(constraints_ok(PW_BAD_UNGATED_Y1, WIT_BAD_UNGATED_Y1))),
      assert(mut_ungated_y1(PW_BAD_UNGATED_Y1, WIT_BAD_UNGATED_Y1)),
    }
  }

}
