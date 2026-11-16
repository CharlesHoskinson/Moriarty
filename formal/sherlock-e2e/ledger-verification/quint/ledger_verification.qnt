// -*- mode: Bluespec; -*-
// Independent Quint specification for bounded context Ledger Verification (lane e2e-ledger-verification).
// Sources:
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-ledger-verifier/spec.md
//     (LVR-001..008)
//   - openspec/changes/sherlock-processor-final-20261111/specs/sherlock-l-and-wrap/spec.md
//     (adopted LW-017)
//   - openspec/changes/sherlock-processor-final-20261111/specs/sherlock-midnight-side/spec.md
//     (adopted MID-006)
//   - openspec/changes/sherlock-e2e-prototype-20261115/architecture/DDD.md
//     (Ledger Verification rows in sections 1..4 and 6..8)

module ledger_verification {
  // =========================================================================
  // Constants (pinned versions, counts, and vector bounds from the EARS specs)
  // =========================================================================

  // LVR-002: 4 midnight-ledger crates resolved to rev a2b30128.
  const PINNED_LEDGER_CRATE_COUNT: int
  // LVR-004: 7 vectors with one changed public input.
  const CHANGED_LANE_CONTROL_COUNT: int
  // LVR-004: 3 proofs with one flipped byte.
  const FLIPPED_BYTE_CONTROL_COUNT: int
  // LVR-004 Scenario / LW-003 / MID-003: public-input call vector length (1,026 in main).
  const CALL_VECTOR_LEN: int
  // LVR-001, LVR-002: abstract identifier for stock ledger 9.1.0.1 git rev a2b30128ca4b660b73a068e29b56b48ede8c9a74.
  const PINNED_LEDGER_REV_ID: int
  // MID-006: abstract identifier for Preview node build "2.1.101-bbd616b1".
  const PINNED_PREVIEW_NODE_BUILD_ID: int
  // LVR-006 Scenario: node custom error code 115 corresponding to native "Invalid proof".
  const NODE_INVALID_PROOF_ERR_CODE: int
  // Mutant selector: 0 = honest specification; 1..11 = individual guard mutants.
  const MUTANT_ID: int

  // =========================================================================
  // Mutant identifiers
  // =========================================================================

  pure val MUT_NONE: int = 0
  pure val MUT_LVR001_UNPINNED_VERIFIER: int = 1
  pure val MUT_LVR002_ALLOW_PATH_OVERRIDE: int = 2
  pure val MUT_LVR003_ACCEPT_G4_KEY: int = 3
  pure val MUT_LVR004_SKIP_TAMPER_CONTROLS: int = 4
  pure val MUT_LVR005_ZERO_EXIT_ACCEPTED_CTRL: int = 5
  pure val MUT_LVR005_ZERO_EXIT_REFUSED_HONEST: int = 6
  pure val MUT_LVR006_SUBMIT_WITHOUT_NATIVE: int = 7
  pure val MUT_LVR007_SUBMIT_NATIVELY_REFUSED: int = 8
  pure val MUT_LVR008_ALLOW_SECOND_GENESIS: int = 9
  pure val MUT_LW017_REPORT_G4_ACCEPTED: int = 10
  pure val MUT_MID006_PREVIEW_PIN_DRIFT: int = 11

  // =========================================================================
  // Domain types
  // =========================================================================

  type CrateSourceKind =
    | GitPinnedSource
    | PatchCratesIoSource
    | LocalPathOverride
    | WrongGitRevSource

  type VkFraming =
    | MidnightVkZkStdLib
    | G4LaneGateKey
    | MalformedVkFraming

  type CrateDepRecord = {
    crateName: str,
    sourceKind: CrateSourceKind,
    revId: int
  }

  type WrapBundleInput = {
    bundleId: int,
    vkFraming: VkFraming,
    publicInputCount: int,
    honestProofValid: bool,
    forgedControlPassesVerifier: bool
  }

  type LedgerCheckRunRecord = {
    checkId: int,
    bundleId: int,
    verifierCrateRevId: int,
    usedStockVerifierVerify: bool,
    usedMidnightVkDeserialize: bool,
    vkFraming: VkFraming,
    keyReadAccepted: bool,
    honestProofAccepted: bool,
    changedLanesChecked: Set[int],
    changedLaneRefusedCount: int,
    flippedByteCheckedCount: int,
    flippedByteRefusedCount: int,
    anyTamperControlAccepted: bool,
    exitCode: int,
    verdictOk: bool,
    g4ReportedAccepted: bool
  }

  type FolderChainRecord = {
    folderId: int,
    connectedOnce: bool,
    initialGenesisHash: int,
    recordedGenesisHash: int,
    recordedNodeBuildId: int,
    observedNodeBuilds: Set[int],
    chainMismatchStopped: bool,
    lastConnectStopReason: str
  }

