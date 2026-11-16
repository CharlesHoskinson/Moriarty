// Requirement-level `.expect(...)` scenario tests for Sherlock E2E Key Management
// Lane: e2e-key-management

module key_management_test {
  import key_management(
    NODE_ROWS_REQUIRED = 7,
    FOREIGN_ROWS_COUNT = 2,
    KEYDAG_IDX = 3,
    RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6,
    REQUIRED_GATE_COUNT = 12,
    SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000,
    MIN_GRINDABLE_DECIBITS = 1010,
    MAX_WRAP_K = 21,
    MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025,
    DEFAULT_GRIND_DECIBITS = 1030,
    DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 0
  ).* from "./key_management"

  // Constant consistency check across spec bounds
  run km_ConsistencyConstantsTest =
    init.expect(consistency_constants and inv)

  // UKS-001: Two suite programs (`t02 increment` and `d04 withdraw`) settle under the
  //          same universal key manifest sha256 and two different `key_root` values.
  run km_Uks001TwoSuiteProgramsOneKeysetTwoRootsTest =
    init
      .then(startAndRunKeygen(7))
      .then(runGatesAndD3Pass)
      .then(distributeKeysToBox(BoxMuninn))
      .then(buildProgramKeyRoot(ProgT02Increment, 4))
      .then(buildProgramKeyRoot(ProgD04Withdraw, 4))
      .then(startProofOnBox(BoxMuninn, ProgT02Increment))
      .then(verifyAndSettleFullTree(BoxMuninn, ProgT02Increment, false, false))
      .then(startProofOnBox(BoxMuninn, ProgD04Withdraw))
      .then(verifyAndSettlePathC(BoxMuninn, ProgD04Withdraw))
      .expect(all {
        km.settledProofs.size() == 2,
        km.programKeyRoots.get(ProgT02Increment).keyRootDigest
          != km.programKeyRoots.get(ProgD04Withdraw).keyRootDigest,
        inv_UKS_001,
        inv,
      })

  // UKS-002: `keys_d` has 7 node rows (rows 4..6 holding NW, NW2, N3 over NW2) and foreign rows.
  run km_Uks002SevenNodeRowsMergeWork4To6Test =
    init
      .then(startAndRunKeygen(7))
      .expect(all {
        km.keysD.built,
        km.keysD.nodeRowCount == 7,
        km.keysD.row4NwKey > 0,
        km.keysD.row5Nw2Key > 0,
        km.keysD.row6N3Nw2Key > 0,
        Set(km.keysD.row4NwKey, km.keysD.row5Nw2Key, km.keysD.row6N3Nw2Key).size() == 3,
        km.keysD.foreignRowCount == 2,
        inv_UKS_002,
      })

  // UKS-003: R_SL chip set 3 leaf index differs from `KEYDAG_IDX` (3) and chip sets 0..2;
  //          Path C opens DAG leaf and R_SL key leaf at distinct positions.
  run km_Uks003RslLeafDistinctAndPathCOpensTest =
    init
      .then(startAndRunKeygen(7))
      .then(runGatesAndD3Pass)
      .then(distributeKeysToBox(BoxMuninn))
      .expect(not(canBuildProgramKeyRoot(km, ProgT02Increment, 3, KEYDAG_IDX, 0)))
      .then(refuseCollidingRslLeafIdx(3))
      .expect(km.lastRefusal == RefusalCollidingRslLeafIdx((3, 3)))
      .then(buildProgramKeyRoot(ProgT02Increment, 4))
      .then(startProofOnBox(BoxMuninn, ProgT02Increment))
      .then(verifyAndSettlePathC(BoxMuninn, ProgT02Increment))
      .expect(all {
        km.programKeyRoots.get(ProgT02Increment).rslLeafIdx == 4,
        km.programKeyRoots.get(ProgT02Increment).keydagIdx == 3,
        inv_UKS_003,
        inv_SPL_005,
      })

