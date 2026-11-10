// Unit & Exhaustive Tests for Part 2: M3 Aligned-Segment Memory Root
// Run with:
//   quint test --main m3segTest qa/gemini-quint/chipleaf-pairrow-m3/m3seg_test.qnt

module m3segTest {
  import m3seg(S = 4, C = 4, MUTANT = "none").* from "./m3seg"

  // ---------------------------------------------------------------- Completeness & Sentinel Tests
  // 1. D7 sentinel spill (0, NIL) with NIL = 99 != 0: POP at depth 1 is accepted and refines
  run honestD7SentinelPopTest = all {
    assert(acceptsWithMutant("none", hw_d7_sentinel_pop)),
    assert(refines(hw_d7_sentinel_pop)),
    assert(hw_d7_sentinel_pop.leaf1.memRows[0].cv == { a: 0, link: NIL_DEFAULT }),
    assert(hw_d7_sentinel_pop.leaf2.pageRows[1].cv == { a: 0, link: NIL_DEFAULT })
  }

  // 2. Completeness failure under rf.rs's NIL = 0:
  //    D7 depth-0 PUSH spills (0, 0) == ZERO_CELL; POP at depth 1 in Leaf 2 reads (0, 0)
  //    and is REJECTED by M3's zero-tuple refusal on reads!
  run completenessFailureNilZeroTest = all {
    assert(w_d7_sentinel_nil_zero.leaf1.memRows[0].cv == ZERO_CELL),
    assert(leafAccepted("none", w_d7_sentinel_nil_zero.leaf1)),
    assert(not(leafAccepted("none", w_d7_sentinel_nil_zero.leaf2))),
    assert(not(acceptsWithMutant("none", w_d7_sentinel_nil_zero)))
  }

  // 3. Honest zero-payload data record with a non-zero tag/link word (0, 1) != (0, 0):
  //    Accepted and refines when read back
  run honestZeroPayloadTaggedTest = all {
    assert(acceptsWithMutant("none", hw_zero_payload_tagged)),
    assert(refines(hw_zero_payload_tagged))
  }

  // 4. Completeness failure when an honest data record is untagged all-zero (0, 0):
  //    Writing (0, 0) and reading it back is REJECTED by zero-tuple refusal on reads,
  //    AND committedNonZeroCells omits it from the active written set.
  run completenessFailureUntaggedZeroRecordTest = all {
    assert(not(acceptsWithMutant("none", w_zero_record_untagged)))
  }

  // ---------------------------------------------------------------- Leaf Partition & Observability Tests
  // Same 4-step program (Prog 0: PUSH 10, PUSH 20, POP, POP) under 3 leaf partitions:
  //   - hw_part_2_2: [2, 2] (Leaf 1 does 2 PUSHes; Leaf 2 does 2 POPs and 0 appends)
  //   - hw_part_1_3: [1, 3] (Leaf 1 does 1 PUSH; Leaf 2 does 1 PUSH + 2 POPs)
  //   - hw_part_4_0: [4, 0] (Leaf 1 does all 4 steps; Leaf 2 does 0 steps and 0 appends)
  run partitionsAcceptedAndRefineTest = all {
    assert(acceptsWithMutant("none", hw_part_2_2)),
    assert(refines(hw_part_2_2)),
    assert(acceptsWithMutant("none", hw_part_1_3)),
    assert(refines(hw_part_1_3)),
    assert(acceptsWithMutant("none", hw_part_4_0)),
    assert(refines(hw_part_4_0)),
    // A leaf with 0 appends (Leaf 2 in [2, 2] and [4, 0]) STILL allocates C = 4 addresses:
    assert(hw_part_2_2.leaf2.closeStep.used == 0),
    assert(hw_part_2_2.leaf1.post.runningLen == 4),
    assert(hw_part_2_2.leaf2.post.runningLen == 8),
    assert(hw_part_4_0.leaf2.closeStep.used == 0),
    assert(hw_part_4_0.leaf2.post.runningLen == 8),
    // Address-valued register `fp` makes the leaf partition OBSERVABLE:
    // After the second PUSH (step 1 of Prog 0), `fp` is 1 in partition [2, 2],
    // whereas `fp` is 4 (base2 + 0) in partition [1, 3]!
    assert(hw_part_2_2.leaf1.slots[1].post.fp == 1),
    assert(hw_part_1_3.leaf2.slots[0].post.fp == 4),
    // Likewise, the committed address of the second stack frame in mem_root differs (addr 1 vs addr 4):
    assert(committedNonZeroCells(hw_part_2_2.leaf2.post.memRoot) != committedNonZeroCells(hw_part_1_3.leaf2.post.memRoot)),
    // Yet the value registers (`top`, `r0`, `depth`) at the end of the 4-step program agree:
    assert(hw_part_2_2.leaf2.post.r0 == hw_part_1_3.leaf2.post.r0),
    assert(hw_part_2_2.leaf2.post.top == hw_part_1_3.leaf2.post.top),
    assert(hw_part_2_2.leaf2.post.depth == hw_part_1_3.leaf2.post.depth)
  }

