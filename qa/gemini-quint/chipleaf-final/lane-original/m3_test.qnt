// Unit, Counterexample, Non-Vacuity, and Exhaustive Tests — Lane m3: M3 Memory Commitment (As Built)
// Run:
//   quint test --main m3_test qa/gemini-quint/chipleaf-final/m3_test.qnt

module m3_test {
  import m3(C = 4, D = 3, MUTANT = "none").* from "./m3"

  // ---------------------------------------------------------------- Exhaustive Pure Enumeration Helpers
  pure def allSatisfyRefine(mt: str): bool =
    PROG_IDS.fold(true, (okP, progId) =>
      PART_IDS.fold(okP, (okPt, partId) =>
        TAMPER_KINDS.fold(okPt, (okTk, tKind) =>
          ROW_DOM.fold(okTk, (okR, rowIdx) =>
            VAL_DOM.fold(okR, (okV, newVal) => {
              val cand = applyTamper(mt, progId, partId, tKind, rowIdx, newVal)
              okV and (acceptsWithMutant(mt, cand) implies refinesWith(mt, cand))
            })
          )
        )
      )
    )

  // ---------------------------------------------------------------- 1. Honest Witnesses & As-Built Properties
  run honestLeavesAndPartitionsTest = all {
    // Partition 0: Leaf 1 has 1 active row (writes & pops D7 zero sentinel (0,0,0)); Leaf 2 has 0 active rows
    assert(acceptsWithMutant("none", hw_0_0)),
    assert(refinesWith("none", hw_0_0)),
    assert(hw_0_0.leaf1.memRows[0].act == 1),
    assert(hw_0_0.leaf1.memRows[0].bus == { chip: CHIP_MEM, key: C, a: 0, b: 0, c: 0 }),
    assert(hw_0_0.leaf1.memRows[1].act == 0),
    assert(hw_0_0.leaf1.memRows[1].bus == ZERO_TUPLE),
    assert(hw_0_0.leaf2.memRows[0].act == 0),
    assert(recordHash("none", 1, ZERO_CELL3) != recordHash("none", 0, ZERO_CELL3)),

    // Partition 1: Leaf 1 has C active rows (full); Leaf 2 has 2 active rows (partial) + intra-leaf POP
    assert(acceptsWithMutant("none", hw_0_1)),
    assert(refinesWith("none", hw_0_1)),
    assert(hw_0_1.leaf1.memRows[C - 1].act == 1),
    assert(hw_0_1.leaf2.memRows[0].act == 1),
    assert(hw_0_1.leaf2.memRows[1].act == 1),
    assert(hw_0_1.leaf2.memRows[2].act == 0),

    // Partition 2: Both Leaf 1 and Leaf 2 have C active rows (full segments)
    assert(acceptsWithMutant("none", hw_1_2)),
    assert(refinesWith("none", hw_1_2)),
    assert(hw_1_2.leaf1.memRows[C - 1].act == 1),
    assert(hw_1_2.leaf2.memRows[C - 1].act == 1),

    // Two leaves chain: Leaf 2's pre_d equals Leaf 1's post_d, with len = base + C
    assert(hw_0_1.leaf2.instPreD == hw_0_1.leaf1.instPostD),
    assert(hw_0_1.leaf1.postDig.words.len == 2 * C),
    assert(hw_0_1.leaf2.postDig.words.len == 3 * C),
  }

  // As-built difference vs m3seg.qnt: there is no PAGE chip, so a POP in Leaf 2 of a frame pushed in Leaf 1
  // (fp = C < base2 = 2 * C) has no provider row on the bus and is refused by the bus lookup.
  pure val w_cross_leaf_pop: ChainW = {
    val pre1 = genesisRegs(0, C)
    val r1 = [RULE_PUSH, RULE_OFF, RULE_OFF, RULE_OFF]
    val r2 = [RULE_POP, RULE_OFF, RULE_OFF, RULE_OFF]
    val emptyMem: Set[MemEntry] = Set()
    val b1 = buildHonestLeaf("none", pre1, r1, emptyMem, EMPTY_TREE_MAP)
    val b2 = buildHonestLeaf("none", b1.leaf.postDig.words, r2, b1.memAfter, b1.treeAfter)
    { progId: 0, partId: 0, leaf1: b1.leaf, leaf2: b2.leaf }
  }

  run crossLeafReadRefusedWithoutPageTest = all {
    assert(leafAccepted("none", w_cross_leaf_pop.leaf1)),
    assert(w_cross_leaf_pop.leaf2.slots[0].l1.key == C),
    assert(not(memBusLookupOk("none", w_cross_leaf_pop.leaf2))),
    assert(not(acceptsWithMutant("none", w_cross_leaf_pop))),
  }

