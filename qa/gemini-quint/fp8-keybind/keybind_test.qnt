// -*- mode: quint; -*-
// Deterministic & Mutation Tests for FP-8 KeyBind Specification

module keybind_test {
  import keybind.* from "./keybind"

  // 1. Completeness: Honest chain accepted at N3 depth 1 (app -> leaf -> N1 -> N2 -> N3(d1) -> L)
  run honestDepth1Test =
    init
      .then(stepHonest(KEY_A, 1))
      .expect(all {
        treeAccepted(currentTree, MutNone),
        bindHoldsOnTree(currentTree),
        slotHygieneOnTree(currentTree),
        selectorConsistencyOnTree(currentTree),
        COMPLETE,
        BIND,
        SOUND_ALL,
      })

  // 2. Completeness: Honest chain accepted at N3 depth 2 (L over depth-2 N3 child)
  run honestDepth2Test =
    init
      .then(stepHonest(KEY_A, 2))
      .expect(all {
        treeAccepted(currentTree, MutNone),
        bindHoldsOnTree(currentTree),
        slotHygieneOnTree(currentTree),
        selectorConsistencyOnTree(currentTree),
        COMPLETE,
        BIND,
        SOUND_ALL,
      })

  // 3. Completeness: Honest chain accepted at N3 depth 3 (app -> leaf -> N1 -> N2 -> N3(d1) -> N3(d2) -> N3(d3) -> L)
  run honestDepth3Test =
    init
      .then(stepHonest(KEY_B, 3))
      .expect(all {
        treeAccepted(currentTree, MutNone),
        bindHoldsOnTree(currentTree),
        slotHygieneOnTree(currentTree),
        selectorConsistencyOnTree(currentTree),
        COMPLETE,
        BIND,
        SOUND_ALL,
      })

  // 4. Question 6: Leaf role fails closed if an app proof has public values or cached traces
  run leafFailsClosedOnAppPvsOrCachedTest = {
    val lfPvs = honestNodePvs(RoleLeaf, ZERO_PVS, HONEST_APP_A)
    val sHonest = appBusSends(KEY_A, false, false)
    val sWithPvs = appBusSends(KEY_A, true, false)
    val sWithCc = appBusSends(KEY_A, false, true)
    val w = honestRowWitness(RoleLeaf, sHonest)
    val rPv = honestRootPvs(lfPvs)
    init.expect(all {
      evalRow(RoleLeaf, sHonest, w, lfPvs, rPv, MutNone),
      not(evalRow(RoleLeaf, sWithPvs, w, lfPvs, rPv, MutNone)),
      not(evalRow(RoleLeaf, sWithCc, w, lfPvs, rPv, MutNone)),
    })
  }

  // 5. Soundness: Two different app keys (KEY_A and KEY_B) in one tree are refused
  run mixedAppKeysRefusedTest =
    init
      .then(stepMixedAppKeys(2))
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        not(bindHoldsOnTree(currentTree)),
        BIND,
      })

  // 6. Soundness: Mixed child depths ([depth 1, depth 2]) under N3 are refused
  run mixedDepthsRefusedTest =
    init
      .then(stepMixedDepthsN3)
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        not(bindHoldsOnTree(currentTree)),
        BIND,
      })

  // 7. Soundness: Exposed app commit differing from proven app key is refused
  run tamperedExposedAppRefusedTest =
    init
      .then(stepTamperExposedApp(KEY_A, 2))
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        not(bindHoldsOnTree(currentTree)),
        BIND,
      })

  // 8. Mutant 1: mut_no_unset_check (drop parent's unset checks of slots above child)
  run mutantNoUnsetCheckTest =
    init
      .then(stepCexMutant(1))
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        mut_no_unset_check(currentTree),
        not(slotHygieneOnTree(currentTree)),
        SOUND_ALL,
        not(inv_mut_no_unset_check),
      })

  // 9. Mutant 2: mut_no_cc_receive (drop cached-commitment receive)
  run mutantNoCcReceiveTest =
    init
      .then(stepCexMutant(2))
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        mut_no_cc_receive(currentTree),
        not(bindHoldsOnTree(currentTree)),
        BIND,
        not(inv_mut_no_cc_receive),
      })

  // 10. Mutant 3: mut_root_no_n2_pin (drop Root's N2 pin)
  run mutantRootNoN2PinTest =
    init
      .then(stepCexMutant(3))
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        mut_root_no_n2_pin(currentTree),
        not(bindHoldsOnTree(currentTree)),
        BIND,
        not(inv_mut_root_no_n2_pin),
      })

  // 11. Mutant 4: mut_root_no_prehash_pin (Root receives pre-hash into a free column)
  run mutantRootNoPrehashPinTest =
    init
      .then(stepCexMutant(4))
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        mut_root_no_prehash_pin(currentTree),
        not(bindHoldsOnTree(currentTree)),
        BIND,
        not(inv_mut_root_no_prehash_pin),
      })

  // 12. Mutant 5: mut_n3_no_copy_n2 (N3 does not copy n2 from child when s31/s3x)
  run mutantN3NoCopyN2Test =
    init
      .then(stepCexMutant(5))
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        mut_n3_no_copy_n2(currentTree),
        not(bindHoldsOnTree(currentTree)),
        BIND,
        not(inv_mut_n3_no_copy_n2),
      })

  // 13. Mutant 6: mut_selector_free (drop depth tests of s31/s3x)
  run mutantSelectorFreeTest =
    init
      .then(stepCexMutant(6))
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        mut_selector_free(currentTree),
        not(selectorConsistencyOnTree(currentTree)),
        SOUND_ALL,
        not(inv_mut_selector_free),
      })

  // 14. Mutant 7: mut_lfr_no_pin (L's own child DAG not pinned to K3)
  run mutantLfrNoPinTest =
    init
      .then(stepCexMutant(7))
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        mut_lfr_no_pin(currentTree),
        not(bindHoldsOnTree(currentTree)),
        BIND,
        not(inv_mut_lfr_no_pin),
      })

  // 15. Mutant 8: mut_n3_no_copy_n3 (N3 does not copy n3 from child when s3x)
  run mutantN3NoCopyN3Test =
    init
      .then(stepCexMutant(8))
      .expect(all {
        not(treeAccepted(currentTree, MutNone)),
        mut_n3_no_copy_n3(currentTree),
        not(bindHoldsOnTree(currentTree)),
        BIND,
        not(inv_mut_n3_no_copy_n3),
      })

}