  // ---------------------------------------------------------------- Required Mutant Counterexample Tests (M1..M7)
  // M1 `padding_nonzero`: padding row k = 2 in Leaf 1 holds non-zero (3, 1); committed in mem_root without append
  run mutantM1PaddingNonzeroTest = all {
    assert(acceptsWithMutant("padding_nonzero", cex_M1_padding_nonzero)),
    assert(not(acceptsWithMutant("none", cex_M1_padding_nonzero))),
    assert(not(refines(cex_M1_padding_nonzero)))
  }

  // M2 `append_skips`: append advances runningLen by 2, leaving active hole at row k = 1 filled with unrequested (3, 1)
  run mutantM2AppendSkipsTest = all {
    assert(acceptsWithMutant("append_skips", cex_M2_append_skips)),
    assert(not(acceptsWithMutant("none", cex_M2_append_skips))),
    assert(not(refines(cex_M2_append_skips)))
  }

  // M3 `padding_read`: Leaf 2 reads unwritten padding cell at addr = 2 via PAGE, getting ZERO_CELL
  run mutantM3PaddingReadTest = all {
    assert(acceptsWithMutant("padding_read", cex_M3_padding_read)),
    assert(not(acceptsWithMutant("none", cex_M3_padding_read))),
    assert(not(refines(cex_M3_padding_read)))
  }

  // M4 `close_len_running`: Leaf 2 closes with post_d.len = base + used (6) instead of base + C (8),
  //    desynchronizing post_d.len (6 / 4 == 1) and post_d.mem_root (segment 1 already filled)
  run mutantM4CloseLenRunningTest = all {
    assert(acceptsWithMutant("close_len_running", cex_M4_close_len_running)),
    assert(not(acceptsWithMutant("none", cex_M4_close_len_running))),
    assert(not(lenRootSynced(cex_M4_close_len_running.leaf2.post))),
    assert(not(refines(cex_M4_close_len_running)))
  }

  // M5 `base_unaligned`: Genesis starts at base = 1 (as in rf.rs); MEM row 0 writes addr = 1 into offset 0
  //    of segment 0, whereas PAGE in Leaf 2 opens addr = 1 at offset 1 % 4 = 1 (reading addr 2's value!)
  run mutantM5BaseUnalignedTest = all {
    assert(acceptsWithMutant("base_unaligned", cex_M5_base_unaligned)),
    assert(not(acceptsWithMutant("none", cex_M5_base_unaligned))),
    assert(not(refines(cex_M5_base_unaligned)))
  }

  // M6 `no_empty_slot_check`: Sibling preservation / empty-slot check omitted on root update;
  //    Leaf 2 overwrites segment 0 in post_d.mem_root with forged cells
  run mutantM6NoEmptySlotCheckTest = all {
    assert(acceptsWithMutant("no_empty_slot_check", cex_M6_no_empty_slot_check)),
    assert(not(acceptsWithMutant("none", cex_M6_no_empty_slot_check))),
    assert(not(refines(cex_M6_no_empty_slot_check)))
  }

  // M7 `page_serves_segment`: D2's addr < base dropped on PAGE (combined with M3 zero-refusal drop);
  //    intra-leaf read of addr = 0 in Leaf 1 is served stale ZERO_CELL from pre_d.mem_root by PAGE
  run mutantM7PageServesSegmentTest = all {
    assert(acceptsWithMutant("page_serves_segment", cex_M7_page_serves_segment)),
    assert(not(acceptsWithMutant("none", cex_M7_page_serves_segment))),
    assert(not(refines(cex_M7_page_serves_segment)))
  }

  // Interaction of M7 with D3 zero-refusal and len/root sync (`page_serves_segment_alone`):
  // When zero-tuple refusal on reads and len/root sync hold, pre_d.mem_root has empty segments for all
  // addr >= base, so any PAGE opening for addr >= base returns ZERO_CELL, which reads refuse.
  run m7AloneBlockedByZeroRefusalTest = all {
    assert(not(acceptsWithMutant("page_serves_segment_alone", cex_M7_page_serves_segment)))
  }