  type SettleCallRecord = {
    callId: int,
    folderId: int,
    isHonestCall: bool,
    proofValidOnLedger9: bool,
    nativeValidatorRan: bool,
    nativeValidatorAccepted: bool,
    nativeRefusalMsg: str,
    equivalentNodeCustomError: int,
    markedRefusedInState: bool,
    needsFreshProveOnRerun: bool,
    submittedToNode: bool
  }

  type LvrState = {
    crateDeps: Set[CrateDepRecord],
    stdlibUnmodified: bool,
    manifestGateChecked: bool,
    manifestGatePassed: bool,
    lastManifestError: str,
    bundles: int -> WrapBundleInput,
    knownBundleIds: Set[int],
    nextCheckId: int,
    checkRuns: Set[LedgerCheckRunRecord],
    folders: int -> FolderChainRecord,
    knownFolderIds: Set[int],
    calls: int -> SettleCallRecord,
    knownCallIds: Set[int],
    previewQualWritten: bool,
    previewReportedNodeBuildId: int,
    previewPinnedNodeBuildId: int,
    lastRunnerError: str
  }

  // =========================================================================
  // Observable state variables
  // =========================================================================

  var lvr: LvrState
  var lastOp: str

  // =========================================================================
  // Pure constants & helper functions
  // =========================================================================

  pure val REQUIRED_LEDGER_CRATES: Set[str] = Set(
    "midnight-transient-crypto",
    "midnight-serialize",
    "midnight-base-crypto",
    "midnight-storage-core"
  )

  // LVR-004 Scenario: lane 0, lane 10 (anchor), lanes 20 and 34 (claim_pub),
  // lane 39 (outcome_pub), the middle lane, and the last lane.
  pure def namedControlLanes(vecLen: int): Set[int] =
    Set(0, 10, 20, 34, 39, vecLen / 2, vecLen - 1)

  pure val SIX_LANES_MISSING_39: Set[int] =
    Set(0, 10, 20, 34, 513, 1025)

  pure def mkPinnedCrateDeps(usePatchCratesIo: bool): Set[CrateDepRecord] = {
    val tcSrc = if (usePatchCratesIo) PatchCratesIoSource else GitPinnedSource
    Set(
      { crateName: "midnight-transient-crypto", sourceKind: tcSrc, revId: PINNED_LEDGER_REV_ID },
      { crateName: "midnight-serialize", sourceKind: GitPinnedSource, revId: PINNED_LEDGER_REV_ID },
      { crateName: "midnight-base-crypto", sourceKind: GitPinnedSource, revId: PINNED_LEDGER_REV_ID },
      { crateName: "midnight-storage-core", sourceKind: GitPinnedSource, revId: PINNED_LEDGER_REV_ID }
    )
  }

  pure def mkOverriddenCrateDeps(useLocalPath: bool): Set[CrateDepRecord] = {
    val badSrc = if (useLocalPath) LocalPathOverride else WrongGitRevSource
    val badRev = if (useLocalPath) PINNED_LEDGER_REV_ID else 9999
    Set(
      { crateName: "midnight-transient-crypto", sourceKind: badSrc, revId: badRev },
      { crateName: "midnight-serialize", sourceKind: GitPinnedSource, revId: PINNED_LEDGER_REV_ID },
      { crateName: "midnight-base-crypto", sourceKind: GitPinnedSource, revId: PINNED_LEDGER_REV_ID },
      { crateName: "midnight-storage-core", sourceKind: GitPinnedSource, revId: PINNED_LEDGER_REV_ID }
    )
  }

  pure def isValidCrateDep(d: CrateDepRecord): bool =
    (d.sourceKind == GitPinnedSource or d.sourceKind == PatchCratesIoSource) and
    d.revId == PINNED_LEDGER_REV_ID

  pure def allFourLedgerCratesPinned(deps: Set[CrateDepRecord]): bool =
    deps.size() == PINNED_LEDGER_CRATE_COUNT and
    REQUIRED_LEDGER_CRATES.forall(cName =>
      deps.exists(d => d.crateName == cName and isValidCrateDep(d))
    )

  pure def emptyFolderRecord(fId: int): FolderChainRecord = {
    folderId: fId,
    connectedOnce: false,
    initialGenesisHash: 0,
    recordedGenesisHash: 0,
    recordedNodeBuildId: 0,
    observedNodeBuilds: Set(),
    chainMismatchStopped: false,
    lastConnectStopReason: "none"
  }

  // Guard predicates (exposed for deterministic tests in ledger_verification_test.qnt without .fail()).
  pure def canPassManifestGate(deps: Set[CrateDepRecord], stdlibStock: bool): bool =
    if (MUTANT_ID == MUT_LVR002_ALLOW_PATH_OVERRIDE)
      deps.size() == PINNED_LEDGER_CRATE_COUNT
    else
      (allFourLedgerCratesPinned(deps) and stdlibStock)

  pure def canRunLedgerCheck(s: LvrState, bId: int): bool =
    s.knownBundleIds.contains(bId) and
    s.nextCheckId <= 6 and
    ((MUTANT_ID == MUT_LVR001_UNPINNED_VERIFIER) or s.manifestGatePassed)

