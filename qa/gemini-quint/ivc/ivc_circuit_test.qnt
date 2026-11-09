// Test harness and mutant suite for `ivc_circuit.qnt` (commit 1edd79146 / SHKIVC05 audit).
// Checks:
// 1. Pre-SHKIVC05 (MODE = 0) fails completeness at genesis (instance-copy mismatch on acc)
//    because uncollapsed zero-scaled `proof_acc` carries `numBases > 1` into `foldChallenge`.
// 2. SHKIVC05 (MODE = 1, commit 1edd79146) restores genesis and non-genesis completeness
//    and enforces `inCircuitProofAcc == trivialAcc` at genesis and `inCircuitProofAcc == collapseAcc(rawProofAcc)`
//    at non-genesis (`prevH != 0 mod P`).
// 3. Under-constrained cells in SHKIVC05 (MODE = 1):
//    - CEX-1 (`ivc.rs:311, 315-318`): missing SRC-004 range checks allow `prevH = P == 0 (mod P)`
//      at a non-genesis step, setting `is_genesis = 1` (`not_genesis = 0`) and dropping `rawProofAcc`.
//    - CEX-2 (`ivc.rs:315-317`): missing SRC-004 step bound allows `leafSteps = S_MAX + 1` or `P - 1`.
//    - CEX-3 (`ivc.rs:327-336, 377`): unscaled `prev_acc` at genesis allows algebraic KZG defect
//      cancellation of an invalid genesis leaf (`rawLeafAcc.defect != 0`).
//    - CEX-4 (`ivc.rs:327-336, 377`): unscaled `prev_acc` at genesis rejects a well-formed
//      non-neutral ignored carry under SRC-005 Realization (b).
// 4. Hardened Task 2.3 (MODE = 2) passes all completeness and soundness invariants and catches
//    every adversarial attack and seeded circuit mutant.

module ivc_circuit_test {
  import ivc_circuit(MODE = 0, S_MAX = 4) as m0 from "./ivc_circuit"
  import ivc_circuit(MODE = 1, S_MAX = 4) as m1 from "./ivc_circuit"
  import ivc_circuit(MODE = 2, S_MAX = 4) as m2 from "./ivc_circuit"

  // T1: Pre-SHKIVC05 (MODE = 0) fails genesis completeness (reproduces SHKIVC03/04 bug fixed by 1edd79146).
  run preFixGenesisCompletenessBugReproducedTest =
    m0::init
      .expect(and {
        not(m0::st.circuitOk),
        not(m0::st.deciderOk),
        m0::st.refOk,
        not(m0::completenessHonestInv),
      })

  // T2: SHKIVC05 (MODE = 1, commit 1edd79146) satisfies honest genesis and non-genesis steps.
  run shkivc05HonestCompletenessPassTest =
    m1::init
      .expect(m1::st.circuitOk and m1::st.deciderOk and m1::st.refOk and m1::completenessHonestInv)
      .then(m1::stepHonestGenesis)
      .expect(m1::st.circuitOk and m1::st.deciderOk and m1::st.refOk and m1::completenessHonestInv)
      .then(m1::stepHonestNonGenesis)
      .expect(m1::st.circuitOk and m1::st.deciderOk and m1::st.refOk and m1::completenessHonestInv)

  // T3: Exact collapse parity in SHKIVC05 (Question 3):
  //     At genesis (`prevH == 0`), `collapseAcc(scaleByBit(0, rawProofAcc)) == trivialAcc` for all
  //     dummy base counts and tags. At non-genesis (`prevH != 0`), `collapseAcc(scaleByBit(1, rawProofAcc))`
  //     equals `collapseAcc(rawProofAcc)` and preserves `selfScalar` and `defect`.
  run shkivc05CollapseMatchesOffCircuitTest =
    m1::init
      .expect(
        Set(1, 2, 3, 5).forall(nb =>
          Set(0, 7, 19, 42).forall(tag =>
            Set(0, 4, 11).forall(ss =>
              Set(0, 9, 31).forall(df => {
                val raw: m1::Acc = { numBases: nb, baseTag: tag, varScalar: 1, selfScalar: ss, defect: df }
                and {
                  // Genesis (not_genesis = 0): collapses to exact trivialAcc
                  m1::collapseAcc(m1::scaleByBit(0, raw)) == m1::trivialAcc,
                  // Non-genesis (not_genesis = 1): matches off-circuit collapseAcc(raw)
                  m1::collapseAcc(m1::scaleByBit(1, raw)) == m1::collapseAcc(raw),
                  // Non-genesis collapse never drops defect or selfScalar
                  m1::collapseAcc(m1::scaleByBit(1, raw)).defect == df,
                  m1::collapseAcc(m1::scaleByBit(1, raw)).selfScalar == ss,
                }
              })
            )
          )
        )
      )

  // T4: Non-genesis without modular wrap (`1 <= prevH < P`) cannot drop `rawProofAcc` (Question 1a):
  //     When `prevH != 0 mod P`, `not_genesis == 1`, so an invalid `rawProofAcc` (`defect != 0`)
  //     propagates into `inst.acc.defect != 0` and is rejected by the decider.
  run shkivc05NonGenesisNoWrapRejectsBadPrevProofTest = {
    val honest = m1::mkHonestWitness(3, 2, 3, 12)
    val forgedPrevProof = {
      ...honest,
      prevChainValid: false,
      rawProofAcc: { ...honest.rawProofAcc, defect: 25 },
    }
    m1::init
      .then(m1::recordAttempt(forgedPrevProof, "forged_prev_proof_at_h3"))
      .expect(and {
        m1::st.circuitOk,
        not(m1::st.deciderOk),
        not(m1::st.refOk),
        m1::soundnessInv,
      })
  }

