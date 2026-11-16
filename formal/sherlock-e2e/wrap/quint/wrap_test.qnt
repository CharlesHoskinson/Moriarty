// -*- mode: Bluespec; -*-
// Deterministic scenario tests for bounded context Wrap (lane e2e-wrap).

module wrap_test {
  import wrap(
    MIN_WRAP_K = 19,
    MAX_WRAP_K = 21,
    MAX_ROWS_AT_K21 = 1782573,
    MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026,
    ANCHOR_SLOT_CONST = 15,
    ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15,
    WRAP_OUTPUT_FILE_COUNT = 3,
    LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7,
    MIN_LOG_RESERVATION_R = 8,
    MUTANT_ID = 0
  ).* from "./wrap"

  // WRP-001, WRP-004, WRP-009, LW-002..006:
  // Honest LfrProofReady call at k = 21 with T10m measured rows (1,685,845) and 1,325-byte VK
  // produces 1 PLONK/KZG/BLS12-381/Blake2b wrap proof and writes all 3 WrapOutput files.
  run wp_honestCpuWrapAndOutputTest =
    init
      .expect(inv and lastOp == "init")
      .then(doKeygenWrap(21, 1685845, 1325, 8, true, false))
      .expect(inv and wrp.eraKeyInitialized and wrp.cpuWrapKey.vkBytes == 1325)
      .then(doReceiveHonestLfrCall(1, 1, OrdMode, 1, 1))
      .expect(inv and wrp.knownCallIds.contains(1))
      .then(doProveWrapCpu(1, false))
      .expect(
        inv and
        lastOp == "proveWrapCpu" and
        wrp.proofRuns.exists(r =>
          r.callId == 1 and
          r.proofCountInRun == 1 and
          r.scheme == PlonkKzgBls12381 and
          r.transcript == Blake2bTranscript and
          r.libraryVerified and
          r.writtenFiles == Set("wrap.vk", "wrap.proof", "wrap.public.json")
        )
      )

  // WRP-002: k = 22 is refused before keygen with "never k = 22" and writes no key.
  run wp_refuseK22Test =
    init
      .expect(not(canKeygenWrap(wrp, 22, 1685845, 1325, 8, true, false)))
      .then(doRefuseKeygenAboveK21(22))
      .expect(
        inv and
        lastOp == "refuseKeygenAboveK21" and
        not(wrp.eraKeyInitialized) and
        wrp.lastRefusalReason == "never k = 22" and
        wrp.keygenAttempts.exists(a =>
          a.requestedK == 22 and
          a.stoppedBeforeKeygen and
          a.stopMessage == "never k = 22" and
          not(a.keyWritten)
        )
      )

  // WRP-003, DG-002: Rows above 1,782,573 at k = 21 stop before keygen; exact 1,782,573 fits.
  run wp_refuseRowGateAndBoundaryFitTest =
    init
      .expect(not(canKeygenWrap(wrp, 21, 1782574, 1325, 8, true, false)))
      .then(doRefuseKeygenRowGate(21, 1782574))
      .expect(
        inv and
        not(wrp.eraKeyInitialized) and
        wrp.keygenAttempts.exists(a => a.measuredRows == 1782574 and a.stoppedBeforeKeygen)
      )
      .then(doKeygenWrap(21, 1782573, 50000, 8, true, false))
      .expect(
        inv and
        wrp.eraKeyInitialized and
        wrp.cpuWrapKey.measuredRows == 1782573 and
        wrp.cpuWrapKey.vkBytes == 50000
      )

  // WRP-004, DG-003, MID-003: Serialized wrap VK > 50,000 bytes stops without writing wrap.vk.
  run wp_refuseVkOver50000BytesTest =
    init
      .expect(not(canKeygenWrap(wrp, 21, 1685845, 50001, 8, true, false)))
      .then(doRefuseVkTooLarge(50001))
      .expect(
        inv and
        lastOp == "refuseVkTooLarge" and
        not(wrp.eraKeyInitialized) and
        wrp.keygenAttempts.exists(a =>
          a.vkBytes == 50001 and a.stoppedWithoutWrapVk and not(a.keyWritten)
        )
      )

