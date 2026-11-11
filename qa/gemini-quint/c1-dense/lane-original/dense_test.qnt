// -*- mode: Bluespec; -*-
//
// Unit, Counterexample, Partition-Independence, and Exhaustive Tamper Tests
// for C1 Dense Tail-Buffered Merkle Memory (`dense.qnt`).
//
// Run:
//   quint typecheck qa/gemini-quint/c1-dense/dense_test.qnt
//   quint test --main dense_test qa/gemini-quint/c1-dense/dense_test.qnt

module dense_test {
  import dense(MUTANT = "none").* from "./dense"

  // ---------------------------------------------------------------- Helper: Multi-leaf partition runner (SRC-023)
  // Run a sequence of leaf chunk sizes `sizes` on `stream` of records, checking every leaf's
  // circuit gates (`checkWindow`, `checkPersist`, `checkCoreAndReads`) and boundary stitch.
  pure def runPartition(stream: List[Rec], sizes: List[int]): { allAccepted: bool, finalState: ArenaState } = {
    val r0 = initRefArena("none")
    val initSt = refArenaState("none", r0)
    val res = range(0, sizes.length()).foldl(
      { curRef: r0, curSt: initSt, offset: 0, ok: true },
      (st, li) => {
        val sz = sizes[li]
        val emptyChunk: List[Rec] = []
        val chunk = range(0, sz).foldl(emptyChunk, (acc, k) => acc.append(stream[st.offset + k]))
        // Use W = 2 flush units so up to 2 segment boundaries can be crossed in one leaf
        val built = buildHonestArenaLeaf("none", st.curRef, 0, 2, chunk, [])
        val lf = built.leaf
        val leafOk = and {
          lf.pre == st.curSt,
          checkWindow("none", lf),
          checkPersist("none", lf),
          checkCoreAndReads("none", lf, lf.winRows)
        }
        {
          curRef: built.refPost,
          curSt: lf.post,
          offset: st.offset + sz,
          ok: st.ok and leafOk
        }
      }
    )
    { allAccepted: res.ok, finalState: res.curSt }
  }

  // ---------------------------------------------------------------- Exhaustive Pure Check Helpers
  pure def exhaustiveAttacksSound(mt: str): bool =
    TAMPER_DOM.fold(true, (ok, tk) => {
      val cand = buildTampered(mt, tk)
      ok and (acceptsChain(mt, cand) implies refinesChain(cand))
    })

  pure def exhaustiveCellTampersSound(mt: str): bool =
    TARGET_DOM.fold(true, (okT, target) =>
      IDX_DOM.fold(okT, (okI, idx) =>
        VAL_DOM.fold(okI, (okV, newVal) => {
          val cand = tamperCell(mt, target, idx, newVal)
          okV and (acceptsChain(mt, cand) implies refinesChain(cand))
        })
      )
    )

  pure def exhaustiveMutantCaught(mt: str): bool =
    not(exhaustiveAttacksSound(mt))

  // ---------------------------------------------------------------- 1. Positive Completeness Tests (Questions 1..8)

  // Base honest 2-leaf chain (`c1_warm_and_cold_reads`)
  run honestBaseChainTest = {
    val hw = buildBaseHonest("none")
    all {
      assert(acceptsChain("none", hw)),
      assert(refinesChain(hw)),
      assert(hw.leaf1.units[0].f == 1),
      assert(hw.leaf2.units[0].f == 0),
      assert(hw.leaf1.reads.length() == 3),
      assert(hw.leaf2.reads.length() == 1),
    }
  }

  // Question 1 (SRC-023, `c1_partition_independence_4_splits`):
  // 9 records appended under 4 different leaf partitions:
  //   P1: [9] (single leaf with W = 2 crossing two segment boundaries)
  //   P2: [4, 5] (aligned boundary then 5-record leaf)
  //   P3: [3, 0, 6] (unaligned leaf, 0-append pass-through leaf, then 2-flush leaf)
  //   P4: [1, 1, 1, 1, 1, 1, 1, 1, 1] (9 single-record leaves)
  run src023PartitionIndependenceTest = {
    val emptyR: List[Rec] = []
    val stream = range(0, 9).foldl(emptyR, (acc, i) => acc.append(mkRec(1 + (i % 4), 100 + i)))
    val refFinal = refArenaState("none", refAppendList("none", initRefArena("none"), stream))
    val p1 = runPartition(stream, [9])
    val p2 = runPartition(stream, [4, 5])
    val p3 = runPartition(stream, [3, 0, 6])
    val p4 = runPartition(stream, [1, 1, 1, 1, 1, 1, 1, 1, 1])
    all {
      assert(p1.allAccepted),
      assert(p2.allAccepted),
      assert(p3.allAccepted),
      assert(p4.allAccepted),
      assert(p1.finalState == refFinal),
      assert(p2.finalState == refFinal),
      assert(p3.finalState == refFinal),
      assert(p4.finalState == refFinal),
    }
  }

