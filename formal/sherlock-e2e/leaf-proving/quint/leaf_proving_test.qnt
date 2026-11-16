// Deterministic unit, scenario, refusal, and consistency tests for Leaf Proving
// Lane: e2e-leaf-proving

module leaf_proving_test {
  import leaf_proving(
    CHIP_SET_RSL       = 3,
    LFP_MAX_APP_AIRS   = 8,
    RSL_BUCKET_LOW     = 6,
    RSL_BUCKET_HIGH    = 9,
    MAX_APP_VK_COLS    = 30000,
    SHARD_STEP_CAP     = 4,
    MAX_SHARDS_PER_RUN = 3,
    SUITE_CALL_TOTAL   = 142,
    JUBJUB_COFACTOR    = 8,
    HAS_KECCAK_KIT     = true,
    HAS_SECP256K1_KIT  = true,
    MUTANT             = "none"
  ).* from "./leaf_proving"

  // LFP-001: Leaves and app shards are BabyBear Poseidon2 STARKs with no BLS12-381 commitment
  run lp_LFP_001_BabyBearStarkTest =
    init
      .then(admitProgram(2))
      .then(proveRslShard(2, 0, TamperNone))
      .expect(
        val slot = shardSlotId(2, 0)
        all {
          lastOp == "proveRslShard",
          appProofs.get(slot).starkCfg == CfgBabyBearPoseidon2,
          not(appProofs.get(slot).hasBls12381),
          appProofs.get(slot).stockVerified,
          leafProofs.get(slot).starkCfg == CfgBabyBearPoseidon2,
          not(leafProofs.get(slot).hasBls12381),
          leafProofs.get(slot).stockVerified,
          inv_LFP_001,
          inv,
        }
      )
      .then(proveRslShard(2, 1, TamperBadFieldCfg).fail())

  // LFP-002 & RSL-001: One app proof (chip set 3) and one leaf-layer proof per shard with universal N1 pre-hash
  run lp_LFP_002_RSL_001_UniversalPrehashTest =
    init
      .then(admitProgram(1))
      .then(admitProgram(4))
      .then(proveRslShard(1, 0, TamperNone))
      .then(proveRslShard(3, 0, TamperNone))
      // lead fix R-leaf-1: N1 takes the leaves after LeafProofsReady (run 3 has 2 shards)
      .then(proveRslShard(3, 1, TamperNone))
      .then(publishLeafProofsReady(1))
      .then(publishLeafProofsReady(3))
      .then(n1AcceptLeaf(1, shardSlotId(1, 0)))
      .then(n1AcceptLeaf(3, shardSlotId(3, 0)))
      .expect(
        val sA05 = shardSlotId(1, 0)
        val sD04 = shardSlotId(3, 0)
        all {
          // Per-program leaf-layer VKs differ (SPL-004), yet both produce universal N1 pre-hash
          programKeys.get(1).leafVkId != programKeys.get(4).leafVkId,
          appProofs.get(sA05).chipSetId == 3,
          appProofs.get(sD04).chipSetId == 3,
          appProofs.get(sD04).totalAppAirs == 9,
          leafProofs.get(sA05).n1PreHash == UNIVERSAL_N1_PREHASH,
          leafProofs.get(sD04).n1PreHash == UNIVERSAL_N1_PREHASH,
          universalKeys.internalRecVk == UNIVERSAL_INTERNAL_VK,
          inv_LFP_002,
          inv_RSL_001,
          inv_SPL_004,
          inv,
        }
      )
      .then(proveRslShard(3, 1, TamperPrehashDrift).fail())

  // LFP-003 & RSL-002: SHA-256 + Keccak-256 in one program (> 8 non-FP-1 AIRs) is refused with zero proofs
  run lp_LFP_003_RSL_002_DualHashRefusedTest =
    init
      .then(refuseProgramOverAirs(7))
      .expect(all {
        lastOp == "refuseProgramOverAirs",
        not(admittedProgs.contains(7)),
        appProofs.keys().size() == 0,
        leafProofs.keys().size() == 0,
        refusalLog.contains({ code: RefOverMaxAirs, targetId: 7 }),
        inv_LFP_003,
        inv_RSL_002,
        inv,
      })
      .then(admitProgram(7).fail())