  // WRP-006, LW-012, ZK-003: Two production proofs of one call draw fresh OS CSPRNG blinders,
  // differ byte-for-byte, and both verify; seeded blinder in ProdBuild is refused.
  run wp_twoProdProofsDifferAndSeededProdRefusedTest =
    init
      .then(doKeygenWrap(21, 1685845, 1325, 8, true, false))
      .then(doReceiveHonestLfrCall(1, 1, OrdMode, 1, 1))
      .expect(not(canProveSeeded(wrp, 1, ProdBuild, true)))
      .then(doRefuseSeededBlinderInProd(1))
      .expect(inv and wrp.lastRefusalReason == "seeded_blinder_refused_in_prod")
      .then(doProveWrapCpu(1, false))
      .then(doProveWrapCpu(1, false))
      .expect(
        inv and
        wrp.proofRuns.exists(r1 =>
          wrp.proofRuns.exists(r2 =>
            r1.runId != r2.runId and
            r1.callId == 1 and r2.callId == 1 and
            r1.libraryVerified and r2.libraryVerified and
            r1.blinderSource == OsCsprng and r2.blinderSource == OsCsprng and
            r1.proofBytesTag != r2.proofBytesTag
          )
        )
      )

  // WRP-007, ANC-001: Prover-chosen anchor slots (0, 1, 3 != 15) and ordinary calls carrying
  // chunk anchor-write prologues are refused at decode step 5b; honest ChunkMode slot 15 succeeds.
  run wp_refuseForgedAnchorSlotAndAcceptChunkSlot15Test =
    init
      .then(doKeygenWrap(21, 1685845, 1325, 8, true, false))
      .then(doReceiveForgedLfrCall(3, 1))
      .expect(not(canProveCall(wrp, 3)))
      .then(doRefuseInvalidWrapWitness(3))
      .expect(inv and wrp.lastRefusalReason == "witness_refused")
      .then(doReceiveForgedLfrCall(4, 2))
      .expect(not(canProveCall(wrp, 4)))
      .then(doRefuseInvalidWrapWitness(4))
      .expect(inv and wrp.lastRefusalReason == "witness_refused")
      .then(doReceiveHonestLfrCall(2, 1, ChunkMode, 1, 2))
      .then(doProveWrapCpu(2, false))
      .expect(
        inv and
        wrp.proofRuns.exists(r =>
          r.callId == 2 and
          r.callMode == ChunkMode and
          r.lane6ReadKey == 15 and
          r.hasChunkWritePrologue and
          r.lane15WriteKey == 15 and
          r.aInTag == TagProgc and
          r.aOutTag == TagLock
        )
      )

  // WRP-008: Stale key mismatch against E6_EXPECT_VK stops with "the wrap key differs" and writes no file;
  // matching E6_EXPECT_VK writes all 3 files.
  run wp_expectVkMismatchRefusedAndMatchAcceptedTest =
    init
      .then(doKeygenWrap(21, 1685845, 1325, 8, true, false))
      .then(doReceiveHonestLfrCall(1, 1, OrdMode, 1, 1))
      .then(doConfigureExpectVk(true, 99))
      .expect(not(canProveCall(wrp, 1)))
      .then(doRefuseExpectVkMismatch(1))
      .expect(
        inv and
        wrp.lastRefusalReason == "the wrap key differs" and
        wrp.proofRuns.exists(r => r.stoppedOnVkMismatch and r.writtenFiles == Set())
      )
      .then(doConfigureExpectVk(true, 1))
      .expect(canProveCall(wrp, 1))
      .then(doProveWrapCpu(1, false))
      .expect(
        inv and
        wrp.proofRuns.exists(r =>
          r.libraryVerified and r.expectVkSet and r.expectedVkId == 1 and
          r.writtenFiles == WRAP_OUTPUT_FILES
        )
      )

  // WRP-010, MID-002: Two programs deploy with one byte-equal universal wrap key across OrdMode and ChunkMode.
  run wp_universalKeyAcrossProgramsAndModesTest =
    init
      .then(doKeygenWrap(21, 1685845, 1325, 8, true, false))
      .then(doDeployProgramLedger(1, false, false))
      .then(doDeployProgramLedger(2, false, false))
      .then(doReceiveHonestLfrCall(1, 1, OrdMode, 1, 1))
      .then(doReceiveHonestLfrCall(2, 2, ChunkMode, 2, 2))
      .then(doProveWrapCpu(1, false))
      .then(doProveWrapCpu(2, false))
      .expect(
        inv and
        wrp.eraWrapVkSet == Set(1) and
        wrp.deployments.forall(d => d.installedWriteVkId == 1 and d.readEntriesKeepNativeVk) and
        wrp.proofRuns.forall(r => r.vkId == 1)
      )