  // Question 2 (`c1_two_flushes_one_leaf`): W = 2 flushes in one leaf + pass-through second leaf
  run twoFlushUnitsAndPassThroughTest = {
    val hw2 = buildTwoFlushHonest("none")
    all {
      assert(acceptsChain("none", hw2)),
      assert(refinesChain(hw2)),
      assert(hw2.leaf1.units[0].f == 1),
      assert(hw2.leaf1.units[1].f == 1),
      assert(hw2.leaf2.pre == hw2.leaf2.post),
    }
  }

  // Question 3 (`c1_pass_through_unchanged`): 0-append pass-through at all tail occupancies r = 0..3
  run passThroughUnchangedTest = {
    val r0 = initRefArena("none")
    val r1 = refAppendList("none", r0, [mkRec(KIND_DATA, 1)])
    val r2 = refAppendList("none", r1, [mkRec(KIND_FIELD, 2)])
    val r3 = refAppendList("none", r2, [mkRec(KIND_ENV, 3)])
    val b0 = buildHonestArenaLeaf("none", r0, 0, 1, [], []).leaf
    val b1 = buildHonestArenaLeaf("none", r1, 0, 1, [], []).leaf
    val b2 = buildHonestArenaLeaf("none", r2, 0, 1, [], []).leaf
    val b3 = buildHonestArenaLeaf("none", r3, 0, 1, [], []).leaf
    all {
      assert(checkWindow("none", b0) and checkPersist("none", b0) and b0.pre == b0.post),
      assert(checkWindow("none", b1) and checkPersist("none", b1) and b1.pre == b1.post),
      assert(checkWindow("none", b2) and checkPersist("none", b2) and b2.pre == b2.post),
      assert(checkWindow("none", b3) and checkPersist("none", b3) and b3.pre == b3.post),
    }
  }

  // Question 4 (`c1_four_arenas_shared_bus`): 2 arenas (`KIND_DATA` and `KIND_FIELD`) sharing the bus
  run fourArenasSharedBusIsolationTest = {
    val base = buildBaseHonest("none")
    val r0 = initRefArena("none")
    val refA1Pre = refAppendList("none", r0, [mkRec(KIND_DATA, 10), mkRec(KIND_DATA, 11)])
    val bA1 = buildHonestArenaLeaf("none", refA1Pre, KIND_DATA, 1, [mkRec(KIND_DATA, 12)], [{ addr: 1, atPost: true }])
    val bA1Next = buildHonestArenaLeaf("none", bA1.refPost, KIND_DATA, 1, [], [])
    val refA2Pre = refAppendList("none", r0, [mkRec(KIND_FIELD, 50)])
    val bA2 = buildHonestArenaLeaf("none", refA2Pre, KIND_FIELD, 1, [mkRec(KIND_FIELD, 51)], [{ addr: 0, atPost: true }])
    val cw: ChainW = {
      ...base,
      leaf1: bA1.leaf,
      leaf2: bA1Next.leaf,
      hasArena2: true,
      arena2Leaf: bA2.leaf,
      refPost1: refArenaState("none", bA1.refPost),
      refPost2: refArenaState("none", bA1Next.refPost),
      refReadVals1: bA1.expectedReads,
      refReadVals2: [],
      refArena2Post: refArenaState("none", bA2.refPost)
    }
    all {
      assert(acceptsChain("none", cw)),
      assert(refinesChain(cw)),
    }
  }