  // LFP-004: App key at 30,000 columns accepted; app key at 30,001 columns refused
  run lp_LFP_004_AppKeyColumnBoundTest =
    init
      .then(admitProgram(5))
      .expect(programKeys.get(5).appVkCols == 30000 and inv_LFP_004)
      .then(refuseProgramOverCols(8))
      .expect(all {
        lastOp == "refuseProgramOverCols",
        not(admittedProgs.contains(8)),
        refusalLog.contains({ code: RefOverVkCols, targetId: 8 }),
        inv_LFP_004,
        inv,
      })
      .then(admitProgram(8).fail())

  // LFP-005: Fixed ordered AIR height vectors across shards; swapped AIR 1 / AIR 2 heights refused
  run lp_LFP_005_FixedShardHeightsAndSwapRefusedTest =
    init
      .then(admitProgram(2))
      .then(refuseRslShardTamper(2, 0, TamperSwappedHeights))
      .then(proveRslShard(2, 0, TamperNone))
      .then(proveRslShard(2, 1, TamperNone))
      .expect(
        val s0 = shardSlotId(2, 0)
        val s1 = shardSlotId(2, 1)
        all {
          refusalLog.contains({ code: RefBadShardHeights, targetId: s0 }),
          leafProofs.get(s0).airHeights == [16, 32, 8],
          leafProofs.get(s1).airHeights == [16, 32, 8],
          leafProofs.get(s0).airHeights == leafProofs.get(s1).airHeights,
          inv_LFP_005,
          inv,
        }
      )
      .then(proveRslShard(2, 2, TamperSwappedHeights).fail())

  // LFP-006 & LFP-007: Leaf exposes prog_d; N1 accepts matching prog_d and refuses leaf of another program
  run lp_LFP_006_LFP_007_ProgDBindingAndN1RefusalTest =
    init
      .then(admitProgram(1))
      .then(admitProgram(2))
      .then(refuseRslShardTamper(1, 0, TamperWrongProgDSlot))
      .then(proveRslShard(1, 0, TamperNone))
      .then(proveRslShard(2, 0, TamperNone))
      // Run 1 (progD 101) tries to merge a valid leaf of Run 2 (progD 102): N1 refuses!
      .then(n1RefuseWrongProgD(1, shardSlotId(2, 0)))
      .then(publishLeafProofsReady(1))
      .then(n1AcceptLeaf(1, shardSlotId(1, 0)))
      .expect(all {
        leafProofs.get(shardSlotId(1, 0)).pubProgD == 101,
        leafProofs.get(shardSlotId(2, 0)).pubProgD == 102,
        n1Accepted.get(1) == Set(shardSlotId(1, 0)),
        refusalLog.contains({ code: RefWrongProgDSlot, targetId: shardSlotId(1, 0) }),
        refusalLog.contains({ code: RefN1WrongProgD, targetId: 100 + shardSlotId(2, 0) }),
        inv_LFP_006,
        inv_LFP_007,
        inv,
      })
      .then(n1AcceptLeaf(1, shardSlotId(2, 0)).fail())

  // LFP-008 & LFP-009: Longer 3-shard run adds shards, keeps all keys unchanged, and publishes LeafProofsReady
  run lp_LFP_008_LFP_009_MultiShardAndReadyEventTest =
    init
      .then(admitProgram(2))
      .then(proveRslShard(2, 0, TamperNone))
      .expect(not(canPublishLeafProofsReady(currentSnap, 2)))
      .then(proveRslShard(2, 1, TamperNone))
      .expect(not(canPublishLeafProofsReady(currentSnap, 2)))
      .then(proveRslShard(2, 2, TamperNone))
      .then(publishLeafProofsReady(2))
      .expect(
        val ev = readyEvents.get(2)
        all {
          lastOp == "publishLeafProofsReady",
          ev.entries.length() == 3,
          ev.entries[0].proofSha256 == leafProofs.get(shardSlotId(2, 0)).proofSha256,
          ev.entries[1].proofSha256 == leafProofs.get(shardSlotId(2, 1)).proofSha256,
          ev.entries[2].proofSha256 == leafProofs.get(shardSlotId(2, 2)).proofSha256,
          ev.leafKeyRef == { keyRoot: 1002, chipSetId: 3, airCount: 6 },
          programKeys.get(2) == admittedKeySnap.get(2),
          universalKeys == GENESIS_UNIVERSAL_KEYS,
          inv_LFP_008,
          inv_LFP_009,
          inv,
        }
      )