  pure def canConnectFolder(s: LvrState, fId: int, genesisHash: int, nodeBuildId: int): bool = {
    val folder = s.folders.get(fId)
    val sameGenesis =
      (MUTANT_ID == MUT_LVR008_ALLOW_SECOND_GENESIS) or
      not(folder.connectedOnce) or
      (folder.recordedGenesisHash == genesisHash)
    s.knownFolderIds.contains(fId) and genesisHash > 0 and nodeBuildId > 0 and sameGenesis
  }

  pure def canSubmitCall(s: LvrState, cId: int): bool =
    s.knownCallIds.contains(cId) and {
      val c = s.calls.get(cId)
      val f = s.folders.get(c.folderId)
      val nativeRanOk =
        (MUTANT_ID == MUT_LVR006_SUBMIT_WITHOUT_NATIVE) or c.nativeValidatorRan
      val nativeAcceptedOk =
        (MUTANT_ID == MUT_LVR006_SUBMIT_WITHOUT_NATIVE and not(c.nativeValidatorRan)) or
        (MUTANT_ID == MUT_LVR007_SUBMIT_NATIVELY_REFUSED) or
        c.nativeValidatorAccepted
      f.connectedOnce and
        not(f.chainMismatchStopped) and
        not(c.submittedToNode) and
        nativeRanOk and
        nativeAcceptedOk
    }

  pure def canWritePreviewQualification(reportedBuild: int, pinnedBuild: int): bool =
    if (MUTANT_ID == MUT_MID006_PREVIEW_PIN_DRIFT)
      (reportedBuild > 0 and pinnedBuild > 0)
    else
      (reportedBuild == PINNED_PREVIEW_NODE_BUILD_ID and
        pinnedBuild == PINNED_PREVIEW_NODE_BUILD_ID)

  // =========================================================================
  // Pure consistency & spec-gap witness checks
  // =========================================================================

  val consistency_constants: bool = {
    val lanes = namedControlLanes(CALL_VECTOR_LEN)
    PINNED_LEDGER_CRATE_COUNT == 4 and
    REQUIRED_LEDGER_CRATES.size() == PINNED_LEDGER_CRATE_COUNT and
    CHANGED_LANE_CONTROL_COUNT == 7 and
    FLIPPED_BYTE_CONTROL_COUNT == 3 and
    CALL_VECTOR_LEN >= 42 and
    CALL_VECTOR_LEN <= 65536 and
    lanes.size() == CHANGED_LANE_CONTROL_COUNT and
    lanes.forall(idx => idx >= 0 and idx < CALL_VECTOR_LEN) and
    PINNED_LEDGER_REV_ID == 9101 and
    PINNED_PREVIEW_NODE_BUILD_ID == 21101 and
    NODE_INVALID_PROOF_ERR_CODE == 115
  }

  // Finding Q-ledger-verification-2 witness: LVR-002 checks only the 4 named midnight-ledger crates,
  // so a modified midnight-zk-stdlib (stdlibStock == false) still satisfies allFourLedgerCratesPinned.
  pure val gap_q_lvr_2_stdlib_patch_passes_4crate_check: bool = {
    val deps = mkPinnedCrateDeps(true)
    val stdlibStock = false
    allFourLedgerCratesPinned(deps) and not(stdlibStock)
  }

  // Finding Q-ledger-verification-3 witness: for public input counts 40 or 41 (and 68, 69, 78, 79),
  // the 7 named lanes of LVR-004 Scenario collide into fewer than 7 distinct indices
  // (at N=40 both middle==20 and last==39 collide => size 5; at N=41 middle==20 => size 6).
  pure val gap_q_lvr_3_short_vector_lane_collision: bool =
    namedControlLanes(40).size() == 5 and
    namedControlLanes(41).size() == 6 and
    namedControlLanes(68).size() == 6 and
    namedControlLanes(78).size() == 6 and
    namedControlLanes(42).size() == 7

  // =========================================================================
  // Initialization
  // =========================================================================