  // ---------------------------------------------------------------- Exhaustive Pure Enumeration
  pure def allSatisfyRefine(mt: str): bool =
    PROG_IDS.forall(progId =>
      PART_IDS.forall(partId =>
        TAMPER_KINDS.forall(tKind =>
          ROW_DOM.forall(rowIdx =>
            VAL_DOM.forall(newVal => {
              val cand = applyTamper(mt, progId, partId, tKind, rowIdx, newVal)
              acceptsWithMutant(mt, cand) implies refines(cand)
            })
          )
        )
      )
    )

  run exhaustiveSoundTest = all {
    assert(allSatisfyRefine("none")),
    assert(allSatisfyRefine("page_serves_segment_alone"))
  }

  run exhaustiveMutantsCaughtTest = all {
    assert(not(allSatisfyRefine("padding_nonzero"))),
    assert(not(allSatisfyRefine("append_skips"))),
    assert(not(allSatisfyRefine("padding_read"))),
    assert(not(allSatisfyRefine("close_len_running"))),
    assert(not(allSatisfyRefine("base_unaligned"))),
    assert(not(allSatisfyRefine("no_empty_slot_check"))),
    assert(not(allSatisfyRefine("page_serves_segment")))
  }

  // ---------------------------------------------------------------- Lead additions (2026-11-10)
  // Zero-tuple spill with NIL != 0 once genesis len = 0: PUSH 0 sets fp := 0, so the next PUSH
  // spills (top = 0, fp = 0) == ZERO_CELL at address 1. The POP that loads it is refused. NIL != 0
  // alone does not close the completeness gap; every honest record must differ from the zero tuple.
  pure val w_push_zero_at_addr0: ChainW = {
    val insns1 = [
      { op: OP_PUSH, arg: 0, tagLink: 0 },
      { op: OP_PUSH, arg: 5, tagLink: 0 },
      { op: OP_POP, arg: 0, tagLink: 0 },
      { op: OP_POP, arg: 0, tagLink: 0 }
    ]
    val pre1 = initRegState(0, NIL_DEFAULT)
    val b1 = buildLeafFromInsns("none", pre1, insns1, 4, emptyWrittenMap, -1, ZERO_CELL, false)
    val b2 = buildLeafFromInsns("none", b1.leaf.post, [NO_INSN, NO_INSN, NO_INSN, NO_INSN], 0, b1.nextMap, -1, ZERO_CELL, false)
    { progId: 0, partId: 2, nilVal: NIL_DEFAULT, leaf1: b1.leaf, leaf2: b2.leaf }
  }

  run completenessFailurePushZeroAtAddr0Test = all {
    assert(w_push_zero_at_addr0.leaf1.memRows[1].cv == ZERO_CELL),
    assert(not(acceptsWithMutant("none", w_push_zero_at_addr0)))
  }

  // Honest chains under M1, M3, M5, M6, M7 and M7-alone are accepted and refine, so those mutants are
  // caught by a forged witness. M2 (append_skips) and M4 (close_len_running) are excluded: their honest
  // run already diverges (M2 writes with a hole; under M4 the next base is unaligned).
  // Only (program, partition) pairs whose honest chain the sound model accepts are used: Prog 1 loads
  // the literal address 1, which under partition [1, 3] is a padding cell, so the sound model refuses it
  // (the partition is observable through literal addresses).
  pure val VALID_RUNS: Set[(int, int)] =
    tuples(PROG_IDS, PART_IDS).filter(pp =>
      acceptsWithMutant("none", buildHonestChain("none", pp._1, pp._2, NIL_DEFAULT)))

  run validRunsTest = all {
    assert(VALID_RUNS.size() == 5),
    assert(not(VALID_RUNS.contains((1, 1))))
  }

  run honestUnderMutantsRefinesTest = all {
    Set("padding_nonzero", "padding_read", "base_unaligned", "no_empty_slot_check",
        "page_serves_segment", "page_serves_segment_alone").forall(mt =>
      VALID_RUNS.forall(pp => and {
        acceptsWithMutant(mt, buildHonestChain(mt, pp._1, pp._2, NIL_DEFAULT)),
        refines(buildHonestChain(mt, pp._1, pp._2, NIL_DEFAULT))
      }))
  }
}