  // LFP-009 negative: cannot publish LeafProofsReady before all shards of a multi-shard run finish
  run lp_LFP_009_PrematurePublishBlockedTest =
    init
      .then(admitProgram(2))
      .then(proveRslShard(2, 0, TamperNone))
      .then(publishLeafProofsReady(2).fail())

  // LFP-008 negative: rekeying on continuation shard of a multi-shard run is blocked
  run lp_LFP_008_RekeyOnLongRunBlockedTest =
    init
      .then(admitProgram(2))
      .then(proveRslShard(2, 0, TamperNone))
      .then(proveRslShard(2, 1, TamperRekeyLongRun).fail())

  // LFP-010, LFP-011, LFP-012: R_SL FP-1 outcome commitment, ERS-006 prog_d, and tape input_d bindings
  run lp_LFP_010_011_012_RslStatementBindingsTest =
    init
      .then(admitProgram(1))
      .then(admitProgram(2))
      .then(refuseRslShardTamper(2, 0, TamperBadFp1Outcome))
      .then(refuseRslShardTamper(2, 0, TamperZeroRslProgD))
      .then(refuseRslShardTamper(2, 0, TamperWrongTapeInput))
      .then(proveRslShard(1, 0, TamperNone))
      .then(proveRslShard(2, 0, TamperNone))
      .expect(
        val s1 = shardSlotId(1, 0)
        val s2 = shardSlotId(2, 0)
        all {
          refusalLog.contains({ code: RefBadFp1Outcome, targetId: s2 }),
          refusalLog.contains({ code: RefWrongProgDSlot, targetId: s2 }),
          refusalLog.contains({ code: RefWrongTapeInputD, targetId: s2 }),
          leafProofs.get(s1).pubProgD != leafProofs.get(s2).pubProgD,
          leafProofs.get(s1).pubProgD == 101,
          leafProofs.get(s2).pubProgD == 102,
          leafProofs.get(s2).consumedTapeD == leafProofs.get(s2).pubInputD,
          appProofs.get(s2).fp1OutcomeD == computeOutcomeDigest(702, [21, 22]),
          inv_LFP_010,
          inv_LFP_011,
          inv_LFP_012,
          inv,
        }
      )
      .then(proveRslShard(2, 1, TamperBadFp1Outcome).fail())

  // LFP-012 negative: shard consuming a different tape than statement input_d fails AIR check
  run lp_LFP_012_WrongTapeInputBlockedTest =
    init
      .then(admitProgram(2))
      .then(proveRslShard(2, 0, TamperWrongTapeInput).fail())

  // RSL-003: Suite prover records stock-verified call as SuiteProved and failed call as SuiteBlockedVerifierErr
  run lp_RSL_003_SuiteProverAndBlockedTest =
    init
      .then(admitProgram(1))
      .then(admitProgram(2))
      .expect(not(canRunSuiteCallProve(currentSnap, 1, true)))
      .then(proveRslShard(1, 0, TamperNone))
      .then(runSuiteCallProve(1, true))
      .then(recordSuiteCallBlocked(2))
      .expect(all {
        suiteTable.get(1).disposition == SuiteProved,
        suiteTable.get(1).appProofSlot == shardSlotId(1, 0),
        not(suiteTable.get(1).verifierErrorRecorded),
        suiteTable.get(2).disposition == SuiteBlockedVerifierErr,
        suiteTable.get(2).verifierErrorRecorded,
        inv_RSL_003,
        inv,
      })
      .then(runSuiteCallProve(2, false).fail())