  action init = all {
    lvr' = {
      crateDeps: mkPinnedCrateDeps(false),
      stdlibUnmodified: true,
      manifestGateChecked: false,
      manifestGatePassed: false,
      lastManifestError: "none",
      bundles: Map(),
      knownBundleIds: Set(),
      nextCheckId: 1,
      checkRuns: Set(),
      folders: Map(1 -> emptyFolderRecord(1), 2 -> emptyFolderRecord(2)),
      knownFolderIds: Set(1, 2),
      calls: Map(),
      knownCallIds: Set(),
      previewQualWritten: false,
      previewReportedNodeBuildId: 0,
      previewPinnedNodeBuildId: PINNED_PREVIEW_NODE_BUILD_ID,
      lastRunnerError: "none"
    },
    lastOp' = "init"
  }

  // =========================================================================
  // Parameterized actions (used by step and deterministic tests)
  // =========================================================================

  // Implements LVR-001, LVR-002, LW-017: run Cargo metadata manifest gate on pinned rev a2b30128.
  action doCheckManifestPass(usePatchCratesIo: bool, mutOverride: bool): bool = {
    val candidateDeps =
      if (MUTANT_ID == MUT_LVR002_ALLOW_PATH_OVERRIDE and mutOverride)
        mkOverriddenCrateDeps(true)
      else
        mkPinnedCrateDeps(usePatchCratesIo)
    all {
      canPassManifestGate(candidateDeps, true),
      lvr' = {
        ...lvr,
        crateDeps: candidateDeps,
        stdlibUnmodified: true,
        manifestGateChecked: true,
        manifestGatePassed: true,
        lastManifestError: "none"
      },
      lastOp' = "checkManifestPass"
    }
  }

  // Implements LVR-002: refuse a local path override or wrong git rev on any of the 4 ledger crates.
  action doRefuseManifestOverride(useLocalPath: bool): bool = {
    val badDeps = mkOverriddenCrateDeps(useLocalPath)
    all {
      not(allFourLedgerCratesPinned(badDeps)),
      lvr' = {
        ...lvr,
        crateDeps: badDeps,
        manifestGateChecked: true,
        manifestGatePassed: false,
        lastManifestError: if (useLocalPath) "local_path_override_refused" else "wrong_git_rev_refused"
      },
      lastOp' = "refuseManifestOverride"
    }
  }

  // Register a WrapOutput bundle to be checked by sherlock-wrap-ledger-check.
  // Implements LVR-001, LVR-003, LVR-004, LVR-005, LW-017.
  action doStageWrapBundle(
    bId: int,
    framing: VkFraming,
    honestOk: bool,
    forgedCtrlAccepts: bool
  ): bool = all {
    not(lvr.knownBundleIds.contains(bId)),
    val bRec: WrapBundleInput = {
      bundleId: bId,
      vkFraming: framing,
      publicInputCount: CALL_VECTOR_LEN,
      honestProofValid: honestOk,
      forgedControlPassesVerifier: forgedCtrlAccepts
    }
    lvr' = {
      ...lvr,
      bundles: lvr.bundles.put(bId, bRec),
      knownBundleIds: lvr.knownBundleIds.union(Set(bId))
    },
    lastOp' = "stageWrapBundle"
  }

  // Implements LVR-001, LVR-003, LVR-004, LVR-005, LW-017:
  // Execute sherlock-wrap-ledger-check on a staged WrapOutput bundle.
  action doRunLedgerCheckTool(bId: int, mutTrig: bool): bool = all {
    canRunLedgerCheck(lvr, bId),
    val b = lvr.bundles.get(bId)
    val usedRev =
      if (MUTANT_ID == MUT_LVR001_UNPINNED_VERIFIER and mutTrig) 9999
      else PINNED_LEDGER_REV_ID
    val usedStock =
      not(MUTANT_ID == MUT_LVR001_UNPINNED_VERIFIER and mutTrig)
    val keyAccepted =
      if (MUTANT_ID == MUT_LVR003_ACCEPT_G4_KEY and b.vkFraming == G4LaneGateKey) true
      else (b.vkFraming == MidnightVkZkStdLib)
    val lanesChecked =
      if (not(keyAccepted)) Set()
      else if (MUTANT_ID == MUT_LVR004_SKIP_TAMPER_CONTROLS and mutTrig)
        namedControlLanes(CALL_VECTOR_LEN).exclude(Set(39))
      else
        namedControlLanes(CALL_VECTOR_LEN)
    val byteControlsChecked =
      if (not(keyAccepted)) 0
      else if (MUTANT_ID == MUT_LVR004_SKIP_TAMPER_CONTROLS and mutTrig) 2
      else FLIPPED_BYTE_CONTROL_COUNT
    val honestAccepted = keyAccepted and b.honestProofValid
    val ctrlAccepted = keyAccepted and b.forgedControlPassesVerifier
    val laneRefusedCnt = lanesChecked.size()
    val byteRefusedCnt =
      if (ctrlAccepted and byteControlsChecked > 0) byteControlsChecked - 1
      else byteControlsChecked
    val allChecksClean =
      keyAccepted and
      honestAccepted and
      not(ctrlAccepted) and
      laneRefusedCnt == CHANGED_LANE_CONTROL_COUNT and
      byteRefusedCnt == FLIPPED_BYTE_CONTROL_COUNT
    val computedExit =
      if (allChecksClean) 0
      else if (MUTANT_ID == MUT_LVR005_ZERO_EXIT_ACCEPTED_CTRL and ctrlAccepted) 0
      else if (MUTANT_ID == MUT_LVR005_ZERO_EXIT_REFUSED_HONEST and keyAccepted and not(honestAccepted)) 0
      else 1
    val g4Claimed =
      (MUTANT_ID == MUT_LW017_REPORT_G4_ACCEPTED and b.vkFraming == G4LaneGateKey and mutTrig)
    val runRec: LedgerCheckRunRecord = {
      checkId: lvr.nextCheckId,
      bundleId: bId,
      verifierCrateRevId: usedRev,
      usedStockVerifierVerify: usedStock,
      usedMidnightVkDeserialize: true,
      vkFraming: b.vkFraming,
      keyReadAccepted: keyAccepted,
      honestProofAccepted: honestAccepted,
      changedLanesChecked: lanesChecked,
      changedLaneRefusedCount: laneRefusedCnt,
      flippedByteCheckedCount: byteControlsChecked,
      flippedByteRefusedCount: byteRefusedCnt,
      anyTamperControlAccepted: ctrlAccepted,
      exitCode: computedExit,
      verdictOk: (computedExit == 0),
      g4ReportedAccepted: g4Claimed
    }
    lvr' = {
      ...lvr,
      nextCheckId: lvr.nextCheckId + 1,
      checkRuns: lvr.checkRuns.union(Set(runRec))
    },
    lastOp' = "runLedgerCheckTool"
  }

  // Implements LVR-008: connect settle runner to a node for a state folder;
  // records (genesisHash, nodeVersion) on first connect and preserves chain identity on reconnect.
  action doConnectSettleNode(fId: int, genesisHash: int, nodeBuildId: int): bool = all {
    canConnectFolder(lvr, fId, genesisHash, nodeBuildId),
    val prev = lvr.folders.get(fId)
    val firstInitGenesis = if (prev.connectedOnce) prev.initialGenesisHash else genesisHash
    val recGenesis =
      if (prev.connectedOnce and MUTANT_ID != MUT_LVR008_ALLOW_SECOND_GENESIS)
        prev.recordedGenesisHash
      else
        genesisHash
    val recBuild = if (prev.connectedOnce) prev.recordedNodeBuildId else nodeBuildId
    val updatedFolder: FolderChainRecord = {
      ...prev,
      connectedOnce: true,
      initialGenesisHash: firstInitGenesis,
      recordedGenesisHash: recGenesis,
      recordedNodeBuildId: recBuild,
      observedNodeBuilds: prev.observedNodeBuilds.union(Set(nodeBuildId)),
      chainMismatchStopped: false,
      lastConnectStopReason: "none"
    }
    lvr' = {
      ...lvr,
      folders: lvr.folders.put(fId, updatedFolder)
    },
    lastOp' = "connectSettleNode"
  }

  // Implements LVR-008 Scenario: stop with "same local chain as the deploy" when connecting to another genesis.
  action doRefuseSecondGenesisForFolder(fId: int, otherGenesisHash: int): bool = all {
    lvr.knownFolderIds.contains(fId),
    val prev = lvr.folders.get(fId)
    all {
      prev.connectedOnce,
      otherGenesisHash > 0,
      otherGenesisHash != prev.recordedGenesisHash,
      val updatedFolder: FolderChainRecord = {
        ...prev,
        chainMismatchStopped: true,
        lastConnectStopReason: "same local chain as the deploy"
      }
      lvr' = {
        ...lvr,
        folders: lvr.folders.put(fId, updatedFolder),
        lastRunnerError: "same local chain as the deploy"
      },
      lastOp' = "refuseSecondGenesisForFolder"
    }
  }

  // Implements LVR-006, LVR-007: build a ledger call in a state folder ready for pre-submit validation.
  action doBuildLedgerCall(cId: int, fId: int, honestCall: bool, validOnLedger9: bool): bool = all {
    lvr.knownFolderIds.contains(fId),
    not(lvr.knownCallIds.contains(cId)),
    val cRec: SettleCallRecord = {
      callId: cId,
      folderId: fId,
      isHonestCall: honestCall,
      proofValidOnLedger9: validOnLedger9,
      nativeValidatorRan: false,
      nativeValidatorAccepted: false,
      nativeRefusalMsg: "none",
      equivalentNodeCustomError: 0,
      markedRefusedInState: false,
      needsFreshProveOnRerun: false,
      submittedToNode: false
    }
    lvr' = {
      ...lvr,
      calls: lvr.calls.put(cId, cRec),
      knownCallIds: lvr.knownCallIds.union(Set(cId))
    },
    lastOp' = "buildLedgerCall"
  }

  // Implements LVR-006, LVR-007: run native ledger 9.1.0.1 validator before submit;
  // on refusal, records "failed to verify proof: Invalid proof" (equivalent to node custom error 115),
  // marks the call refused in run state, and submits nothing.
  action doRunNativeValidator(cId: int): bool = all {
    lvr.knownCallIds.contains(cId),
    val prev = lvr.calls.get(cId)
    all {
      not(prev.nativeValidatorRan),
      val accepted = prev.proofValidOnLedger9
      val updatedCall: SettleCallRecord = {
        ...prev,
        nativeValidatorRan: true,
        nativeValidatorAccepted: accepted,
        nativeRefusalMsg:
          if (accepted) "none" else "failed to verify proof: Invalid proof",
        equivalentNodeCustomError:
          if (accepted) 0 else NODE_INVALID_PROOF_ERR_CODE,
        markedRefusedInState: not(accepted),
        needsFreshProveOnRerun: not(accepted)
      }
      lvr' = {
        ...lvr,
        calls: lvr.calls.put(cId, updatedCall),
        lastRunnerError:
          if (accepted) lvr.lastRunnerError else "failed to verify proof: Invalid proof"
      },
      lastOp' = "runNativeValidator"
    }
  }

  // Implements LVR-006, LVR-007: submit a natively validated and accepted ledger call to the node.
  action doSubmitValidatedCall(cId: int): bool = all {
    canSubmitCall(lvr, cId),
    val prev = lvr.calls.get(cId)
    val updatedCall: SettleCallRecord = {
      ...prev,
      submittedToNode: true
    }
    lvr' = {
      ...lvr,
      calls: lvr.calls.put(cId, updatedCall)
    },
    lastOp' = "submitValidatedCall"
  }

  // Implements MID-006: write Preview qualification record matching reported node build and Preview pin.
  action doRecordPreviewQualification(reportedBuildId: int, pinnedBuildId: int): bool = all {
    canWritePreviewQualification(reportedBuildId, pinnedBuildId),
    lvr' = {
      ...lvr,
      previewQualWritten: true,
      previewReportedNodeBuildId: reportedBuildId,
      previewPinnedNodeBuildId: pinnedBuildId
    },
    lastOp' = "recordPreviewQualification"
  }

  // Implements MID-006 Scenario: refuse Preview qualification when reported node build differs from Preview pin.
  action doRefusePreviewPinMismatch(reportedBuildId: int): bool = all {
    reportedBuildId != PINNED_PREVIEW_NODE_BUILD_ID,
    lvr' = {
      ...lvr,
      lastRunnerError: "preview_node_build_pin_mismatch"
    },
    lastOp' = "refusePreviewPinMismatch"
  }

  // =========================================================================
  // Nondeterministic step relation
  // =========================================================================

  action step = any {
    // Manifest dependency gate (LVR-001, LVR-002, LW-017)
    {
      nondet usePatch = Set(true, false).oneOf()
      nondet mutOv = Set(true, false).oneOf()
      doCheckManifestPass(usePatch, mutOv)
    },
    {
      nondet localPath = Set(true, false).oneOf()
      doRefuseManifestOverride(localPath)
    },

    // Staged WrapOutput bundles & sherlock-wrap-ledger-check runs (LVR-001, LVR-003, LVR-004, LVR-005, LW-017)
    doStageWrapBundle(1, MidnightVkZkStdLib, true, false),
    {
      nondet badKind = Set(1, 2, 3).oneOf()
      val fr = if (badKind == 1) G4LaneGateKey else MidnightVkZkStdLib
      val hOk = if (badKind == 2) false else true
      val fCtrl = (badKind == 3)
      doStageWrapBundle(2, fr, hOk, fCtrl)
    },
    {
      nondet bId = Set(1, 2).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doRunLedgerCheckTool(bId, mutTrig)
    },

    // Settle runner node connection & genesis identity guard (LVR-008)
    {
      nondet fId = Set(1, 2).oneOf()
      val genPool =
        if (MUTANT_ID == MUT_LVR008_ALLOW_SECOND_GENESIS) Set(100, 200)
        else Set(100)
      nondet gHash = genPool.oneOf()
      nondet bld = Set(PINNED_PREVIEW_NODE_BUILD_ID, 21102).oneOf()
      doConnectSettleNode(fId, gHash, bld)
    },
    {
      nondet fId = Set(1, 2).oneOf()
      doRefuseSecondGenesisForFolder(fId, 200)
    },

    // Settle runner native validation & submission (LVR-006, LVR-007)
    doBuildLedgerCall(1, 1, true, true),
    {
      nondet honestFlag = Set(true, false).oneOf()
      doBuildLedgerCall(2, 1, honestFlag, false)
    },
    {
      nondet cId = Set(1, 2).oneOf()
      doRunNativeValidator(cId)
    },
    {
      nondet cId = Set(1, 2).oneOf()
      doSubmitValidatedCall(cId)
    },

    // Preview node pin check (MID-006)
    {
      val repPool =
        if (MUTANT_ID == MUT_MID006_PREVIEW_PIN_DRIFT)
          Set(PINNED_PREVIEW_NODE_BUILD_ID, 21199)
        else
          Set(PINNED_PREVIEW_NODE_BUILD_ID)
      nondet repBuild = repPool.oneOf()
      doRecordPreviewQualification(repBuild, PINNED_PREVIEW_NODE_BUILD_ID)
    },
    doRefusePreviewPinMismatch(21199)
  }

  // =========================================================================
  // Safety invariants (one inv_<ID> per expressible requirement)
  // =========================================================================

  // LVR-001: Every sherlock-wrap-ledger-check run verifies with VerifierKey::verify of stock
  // midnight-transient-crypto at ledger tag 9.1.0.1 (rev a2b30128) after passing the manifest gate.
  val inv_LVR_001: bool =
    lvr.checkRuns.forall(r =>
      r.usedStockVerifierVerify and
      r.verifierCrateRevId == PINNED_LEDGER_REV_ID
    )

  // LVR-002: Passing the manifest gate requires all 4 midnight-ledger crates to resolve to rev a2b30128
  // via git or [patch.crates-io] (never a local path override or another git revision).
  val inv_LVR_002: bool =
    lvr.manifestGatePassed implies allFourLedgerCratesPinned(lvr.crateDeps)

  // LVR-003: Every ledger check run loads wrap.vk through VerifierKey::deserialize in MidnightVK framing,
  // refusing any G4 lane-gate or malformed key framing.
  val inv_LVR_003: bool =
    lvr.checkRuns.forall(r =>
      r.usedMidnightVkDeserialize and
      ((r.vkFraming != MidnightVkZkStdLib) implies not(r.keyReadAccepted))
    )

  // LVR-004: Whenever a wrap key is loaded and verified, the ledger check tool also checks all 7 named
  // changed-public-input vectors and all 3 flipped-byte proof controls.
  val inv_LVR_004: bool =
    lvr.checkRuns.forall(r =>
      r.keyReadAccepted implies (
        r.changedLanesChecked == namedControlLanes(CALL_VECTOR_LEN) and
        r.changedLanesChecked.size() == CHANGED_LANE_CONTROL_COUNT and
        r.flippedByteCheckedCount == FLIPPED_BYTE_CONTROL_COUNT
      )
    )

  // LVR-005: If the ledger verifier accepts any tamper control or refuses the honest proof (or refuses wrap.vk),
  // then the ledger check tool exits with a nonzero status (verdictOk == false).
  val inv_LVR_005: bool =
    lvr.checkRuns.forall(r =>
      ((r.anyTamperControlAccepted or
        not(r.honestProofAccepted) or
        not(r.keyReadAccepted) or
        r.changedLaneRefusedCount != CHANGED_LANE_CONTROL_COUNT or
        r.flippedByteRefusedCount != FLIPPED_BYTE_CONTROL_COUNT) implies (
        r.exitCode != 0 and not(r.verdictOk)
      )) and
      ((r.exitCode == 0) == r.verdictOk)
    )

  // LVR-006: Every submitted ledger call ran the native 9.1.0.1 validator before submit, and any invalid
  // call checked natively produces "failed to verify proof: Invalid proof" (node custom error 115).
  val inv_LVR_006: bool =
    lvr.knownCallIds.forall(cId => {
      val c = lvr.calls.get(cId)
      (c.submittedToNode implies c.nativeValidatorRan) and
      ((c.nativeValidatorRan and not(c.proofValidOnLedger9)) implies (
        not(c.nativeValidatorAccepted) and
        c.nativeRefusalMsg == "failed to verify proof: Invalid proof" and
        c.equivalentNodeCustomError == NODE_INVALID_PROOF_ERR_CODE
      ))
    })

  // LVR-007: If the native ledger validator refuses a call, the settle runner submits nothing
  // and marks the call refused in the run state (requiring a fresh prove on rerun).
  val inv_LVR_007: bool =
    lvr.knownCallIds.forall(cId => {
      val c = lvr.calls.get(cId)
      (c.nativeValidatorRan and not(c.nativeValidatorAccepted)) implies (
        not(c.submittedToNode) and
        c.markedRefusedInState and
        c.needsFreshProveOnRerun
      )
    })

  // LVR-008: First connect for a state folder records nonzero genesis hash and node version,
  // and no state folder silently switches to a second genesis hash.
  val inv_LVR_008: bool =
    lvr.knownFolderIds.forall(fId => {
      val f = lvr.folders.get(fId)
      f.connectedOnce implies (
        f.recordedGenesisHash > 0 and
        f.recordedNodeBuildId > 0 and
        f.recordedGenesisHash == f.initialGenesisHash
      )
    })

  // Adopted LW-017: Only ZkStdLib keys are ledger-readable; no G4 figure is ever reported as ledger-accepted.
  val inv_LW_017: bool =
    lvr.checkRuns.forall(r =>
      not(r.g4ReportedAccepted) and
      ((r.vkFraming == G4LaneGateKey) implies not(r.keyReadAccepted))
    )

  // Adopted MID-006: Written Preview qualification record names the reported node build equal to the Preview pin.
  val inv_MID_006: bool =
    lvr.previewQualWritten implies (
      lvr.previewReportedNodeBuildId == lvr.previewPinnedNodeBuildId and
      lvr.previewPinnedNodeBuildId == PINNED_PREVIEW_NODE_BUILD_ID
    )

  // Master safety invariant.
  val inv: bool =
    consistency_constants and
    inv_LVR_001 and
    inv_LVR_002 and
    inv_LVR_003 and
    inv_LVR_004 and
    inv_LVR_005 and
    inv_LVR_006 and
    inv_LVR_007 and
    inv_LVR_008 and
    inv_LW_017 and
    inv_MID_006

  // =========================================================================
  // Reachability witnesses (negated goals — MUST be violated under ledger_verification_main)
  // =========================================================================

  // Witness 1: Manifest gate passes and sherlock-wrap-ledger-check accepts an honest ZkStdLib bundle
  // with all 7 changed-lane and 3 flipped-byte tamper controls refused (exit 0).
  val reach_honest_ledger_check_pass: bool = not(
    lvr.manifestGatePassed and
    lvr.checkRuns.exists(r =>
      r.verdictOk and
      r.exitCode == 0 and
      r.changedLaneRefusedCount == CHANGED_LANE_CONTROL_COUNT and
      r.flippedByteRefusedCount == FLIPPED_BYTE_CONTROL_COUNT
    )
  )

  // Witness 2: Nonzero exit occurs on a G4 / failed-control / invalid-honest bundle (LVR-003, LVR-005).
  val reach_ledger_check_nonzero_exit: bool = not(
    lvr.checkRuns.exists(r => r.exitCode != 0 and not(r.verdictOk))
  )

  // Witness 3: Manifest gate refuses a local path override or wrong git rev (LVR-002).
  val reach_manifest_override_refused: bool = not(
    lvr.manifestGateChecked and
    not(lvr.manifestGatePassed) and
    lvr.lastManifestError != "none"
  )

  // Witness 4: Settle runner connects a state folder, runs native validator, and submits an honest call (LVR-006, LVR-008).
  val reach_native_validate_and_submit: bool = not(
    lvr.knownCallIds.exists(cId => {
      val c = lvr.calls.get(cId)
      c.nativeValidatorRan and c.nativeValidatorAccepted and c.submittedToNode
    })
  )

  // Witness 5: Native validator refuses an invalid/tampered call with custom error 115 equivalence
  // and marks it refused without submitting (LVR-006 Scenario, LVR-007).
  val reach_native_refusal_no_submit_115: bool = not(
    lvr.knownCallIds.exists(cId => {
      val c = lvr.calls.get(cId)
      c.nativeValidatorRan and
      not(c.nativeValidatorAccepted) and
      c.equivalentNodeCustomError == NODE_INVALID_PROOF_ERR_CODE and
      c.markedRefusedInState and
      not(c.submittedToNode)
    })
  )

  // Witness 6: Second genesis hash for one state folder stops with "same local chain as the deploy" (LVR-008)
  // and Preview qualification matches the pinned node build (MID-006).
  val reach_second_genesis_refused_and_preview_pinned: bool = not(
    lvr.knownFolderIds.exists(fId =>
      lvr.folders.get(fId).chainMismatchStopped and
      lvr.folders.get(fId).lastConnectStopReason == "same local chain as the deploy"
    ) and
    lvr.previewQualWritten
  )
}