  // Question 6 (`c1_last_segment_two_units`, `c1_counter_edge_2_64_minus_1`, `c1_mut_walk_idx_unconditional_inc`):
  // Honest chain at the last segment (`q_pre = 3`, `len_post = 15 = MAX_LEN - 1`) with `W = 2` is accepted
  // under `idx_j = idx_{j-1} + f_j`, and rejected under `walk_idx_unconditional_inc` (`idx_j = q_pre + j`).
  run lastSegmentCounterEdgeAndTwoUnitsTest = {
    val hwLast = buildLastSegmentHonest("none")
    val badLast = buildLastSegmentHonest("walk_idx_unconditional_inc")
    all {
      assert(acceptsChain("none", hwLast)),
      assert(refinesChain(hwLast)),
      assert(hwLast.leaf2.post.len == MAX_LEN - 1),
      assert(hwLast.leaf2.units[0].idx == TOP_SLOTS - 1),
      assert(hwLast.leaf2.units[1].idx == TOP_SLOTS - 1),
      // Under unconditional `q_pre + j`, unit 1 has `idx = 4 = TOP_SLOTS`, overflowing TOP bits:
      assert(badLast.leaf2.units[1].idx == TOP_SLOTS),
      assert(not(acceptsChain("walk_idx_unconditional_inc", badLast))),
    }
  }

  // Question 8: Capacity domain tags separate `PAD`, `EMPTY_SEG`, and tree heights
  run domainTagInjectivityTest = all {
    assert(padD("none") != emptySeg("none")),
    assert(nodeH("none", 1, 10, 20) != nodeH("none", 2, 10, 20)),
    assert(nodeH("none", 1, 10, 20) != nodeH("none", 3, 10, 20)),
    assert(padD("domain_tag_dropped") == emptySeg("domain_tag_dropped")),
    assert(nodeH("domain_tag_dropped", 1, 10, 20) == nodeH("domain_tag_dropped", 3, 10, 20)),
  }

  // ---------------------------------------------------------------- 2. Honest Witness Non-Vacuity Under All Mutants
  run honestUnderAllMutantsRefinesTest =
    assert(
      Set(
        "forged_tail_across_leaf_boundary",
        "non_canonical_tail",
        "flush_skipped",
        "flush_spurious",
        "address_reuse_occupied_slot",
        "empty_seg_forgery",
        "address_reuse_window_row",
        "read_at_len",
        "read_beyond_len",
        "future_record_read",
        "wrong_kind_warm",
        "wrong_kind_cold",
        "cold_value_forged",
        "cold_route_forged",
        "counter_wrap_2_64",
        "pad_row_on_bus",
        "kind_out_of_range",
        "arena_kind_pinned",
        "close_advances_len",
        "append_value_mismatch",
        "new_walk_unshared_sibs",
        "domain_tag_dropped",
        "top1_walk_empty_shadowed"
      ).forall(mt => {
        val hw = buildBaseHonest(mt)
        acceptsChain(mt, hw) and refinesChain(hw)
      })
    )