  // RSL-004 & RSL-005: keccak256 + secp256k1 call (s02) proves in 9-AIR bucket with universal pre-hash
  run lp_RSL_004_005_KeccakAndSecpKitTest =
    init
      .then(admitProgram(5))
      .then(proveRslShard(4, 0, TamperNone))
      .expect(
        val s = shardSlotId(4, 0)
        all {
          appProofs.get(s).totalAppAirs == 9,
          appProofs.get(s).usesSecp256k1,
          appProofs.get(s).outcomeMatchesNative,
          leafProofs.get(s).n1PreHash == UNIVERSAL_N1_PREHASH,
          inv_RSL_004,
          inv_RSL_005,
          inv,
        }
      )

  // RSL-006, RSL-007, RSL-008: Jubjub subgroup check (0 new AIRs), SHA-256 alignment, canonical words
  run lp_RSL_006_007_008_JubjubShaCanonicalChecksTest =
    init
      .then(admitProgram(3))
      .then(admitProgram(4))
      .then(refuseRslShardTamper(6, 0, TamperJubjubOutSubgroup))
      .then(refuseRslShardTamper(3, 0, TamperMisalignedSha))
      .then(refuseRslShardTamper(3, 0, TamperNonCanonicalWord))
      .then(proveRslShard(6, 0, TamperNone))
      .then(proveRslShard(3, 0, TamperNone))
      .expect(
        val sJub = shardSlotId(6, 0)
        val sSha = shardSlotId(3, 0)
        all {
          refusalLog.contains({ code: RefBadJubjubRow, targetId: sJub }),
          refusalLog.contains({ code: RefMisalignedSha, targetId: sSha }),
          refusalLog.contains({ code: RefNonCanonicalWord, targetId: sSha }),
          appProofs.get(sJub).addedJubjubAirs == 0,
          appProofs.get(sJub).totalAppAirs == 6,
          appProofs.get(sJub).jubjubCheck == JubjubValidSubgroup,
          appProofs.get(sJub).jubjubCofactorChecked == 8,
          appProofs.get(sSha).shaInActive,
          appProofs.get(sSha).shaReqActive,
          appProofs.get(sSha).shaAligned,
          appProofs.get(sSha).wordsCanonical,
          inv_RSL_006,
          inv_RSL_007,
          inv_RSL_008,
          inv,
        }
      )
      .then(proveRslShard(3, 1, TamperNonCanonicalWord).fail())

  // Adopted INIT-001..004, INIT-007, INIT-008: Genesis h_pre = 0 constrained; non-genesis h_pre != 0 free
  run lp_INIT_GenesisVsNonGenesisTest =
    init
      .then(admitProgram(6))
      .then(refuseInterpretedShardTamper(5, 0, TamperInitWitnessGate))
      .then(proveInterpretedShard(5, 0, TamperNone))
      .then(proveInterpretedShard(5, 1, TamperNone))
      .expect(
        val s0 = shardSlotId(5, 0)
        val s1 = shardSlotId(5, 1)
        all {
          refusalLog.contains({ code: RefBadGenesisInit, targetId: s0 }),
          leafProofs.get(s0).hPre == 0,
          leafProofs.get(s0).preStateMatchesInit,
          leafProofs.get(s1).hPre == 4,
          // Non-genesis leaf has preStateMatchesInit == false and STILL verifies!
          not(leafProofs.get(s1).preStateMatchesInit),
          leafProofs.get(s1).prevPostMatchesPre,
          inv_INIT_001,
          inv_INIT_002,
          inv_INIT_003,
          inv_INIT_004,
          inv_INIT_007,
          inv_INIT_008,
          inv,
        }
      )

  // Adopted INIT-001/002 negative: tampered genesis witness at h_pre = 0 is blocked
  run lp_INIT_TamperedGenesisBlockedTest =
    init
      .then(admitProgram(6))
      .then(proveInterpretedShard(5, 0, TamperInitWitnessGate).fail())

