// Unit, Counterexample, and Exhaustive Tamper Tests — Poseidon-only HASH Chip and Bus Lookup
// Run: quint test --main hashchip_test qa/gemini-quint/chipleaf-hash/hashchip_test.qnt

module hashchip_test {
  import hashchip(MUTANT = "none").* from "./hashchip"

  // ---------------------------------------------------------------- Exhaustive Pure Tamper & Request Domains
  pure val EXH_VALS: Set[int] = Set(0, 1, 2, 3, 5, 99)

  // Fold over all single-cell tampers (rIdx in ROW_DOM, cIdx in COL_DOM, v in EXH_VALS)
  // plus canonical attack witnesses: verify `accepted implies refines` for `mt`.
  pure def exhaustiveSingleCellSound(mt: str): bool = {
    val attacksOk = ATTACK_IDS.fold(true, (okA, aid) => {
      val cand = applyAttack(mt, aid)
      okA and (acceptsWithMutant(mt, cand) implies refinesWith(mt, cand))
    })
    val cellsOk = ROW_DOM.fold(true, (okR, rIdx) =>
      COL_DOM.fold(okR, (okC, cIdx) =>
        EXH_VALS.fold(okC, (okV, v) => {
          val cand = tamperSingleCell(mt, rIdx, cIdx, v)
          okV and (acceptsWithMutant(mt, cand) implies refinesWith(mt, cand))
        })
      )
    )
    attacksOk and cellsOk
  }

  // Fold over all request tuples from the finite domain (REQ_ROWS x REQ_LANES x REQ_CHIPS x REQ_KEYS x REQ_INS x REQ_INS x REQ_OUTS)
  pure def exhaustiveRequestTuplesSound(mt: str): bool =
    REQ_ROWS.fold(true, (okRow, rRow) =>
      REQ_LANES.fold(okRow, (okLane, rLane) =>
        REQ_CHIPS.fold(okLane, (okChip, ch) =>
          REQ_KEYS.fold(okChip, (okKey, k) =>
            REQ_INS.fold(okKey, (okIn0, i0) =>
              REQ_INS.fold(okIn0, (okIn1, i1) =>
                REQ_OUTS.fold(okIn1, (okOut, outV) => {
                  val cand = tamperRequestTuple(mt, rRow, rLane, [ch, k, i0, i1, outV])
                  okOut and (acceptsWithMutant(mt, cand) implies refinesWith(mt, cand))
                })
              )
            )
          )
        )
      )
    )

  // Check that there exists an accepted witness in the finite domain that violates refinement for mutant `mt`
  pure def exhaustiveMutantCaught(mt: str): bool =
    not(exhaustiveSingleCellSound(mt) and exhaustiveRequestTuplesSound(mt))

  // ---------------------------------------------------------------- 1. Honest Witness & Provider Reuse Tests
  run honestWitnessAcceptedAndRefinesTest = all {
    assert(acceptsWithMutant("none", hw_honest)),
    assert(refinesWith("none", hw_honest)),
    // Verify in-region sponge outputs match hashRef:
    assert(hw_honest.advice[11][0] == hashRef([1, 3])),
    assert(hw_honest.advice[19][0] == hashRef([1, 2, 3])),
    assert(hw_honest.advice[27][0] == hashRef([1, 2, 1, 3])),
    assert(hw_honest.advice[39][0] == hashRef([2, 1, 3, 2, 1])),
    // Verify request row 46 has both lanes active (lane 0 HASH, lane 1 ALU):
    assert(hw_honest.advice[46][0] == CHIP_HASH),
    assert(hw_honest.advice[46][4] == hashRef([1, 2])),
    assert(hw_honest.advice[46][5] == CHIP_ALU),
    assert(hw_honest.advice[46][9] == aluRef(2, 1, 2)),
    // Verify request row 47 has lane 0 active (HASH [2, 1]) and lane 1 inactive (zero tuple):
    assert(hw_honest.advice[47][0] == CHIP_HASH),
    assert(hw_honest.advice[47][4] == hashRef([2, 1])),
    assert(hw_honest.advice[47][5] == CHIP_NONE),
  }

  run providerReuseAcceptedTest = all {
    assert(acceptsWithMutant("none", hw_provider_reused)),
    assert(refinesWith("none", hw_provider_reused)),
    // Both request row 46 lane 0 and request row 47 lane 0 request (HASH, 2, 1, 2, hashRef([1, 2]))
    assert(hw_provider_reused.advice[46][2] == 1 and hw_provider_reused.advice[46][3] == 2),
    assert(hw_provider_reused.advice[47][2] == 1 and hw_provider_reused.advice[47][3] == 2),
    assert(hw_provider_reused.advice[46][4] == hashRef([1, 2])),
    assert(hw_provider_reused.advice[47][4] == hashRef([1, 2])),
  }

