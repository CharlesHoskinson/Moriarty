// Unit, Counterexample, and Exhaustive Tamper Tests — Part 1: Layout B (Pair Rows)
// Run: quint test --main pairrow_test qa/gemini-quint/chipleaf-pairrow-m3/pairrow_test.qnt

module pairrow_test {
  import pairrow(S = 3, MUTANT = "none").* from "./pairrow"

  // ---------------------------------------------------------------- Finite Single-Cell Tamper Domain
  // All single-cell tampers across both programs, both leaves, all slots, all registers/replicas/routed cells,
  // and values {0, 3, 7} (with downstream cascade).
  pure val EXH_VALS: Set[int] = Set(0, 3, 7)

  pure def allSingleCellTampers(mt: str, progId: int): Set[ChainW] = {
    val baseSet = Set(buildHonestChain(mt, progId))
    // Kind 1 (post[slotIdx][regIdx]) and Kind 2 (pre[slotIdx][regIdx]) across leaves, slots, regs, values
    val regTampers = LEAF_IDXS.fold(baseSet, (accL, lIdx) =>
      SLOT_IDXS.fold(accL, (accS, sIdx) =>
        REG_DOM.fold(accS, (accR, rIdx) =>
          EXH_VALS.fold(accR, (accV, v) =>
            accV
              .union(Set(applyTamper(mt, progId, 1, lIdx, sIdx, rIdx, v, true)))
              .union(Set(applyTamper(mt, progId, 1, lIdx, sIdx, rIdx, v, false)))
              .union(Set(applyTamper(mt, progId, 2, lIdx, sIdx, rIdx, v, true)))
          )
        )
      )
    )
    // Kind 3 (flag replica), Kind 4 (lane replica), Kind 5 (routed operand), Kind 6 (non-prefix enable)
    val cellTampers = LEAF_IDXS.fold(regTampers, (accL, lIdx) =>
      SLOT_IDXS.fold(accL, (accS, sIdx) =>
        EXH_VALS.fold(accS, (accV, v) =>
          accV
            .union(Set(applyTamper(mt, progId, 3, lIdx, sIdx, 0, v, true)))
            .union(Set(applyTamper(mt, progId, 4, lIdx, sIdx, 0, v, true)))
            .union(Set(applyTamper(mt, progId, 5, lIdx, sIdx, 0, v, true)))
            .union(Set(applyTamper(mt, progId, 6, lIdx, sIdx, 0, v, true)))
        )
      )
    )
    cellTampers
  }

  pure def exhaustiveSoundForProg(progId: int): bool =
    allSingleCellTampers("none", progId).fold(true, (ok, cand) =>
      ok and (acceptsWithMutant("none", cand) implies refines(cand))
    )

  pure def exhaustiveMutantCaught(mt: str): bool =
    PROG_IDS.fold(false, (found, progId) =>
      found or allSingleCellTampers(mt, progId).fold(false, (f, cand) =>
        f or (acceptsWithMutant(mt, cand) and not(refinesWith(mt, cand)))
      )
    )

  // ---------------------------------------------------------------- 1. Honest Witness Completeness Tests
  run honestFullTwoLeafTest = all {
    assert(acceptsWithMutant("none", hw_full_two_leaf)),
    assert(refines(hw_full_two_leaf)),
    assert(hw_full_two_leaf.leaf1.steps == 3),
    assert(hw_full_two_leaf.leaf2.steps == 3),
    assert(chainExecutedRules(hw_full_two_leaf) == [RULE_PUSH, RULE_REWRITE, RULE_ARITH, RULE_POP, RULE_PUSH, RULE_REWRITE]),
  }

  run honestDisabledSlotTwoLeafTest = all {
    assert(acceptsWithMutant("none", hw_disabled_slot_two_leaf)),
    assert(refines(hw_disabled_slot_two_leaf)),
    assert(hw_disabled_slot_two_leaf.leaf1.steps == 2),
    assert(hw_disabled_slot_two_leaf.leaf2.steps == 3),
    assert(chainExecutedRules(hw_disabled_slot_two_leaf) == [RULE_PUSH, RULE_REWRITE, RULE_ARITH, RULE_POP, RULE_PUSH]),
  }

  // ---------------------------------------------------------------- 2. Required Mutant Counterexample Tests (P1..P9)
  run mutantP1_frameGateDroppedTest = all {
    assert(acceptsWithMutant("frame_gate_dropped", cex_P1_frame_gate_dropped)),
    assert(not(acceptsWithMutant("none", cex_P1_frame_gate_dropped))),
    assert(not(refines(cex_P1_frame_gate_dropped))),
  }

  run mutantP2_frameGateEnabledOnlyTest = all {
    assert(acceptsWithMutant("frame_gate_enabled_only", cex_P2_frame_gate_enabled_only)),
    assert(not(acceptsWithMutant("none", cex_P2_frame_gate_enabled_only))),
    assert(not(refines(cex_P2_frame_gate_enabled_only))),
  }