  // UKS-004: Rekey writes universal key manifest with universal file digests, `keys_d`,
  //          `LIST.md` sha256, and 3 repo commits; changing `LIST.md` flags second keygen needed.
  run km_Uks004ManifestFieldsAndListChangeSecondKeygenTest =
    init
      .then(startAndRunKeygen(7))
      .expect(all {
        km.manifest.written,
        km.manifest.internalRecSha == km.universalKeySet.internalRecKeyId,
        km.manifest.lKeySha == km.universalKeySet.lKeyId,
        km.manifest.lfrKeySha == km.universalKeySet.lfrKeyId,
        km.manifest.wrapKeySha == km.universalKeySet.wrapKeyId,
        km.manifest.keysDDigest == km.keysD.keysDDigest,
        km.manifest.listSha256 == km.lastKeygenListSha,
        km.manifest.openvmCommit == 11,
        km.manifest.starkBackendCommit == 22,
        km.manifest.moriartyCommit == 33,
        not(km.manifest.holdsPerProgramKey),
        not(km.secondKeygenNeeded),
        inv_UKS_004,
      })
      .then(appendRowAfterKeygen)
      .expect(all {
        km.currentListSha != km.manifest.listSha256,
        km.secondKeygenNeeded,
        inv_UKS_004,
      })

  // UKS-005: Box (`huginn`) holding old T10m manifest refuses to start a proof and names both digests.
  run km_Uks005HuginnOldManifestRefusedTest =
    init
      .then(startAndRunKeygen(7))
      .then(runGatesAndD3Pass)
      .then(distributeKeysToBox(BoxMuninn))
      .then(buildProgramKeyRoot(ProgT02Increment, 4))
      .expect(not(canStartProofOnBox(km, BoxHuginn, ProgT02Increment, 0)))
      .then(refuseStartProofManifestMismatch(BoxHuginn))
      .expect(all {
        not(km.boxes.get(BoxHuginn).proofRunning),
        km.lastRefusal == RefusalManifestMismatch((BoxHuginn, T10M_OLD_MANIFEST_SHA, km.manifest.manifestSha256)),
        inv_UKS_005,
      })

  // UKS-006: Failing `x7` gate prevents `PhaseRekeyDone` and blocks key transfer to boxes.
  run km_Uks006FailingX7BlocksDoneAndBoxTransferTest =
    init
      .then(startAndRunKeygen(7))
      .then(runGatesFailX7)
      .expect(all {
        km.rekeyPhase == PhaseRekeyGateFailed,
        km.gatesRerunCount == 12,
        km.gatesPassedCount == 11,
        km.lastRefusal == RefusalRekeyGateFailed(7),
        ALL_BOXES.forall(b => not(canDistributeKeysToBox(km, b))),
        km.boxes.get(BoxMuninn).installedManifestSha == T10M_OLD_MANIFEST_SHA,
        inv_UKS_006,
      })

  // UKS-007: Manifest status reads `prototype` until the T12 security gate (G1..G5 + FP-8) passes.
  run km_Uks007PrototypeUntilT12GatePassesTest =
    init
      .then(startAndRunKeygen(7))
      .then(runGatesAndD3Pass)
      .expect(all {
        km.manifest.statusMark == MarkPrototype,
        not(securityGateAllPassed(km)),
        inv_UKS_007,
      })
      .then(passSecurityGateAllChecks)
      .expect(all {
        securityGateAllPassed(km),
        km.manifest.statusMark == MarkQualified,
        inv_UKS_007,
      })

  // UKS-008: Setup mode makes, changes and selects no key, shape, `plan_d` or proof-entering value.
  run km_Uks008SetupModeChangesNoKeyOrPlanDTest =
    init
      .then(startAndRunKeygen(7))
      .then(runGatesAndD3Pass)
      .then(distributeKeysToBox(BoxOdin))
      .then(runSetupModeClean(BoxOdin))
      .expect(all {
        km.boxes.get(BoxOdin).setupCompleted,
        km.boxes.get(BoxOdin).installedManifestSha == km.manifest.manifestSha256,
        km.boxes.get(BoxOdin).keyFileCount == 4,
        km.boxes.get(BoxOdin).selectedShape == 1,
        km.boxes.get(BoxOdin).planD == 700,
        km.boxes.get(BoxOdin).proofEnteringValue == 42,
        inv_UKS_008,
      })
      .then(abortSetupOnKeyTamper(BoxOdin))
      .expect(all {
        not(km.boxes.get(BoxOdin).setupCompleted),
        km.lastRefusal == RefusalSetupKeyTamper(BoxOdin),
        inv_UKS_008,
      })