  // ---------------------------------------------------------------- 2. Mutant Counterexample Tests (H1..H14)
  run mutantH1_skipAbsorbFirstTest = all {
    assert(acceptsWithMutant("skip_absorb_first", cex_H1_skip_absorb_first)),
    assert(not(acceptsWithMutant("none", cex_H1_skip_absorb_first))),
    assert(not(refinesWith("skip_absorb_first", cex_H1_skip_absorb_first))),
  }

  run mutantH2_skipAbsorbChainTest = all {
    assert(acceptsWithMutant("skip_absorb_chain", cex_H2_skip_absorb_chain)),
    assert(not(acceptsWithMutant("none", cex_H2_skip_absorb_chain))),
    assert(not(refinesWith("skip_absorb_chain", cex_H2_skip_absorb_chain))),
  }

  run mutantH3_tailAbsorbsIn1Test = all {
    assert(acceptsWithMutant("tail_absorbs_in1", cex_H3_tail_absorbs_in1)),
    assert(not(acceptsWithMutant("none", cex_H3_tail_absorbs_in1))),
    assert(not(refinesWith("tail_absorbs_in1", cex_H3_tail_absorbs_in1))),
  }

  run mutantH4_chainNoPrevTest = all {
    assert(acceptsWithMutant("chain_no_prev", cex_H4_chain_no_prev)),
    assert(not(acceptsWithMutant("none", cex_H4_chain_no_prev))),
    assert(not(refinesWith("chain_no_prev", cex_H4_chain_no_prev))),
  }

  run mutantH5_capacityAdviceTest = all {
    assert(acceptsWithMutant("capacity_advice", cex_H5_capacity_advice)),
    assert(not(acceptsWithMutant("none", cex_H5_capacity_advice))),
    assert(not(refinesWith("capacity_advice", cex_H5_capacity_advice))),
  }

  run mutantH6_lenWrongTest = all {
    assert(acceptsWithMutant("len_wrong", cex_H6_len_wrong)),
    assert(not(acceptsWithMutant("none", cex_H6_len_wrong))),
    assert(not(refinesWith("len_wrong", cex_H6_len_wrong))),
  }

  run mutantH7_keyUnpinnedTest = all {
    assert(acceptsWithMutant("key_unpinned", cex_H7_key_unpinned)),
    assert(not(acceptsWithMutant("none", cex_H7_key_unpinned))),
    assert(not(refinesWith("key_unpinned", cex_H7_key_unpinned))),
  }

  run mutantH8_chipUnpinnedTest = all {
    assert(acceptsWithMutant("chip_unpinned", cex_H8_chip_unpinned)),
    assert(not(acceptsWithMutant("none", cex_H8_chip_unpinned))),
    assert(not(refinesWith("chip_unpinned", cex_H8_chip_unpinned))),
  }

  run mutantH9_outNotCopiedTest = all {
    assert(acceptsWithMutant("out_not_copied", cex_H9_out_not_copied)),
    assert(not(acceptsWithMutant("none", cex_H9_out_not_copied))),
    assert(not(refinesWith("out_not_copied", cex_H9_out_not_copied))),
  }

  run mutantH10_outFromNonfinalTest = all {
    assert(acceptsWithMutant("out_from_nonfinal", cex_H10_out_from_nonfinal)),
    assert(not(acceptsWithMutant("none", cex_H10_out_from_nonfinal))),
    assert(not(refinesWith("out_from_nonfinal", cex_H10_out_from_nonfinal))),
  }

  run mutantH11_provOnChainRowTest = all {
    assert(acceptsWithMutant("prov_on_chain_row", cex_H11_prov_on_chain_row)),
    assert(not(acceptsWithMutant("none", cex_H11_prov_on_chain_row))),
    assert(not(refinesWith("prov_on_chain_row", cex_H11_prov_on_chain_row))),
  }

  run mutantH12_oneInputSpongeTest = all {
    assert(acceptsWithMutant("one_input_sponge", cex_H12_one_input_sponge)),
    assert(not(refinesWith("one_input_sponge", cex_H12_one_input_sponge))),
  }

  run mutantH13_roundGateDroppedTest = all {
    assert(acceptsWithMutant("round_gate_dropped", cex_H13_round_gate_dropped)),
    assert(not(acceptsWithMutant("none", cex_H13_round_gate_dropped))),
    assert(not(refinesWith("round_gate_dropped", cex_H13_round_gate_dropped))),
  }

