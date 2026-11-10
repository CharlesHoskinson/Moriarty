// Unit and Property Tests for the "Sherlock leaf as a chip" Soundness Model
// Run: quint test qa/gemini-quint/chipleaf/chipleaf_test.qnt

module chipleaf_test {
  import chipleaf(S = 3, MUTANT = "none").* from "./chipleaf"

  // Pre-built honest witnesses across the abstract ISA
  pure val hw_append_load: Witness = mkHonestWitness(0, 10, 3)
  pure val hw_push_pop: Witness = mkHonestWitness(3, 10, 3)
  pure val hw_rewrite_top: Witness = mkHonestWitness(5, 10, 3)
  pure val hw_fetch_alu: Witness = mkHonestWitness(6, 10, 3)

  // ---------------------------------------------------------------- 1. Honest Execution Tests
  // Honest run 1: APPEND_MOVE -> LOAD_ALU -> PUBLISH (3 steps, full leaf)
  run honestAppendLoadPublishTest = all {
    assert(acceptsAsWritten(hw_append_load)),
    assert(acceptsHardened(hw_append_load)),
    assert(isSoundWitness(hw_append_load)),
    assert(hw_append_load.steps == 3),
    assert(hw_append_load.halted),
    assert(hw_append_load.post.acc == 21),
    assert(hw_append_load.post.len == 3),
  }

  // Honest run 2: PUSH -> POP -> PUBLISH (linked frame stack round-trip)
  run honestPushPopPublishTest = all {
    assert(acceptsAsWritten(hw_push_pop)),
    assert(acceptsHardened(hw_push_pop)),
    assert(isSoundWitness(hw_push_pop)),
    assert(hw_push_pop.steps == 3),
    assert(hw_push_pop.halted),
    assert(hw_push_pop.post.fp == 0),
    assert(hw_push_pop.post.topVal == 10),
    assert(hw_push_pop.post.acc == 20),
    assert(hw_push_pop.post.len == 3),
  }

  // Honest run 3: REWRITE_TOP -> POP -> PUBLISH (rewriting cached top then popping caller frame 100 at addr 0)
  run honestRewriteTopPopTest = all {
    assert(acceptsAsWritten(hw_rewrite_top)),
    assert(acceptsHardened(hw_rewrite_top)),
    assert(isSoundWitness(hw_rewrite_top)),
    assert(hw_rewrite_top.steps == 3),
    assert(hw_rewrite_top.halted),
    assert(hw_rewrite_top.post.topVal == 100),
    assert(hw_rewrite_top.post.fp == -1),
    assert(hw_rewrite_top.post.acc == 110),
    assert(hw_rewrite_top.post.len == 2),
  }

  // Honest run 4: FETCH_ALU -> PUBLISH (short leaf closing at 2 steps on publication)
  run honestFetchAluShortLeafTest = all {
    assert(acceptsAsWritten(hw_fetch_alu)),
    assert(acceptsHardened(hw_fetch_alu)),
    assert(isSoundWitness(hw_fetch_alu)),
    assert(hw_fetch_alu.steps == 2),
    assert(hw_fetch_alu.halted),
    assert(hw_fetch_alu.post.acc == 25),
  }

  // ---------------------------------------------------------------- 2. Adversarial Witness Tests
  // Each test verifies three facts:
  //   (a) acceptsAsWritten(cex) == true  (accepted by the circuit design as written in study.md)
  //   (b) isSoundWitness(cex)   == false (violates reference semantics)
  //   (c) acceptsHardened(cex)  == false (rejected once the required constraints are added)

  run cex01_forgedBusTupleTest = all {
    assert(acceptsAsWritten(cex_forged_bus_tuple)),
    assert(not(isSoundWitness(cex_forged_bus_tuple))),
    assert(not(acceptsHardened(cex_forged_bus_tuple))),
  }

  run cex02_unrequestedChipRowTest = all {
    assert(acceptsAsWritten(cex_unrequested_chip_row)),
    assert(not(isSoundWitness(cex_unrequested_chip_row))),
    assert(not(acceptsHardened(cex_unrequested_chip_row))),
  }

  run cex03_wrongChipAnswerTest = all {
    assert(acceptsAsWritten(cex_wrong_chip_answer)),
    assert(not(isSoundWitness(cex_wrong_chip_answer))),
    assert(not(acceptsHardened(cex_wrong_chip_answer))),
  }

  run cex04_staleMemoryReadTest = all {
    assert(acceptsAsWritten(cex_stale_memory_read)),
    assert(not(isSoundWitness(cex_stale_memory_read))),
    assert(not(acceptsHardened(cex_stale_memory_read))),
  }

  run cex05_unwrittenCellReadTest = all {
    assert(acceptsAsWritten(cex_unwritten_cell_read)),
    assert(not(isSoundWitness(cex_unwritten_cell_read))),
    assert(not(acceptsHardened(cex_unwritten_cell_read))),
  }

  run cex06_wrongAluResultTest = all {
    assert(acceptsAsWritten(cex_wrong_alu_result)),
    assert(not(isSoundWitness(cex_wrong_alu_result))),
    assert(not(acceptsHardened(cex_wrong_alu_result))),
  }

  run cex07_wrongNextPcTest = all {
    assert(acceptsAsWritten(cex_wrong_next_pc)),
    assert(not(isSoundWitness(cex_wrong_next_pc))),
    assert(not(acceptsHardened(cex_wrong_next_pc))),
  }

  run cex08_chipTableOverflowTest = all {
    assert(acceptsAsWritten(cex_chip_table_overflow)),
    assert(not(isSoundWitness(cex_chip_table_overflow))),
    assert(not(acceptsHardened(cex_chip_table_overflow))),
  }

  run cex09_rowReplayAcrossStepsTest = all {
    assert(acceptsAsWritten(cex_row_replay_across_steps)),
    assert(not(isSoundWitness(cex_row_replay_across_steps))),
    assert(not(acceptsHardened(cex_row_replay_across_steps))),
  }

  run cex10_midStepLeafEndTest = all {
    assert(acceptsAsWritten(cex_mid_step_leaf_end)),
    assert(not(isSoundWitness(cex_mid_step_leaf_end))),
    assert(not(acceptsHardened(cex_mid_step_leaf_end))),
  }

  run cex11_duplicateMemAddrTest = all {
    assert(acceptsAsWritten(cex_duplicate_mem_addr)),
    assert(not(isSoundWitness(cex_duplicate_mem_addr))),
    assert(not(acceptsHardened(cex_duplicate_mem_addr))),
  }

  run cex12_stackTopRewriteCorruptTest = all {
    assert(acceptsAsWritten(cex_stack_top_rewrite_corrupt)),
    assert(not(isSoundWitness(cex_stack_top_rewrite_corrupt))),
    assert(not(acceptsHardened(cex_stack_top_rewrite_corrupt))),
  }

  run cex13_forgedFrameLinkTest = all {
    assert(acceptsAsWritten(cex_forged_frame_link)),
    assert(not(isSoundWitness(cex_forged_frame_link))),
    assert(not(acceptsHardened(cex_forged_frame_link))),
  }

  // ---------------------------------------------------------------- 3. Completeness Check Test
  run completenessGapTest = all {
    assert(completenessSingleStepExceedsMem),
  }

  // ---------------------------------------------------------------- 4. State Machine Invariant Test
  run stateMachineStepSoundnessTest =
    init.then(step).expect(invSound and invTamperRefused)
}
