// -*- mode: Bluespec; -*-
// E3 J8 Unit, Adversarial Generator & Mutant Tests for OnionAir (qa/gemini-quint/e3-onion/r2/onion2.qnt)
// All test names end with `Test` so `quint test` runs them by default without `--match`.

module onion2_test {

  import onion2.* from "./onion2"

  // ---- 1. Completeness Tests on Honest Traces ----------------------------------------------

  run completeEmptyTest = {
    all {
      assert(system_ok(honest_empty_sys)),
      assert(tie_is_sound(honest_empty_sys)),
      assert(honest_empty_sys.stmt_tie.n_used == 0),
      assert(list_eq(honest_empty_sys.stmt_tie.d, d_empty_native))
    }
  }

  run completePubTest = {
    all {
      assert(system_ok(honest_pub_sys)),
      assert(tie_is_sound(honest_pub_sys)),
      assert(honest_pub_sys.stmt_tie.n_used == 1)
    }
  }

  run completeDigestTest = {
    all {
      assert(system_ok(honest_digest_sys)),
      assert(tie_is_sound(honest_digest_sys)),
      assert(honest_digest_sys.stmt_tie.n_used == 1)
    }
  }

  run completeH2Test = {
    all {
      assert(system_ok(honest_h2_sys)),
      assert(tie_is_sound(honest_h2_sys)),
      assert(honest_h2_sys.stmt_tie.n_used == 1)
    }
  }

  run completeTwoEntriesTest = {
    all {
      assert(system_ok(honest_two_entries_sys)),
      assert(tie_is_sound(honest_two_entries_sys)),
      assert(honest_two_entries_sys.stmt_tie.n_used == 2)
    }
  }

  // ---- 2. Non-Honest Adversarial Generator Acceptance & Soundness Tests --------------------

  // Verifies that the non-honest raw trace generator (gen_adv_1entry / gen_adv_2entry) produces
  // accepted traces across all 5 log shapes (PUB, DIGEST, H2, PUB+PUB, PUB+DIGEST) and 3 limb seeds,
  // and that every accepted trace satisfies tie_is_sound.
  run advAcceptedSynthesizedTest = {
    val a0 = build_adv_from_choices(0, 0, OP_PUB, 0, 0, TAG_ENT, 1, 1, 1)
    val a1 = build_adv_from_choices(1, 1, OP_PUB, 0, 0, TAG_ENT, 1, 1, 1)
    val a2 = build_adv_from_choices(2, 2, OP_PUB, 0, 0, TAG_ENT, 1, 1, 1)
    val a3 = build_adv_from_choices(0, 3, OP_PUB, 0, 0, TAG_ENT, 1, 1, 1)
    val a4 = build_adv_from_choices(1, 4, OP_PUB, 0, 0, TAG_ENT, 1, 1, 1)
    all {
      assert(system_ok(a0) and tie_is_sound(a0)),
      assert(system_ok(a1) and tie_is_sound(a1)),
      assert(system_ok(a2) and tie_is_sound(a2)),
      assert(system_ok(a3) and tie_is_sound(a3)),
      assert(system_ok(a4) and tie_is_sound(a4))
    }
  }

  // ---- 3. Mutant Detection Tests (7 Round-1 Mutants + 5 Round-2 Mutants) -------------------
  // Each mutant test asserts three things:
  //   1. The mutant accepts the counterexample trace (mut_*(cex_*) == true),
  //   2. The counterexample's claimed tie is NOT the native onion (not(tie_is_sound(cex_*))),
  //   3. The real constraint system rejects the counterexample (not(system_ok(cex_*))).

  // Mutant 1 (R1): mut_no_off_chain
  run mutNoOffChainTest = {
    all {
      assert(mut_no_off_chain(cex_no_off_chain)),
      assert(not(tie_is_sound(cex_no_off_chain))),
      assert(not(system_ok(cex_no_off_chain)))
    }
  }

  // Mutant 2 (R1): mut_fa_reads_w0
  run mutFaReadsW0Test = {
    all {
      assert(mut_fa_reads_w0(cex_fa_reads_w0)),
      assert(not(tie_is_sound(cex_fa_reads_w0))),
      assert(not(system_ok(cex_fa_reads_w0))),
      // Also check that if FA did not skip j < 3 on VAL_BUS, honest traces would fail VAL_BUS:
      assert(not(val_bus_balanced_cfg(honest_pub_sys.values, honest_pub_sys.trace, false)))
    }
  }

  // Mutant 3 (R1): mut_no_last_count
  run mutNoLastCountTest = {
    all {
      assert(mut_no_last_count(cex_no_last_count)),
      assert(not(tie_is_sound(cex_no_last_count))),
      assert(not(system_ok(cex_no_last_count)))
    }
  }