  run mutantH14_provZeroNotZeroTest = all {
    assert(acceptsWithMutant("prov_zero_not_zero", cex_H14_prov_zero_not_zero)),
    assert(not(acceptsWithMutant("none", cex_H14_prov_zero_not_zero))),
    assert(not(refinesWith("prov_zero_not_zero", cex_H14_prov_zero_not_zero))),
  }

  // LEAD ADDITION: H8 in the HASH direction. Under `chip_unpinned` the ALU provider row 44 is
  // relabelled CHIP_HASH; the ALU request on row 46 lane 1 is dropped (zero tuple) and row 47
  // requests (HASH, 2, 1, 2, aluOut): a HASH request receives a non-sponge output. Under the sound
  // gates the ALU pin (chip == CHIP_ALU on q_alu rows) refuses it. This is the witness for C5's
  // premise that every other provider pins its own chip id.
  pure val cex_H8_hash_direction: Witness = {
    val base = buildHonestWitness("chip_unpinned")
    val aluOut = aluRef(2, 1, 2)
    val aluRow = base.advice[44].replaceAt(0, CHIP_HASH)
    val req0 = [CHIP_HASH, BUS_KEY, 1, 2, base.advice[0][4], 0, 0, 0, 0, 0]
    val req1 = [CHIP_HASH, BUS_KEY, 1, 2, aluOut, 0, 0, 0, 0, 0]
    { advice: base.advice.replaceAt(44, aluRow).replaceAt(46, req0).replaceAt(47, req1) }
  }

  run mutantH8_hashDirectionTest = all {
    assert(acceptsWithMutant("chip_unpinned", cex_H8_hash_direction)),
    assert(not(acceptsWithMutant("none", cex_H8_hash_direction))),
    assert(not(refinesWith("chip_unpinned", cex_H8_hash_direction))),
  }

  // ---------------------------------------------------------------- 3. Honest Witness Under Mutants Tests
  // For the 9 mutants that relax a constraint or leave an advice cell free (H3, H5, H7, H8, H9, H11, H12, H13, H14),
  // the honest witness built under the mutant is accepted AND refines, proving the violation is a real tamper attack.
  run honestUnderMutantsRefinesTest =
    assert(
      Set(
        "tail_absorbs_in1",
        "capacity_advice",
        "key_unpinned",
        "chip_unpinned",
        "out_not_copied",
        "prov_on_chain_row",
        "one_input_sponge",
        "round_gate_dropped",
        "prov_zero_not_zero"
      ).forall(mt => {
        val hw = buildHonestWitness(mt)
        acceptsWithMutant(mt, hw) and refinesWith(mt, hw)
      })
    )

  // For the 5 mutants that directly alter the honest gate equation or fixed layout (H1, H2, H4, H6, H10),
  // the witness computed according to the mutant's gate/layout is accepted by the mutant and fails refinement.
  run gateOrLayoutMutantsDivergeTest =
    assert(
      Set(
        "skip_absorb_first",
        "skip_absorb_chain",
        "chain_no_prev",
        "len_wrong",
        "out_from_nonfinal"
      ).forall(mt => {
        val mw = buildHonestWitness(mt)
        acceptsWithMutant(mt, mw) and not(refinesWith(mt, mw)) and not(acceptsWithMutant(mt, hw_honest))
      })
    )

  // ---------------------------------------------------------------- 4. Exhaustive Pure Domain Checks
  run exhaustiveSoundSingleCellTest =
    assert(exhaustiveSingleCellSound("none"))

  run exhaustiveSoundRequestTuplesTest =
    assert(exhaustiveRequestTuplesSound("none"))

  run exhaustiveMutantsCaughtTest = all {
    assert(exhaustiveMutantCaught("skip_absorb_first")),
    assert(exhaustiveMutantCaught("skip_absorb_chain")),
    assert(exhaustiveMutantCaught("tail_absorbs_in1")),
    assert(exhaustiveMutantCaught("chain_no_prev")),
    assert(exhaustiveMutantCaught("capacity_advice")),
    assert(exhaustiveMutantCaught("len_wrong")),
    assert(exhaustiveMutantCaught("key_unpinned")),
    assert(exhaustiveMutantCaught("chip_unpinned")),
    assert(exhaustiveMutantCaught("out_not_copied")),
    assert(exhaustiveMutantCaught("out_from_nonfinal")),
    assert(exhaustiveMutantCaught("prov_on_chain_row")),
    assert(exhaustiveMutantCaught("one_input_sponge")),
    assert(exhaustiveMutantCaught("round_gate_dropped")),
    assert(exhaustiveMutantCaught("prov_zero_not_zero")),
  }
}