  // UKS-009: Non-descendant keygen tree refuses to start and makes no key.
  run km_Uks009NonDescendantTreeRefusedTest =
    init
      .then(checkoutNonDescendantTree)
      .expect(not(canStartKeygen(km, 0)))
      .then(refuseRekeyNonDescendantTree)
      .expect(all {
        km.keygenEpoch == 0,
        not(km.manifest.written),
        km.lastRefusal == RefusalNonDescendantTree,
        inv_UKS_009,
      })

  // UKS-010: Missing FP-3 row in `LIST.md` refuses to start and names the missing row.
  run km_Uks010MissingFp3RowRefusedTest =
    init
      .then(dropListRowBeforeRekey(RowFp3))
      .expect(not(canStartKeygen(km, 0)))
      .then(refuseRekeyMissingRows)
      .expect(all {
        km.keygenEpoch == 0,
        km.lastRefusal == RefusalMissingListRows(Set(RowFp3)),
        inv_UKS_010,
      })

  // RKY-001: One rekey from one list; adding a row after keygen marks second keygen needed,
  //          and running the second keygen clears it under the updated list sha256.
  run km_Rky001OneRekeyAndSecondRekeyAfterAppendTest =
    init
      .then(startAndRunKeygen(7))
      .expect(km.keygenEpoch == 1 and not(km.secondKeygenNeeded))
      .then(appendRowAfterKeygen)
      .expect(km.secondKeygenNeeded and inv_RKY_001)
      .then(startAndRunKeygen(7))
      .expect(all {
        km.keygenEpoch == 2,
        not(km.secondKeygenNeeded),
        km.manifest.listSha256 == km.currentListSha,
        inv_RKY_001,
      })

  // RKY-002: Uncommitted row (`RowAnchorSlot15` / T12-SLOT15) refuses to start rekey.
  run km_Rky002UncommittedSlot15RowRefusedTest =
    init
      .then(markRowUncommitted(RowAnchorSlot15))
      .expect(not(canStartKeygen(km, 0)))
      .then(refuseRekeyUncommittedRow(RowAnchorSlot15))
      .expect(all {
        km.keygenEpoch == 0,
        km.lastRefusal == RefusalUncommittedRow(RowAnchorSlot15),
        inv_RKY_002,
      })

  // RKY-005: Grindable term at 100.9 bits (`1009 < 1010`) fails the D3 security gate.
  run km_Rky005Grindable100Point9BitsFailsGateTest =
    init
      .then(startAndRunKeygen(7))
      .then(runD3FailGrindable100Point9)
      .expect(all {
        km.rekeyPhase == PhaseRekeyGateFailed,
        km.d3MinGrindableDeciBits == 1009,
        km.lastRefusal == RefusalD3SecurityFailed((1025, 1009)),
        ALL_BOXES.forall(b => not(canDistributeKeysToBox(km, b))),
        inv_RKY_005,
        inv,
      })

  // Adopted SPL-005: Unlisted child key is refused; listed child key settles and forwards `key_root`.
  run km_Spl005FullTreeAndPathCInclusionAndRefusalTest =
    init
      .then(startAndRunKeygen(7))
      .then(runGatesAndD3Pass)
      .then(distributeKeysToBox(BoxMuninn))
      .then(buildProgramKeyRoot(ProgT02Increment, 4))
      .then(startProofOnBox(BoxMuninn, ProgT02Increment))
      .expect(not(canVerifyFullTree(km, BoxMuninn, ProgT02Increment, false, true, 0)))
      .then(refuseUnlistedChildKey(ProgT02Increment))
      .expect(km.lastRefusal == RefusalUnlistedChildKey((ProgT02Increment, 9999)))
      .then(verifyAndSettleFullTree(BoxMuninn, ProgT02Increment, false, false))
      .expect(km.settledProofs.size() == 1 and inv_SPL_005)

  // Adopted SQG-011: Freeze request before G1..G5 + FP-8 is refused; succeeds after gate passes.
  run km_Sqg011RefuseFreezeBeforeGateAllowAfterTest =
    init
      .then(startAndRunKeygen(7))
      .then(runGatesAndD3Pass)
      .expect(not(canFreezeUniversalKeys(km, 0)))
      .then(refuseFreezeBeforeGate)
      .expect(all {
        not(km.universalKeysFrozen),
        km.lastRefusal == RefusalFreezeBeforeSecurityGate,
        inv_SQG_011,
      })
      .then(passSecurityGateAllChecks)
      .then(freezeUniversalKeys)
      .expect(all {
        km.universalKeysFrozen,
        inv_SQG_011,
        inv,
      })
}