// ===========================================================================
// Concrete instances: main, edge instances, and guard mutants
// ===========================================================================

module ledger_verification_main {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4,
    CHANGED_LANE_CONTROL_COUNT = 7,
    FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026,
    PINNED_LEDGER_REV_ID = 9101,
    PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115,
    MUTANT_ID = 0
  ).*
}

// Smallest legal public-input vector length (42) at which all 7 named lanes
// {0, 10, 20, 34, 39, middle=21, last=41} of LVR-004 Scenario are in-bounds and distinct.
module ledger_verification_edge_min {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4,
    CHANGED_LANE_CONTROL_COUNT = 7,
    FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 42,
    PINNED_LEDGER_REV_ID = 9101,
    PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115,
    MUTANT_ID = 0
  ).*
}

// Exact MID-003 ledger public-input ceiling (65,536 public inputs: middle=32,768, last=65,535).
module ledger_verification_edge_at_limit {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4,
    CHANGED_LANE_CONTROL_COUNT = 7,
    FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 65536,
    PINNED_LEDGER_REV_ID = 9101,
    PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115,
    MUTANT_ID = 0
  ).*
}

// Mutants (each removes or weakens one refusal/binding guard and must break its named invariant)

module ledger_verification_m_lvr001_unpinned_verifier {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 1
  ).*
}

module ledger_verification_m_lvr002_allow_path_override {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 2
  ).*
}

module ledger_verification_m_lvr003_accept_g4_key {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 3
  ).*
}

module ledger_verification_m_lvr004_skip_tamper_controls {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 4
  ).*
}

module ledger_verification_m_lvr005_zero_exit_accepted_ctrl {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 5
  ).*
}

module ledger_verification_m_lvr005_zero_exit_refused_honest {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 6
  ).*
}

module ledger_verification_m_lvr006_submit_without_native {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 7
  ).*
}

module ledger_verification_m_lvr007_submit_natively_refused {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 8
  ).*
}

module ledger_verification_m_lvr008_allow_second_genesis {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 9
  ).*
}

module ledger_verification_m_lw017_report_g4_accepted {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 10
  ).*
}

module ledger_verification_m_mid006_preview_pin_drift {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4, CHANGED_LANE_CONTROL_COUNT = 7, FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026, PINNED_LEDGER_REV_ID = 9101, PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115, MUTANT_ID = 11
  ).*
}