  // WRP-011, GPW-001, GPW-003, ZK-003:
  // GPU wrap shares CPU VK; under TestSeedBuild + GW_DET=1, CPU and GPU produce identical proof bytes;
  // when no GPU is usable, CPU fallback proves the call.
  run wp_gpuSharesKeyDetSeedMatchesAndCpuFallbackTest =
    init
      .then(doKeygenWrap(21, 1685845, 1325, 8, true, false))
      .then(doConfigureGpuProver(true, false))
      .expect(inv and wrp.gpuConfigured and wrp.gpuWrapVkId == wrp.cpuWrapKey.vkId)
      .then(doReceiveHonestLfrCall(1, 1, OrdMode, 1, 1))
      .then(doProveSeededDetWrap(1, CpuDevice, TestSeedBuild, 42))
      .then(doProveSeededDetWrap(1, GpuDevice, TestSeedBuild, 42))
      .expect(
        inv and
        wrp.proofRuns.exists(rCpu =>
          wrp.proofRuns.exists(rGpu =>
            rCpu.device == CpuDevice and rGpu.device == GpuDevice and
            rCpu.vkId == rGpu.vkId and
            rCpu.proofBytesTag == rGpu.proofBytesTag
          )
        )
      )
      .then(doConfigureGpuProver(false, false))
      .expect(inv and not(wrp.usableGpuPresent) and canProveCall(wrp, 1))
      .then(doProveWrapCpu(1, false))
      .expect(inv)

  // WRP-012, LW-014, LW-016:
  // Missing LW-004 check or all-NOP L_cached benchmark cannot qualify T25 or deploy T25;
  // all 5 real LW passes close qualification and enable T25 deploy; LW-014 short log capacity is refused.
  run wp_t25QualificationAll5LwChecksAndLw014Lw016Test =
    init
      .expect(not(canKeygenWrap(wrp, 21, 1685845, 1325, 3, true, false)))
      .then(doRefuseKeygenLw014(3, true, false))
      .expect(inv and wrp.lastRefusalReason == "lw014_keygen_refused")
      .then(doKeygenWrap(21, 1685845, 1325, 8, true, false))
      .then(doRecordT25LwPasses(false, false))
      .expect(not(canCloseT25Qualification(wrp)))
      .then(doRefuseIncompleteT25Qualification(false))
      .expect(inv and wrp.lastRefusalReason == "t25_qualification_incomplete" and not(wrp.t25Qualified))
      .then(doRefuseIncompleteT25Qualification(true))
      .expect(inv and wrp.lastRefusalReason == "all_nop_bench_qualifies_no_gate" and not(wrp.t25Qualified))
      .then(doRecordT25LwPasses(true, false))
      .expect(canCloseT25Qualification(wrp))
      .then(doCloseT25Qualification)
      .expect(inv and wrp.t25Qualified and REQUIRED_LW_CHECKS.subseteq(wrp.t25PassedChecks))
      .then(doDeployProgramLedger(1, true, false))
      .expect(inv and wrp.deployments.exists(d => d.progId == 1 and d.usesT25Key))

  // RKY-003, RKY-004, Findings Q-wrap-1 & Q-wrap-3 pure witnesses:
  // Rekey refuses row/VK overflow (writes lever list) and completes at k = 21 with 1,685,845 rows & 1,325 bytes.
  run wp_rekeyBoundsAndSpecGapWitnessesTest =
    init
      .expect(
        consistency_constants and
        gap_q_wrap_1_k20_row_overflow and
        gap_q_wrap_3_ord_lane15_zero and
        not(canCompleteRekey(wrp, 21, 1782574, 1325)) and
        not(canCompleteRekey(wrp, 21, 1685845, 50001))
      )
      .then(doRefuseRekeyBoundsOverflow(1782574, 1325))
      .expect(inv and wrp.rekeyLeverListWritten and not(wrp.rekeyCompleted))
      .then(doCompleteRekeyWrapMeasurement(1685845, 1325))
      .expect(
        inv and
        wrp.rekeyCompleted and
        wrp.rekeyMeasuredK == 21 and
        wrp.rekeyMeasuredRows == 1685845 and
        wrp.rekeyMeasuredVkBytes == 1325
      )
}