  // Mutant 4 (R1): mut_fin_on_b1
  run mutFinOnB1Test = {
    all {
      assert(mut_fin_on_b1(cex_fin_on_b1)),
      assert(not(tie_is_sound(cex_fin_on_b1))),
      assert(not(system_ok(cex_fin_on_b1)))
    }
  }

  // Mutant 5 (R1): mut_cap_not_chained
  run mutCapNotChainedTest = {
    all {
      assert(mut_cap_not_chained(cex_cap_not_chained)),
      assert(not(tie_is_sound(cex_cap_not_chained))),
      assert(not(system_ok(cex_cap_not_chained)))
    }
  }

  // Mutant 6 (R1): mut_cnt_no_b2
  run mutCntNoB2Test = {
    all {
      assert(mut_cnt_no_b2(cex_cnt_no_b2)),
      assert(not(tie_is_sound(cex_cnt_no_b2))),
      assert(not(system_ok(cex_cnt_no_b2)))
    }
  }

  // Mutant 7 (R1): mut_d_not_chained
  run mutDNotChainedTest = {
    all {
      assert(mut_d_not_chained(cex_d_not_chained)),
      assert(not(tie_is_sound(cex_d_not_chained))),
      assert(not(system_ok(cex_d_not_chained)))
    }
  }

  // Mutant 8 (R2 new): mut_no_fa_pos_zero (drops fa * pos == 0 while keeping pos + c == total)
  run mutNoFaPosZeroTest = {
    all {
      assert(mut_no_fa_pos_zero(cex_no_fa_pos_zero)),
      assert(not(tie_is_sound(cex_no_fa_pos_zero))),
      assert(not(system_ok(cex_no_fa_pos_zero)))
    }
  }

  // Mutant 9 (R2 new): mut_no_same_off (drops same * (y.off - x.off) == 0)
  run mutNoSameOffTest = {
    all {
      assert(mut_no_same_off(cex_no_same_off)),
      assert(not(tie_is_sound(cex_no_same_off))),
      assert(not(system_ok(cex_no_same_off)))
    }
  }

  // Mutant 10 (R2 new): mut_no_same_k (drops same * (y.k_vals - x.k_vals) == 0)
  run mutNoSameKTest = {
    all {
      assert(mut_no_same_k(cex_no_same_k)),
      assert(not(tie_is_sound(cex_no_same_k))),
      assert(not(system_ok(cex_no_same_k)))
    }
  }

  // Mutant 11 (R2 new): mut_no_b2_off_step (drops b2 * y.fa * (y.off - x.off - x.k_vals) == 0 alone, keeping x.sel_e * y.off == 0)
  run mutNoB2OffStepTest = {
    all {
      assert(mut_no_b2_off_step(cex_no_b2_off_step)),
      assert(not(tie_is_sound(cex_no_b2_off_step))),
      assert(not(system_ok(cex_no_b2_off_step)))
    }
  }

  // Mutant 12 (R2 new): mut_early_fin (drops x.sel_fin - ax * (1 - ay) == 0 while keeping TIE_BUS balance)
  run mutEarlyFinTest = {
    all {
      assert(mut_early_fin(cex_early_fin)),
      assert(not(tie_is_sound(cex_early_fin))),
      assert(not(system_ok(cex_early_fin)))
    }
  }

  // Defense-in-depth check: on 2 entries, x[O_E] * y[O_OFF] == 0 and b2 * y[O_FA] * (y[O_OFF] - x[O_OFF] - x[O_K]) == 0
  // each independently reject the 2-entry swap cex_no_off_chain.
  run defenseFirstOffZeroTest = {
    all {
      assert(not(mut_no_first_off_zero_only(cex_no_off_chain))),
      assert(not(mut_no_b2_off_step(cex_no_off_chain))),
      assert(not(mut_no_first_off_zero_only(cex_no_b2_off_step)))
    }
  }

  // ---- 4. State Machine & Nondeterministic Adversarial Generator Tests ---------------------

  run stateMachineSoundAndCompleteTest = {
    init
      .then(all {
        assert(SOUND),
        assert(COMPLETE),
        step_adv
      })
      .then(all {
        assert(SOUND),
        assert(COMPLETE),
        step_tamper
      })
      .then(all {
        assert(SOUND),
        assert(COMPLETE),
        step_honest
      })
      .expect(SOUND and COMPLETE)
  }

  run advGeneratorSoundnessTest = {
    init
      .then(4.reps(_i => all {
        assert(SOUND),
        step_adv
      }))
      .expect(SOUND and state.adv_total == 4)
  }

}