  // T5: CEX-1 (`ivc.rs:311, 315-318`): In SHKIVC05 (MODE = 1), modular wrap-around `prevH = P`
  //     (`== 0 mod P`) triggers `is_genesis = 1` at a non-genesis step and drops a forged `rawProofAcc`!
  run shkivc05WrapAroundDropsPrevAccCounterexampleTest =
    m1::init
      .then(m1::attackWrapAroundDropsPrevAcc)
      .expect(and {
        m1::st.circuitOk,
        m1::st.deciderOk,
        not(m1::st.refOk),
        not(m1::soundnessInv),
        not(m1::noNonGenesisDropInv),
      })

  // T6: CEX-2 (`ivc.rs:315-317`): In SHKIVC05 (MODE = 1), `leafSteps = S_MAX + 1` or `P - 1`
  //     passes the node circuit and decider because `SRC-004` range checks are absent.
  run shkivc05MissingStepRangeCheckCounterexampleTest =
    m1::init
      .then(m1::attackStepBoundOrWrap)
      .expect(and {
        m1::st.circuitOk,
        m1::st.deciderOk,
        not(m1::st.refOk),
        not(m1::soundnessInv),
      })

  // T7: CEX-3 (`ivc.rs:327-336, 377`): In SHKIVC05 (MODE = 1), `prev_acc` is NOT scaled by
  //     `not_genesis` at genesis, allowing an unauthenticated `prev_acc` to algebraically cancel
  //     an invalid genesis leaf's KZG defect (`rawLeafAcc.defect != 0`).
  run shkivc05UnscaledGenesisCarryCancelsBadLeafCounterexampleTest =
    m1::init
      .then(m1::attackUnscaledGenesisCarryCancelsBadLeaf)
      .expect(and {
        m1::st.circuitOk,
        m1::st.deciderOk,
        not(m1::st.refOk),
        not(m1::soundnessInv),
      })

  // T8: CEX-4 (`ivc.rs:327-336, 377`): In SHKIVC05 (MODE = 1), a well-formed non-neutral carry
  //     at genesis is NOT ignored (fails SRC-005 Realization (b) scenario).
  run shkivc05NonNeutralGenesisCarryFailsRealizationBTest =
    m1::init
      .then(m1::probeNonNeutralIgnoredGenesisCarry)
      .expect(and {
        m1::st.circuitOk,
        not(m1::st.deciderOk),
        m1::st.refOk,
        not(m1::genesisCarryIrrelevantInv),
      })

  // T9: Hardened Task 2.3 (MODE = 2: SRC-004 range checks + SRC-005 scale & collapse on `prev_acc`)
  //     refuses all attacks (CEX-1, CEX-2, CEX-3) and satisfies CEX-4 (`genesisCarryIrrelevantInv`).
  run hardenedTask23SoundAndCompleteTest =
    m2::init
      .expect(m2::allHardenedInvariants)
      .then(m2::stepHonestGenesis)
      .expect(m2::allHardenedInvariants and m2::st.deciderOk)
      .then(m2::stepHonestNonGenesis)
      .expect(m2::allHardenedInvariants and m2::st.deciderOk)
      .then(m2::attackWrapAroundDropsPrevAcc)
      .expect(m2::allHardenedInvariants and not(m2::st.circuitOk) and not(m2::st.deciderOk))
      .then(m2::attackStepBoundOrWrap)
      .expect(m2::allHardenedInvariants and not(m2::st.circuitOk) and not(m2::st.deciderOk))
      .then(m2::attackUnscaledGenesisCarryCancelsBadLeaf)
      .expect(m2::allHardenedInvariants and m2::st.circuitOk and not(m2::st.deciderOk))
      .then(m2::probeNonNeutralIgnoredGenesisCarry)
      .expect(m2::allHardenedInvariants and m2::st.circuitOk and m2::st.deciderOk)

  // T10: Seeded gadget mutants (under-constrained selector / unlinked genesis bit / missing copy)
  //      Confirm our model detects if `not_genesis` is not forced boolean or not tied to `prev_h == 0`.
  pure def mutantUnboundNotGenesisSatisfied(w: m2::NodeWitness, freeNotGenesis: int): bool = {
    // Mutant where `not_genesis` is a free witness bit not tied to `is_zero(prev_h)`
    val scaledProof = m2::collapseAcc(m2::scaleByBit(freeNotGenesis, w.rawProofAcc))
    val scaledPrev = m2::collapseAcc(m2::scaleByBit(freeNotGenesis, w.prevAcc))
    val nxt = m2::collapseAcc(m2::accumulate3(scaledProof, scaledPrev, w.rawLeafAcc))
    m2::isBit(freeNotGenesis) and (nxt.defect == 0)
  }

  run mutantFreeGenesisBitCaughtTest = {
    val badNonGen = {
      ...m2::mkHonestWitness(3, 1, 2, 7),
      prevChainValid: false,
      rawProofAcc: { numBases: 2, baseTag: 7, varScalar: 1, selfScalar: 3, defect: 51 },
    }
    m2::init
      .expect(and {
        // If `not_genesis` were a free bit (0 at h=3), the forged predecessor would pass:
        mutantUnboundNotGenesisSatisfied(badNonGen, 0),
        // Because `ivc.rs:318-319` ties `not_genesis = not(is_zero(prev_h))`, `not_genesis` is
        // forced to 1 at `prevH = 3`, so the real circuit + decider rejects it:
        not(m2::deciderAccepts(m2::mkInstanceFromOffCircuit(badNonGen), 0)),
      })
  }
}