  run mutantP3_chainLinkDroppedTest = all {
    assert(acceptsWithMutant("chain_link_dropped", cex_P3_chain_link_dropped)),
    assert(not(acceptsWithMutant("none", cex_P3_chain_link_dropped))),
    assert(not(refines(cex_P3_chain_link_dropped))),
  }

  run mutantP4_flagReplicaUnlinkedTest = all {
    assert(acceptsWithMutant("flag_replica_unlinked", cex_P4_flag_replica_unlinked)),
    assert(not(acceptsWithMutant("none", cex_P4_flag_replica_unlinked))),
    assert(not(refines(cex_P4_flag_replica_unlinked))),
  }

  run mutantP5_laneReplicaUnlinkedTest = all {
    assert(acceptsWithMutant("lane_replica_unlinked", cex_P5_lane_replica_unlinked)),
    assert(not(acceptsWithMutant("none", cex_P5_lane_replica_unlinked))),
    assert(not(refines(cex_P5_lane_replica_unlinked))),
  }

  run mutantP6_routedOperandUnlinkedTest = all {
    assert(acceptsWithMutant("routed_operand_unlinked", cex_P6_routed_operand_unlinked)),
    assert(not(acceptsWithMutant("none", cex_P6_routed_operand_unlinked))),
    assert(not(refines(cex_P6_routed_operand_unlinked))),
  }

  run mutantP7_postDWrongSlotTest = all {
    assert(acceptsWithMutant("post_d_wrong_slot", cex_P7_post_d_wrong_slot)),
    assert(not(acceptsWithMutant("none", cex_P7_post_d_wrong_slot))),
    assert(not(refines(cex_P7_post_d_wrong_slot))),
  }

  run mutantP8_digestOmitsRegisterTest = all {
    assert(acceptsWithMutant("digest_omits_register", cex_P8_digest_omits_register)),
    assert(not(acceptsWithMutant("none", cex_P8_digest_omits_register))),
    assert(not(refines(cex_P8_digest_omits_register))),
  }

  run mutantP9_scratchReadTest = all {
    assert(acceptsWithMutant("scratch_read", cex_P9_scratch_read)),
    assert(not(acceptsWithMutant("none", cex_P9_scratch_read))),
    assert(not(refinesWith("scratch_read", cex_P9_scratch_read))),
    // Lead repair: the honest chain under the scratch_read design is accepted and refines, so the
    // violation comes from the forged leaf-2 V0, not from a reference mismatch.
    assert(acceptsWithMutant("scratch_read", buildHonestChain("scratch_read", 0))),
    assert(refinesWith("scratch_read", buildHonestChain("scratch_read", 0))),
  }

  // Lead addition: for every mutant except post_d_wrong_slot (whose only accepted chain is the lossy
  // one), the honest chain built under the mutant is accepted and refines. So each caught mutant is
  // caught by a forged witness, not by an honest run that already diverges.
  run honestUnderMutantsRefinesTest = all {
    Set("frame_gate_dropped", "frame_gate_enabled_only", "chain_link_dropped", "flag_replica_unlinked",
        "lane_replica_unlinked", "routed_operand_unlinked", "digest_omits_register", "scratch_read",
        "enable_not_prefix").forall(mt => PROG_IDS.forall(pid => and {
      acceptsWithMutant(mt, buildHonestChain(mt, pid)),
      refinesWith(mt, buildHonestChain(mt, pid))
    }))
  }

  // Enable prefix dropped: accepted by `enable_not_prefix`, rejected by `none`,
  // and STILL satisfies register refinement `refines` because disabled slots are forced to be identity!
  run enableNotPrefixAnalysisTest = all {
    assert(acceptsWithMutant("enable_not_prefix", w_enable_not_prefix)),
    assert(not(acceptsWithMutant("none", w_enable_not_prefix))),
    assert(refines(w_enable_not_prefix)),
  }

  // ---------------------------------------------------------------- 3. Exhaustive Pure Tamper Domain Checks
  run exhaustiveSoundProg0Test = assert(exhaustiveSoundForProg(0))
  run exhaustiveSoundProg1Test = assert(exhaustiveSoundForProg(1))

  run exhaustiveMutantsCaughtTest = all {
    assert(exhaustiveMutantCaught("frame_gate_dropped")),
    assert(exhaustiveMutantCaught("frame_gate_enabled_only")),
    assert(exhaustiveMutantCaught("chain_link_dropped")),
    assert(exhaustiveMutantCaught("flag_replica_unlinked")),
    assert(exhaustiveMutantCaught("lane_replica_unlinked")),
    assert(exhaustiveMutantCaught("routed_operand_unlinked")),
    assert(exhaustiveMutantCaught("post_d_wrong_slot")),
    assert(exhaustiveMutantCaught("digest_omits_register")),
    assert(exhaustiveMutantCaught("scratch_read")),
  }
}