  // C1: undigested scratch register v0 is write-only; changing slot 0's v0 changes neither statement nor acceptance
  run scratchRegisterWriteOnlyTest = {
    val sl0 = hw_0_1.leaf1.slots[0]
    val sl0Upd: SlotW = { ...sl0, pre: { ...sl0.pre, v0: 12345 } }
    val l1Upd: LeafW = {
      ...hw_0_1.leaf1,
      slots: hw_0_1.leaf1.slots.replaceAt(0, sl0Upd),
      preDig: { ...hw_0_1.leaf1.preDig, words: { ...hw_0_1.leaf1.preDig.words, v0: 12345 } }
    }
    val wUpd: ChainW = { ...hw_0_1, leaf1: l1Upd }
    all {
      assert(wUpd.leaf1.instPreD == hw_0_1.leaf1.instPreD),
      assert(acceptsWithMutant("none", wUpd)),
      assert(refinesWith("none", wUpd)),
    }
  }

  // HIn::Val free advice in0/in1 on walker compressions is fixed by `m3 walk` gate
  run freeWalkSelectPinTest = {
    val forgedSelect = applyTamper("none", 0, 1, 2, 0, 1)
    assert(not(acceptsWithMutant("none", forgedSelect)))
  }

  // ---------------------------------------------------------------- 2. Mutant Counterexample Tests (M1..M15)
  run mutantM1_sibNotSharedTest = all {
    assert(acceptsWithMutant("sib_not_shared", cex_M1_sib_not_shared)),
    assert(not(acceptsWithMutant("none", cex_M1_sib_not_shared))),
    assert(not(refinesWith("sib_not_shared", cex_M1_sib_not_shared))),
  }

  run mutantM2_bitNotSharedTest = all {
    assert(acceptsWithMutant("bit_not_shared", cex_M2_bit_not_shared)),
    assert(not(acceptsWithMutant("none", cex_M2_bit_not_shared))),
    assert(not(refinesWith("bit_not_shared", cex_M2_bit_not_shared))),
  }

  run mutantM3_accNotChainedTest = all {
    assert(acceptsWithMutant("acc_not_chained", cex_M3_acc_not_chained)),
    assert(not(acceptsWithMutant("none", cex_M3_acc_not_chained))),
    assert(not(refinesWith("acc_not_chained", cex_M3_acc_not_chained))),
  }

  run mutantM4_walkLastDroppedTest = all {
    assert(acceptsWithMutant("walk_last_dropped", cex_M4_walk_last_dropped)),
    assert(not(acceptsWithMutant("none", cex_M4_walk_last_dropped))),
    assert(not(refinesWith("walk_last_dropped", cex_M4_walk_last_dropped))),
  }

  run mutantM5_bitNotBooleanTest = all {
    assert(acceptsWithMutant("bit_not_boolean", cex_M5_bit_not_boolean)),
    assert(not(acceptsWithMutant("none", cex_M5_bit_not_boolean))),
    assert(not(refinesWith("bit_not_boolean", cex_M5_bit_not_boolean))),
  }

  run mutantM6_xZeroDroppedTest = all {
    assert(acceptsWithMutant("x_zero_dropped", cex_M6_x_zero_dropped)),
    assert(not(acceptsWithMutant("none", cex_M6_x_zero_dropped))),
    assert(not(refinesWith("x_zero_dropped", cex_M6_x_zero_dropped))),
  }

  run mutantM7_baseUnalignedTest = all {
    assert(acceptsWithMutant("base_unaligned", cex_M7_base_unaligned)),
    assert(not(acceptsWithMutant("none", cex_M7_base_unaligned))),
    assert(not(refinesWith("base_unaligned", cex_M7_base_unaligned))),
  }

  run mutantM8_closeRunningTest = all {
    assert(acceptsWithMutant("close_running", cex_M8_close_running)),
    assert(not(acceptsWithMutant("none", cex_M8_close_running))),
    assert(not(refinesWith("close_running", cex_M8_close_running))),
  }

  run mutantM9_firstDroppedTest = all {
    assert(acceptsWithMutant("first_dropped", cex_M9_first_dropped)),
    assert(not(acceptsWithMutant("none", cex_M9_first_dropped))),
    assert(not(refinesWith("first_dropped", cex_M9_first_dropped))),
  }

  run mutantM10_lastDroppedTest = all {
    assert(acceptsWithMutant("last_dropped", cex_M10_last_dropped)),
    assert(not(acceptsWithMutant("none", cex_M10_last_dropped))),
    assert(not(refinesWith("last_dropped", cex_M10_last_dropped))),
  }