  // ---------------------------------------------------------------- 3. Per-Mutant Counterexample Tests (1..23)
  run mutant01_forgedTailAcrossLeafBoundaryTest = {
    val cex = buildTampered("forged_tail_across_leaf_boundary", 1)
    all {
      assert(acceptsChain("forged_tail_across_leaf_boundary", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant02_nonCanonicalTailTest = {
    val cex = buildTampered("non_canonical_tail", 2)
    all {
      assert(acceptsChain("non_canonical_tail", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant03_flushSkippedTest = {
    val cex = buildTampered("flush_skipped", 3)
    all {
      assert(acceptsChain("flush_skipped", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant04_flushSpuriousTest = {
    val cex = buildTampered("flush_spurious", 4)
    all {
      assert(acceptsChain("flush_spurious", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant05_addressReuseOccupiedSlotTest = {
    val cex = buildTampered("address_reuse_occupied_slot", 5)
    all {
      assert(acceptsChain("address_reuse_occupied_slot", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant06_emptySegForgeryTest = {
    val cex = buildTampered("empty_seg_forgery", 6)
    all {
      assert(acceptsChain("empty_seg_forgery", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant07_addressReuseWindowRowTest = {
    val cex = buildTampered("address_reuse_window_row", 7)
    all {
      assert(acceptsChain("address_reuse_window_row", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant08_readAtLenTest = {
    val cex = buildTampered("read_at_len", 8)
    all {
      assert(acceptsChain("read_at_len", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant09_readBeyondLenTest = {
    val cex = buildTampered("read_beyond_len", 9)
    all {
      assert(acceptsChain("read_beyond_len", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant10_futureRecordReadTest = {
    val cex = buildTampered("future_record_read", 10)
    all {
      assert(acceptsChain("future_record_read", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant11_wrongKindWarmTest = {
    val cex = buildTampered("wrong_kind_warm", 11)
    all {
      assert(acceptsChain("wrong_kind_warm", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant12_wrongKindColdTest = {
    val cex = buildTampered("wrong_kind_cold", 12)
    all {
      assert(acceptsChain("wrong_kind_cold", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant13_coldValueForgedTest = {
    val cex = buildTampered("cold_value_forged", 13)
    all {
      assert(acceptsChain("cold_value_forged", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant14_coldRouteForgedTest = {
    val cex = buildTampered("cold_route_forged", 14)
    all {
      assert(acceptsChain("cold_route_forged", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant15_counterWrap264Test = {
    val cex = buildTampered("counter_wrap_2_64", 15)
    all {
      assert(acceptsChain("counter_wrap_2_64", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant16_padRowOnBusTest = {
    val cex = buildTampered("pad_row_on_bus", 16)
    all {
      assert(acceptsChain("pad_row_on_bus", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant17_kindOutOfRangeTest = {
    val cex = buildTampered("kind_out_of_range", 17)
    all {
      assert(acceptsChain("kind_out_of_range", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant18_arenaKindPinnedTest = {
    val cex = buildTampered("arena_kind_pinned", 18)
    all {
      assert(acceptsChain("arena_kind_pinned", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant19_closeAdvancesLenTest = {
    val cex = buildTampered("close_advances_len", 19)
    all {
      assert(acceptsChain("close_advances_len", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant20_appendValueMismatchTest = {
    val cex = buildTampered("append_value_mismatch", 20)
    all {
      assert(acceptsChain("append_value_mismatch", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant21_newWalkUnsharedSibsTest = {
    val cex = buildTampered("new_walk_unshared_sibs", 21)
    all {
      assert(acceptsChain("new_walk_unshared_sibs", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant22_domainTagDroppedTest = {
    val cex = buildTampered("domain_tag_dropped", 22)
    val cexSound = buildTampered("none", 22)
    all {
      assert(acceptsChain("domain_tag_dropped", cex)),
      assert(not(acceptsChain("none", cexSound))),
      assert(not(refinesChain(cex))),
    }
  }

  run mutant23_top1WalkEmptyShadowedTest = {
    val cex = buildTampered("top1_walk_empty_shadowed", 23)
    all {
      assert(acceptsChain("top1_walk_empty_shadowed", cex)),
      assert(not(acceptsChain("none", cex))),
      assert(not(refinesChain(cex))),
    }
  }

  // ---------------------------------------------------------------- 4. Exhaustive Pure Domain Checks
  run exhaustiveSoundAttacksTest =
    assert(exhaustiveAttacksSound("none"))

  run exhaustiveSoundCellTampersTest =
    assert(exhaustiveCellTampersSound("none"))

  run exhaustiveAllMutantsCaughtTest = all {
    assert(exhaustiveMutantCaught("forged_tail_across_leaf_boundary")),
    assert(exhaustiveMutantCaught("non_canonical_tail")),
    assert(exhaustiveMutantCaught("flush_skipped")),
    assert(exhaustiveMutantCaught("flush_spurious")),
    assert(exhaustiveMutantCaught("address_reuse_occupied_slot")),
    assert(exhaustiveMutantCaught("empty_seg_forgery")),
    assert(exhaustiveMutantCaught("address_reuse_window_row")),
    assert(exhaustiveMutantCaught("read_at_len")),
    assert(exhaustiveMutantCaught("read_beyond_len")),
    assert(exhaustiveMutantCaught("future_record_read")),
    assert(exhaustiveMutantCaught("wrong_kind_warm")),
    assert(exhaustiveMutantCaught("wrong_kind_cold")),
    assert(exhaustiveMutantCaught("cold_value_forged")),
    assert(exhaustiveMutantCaught("cold_route_forged")),
    assert(exhaustiveMutantCaught("counter_wrap_2_64")),
    assert(exhaustiveMutantCaught("pad_row_on_bus")),
    assert(exhaustiveMutantCaught("kind_out_of_range")),
    assert(exhaustiveMutantCaught("arena_kind_pinned")),
    assert(exhaustiveMutantCaught("close_advances_len")),
    assert(exhaustiveMutantCaught("append_value_mismatch")),
    assert(exhaustiveMutantCaught("new_walk_unshared_sibs")),
    assert(exhaustiveMutantCaught("domain_tag_dropped")),
    assert(exhaustiveMutantCaught("top1_walk_empty_shadowed")),
  }
}
