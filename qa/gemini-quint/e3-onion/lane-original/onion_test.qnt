// -*- mode: Bluespec; -*-
// E3 J8 Unit & Mutant Tests for OnionAir (qa/gemini-quint/e3-onion/onion.qnt)

module onion_test {

  import onion.* from "./onion"

  // ---- Completeness Tests on Honest Traces --------------------------------------------------

  run test_complete_empty = {
    all {
      assert(system_ok(honest_empty_sys)),
      assert(tie_is_sound(honest_empty_sys)),
      assert(honest_empty_sys.stmt_tie.n_used == 0),
      assert(list_eq(honest_empty_sys.stmt_tie.d, d_empty_native))
    }
  }

  run test_complete_pub = {
    all {
      assert(system_ok(honest_pub_sys)),
      assert(tie_is_sound(honest_pub_sys)),
      assert(honest_pub_sys.stmt_tie.n_used == 1)
    }
  }

  run test_complete_digest = {
    all {
      assert(system_ok(honest_digest_sys)),
      assert(tie_is_sound(honest_digest_sys)),
      assert(honest_digest_sys.stmt_tie.n_used == 1)
    }
  }

  run test_complete_h2 = {
    all {
      assert(system_ok(honest_h2_sys)),
      assert(tie_is_sound(honest_h2_sys)),
      assert(honest_h2_sys.stmt_tie.n_used == 1)
    }
  }

  run test_complete_two_entries = {
    all {
      assert(system_ok(honest_two_entries_sys)),
      assert(tie_is_sound(honest_two_entries_sys)),
      assert(honest_two_entries_sys.stmt_tie.n_used == 2)
    }
  }

  // ---- Mutant Detection Tests (each mutant passes cex, fails SOUND, and is caught by real) --

  // Mutant 1: mut_no_off_chain
  run test_mut_no_off_chain = {
    all {
      assert(mut_no_off_chain(cex_no_off_chain)),
      assert(not(tie_is_sound(cex_no_off_chain))),
      assert(not(system_ok(cex_no_off_chain)))
    }
  }

  // Mutant 2: mut_fa_reads_w0
  run test_mut_fa_reads_w0 = {
    all {
      assert(mut_fa_reads_w0(cex_fa_reads_w0)),
      assert(not(tie_is_sound(cex_fa_reads_w0))),
      assert(not(system_ok(cex_fa_reads_w0))),
      // Also check that if FA did not skip j < 3 on VAL_BUS, honest traces would fail VAL_BUS:
      assert(not(val_bus_balanced_cfg(honest_pub_sys.values, honest_pub_sys.trace, false)))
    }
  }

  // Mutant 3: mut_no_last_count
  run test_mut_no_last_count = {
    all {
      assert(mut_no_last_count(cex_no_last_count)),
      assert(not(tie_is_sound(cex_no_last_count))),
      assert(not(system_ok(cex_no_last_count)))
    }
  }

  // Mutant 4: mut_fin_on_b1
  run test_mut_fin_on_b1 = {
    all {
      assert(mut_fin_on_b1(cex_fin_on_b1)),
      assert(not(tie_is_sound(cex_fin_on_b1))),
      assert(not(system_ok(cex_fin_on_b1)))
    }
  }

  // Mutant 5: mut_cap_not_chained
  run test_mut_cap_not_chained = {
    all {
      assert(mut_cap_not_chained(cex_cap_not_chained)),
      assert(not(tie_is_sound(cex_cap_not_chained))),
      assert(not(system_ok(cex_cap_not_chained)))
    }
  }

  // Mutant 6: mut_cnt_no_b2
  run test_mut_cnt_no_b2 = {
    all {
      assert(mut_cnt_no_b2(cex_cnt_no_b2)),
      assert(not(tie_is_sound(cex_cnt_no_b2))),
      assert(not(system_ok(cex_cnt_no_b2)))
    }
  }

  // Mutant 7: mut_d_not_chained
  run test_mut_d_not_chained = {
    all {
      assert(mut_d_not_chained(cex_d_not_chained)),
      assert(not(tie_is_sound(cex_d_not_chained))),
      assert(not(system_ok(cex_d_not_chained)))
    }
  }

  // State machine trace test: step through all states and assert SOUND_AND_COMPLETE
  run test_state_machine_sound_and_complete = {
    init.then(all {
      assert(SOUND),
      assert(COMPLETE),
      step
    }).then(all {
      assert(SOUND),
      assert(COMPLETE)
    })
  }

}