  run mutantM11_prefixEndDroppedTest = all {
    assert(acceptsWithMutant("prefix_end_dropped", cex_M11_prefix_end_dropped)),
    assert(not(acceptsWithMutant("none", cex_M11_prefix_end_dropped))),
    assert(not(refinesWith("prefix_end_dropped", cex_M11_prefix_end_dropped))),
  }

  run mutantM12_tagDroppedTest = all {
    assert(acceptsWithMutant("tag_dropped", cex_M12_tag_dropped)),
    assert(not(acceptsWithMutant("none", cex_M12_tag_dropped))),
    assert(not(refinesWith("tag_dropped", cex_M12_tag_dropped))),
  }

  run mutantM13_chipUnscaledTest = all {
    assert(acceptsWithMutant("chip_unscaled", cex_M13_chip_unscaled)),
    assert(not(acceptsWithMutant("none", cex_M13_chip_unscaled))),
    assert(not(refinesWith("chip_unscaled", cex_M13_chip_unscaled))),
  }

  run mutantM14_lenEndFreeTest = all {
    assert(acceptsWithMutant("len_end_free", cex_M14_len_end_free)),
    assert(not(acceptsWithMutant("none", cex_M14_len_end_free))),
    assert(not(refinesWith("len_end_free", cex_M14_len_end_free))),
  }

  run mutantM15_digestSkipsRootTest = all {
    assert(acceptsWithMutant("digest_skips_root", cex_M15_digest_skips_root)),
    assert(not(acceptsWithMutant("none", cex_M15_digest_skips_root))),
    assert(not(refinesWith("digest_skips_root", cex_M15_digest_skips_root))),
  }

  // ---------------------------------------------------------------- 3. Honest Witness Under Mutants Tests (Non-Vacuity)
  run honestUnderMutantsRefinesTest = all {
    // M1..M7, M9..M11, M13..M15: honest witness refines on every (progId, partId)
    assert(
      Set(
        "sib_not_shared",
        "bit_not_shared",
        "acc_not_chained",
        "walk_last_dropped",
        "bit_not_boolean",
        "x_zero_dropped",
        "base_unaligned",
        "first_dropped",
        "last_dropped",
        "prefix_end_dropped",
        "chip_unscaled",
        "len_end_free",
        "digest_skips_root"
      ).forall(mt =>
        PROG_IDS.forall(progId =>
          PART_IDS.forall(partId => {
            val hw = buildHonestChain(mt, progId, partId)
            acceptsWithMutant(mt, hw) and refinesWith(mt, hw)
          })
        )
      )
    ),
    // M8 `close_running`: honest witness refines on full segments (partId == 2, where base + used == base + C)
    assert(
      PROG_IDS.forall(progId => {
        val hwFull = buildHonestChain("close_running", progId, 2)
        acceptsWithMutant("close_running", hwFull) and refinesWith("close_running", hwFull)
      })
    ),
    // M12 `tag_dropped`: honest witness refines when written records are non-zero (progId == 1)
    assert(
      PART_IDS.forall(partId => {
        val hwNonZero = buildHonestChain("tag_dropped", 1, partId)
        acceptsWithMutant("tag_dropped", hwNonZero) and refinesWith("tag_dropped", hwNonZero)
      })
    ),
  }

  // ---------------------------------------------------------------- 4. Exhaustive Pure Domain Checks
  run exhaustiveSoundTest =
    assert(allSatisfyRefine("none"))

  run exhaustiveMutantsCaughtTest = all {
    assert(not(allSatisfyRefine("sib_not_shared"))),
    assert(not(allSatisfyRefine("bit_not_shared"))),
    assert(not(allSatisfyRefine("acc_not_chained"))),
    assert(not(allSatisfyRefine("walk_last_dropped"))),
    assert(not(allSatisfyRefine("bit_not_boolean"))),
    assert(not(allSatisfyRefine("x_zero_dropped"))),
    assert(not(allSatisfyRefine("base_unaligned"))),
    assert(not(allSatisfyRefine("close_running"))),
    assert(not(allSatisfyRefine("first_dropped"))),
    assert(not(allSatisfyRefine("last_dropped"))),
    assert(not(allSatisfyRefine("prefix_end_dropped"))),
    assert(not(allSatisfyRefine("tag_dropped"))),
    assert(not(allSatisfyRefine("chip_unscaled"))),
    assert(not(allSatisfyRefine("len_end_free"))),
    assert(not(allSatisfyRefine("digest_skips_root"))),
  }
}