  // Consistency checks for SPEC-CHECKED LPC-018 and exposed contradictions Q-leaf-proving-1 & Q-leaf-proving-2
  run lp_ConsistencyAndContradictionWitnessesTest =
    init
      .expect(all {
        consistency_LPC_018,
        // Q-leaf-proving-1: RSL_BUCKET_HIGH (9) > LFP_MAX_APP_AIRS (8)
        not(consistency_LFP_003_vs_RSL_002),
        // Q-leaf-proving-2: SHA-256 + Keccak-256 in one circuit (11 non-FP-1 AIRs) > LFP_MAX_APP_AIRS (8)
        not(consistency_RSL_004_007_vs_LFP_003),
      })
  // Regression R-leaf-1 (lead replay, traces/regressions/R-leaf-1.itf.json): N1 consumes the
  // run's leaves through LeafProofsReady (LFP-009, DDD.md sections 2 and 4), so it cannot accept
  // a leaf before the record exists. Fails on the lane's model, passes after the lead fix.
  run lp_R_leaf_1_N1WaitsForLeafProofsReadyTest =
    init
      .then(admitProgram(1))
      .then(proveRslShard(1, 0, TamperNone))
      .expect(not(canN1AcceptLeaf(currentSnap, 1, shardSlotId(1, 0))))
      .then(publishLeafProofsReady(1))
      .then(n1AcceptLeaf(1, shardSlotId(1, 0)))
      .expect(n1Accepted.get(1) == Set(shardSlotId(1, 0)))
}

// Separate test module witnessing Finding Q-leaf-proving-1 on literal LFP-003 (totalAppAirs <= 8):
// 9-AIR R_SL programs d04 (prog 4) and s02 (prog 5) are refused!
module leaf_proving_conflict_test {
  import leaf_proving(
    CHIP_SET_RSL       = 3,
    LFP_MAX_APP_AIRS   = 8,
    RSL_BUCKET_LOW     = 6,
    RSL_BUCKET_HIGH    = 9,
    MAX_APP_VK_COLS    = 30000,
    SHARD_STEP_CAP     = 4,
    MAX_SHARDS_PER_RUN = 3,
    SUITE_CALL_TOTAL   = 142,
    JUBJUB_COFACTOR    = 8,
    HAS_KECCAK_KIT     = true,
    HAS_SECP256K1_KIT  = true,
    MUTANT             = "lfp003_literal_8_vs_9"
  ).* from "./leaf_proving"

  run lp_LiteralLfp003RefusesNineAirProgramsTest =
    init
      .then(admitProgram(1))
      .then(refuseProgramOverAirs(4))
      .then(refuseProgramOverAirs(5))
      .expect(all {
        admittedProgs == Set(1),
        refusalLog.contains({ code: RefOverMaxAirs, targetId: 4 }),
        refusalLog.contains({ code: RefOverMaxAirs, targetId: 5 }),
      })
      .then(admitProgram(4).fail())
}

// Edge-minimal test module: verifies exact measured VK bound 16,346, SHARD_STEP_CAP = 1, and disabled kits
module leaf_proving_edge_minimal_test {
  import leaf_proving(
    CHIP_SET_RSL       = 3,
    LFP_MAX_APP_AIRS   = 8,
    RSL_BUCKET_LOW     = 6,
    RSL_BUCKET_HIGH    = 9,
    MAX_APP_VK_COLS    = 16346,
    SHARD_STEP_CAP     = 1,
    MAX_SHARDS_PER_RUN = 2,
    SUITE_CALL_TOTAL   = 5,
    JUBJUB_COFACTOR    = 8,
    HAS_KECCAK_KIT     = false,
    HAS_SECP256K1_KIT  = false,
    MUTANT             = "none"
  ).* from "./leaf_proving"

  run lp_EdgeMinimalExact16346BoundAndDisabledKitsTest =
    init
      // d04 has exactly 16,346 columns -> admitted at exact limit!
      .then(admitProgram(4))
      .then(proveRslShard(3, 0, TamperNone))
      .then(proveRslShard(3, 1, TamperNone))
      .then(publishLeafProofsReady(3))
      .expect(all {
        programKeys.get(4).appVkCols == 16346,
        readyEvents.get(3).entries.length() == 2,
        inv,
      })
      // s02 has 30,000 columns and requires keccak256 + secp256k1 -> blocked!
      .then(admitProgram(5).fail())
}
