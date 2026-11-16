// Quint formal specification for Sherlock E2E Bounded Context: Assurance
// Lane: e2e-assurance
// Own IDs: ZKE-001 .. ZKE-007, SSG-001 .. SSG-007
// Completion IDs: SQG-014, SQG-015, SQG-016, ZK-012 .. ZK-019, LPC-004, LPC-016
// Adopted IDs modelled here: CPR-001, DG-001, SQG-001, ZK-001, ZK-002, ZK-004, ZK-007
// (ZK-019 is a static Lean `#print axioms` check -> `SPEC-CHECKED candidate` + pure `consistency_zk019`)

module assurance {
  // =========================================================================
  // 1. Module Constants (Spec Numeric Bounds & Mutant Selector)
  // =========================================================================

  const EXIT_CODE_SEED_REFUSED: int      // ZKE-001, ZK-014: exit code 4 on seed guard violation
  const SECRET_FILE_MODE: int            // ZK-012, ZKE-004: Unix file mode 0600
  const MIN_D3_TERM_DECIBITS: int        // SSG-001, SQG-001: 1000 (= 100.0 bits, in tenths of a bit)
  const MIN_D3_GRINDABLE_DECIBITS: int   // SSG-001 Scenario: 1010 (= 101.0 bits, in tenths of a bit)
  const MIN_WRAP_K: int                  // CPR-001: wrap k in {19, 20, 21} -> min 19
  const MAX_WRAP_K: int                  // DG-001, CPR-001: no BLS12-381 circuit above k = 21
  const G3_TRANSCRIPT_LEGS: int          // SQG-014: 5 transcript legs compared in G3
  const MAX_GATE_FAILURES_AFK: int       // SQG-015: row T12 stops when a gate fails twice (2)
  const NOT_ATTEMPTED_INIT_COUNT: int    // SSG-004 Scenario: 8 NOT ATTEMPTED rows in COUNTEREXAMPLES.md
  const MUTANT_ID: int                   // 0 = sound; 1..24 = single-guard/binding mutants

  // Mutant ID tags
  pure val MUT_NONE: int                                     = 0
  pure val MUT_ZKE001_CONST_RP_WITHOUT_FEATURE: int          = 1
  pure val MUT_ZKE002_SETTLE_TEST_SEED: int                  = 2
  pure val MUT_ZKE003_PUBLISH_SECRET_CLASS: int              = 3
  pure val MUT_ZKE004_TRANSFER_SECRET_OUTSIDE_ROSTER: int    = 4
  pure val MUT_ZKE005_REDRAW_RP_WHEN_MISSING: int            = 5
  pure val MUT_ZKE006_DEBUG_PRINTS_SECRET: int               = 6
  pure val MUT_ZKE007_DETERMINISTIC_WRAP_BLINDER: int        = 7
  pure val MUT_SSG001_ACCEPT_LOW_D3_APP_KEY: int             = 8
  pure val MUT_SSG002_ALLOW_PLONK_IN_STARK_CRATE: int        = 9
  pure val MUT_SSG003_PASS_GATE_WITH_CONFIRMED_CX: int       = 10
  pure val MUT_SSG004_OMIT_NOT_ATTEMPTED_ROWS: int           = 11
  pure val MUT_SSG005_MISSING_CIRCUIT_CHANGE_ROW: int        = 12
  pure val MUT_SSG006_QUALIFY_ON_STALE_G5: int               = 13
  pure val MUT_SSG007_CLAIM_QUALIFIED_BEFORE_GATE: int       = 14
  pure val MUT_SQG014_PASS_G3_ON_LEG_MISMATCH: int           = 15
  pure val MUT_SQG015_IGNORE_DOUBLE_GATE_FAIL: int           = 16
  pure val MUT_SQG016_PASS_WITH_UNVERIFIED_STEP: int         = 17
  pure val MUT_ZK012_ACCEPT_BAD_SECRET_MODE: int             = 18
  pure val MUT_ZK013_REUSE_RUN_SALT: int                     = 19
  pure val MUT_ZK014_ALLOW_GW_DET_IN_RELEASE: int            = 20
  pure val MUT_ZK016_ALLOW_REPLAYED_SLOT: int                = 21
  pure val MUT_ZK018_ALLOW_CASE_B_UNOWNED_JOB: int           = 22
  pure val MUT_LPC016_COUNT_DONE_WITHOUT_VERDICT: int        = 23
  pure val MUT_CPR001_ALLOW_WRAP_K22: int                    = 24

  // =========================================================================
  // 2. Domain Enums & Cohesive Record Types
  // =========================================================================

  type BoxHost =
    | HostMuninn
    | HostHuginn
    | HostOdin
    | HostExternalHelper

  type SecretClass =
    | SecWitnessWord
    | SecStarkProofBytes
    | SecDeployRP
    | SecInputSalt
    | SecPlainProgD

  type SecretFileKind =
    | FileDeploySecret
    | FileRunSecret
    | FileWitness
    | FileExecTrace
    | FileStarkProof

  type RandomnessSource =
    | RandOsCsprng
    | RandConstant
    | RandEnvSeed

  type WrapDeviceKind =
    | WrapCpu
    | WrapGpu

  type KeyQualificationStatus =
    | KeyPrototype
    | KeyQualified

  type RunSecurityClaim =
    | SecNotQualified
    | SecQualified

  type AssuranceRefusal =
    | AssureRefusalNone
    | AssureRefusalConstRpExit4
    | AssureRefusalEnvSeedExit4
    | AssureRefusalTestSeedSettleBlocked
    | AssureRefusalSecretTransferOutsideRoster((BoxHost, SecretFileKind))
    | AssureRefusalMissingDeploySecret
    | AssureRefusalBadDeploySecretMode(int)
    | AssureRefusalReplayedBlindingSlot(int)
    | AssureRefusalCaseBUnownedProverHost(BoxHost)
    | AssureRefusalLowAppKeyD3((int, int))
    | AssureRefusalPlonkDepOrWrapK(int)
    | AssureRefusalGateBlockedConfirmedCx(int)
    | AssureRefusalGateBlockedStaleG5((int, int))
    | AssureRefusalGateBlockedG3Mismatch(int)
    | AssureRefusalGateBlockedUnverifiedStep
    | AssureRefusalWrapKeyLeaksLTranscript

  type DeploymentSecretState = {
    existsOnBox: bool,
    fileMode: int,
    rPValue: int,
    rPSource: RandomnessSource,
    drawCount: int,
  }

  type PreparedRunRec = {
    runSeq: int,
    inputId: int,
    hostBox: BoxHost,
    builtWithTestSeed: bool,
    deploySecretWasPresent: bool,
    deploySecretMode: int,
    usedRP: int,
    rPSource: RandomnessSource,
    inputSalt: int,
    claimPub: int,
    securityClaimInRecord: RunSecurityClaim,
    gateWasPassedAtWrite: bool,
  }

  type WrapProofRec = {
    proofSeq: int,
    hostBox: BoxHost,
    stmtId: int,
    device: WrapDeviceKind,
    builtWithTestSeed: bool,
    envSeedPassed: bool,
    blindingSlotId: int,
    usedOsRng: bool,
    lTranscriptInPublicOrConst: bool,
    proofBytesDigest: int,
    ledger9101Verified: bool,
  }

  type SettledCallRec = {
    proofSeq: int,
    builtWithTestSeed: bool,
    publishedOnlyWrapAndCallVec: bool,
    leakedSecretClasses: Set[SecretClass],
    lTranscriptPublic: bool,
    anchorHoldsPlainProgD: bool,
  }

  type SecretTransferRec = {
    srcHost: BoxHost,
    dstHost: BoxHost,
    fileKind: SecretFileKind,
    destFileMode: int,
  }

  type DebugLogRec = {
    runSeq: int,
    containsRPWord: bool,
    containsSaltWord: bool,
  }

  type AppKeyD3Rec = {
    progId: int,
    columns: int,
    minTermDeciBits: int,
    minGrindableDeciBits: int,
  }

  type CircuitChangeRec = {
    changeId: int,
    inChangesMd: bool,
    laneNamed: bool,
    carbonQuintRun: bool,
    leadVerdictFiled: bool,
    closed: bool,
  }

  type LoopRowRec = {
    rowId: int,
    rowDoneRaw: bool,
    hasLeadVerdict: bool,
    countedDoneForT15: bool,
  }

  type AssuranceState = {
    ownedRoster: Set[BoxHost],
    activeManifestSha: int,
    manifestStatus: KeyQualificationStatus,
    deployedRP: int,
    boxDeploySecrets: BoxHost -> DeploymentSecretState,
    nextRunSeq: int,
    nextProofSeq: int,
    preparedRuns: Set[PreparedRunRec],
    wrapProofs: Set[WrapProofRec],
    usedBlindingSlots: Set[int],
    settledCalls: Set[SettledCallRec],
    admittedSecretTransfers: Set[SecretTransferRec],
    debugLogs: Set[DebugLogRec],
    checkedAppKeys: Set[AppKeyD3Rec],

    // Crate & wrap circuit bounds (SSG-002, DG-001, CPR-001, ZK-004)
    t25CrateGatePassed: bool,
    starkCratesHavePlonkOrKzg: bool,
    wrapKeyLogK: int,
    wrapKeyFixesLTranscript: bool,

    // T12 / T13 / T15 assurance & security gate state (SSG-003..007, SQG-014..016, LPC-004, LPC-016)
    unfixedConfirmedCxCount: int,
    notAttemptedCxIds: Set[int],
    unverifiedRunbookSteps: int,
    g3MatchingLegsCount: int,
    g3Passed: bool,
    g1g2g4Fp8LedgerPassed: bool,
    g5MappedManifestSha: int,
    gateFailureStreak: int,
    t12RowStopped: bool,
    gatePassed: bool,
    gateRecordWritten: bool,
    gateRecordManifestSha: int,
    gateRecordOpenCxIds: Set[int],

    changesListAuditPassed: bool,
    circuitChanges: Set[CircuitChangeRec],
    loopRows: Set[LoopRowRec],

    lastExitCode: int,
    lastRefusal: AssuranceRefusal,
  }

  // =========================================================================
  // 3. Domain Constants & Pure Functional Layer (Step 3)
  // =========================================================================

  pure val OWNED_BOXES: Set[BoxHost] = Set(HostMuninn, HostHuginn, HostOdin)
  pure val ALL_BOX_HOSTS: Set[BoxHost] = Set(HostMuninn, HostHuginn, HostOdin, HostExternalHelper)

  pure val T10M_MANIFEST_SHA: int = 100
  pure val T25_MANIFEST_SHA: int = 250
  pure val DEPLOYED_RP_VAL: int = 7001

  // Pure check for ZK-019 (`SPEC-CHECKED candidate`: allowed Lean 4 kernel axioms)
  pure val ALLOWED_LEAN_AXIOMS: Set[str] = Set("propext", "Quot.sound", "Classical.choice")

  val consistency_zk019: bool =
    ALLOWED_LEAN_AXIOMS.size() == 3 and not(ALLOWED_LEAN_AXIOMS.contains("sorryAx"))

  val consistency_constants: bool = and {
    EXIT_CODE_SEED_REFUSED == 4,
    SECRET_FILE_MODE == 600,
    MIN_D3_TERM_DECIBITS >= 1000,
    MIN_D3_GRINDABLE_DECIBITS >= 1010,
    MIN_WRAP_K == 19,
    MAX_WRAP_K == 21,
    G3_TRANSCRIPT_LEGS == 5,
    MAX_GATE_FAILURES_AFK == 2,
    NOT_ATTEMPTED_INIT_COUNT >= 1,
    consistency_zk019,
  }

  // Pure guard: can a run be prepared on `h`? (ZKE-001, ZKE-005, ZK-012, ZK-018, ZK-001)
  pure def canPrepareRun(
    s: AssuranceState,
    h: BoxHost,
    testSeedBuild: bool,
    useConstRpPath: bool,
    mutId: int
  ): bool =
    val hostOk =
      if (mutId == MUT_ZK018_ALLOW_CASE_B_UNOWNED_JOB) true
      else s.ownedRoster.contains(h)
    val dSec = s.boxDeploySecrets.get(h)
    val secretPresentOk =
      if (mutId == MUT_ZKE005_REDRAW_RP_WHEN_MISSING) true
      else dSec.existsOnBox
    val modeOk =
      if (mutId == MUT_ZK012_ACCEPT_BAD_SECRET_MODE) true
      else (not(dSec.existsOnBox) or dSec.fileMode == SECRET_FILE_MODE)
    val constGuardOk =
      if (mutId == MUT_ZKE001_CONST_RP_WITHOUT_FEATURE) true
      else (not(useConstRpPath) or testSeedBuild)
    hostOk and secretPresentOk and modeOk and constGuardOk

  // Pure guard: can a wrap proof be generated? (ZK-014, ZK-016, ZK-018, ZK-004)
  pure def canMakeWrapProof(
    s: AssuranceState,
    h: BoxHost,
    testSeedBuild: bool,
    envSeedPassed: bool,
    slotId: int,
    mutId: int
  ): bool =
    val hostOk =
      if (mutId == MUT_ZK018_ALLOW_CASE_B_UNOWNED_JOB) true
      else s.ownedRoster.contains(h)
    val seedOk =
      if (mutId == MUT_ZK014_ALLOW_GW_DET_IN_RELEASE) true
      else (not(envSeedPassed) or testSeedBuild)
    val slotOk =
      if (mutId == MUT_ZK016_ALLOW_REPLAYED_SLOT) true
      else not(s.usedBlindingSlots.contains(slotId))
    hostOk and seedOk and slotOk and not(s.wrapKeyFixesLTranscript)

  // Pure guard: can a wrap proof be submitted to Preview settlement? (ZKE-002)
  pure def canSettleWrapProof(wp: WrapProofRec, mutId: int): bool =
    val notTestSeed =
      if (mutId == MUT_ZKE002_SETTLE_TEST_SEED) true
      else not(wp.builtWithTestSeed)
    wp.ledger9101Verified and notTestSeed

  // Pure guard: can the T12 security gate pass and qualify the active key manifest?
  // (SSG-003, SSG-006, SQG-014, SQG-015, SQG-016)
  pure def canPassSecurityGate(s: AssuranceState, mutId: int): bool =
    val cxOk =
      if (mutId == MUT_SSG003_PASS_GATE_WITH_CONFIRMED_CX) true
      else (s.unfixedConfirmedCxCount == 0)
    val stepsVerifiedOk =
      if (mutId == MUT_SQG016_PASS_WITH_UNVERIFIED_STEP) true
      else (s.unverifiedRunbookSteps == 0)
    val g5FreshOk =
      if (mutId == MUT_SSG006_QUALIFY_ON_STALE_G5) (s.g5MappedManifestSha > 0)
      else (s.g5MappedManifestSha == s.activeManifestSha)
    not(s.t12RowStopped)
      and s.g3Passed
      and s.g1g2g4Fp8LedgerPassed
      and g5FreshOk
      and cxOk
      and stepsVerifiedOk

  // =========================================================================
  // 4. State Variables & Initialization (Step 4)
  // =========================================================================

  var assure: AssuranceState
  var lastOp: str

  action init: bool =
    val muninnSecret: DeploymentSecretState = {
      existsOnBox: true,
      fileMode: SECRET_FILE_MODE,
      rPValue: DEPLOYED_RP_VAL,
      rPSource: RandOsCsprng,
      drawCount: 1,
    }
    val absentSecret: DeploymentSecretState = {
      existsOnBox: false,
      fileMode: 0,
      rPValue: 0,
      rPSource: RandOsCsprng,
      drawCount: 0,
    }
    all {
      assure' = {
        ownedRoster: OWNED_BOXES,
        activeManifestSha: T25_MANIFEST_SHA,
        manifestStatus: KeyPrototype,
        deployedRP: DEPLOYED_RP_VAL,
        boxDeploySecrets: Map(
          HostMuninn -> muninnSecret,
          HostHuginn -> muninnSecret,
          HostOdin -> absentSecret,
          HostExternalHelper -> absentSecret
        ),
        nextRunSeq: 1,
        nextProofSeq: 1,
        preparedRuns: Set(),
        wrapProofs: Set(),
        usedBlindingSlots: Set(),
        settledCalls: Set(),
        admittedSecretTransfers: Set(),
        debugLogs: Set(),
        checkedAppKeys: Set(),
        t25CrateGatePassed: false,
        starkCratesHavePlonkOrKzg: false,
        wrapKeyLogK: 21,
        wrapKeyFixesLTranscript: false,
        unfixedConfirmedCxCount: 1,
        notAttemptedCxIds: 1.to(NOT_ATTEMPTED_INIT_COUNT),
        unverifiedRunbookSteps: 2,
        g3MatchingLegsCount: 0,
        g3Passed: false,
        g1g2g4Fp8LedgerPassed: false,
        g5MappedManifestSha: T10M_MANIFEST_SHA,
        gateFailureStreak: 0,
        t12RowStopped: false,
        gatePassed: false,
        gateRecordWritten: false,
        gateRecordManifestSha: 0,
        gateRecordOpenCxIds: Set(),
        changesListAuditPassed: false,
        circuitChanges: Set(
          { changeId: 1, inChangesMd: true, laneNamed: true, carbonQuintRun: true, leadVerdictFiled: true, closed: true }
        ),
        loopRows: Set(
          { rowId: 17, rowDoneRaw: true, hasLeadVerdict: false, countedDoneForT15: false }
        ),
        lastExitCode: 0,
        lastRefusal: AssureRefusalNone,
      },
      lastOp' = "init",
    }

  // =========================================================================
  // 5. Guarded Actions (Tagged with EARS Requirement IDs)
  // =========================================================================

  // Implements ZKE-004, ZK-012, ZK-001: Transfer `deploy.secret` (or witness/STARK file) to an owned box with mode 0600
  action transferSecretFileToBox(srcHost: BoxHost, dstHost: BoxHost, fKind: SecretFileKind): bool =
    val rosterOk =
      if (MUTANT_ID == MUT_ZKE004_TRANSFER_SECRET_OUTSIDE_ROSTER) true
      else (assure.ownedRoster.contains(srcHost) and assure.ownedRoster.contains(dstHost))
    all {
      srcHost != dstHost,
      rosterOk,
      assure.admittedSecretTransfers.size() < 6,
      val tf: SecretTransferRec = {
        srcHost: srcHost,
        dstHost: dstHost,
        fileKind: fKind,
        destFileMode: SECRET_FILE_MODE,
      }
      val updatedSecrets =
        if (fKind == FileDeploySecret and assure.boxDeploySecrets.get(srcHost).existsOnBox)
          assure.boxDeploySecrets.put(dstHost, {
            ...assure.boxDeploySecrets.get(srcHost),
            fileMode: SECRET_FILE_MODE,
          })
        else
          assure.boxDeploySecrets
      assure' = {
        ...assure,
        boxDeploySecrets: updatedSecrets,
        admittedSecretTransfers: assure.admittedSecretTransfers.union(Set(tf)),
        lastRefusal: AssureRefusalNone,
      },
      lastOp' = "transferSecretFileToBox",
    }

  // Implements ZKE-004, ZK-018, ZK-001: Refuse secret/witness/STARK transfer to host outside owned roster
  action refuseSecretTransferOutsideRoster(dstHost: BoxHost, fKind: SecretFileKind): bool = all {
    not(assure.ownedRoster.contains(dstHost)),
    assure' = {
      ...assure,
      lastRefusal: AssureRefusalSecretTransferOutsideRoster((dstHost, fKind)),
    },
    lastOp' = "refuseSecretTransferOutsideRoster",
  }

  // Precondition helper for ZK-012 Scenario: set group/world permission bits (`0644`) on a box's `deploy.secret`
  action stageWorldReadableDeploySecret(h: BoxHost): bool = all {
    assure.ownedRoster.contains(h),
    assure.boxDeploySecrets.get(h).existsOnBox,
    assure.boxDeploySecrets.get(h).fileMode == SECRET_FILE_MODE,
    assure' = {
      ...assure,
      boxDeploySecrets: assure.boxDeploySecrets.put(h, {
        ...assure.boxDeploySecrets.get(h),
        fileMode: 644,
      }),
    },
    lastOp' = "stageWorldReadableDeploySecret",
  }

  // Implements ZK-012 Scenario: Refuse `deploy.secret` with group or world permission bits (`mode != 0600`)
  action refuseBadModeDeploySecret(h: BoxHost): bool =
    val dSec = assure.boxDeploySecrets.get(h)
    all {
      dSec.existsOnBox,
      dSec.fileMode != SECRET_FILE_MODE,
      assure' = {
        ...assure,
        lastRefusal: AssureRefusalBadDeploySecretMode(dSec.fileMode),
      },
      lastOp' = "refuseBadModeDeploySecret",
    }

  // Implements ZKE-005 Scenario: Missing `deploy.secret` on a box refuses call and draws no new `r_P`
  action refuseMissingDeploySecret(h: BoxHost): bool = all {
    assure.ownedRoster.contains(h),
    not(assure.boxDeploySecrets.get(h).existsOnBox),
    assure' = {
      ...assure,
      lastRefusal: AssureRefusalMissingDeploySecret,
    },
    lastOp' = "refuseMissingDeploySecret",
  }

  // Implements ZKE-001: Build without `test-seed` reaching constant `r_P` path exits with code 4 and writes no proof
  action refuseConstantRpExit4: bool = all {
    assure' = {
      ...assure,
      lastExitCode: EXIT_CODE_SEED_REFUSED,
      lastRefusal: AssureRefusalConstRpExit4,
    },
    lastOp' = "refuseConstantRpExit4",
  }

  // Implements ZK-018, ZK-001: Refuse Case B job sent to unowned helper host
  action refuseCaseBUnownedHost(h: BoxHost): bool = all {
    not(assure.ownedRoster.contains(h)),
    assure' = {
      ...assure,
      lastRefusal: AssureRefusalCaseBUnownedProverHost(h),
    },
    lastOp' = "refuseCaseBUnownedHost",
  }

  // Implements ZKE-001, ZKE-005, ZKE-006, SSG-007, ZK-012, ZK-013, ZK-018:
  // Prepare a run loading `r_P` from `deploy.secret`, drawing fresh salt into `run.secret`, formatting debug log safely,
  // and writing `not qualified` in run record while gate has not passed
  action prepareRunAndLog(
    h: BoxHost,
    inputId: int,
    testSeedBuild: bool,
    useConstRpPath: bool
  ): bool =
    val seqN = assure.nextRunSeq
    val dSec = assure.boxDeploySecrets.get(h)
    val usedRpVal =
      if (useConstRpPath) 1001
      else if (dSec.existsOnBox) dSec.rPValue
      else 9999
    val rpSrc = if (useConstRpPath) RandConstant else RandOsCsprng
    val saltVal =
      if (MUTANT_ID == MUT_ZK013_REUSE_RUN_SALT) 4444
      else (8000 + seqN)
    val secClaim =
      if (assure.gatePassed and assure.gateRecordManifestSha == assure.activeManifestSha) SecQualified
      else if (MUTANT_ID == MUT_SSG007_CLAIM_QUALIFIED_BEFORE_GATE) SecQualified
      else SecNotQualified
    val leakDebug = (MUTANT_ID == MUT_ZKE006_DEBUG_PRINTS_SECRET)
    all {
      seqN <= 6,
      canPrepareRun(assure, h, testSeedBuild, useConstRpPath, MUTANT_ID),
      val runRec: PreparedRunRec = {
        runSeq: seqN,
        inputId: inputId,
        hostBox: h,
        builtWithTestSeed: testSeedBuild,
        deploySecretWasPresent: dSec.existsOnBox,
        deploySecretMode: dSec.fileMode,
        usedRP: usedRpVal,
        rPSource: rpSrc,
        inputSalt: saltVal,
        claimPub: 50000 + 100 * inputId + saltVal,
        securityClaimInRecord: secClaim,
        gateWasPassedAtWrite: assure.gatePassed,
      }
      val logRec: DebugLogRec = {
        runSeq: seqN,
        containsRPWord: leakDebug,
        containsSaltWord: leakDebug,
      }
      assure' = {
        ...assure,
        nextRunSeq: seqN + 1,
        preparedRuns: assure.preparedRuns.union(Set(runRec)),
        debugLogs: assure.debugLogs.union(Set(logRec)),
        lastExitCode: 0,
        lastRefusal: AssureRefusalNone,
      },
      lastOp' = "prepareRunAndLog",
    }

  // Implements ZK-014: Release wrap build receiving `GW_DET` / `SHERLOCK_TEST_SEED` exits 4 and writes no proof
  action refuseEnvSeedInReleaseWrapExit4: bool = all {
    assure' = {
      ...assure,
      lastExitCode: EXIT_CODE_SEED_REFUSED,
      lastRefusal: AssureRefusalEnvSeedExit4,
    },
    lastOp' = "refuseEnvSeedInReleaseWrapExit4",
  }

  // Implements ZK-016: Refuse replayed blinding slot ID on `wrapd serve`
  action refuseReplayedBlindingSlot(slotId: int): bool = all {
    assure.usedBlindingSlots.contains(slotId),
    assure' = {
      ...assure,
      lastRefusal: AssureRefusalReplayedBlindingSlot(slotId),
    },
    lastOp' = "refuseReplayedBlindingSlot",
  }

  // Implements ZKE-007, ZK-014, ZK-015, ZK-016, ZK-017, ZK-018, ZK-004:
  // Generate blinded wrap proof on CPU or GPU using fresh `OsRng` blinders and verify under ledger 9.1.0.1
  action makeWrapProof(
    h: BoxHost,
    stmtId: int,
    dev: WrapDeviceKind,
    testSeedBuild: bool,
    envSeedPassed: bool,
    slotId: int
  ): bool =
    val pSeq = assure.nextProofSeq
    val detStreamBug = (MUTANT_ID == MUT_ZKE007_DETERMINISTIC_WRAP_BLINDER)
    val bytesDig =
      if (detStreamBug) (20000 + stmtId)
      else (20000 + 100 * stmtId + 10 * pSeq + (if (dev == WrapGpu) 1 else 2))
    all {
      pSeq <= 8,
      canMakeWrapProof(assure, h, testSeedBuild, envSeedPassed, slotId, MUTANT_ID),
      val wp: WrapProofRec = {
        proofSeq: pSeq,
        hostBox: h,
        stmtId: stmtId,
        device: dev,
        builtWithTestSeed: testSeedBuild,
        envSeedPassed: envSeedPassed,
        blindingSlotId: slotId,
        usedOsRng: not(envSeedPassed),
        lTranscriptInPublicOrConst: assure.wrapKeyFixesLTranscript,
        proofBytesDigest: bytesDig,
        ledger9101Verified: true,
      }
      assure' = {
        ...assure,
        nextProofSeq: pSeq + 1,
        wrapProofs: assure.wrapProofs.union(Set(wp)),
        usedBlindingSlots: assure.usedBlindingSlots.union(Set(slotId)),
        lastExitCode: 0,
        lastRefusal: AssureRefusalNone,
      },
      lastOp' = "makeWrapProof",
    }

  // Implements ZKE-002 Scenario: Settlement runner refuses wrap proof produced by `test-seed` binary
  action refuseSettleTestSeedProof(pSeq: int): bool = all {
    assure.wrapProofs.exists(w => w.proofSeq == pSeq and w.builtWithTestSeed),
    assure' = {
      ...assure,
      lastRefusal: AssureRefusalTestSeedSettleBlocked,
    },
    lastOp' = "refuseSettleTestSeedProof",
  }

  // Implements ZKE-002, ZKE-003, ZK-002, ZK-004, ZK-007:
  // Settle a production wrap proof publishing ONLY call vector + blinded wrap proof with 0 secret hits
  action settleWrapProofAndScan(pSeq: int): bool =
    val matching = assure.wrapProofs.filter(w => w.proofSeq == pSeq and canSettleWrapProof(w, MUTANT_ID))
    all {
      matching.size() > 0,
      val isSeeded = matching.exists(w => w.builtWithTestSeed)
      val leaksLTrans = matching.exists(w => w.lTranscriptInPublicOrConst)
      val leakClass =
        if (MUTANT_ID == MUT_ZKE003_PUBLISH_SECRET_CLASS) Set(SecPlainProgD)
        else Set()
      val sc: SettledCallRec = {
        proofSeq: pSeq,
        builtWithTestSeed: isSeeded,
        publishedOnlyWrapAndCallVec: (leakClass.size() == 0),
        leakedSecretClasses: leakClass,
        lTranscriptPublic: leaksLTrans,
        anchorHoldsPlainProgD: (MUTANT_ID == MUT_ZKE003_PUBLISH_SECRET_CLASS),
      }
      assure' = {
        ...assure,
        settledCalls: assure.settledCalls.union(Set(sc)),
        lastRefusal: AssureRefusalNone,
      },
      lastOp' = "settleWrapProofAndScan",
    }

  // Implements SSG-001, SQG-001: Check per-program R_SL app key with `fp8_d3` (>= 100.0 bits all, >= 101.0 bits grindable)
  action checkAppKeyD3(progId: int, cols: int, minTermDeci: int, minGrindDeci: int): bool =
    val d3Ok =
      if (MUTANT_ID == MUT_SSG001_ACCEPT_LOW_D3_APP_KEY) true
      else (minTermDeci >= MIN_D3_TERM_DECIBITS and minGrindDeci >= MIN_D3_GRINDABLE_DECIBITS)
    all {
      d3Ok,
      assure.checkedAppKeys.size() < 4,
      val rec: AppKeyD3Rec = {
        progId: progId,
        columns: cols,
        minTermDeciBits: minTermDeci,
        minGrindableDeciBits: minGrindDeci,
      }
      assure' = {
        ...assure,
        checkedAppKeys: assure.checkedAppKeys.union(Set(rec)),
        lastRefusal: AssureRefusalNone,
      },
      lastOp' = "checkAppKeyD3",
    }

  // Implements SSG-001: Refuse per-program app key failing D3 100/101-bit threshold
  action refuseLowD3AppKey(minTermDeci: int, minGrindDeci: int): bool = all {
    minTermDeci < MIN_D3_TERM_DECIBITS or minGrindDeci < MIN_D3_GRINDABLE_DECIBITS,
    assure' = {
      ...assure,
      lastRefusal: AssureRefusalLowAppKeyD3((minTermDeci, minGrindDeci)),
    },
    lastOp' = "refuseLowD3AppKey",
  }

  // Implements SSG-002, DG-001, CPR-001, ZK-004:
  // Audit that STARK crates have no PLONK/KZG dependency, wrap `k in 19..21`, and no `L` transcript fixed in wrap VK
  action auditCratesAndWrapBoundsPass(candWrapK: int, hasPlonkInStark: bool): bool =
    val crateOk =
      if (MUTANT_ID == MUT_SSG002_ALLOW_PLONK_IN_STARK_CRATE) true
      else not(hasPlonkInStark)
    val kOk =
      if (MUTANT_ID == MUT_CPR001_ALLOW_WRAP_K22) (candWrapK >= MIN_WRAP_K)
      else (candWrapK >= MIN_WRAP_K and candWrapK <= MAX_WRAP_K)
    all {
      crateOk,
      kOk,
      not(assure.wrapKeyFixesLTranscript),
      assure' = {
        ...assure,
        t25CrateGatePassed: true,
        starkCratesHavePlonkOrKzg: hasPlonkInStark,
        wrapKeyLogK: candWrapK,
        lastRefusal: AssureRefusalNone,
      },
      lastOp' = "auditCratesAndWrapBoundsPass",
    }

  // Implements SSG-002, DG-001, CPR-001: Refuse `midnight-proofs` in STARK crate or `k = 22` wrap circuit
  action refusePlonkInStarkOrWrapAboveK21(candWrapK: int, hasPlonkInStark: bool): bool = all {
    hasPlonkInStark or candWrapK > MAX_WRAP_K or candWrapK < MIN_WRAP_K,
    assure' = {
      ...assure,
      lastRefusal: AssureRefusalPlonkDepOrWrapK(candWrapK),
    },
    lastOp' = "refusePlonkInStarkOrWrapAboveK21",
  }

  // Implements ZK-004 Scenario: Refuse wrap key fixing a challenge, query position, or `X` from `L` transcript
  action refuseWrapKeyFixingLTranscript: bool = all {
    assure' = {
      ...assure,
      lastRefusal: AssureRefusalWrapKeyLeaksLTranscript,
    },
    lastOp' = "refuseWrapKeyFixingLTranscript",
  }

  // Implements SQG-014: Run G3 comparing all 5 transcript legs on rekeyed keys
  action runG3TranscriptCheck(matchingLegs: int): bool =
    val passG3 =
      if (MUTANT_ID == MUT_SQG014_PASS_G3_ON_LEG_MISMATCH) true
      else (matchingLegs == G3_TRANSCRIPT_LEGS)
    all {
      not(assure.t12RowStopped),
      matchingLegs >= 0 and matchingLegs <= G3_TRANSCRIPT_LEGS,
      val nextStreak = if (passG3) assure.gateFailureStreak else (assure.gateFailureStreak + 1)
      val stopT12 =
        if (MUTANT_ID == MUT_SQG015_IGNORE_DOUBLE_GATE_FAIL) false
        else (nextStreak >= MAX_GATE_FAILURES_AFK)
      assure' = {
        ...assure,
        g3MatchingLegsCount: matchingLegs,
        g3Passed: passG3,
        gateFailureStreak: nextStreak,
        t12RowStopped: stopT12,
        lastRefusal: if (passG3) AssureRefusalNone else AssureRefusalGateBlockedG3Mismatch(matchingLegs),
      },
      lastOp' = "runG3TranscriptCheck",
    }

  // Implements SQG-015, SSG-003, SQG-016: Close open prerequisites (fix CONFIRMED cx, verify runbook steps,
  // run G1/G2/G4/FP-8/ledger 9.1.0.1, and update G5 mapping to T25 parameters)
  action completeT12ChecksAndFixes: bool = all {
    not(assure.t12RowStopped),
    val nextCx =
      if (MUTANT_ID == MUT_SSG003_PASS_GATE_WITH_CONFIRMED_CX) assure.unfixedConfirmedCxCount else 0
    val nextUnv =
      if (MUTANT_ID == MUT_SQG016_PASS_WITH_UNVERIFIED_STEP) assure.unverifiedRunbookSteps else 0
    val nextG5 =
      if (MUTANT_ID == MUT_SSG006_QUALIFY_ON_STALE_G5) assure.g5MappedManifestSha else assure.activeManifestSha
    assure' = {
      ...assure,
      unfixedConfirmedCxCount: nextCx,
      unverifiedRunbookSteps: nextUnv,
      g3MatchingLegsCount: G3_TRANSCRIPT_LEGS,
      g3Passed: true,
      g1g2g4Fp8LedgerPassed: true,
      g5MappedManifestSha: nextG5,
      lastRefusal: AssureRefusalNone,
    },
    lastOp' = "completeT12ChecksAndFixes",
  }

  // Implements SQG-015 Scenario: Record a gate check failure; if a gate fails twice (`>= 2`), row T12 stops
  action recordGateCheckFailure: bool = all {
    not(assure.t12RowStopped),
    not(assure.gatePassed),
    val nextStreak = assure.gateFailureStreak + 1
    val stopNow =
      if (MUTANT_ID == MUT_SQG015_IGNORE_DOUBLE_GATE_FAIL) false
      else (nextStreak >= MAX_GATE_FAILURES_AFK)
    assure' = {
      ...assure,
      gateFailureStreak: nextStreak,
      t12RowStopped: stopNow,
    },
    lastOp' = "recordGateCheckFailure",
  }

  // Implements SSG-003, SSG-006, SQG-016: Refuse security gate pass when CONFIRMED cx, stale G5, or UNVERIFIED step remains
  action refuseSecurityGateEvaluation: bool = all {
    assure.unfixedConfirmedCxCount > 0
      or assure.unverifiedRunbookSteps > 0
      or assure.g5MappedManifestSha != assure.activeManifestSha,
    val refReason =
      if (assure.unfixedConfirmedCxCount > 0)
        AssureRefusalGateBlockedConfirmedCx(assure.unfixedConfirmedCxCount)
      else if (assure.g5MappedManifestSha != assure.activeManifestSha)
        AssureRefusalGateBlockedStaleG5((assure.g5MappedManifestSha, assure.activeManifestSha))
      else
        AssureRefusalGateBlockedUnverifiedStep
    assure' = {
      ...assure,
      lastRefusal: refReason,
    },
    lastOp' = "refuseSecurityGateEvaluation",
  }

  // Implements SSG-004, SSG-006, SSG-007: Write gate record (listing NOT ATTEMPTED open rows + manifest sha256)
  // and move key set status from `KeyPrototype` to `KeyQualified`
  action writePassingGateRecordAndQualify: bool = all {
    not(assure.gatePassed),
    canPassSecurityGate(assure, MUTANT_ID),
    val openListed =
      if (MUTANT_ID == MUT_SSG004_OMIT_NOT_ATTEMPTED_ROWS) Set()
      else assure.notAttemptedCxIds
    assure' = {
      ...assure,
      gatePassed: true,
      gateRecordWritten: true,
      gateRecordManifestSha: assure.activeManifestSha,
      gateRecordOpenCxIds: openListed,
      manifestStatus: KeyQualified,
      lastRefusal: AssureRefusalNone,
    },
    lastOp' = "writePassingGateRecordAndQualify",
  }

  // Implements SSG-005, LPC-004: Land a post-`c4bd312` circuit change and audit `CHANGES.md` + Carbon Quint verdict
  action landAndAuditCircuitChange(withRowAndVerdict: bool): bool =
    val auditOk =
      if (MUTANT_ID == MUT_SSG005_MISSING_CIRCUIT_CHANGE_ROW) true
      else withRowAndVerdict
    all {
      auditOk,
      assure.circuitChanges.size() < 3,
      val nextId = assure.circuitChanges.size() + 1
      val cRec: CircuitChangeRec = {
        changeId: nextId,
        inChangesMd: withRowAndVerdict,
        laneNamed: withRowAndVerdict,
        carbonQuintRun: withRowAndVerdict,
        leadVerdictFiled: withRowAndVerdict,
        closed: true,
      }
      assure' = {
        ...assure,
        changesListAuditPassed: true,
        circuitChanges: assure.circuitChanges.union(Set(cRec)),
      },
      lastOp' = "landAndAuditCircuitChange",
    }

  // Implements LPC-016: Evaluate whether a DONE loop row counts as DONE for T15 (requires lead verdict in `qa/gemini-quint`)
  action evaluateLoopRowForT15(fileVerdictFirst: bool): bool =
    val updatedRows = assure.loopRows.map(r => {
      val verdictNow = r.hasLeadVerdict or fileVerdictFirst
      val countDone =
        if (MUTANT_ID == MUT_LPC016_COUNT_DONE_WITHOUT_VERDICT) r.rowDoneRaw
        else (r.rowDoneRaw and verdictNow)
      { ...r, hasLeadVerdict: verdictNow, countedDoneForT15: countDone }
    })
    all {
      assure' = {
        ...assure,
        loopRows: updatedRows,
      },
      lastOp' = "evaluateLoopRowForT15",
    }

  // =========================================================================
  // 6. Nondeterministic `step` Relation (Step 6)
  // =========================================================================

  action step: bool = {
    nondet ownedBox = OWNED_BOXES.oneOf()
    nondet anyHost = ALL_BOX_HOSTS.oneOf()
    nondet fileChoice = Set(FileDeploySecret, FileRunSecret, FileWitness, FileExecTrace, FileStarkProof).oneOf()
    nondet devChoice = Set(WrapCpu, WrapGpu).oneOf()
    nondet boolFlag = Set(false, true).oneOf()
    nondet slotChoice = Set(1, 2, 3, 4).oneOf()
    val runHost = if (MUTANT_ID == MUT_ZK018_ALLOW_CASE_B_UNOWNED_JOB) anyHost else ownedBox
    val constRpFlag = (MUTANT_ID == MUT_ZKE001_CONST_RP_WITHOUT_FEATURE)
    val envSeedFlag = (MUTANT_ID == MUT_ZK014_ALLOW_GW_DET_IN_RELEASE)
    val transferDst = if (MUTANT_ID == MUT_ZKE004_TRANSFER_SECRET_OUTSIDE_ROSTER) HostExternalHelper else HostOdin
    val appTermBits = if (MUTANT_ID == MUT_SSG001_ACCEPT_LOW_D3_APP_KEY) 990 else MIN_D3_TERM_DECIBITS
    val appGrindBits = if (MUTANT_ID == MUT_SSG001_ACCEPT_LOW_D3_APP_KEY) 1005 else MIN_D3_GRINDABLE_DECIBITS
    val candK = if (MUTANT_ID == MUT_CPR001_ALLOW_WRAP_K22) 22 else MAX_WRAP_K
    val plonkDepFlag = (MUTANT_ID == MUT_SSG002_ALLOW_PLONK_IN_STARK_CRATE)
    val g3Legs = if (MUTANT_ID == MUT_SQG014_PASS_G3_ON_LEG_MISMATCH) 4 else G3_TRANSCRIPT_LEGS
    val changeValid = not(MUTANT_ID == MUT_SSG005_MISSING_CIRCUIT_CHANGE_ROW)
    val fileVerdictFlag = if (MUTANT_ID == MUT_LPC016_COUNT_DONE_WITHOUT_VERDICT) false else boolFlag
    any {
      transferSecretFileToBox(HostMuninn, transferDst, fileChoice),
      all { assure.lastRefusal == AssureRefusalNone, refuseSecretTransferOutsideRoster(HostExternalHelper, FileRunSecret) },
      stageWorldReadableDeploySecret(HostHuginn),
      all { assure.lastRefusal == AssureRefusalNone, refuseBadModeDeploySecret(HostHuginn) },
      all { assure.lastRefusal == AssureRefusalNone, refuseMissingDeploySecret(HostOdin) },
      all { assure.lastRefusal == AssureRefusalNone, refuseConstantRpExit4 },
      all { assure.lastRefusal == AssureRefusalNone, refuseCaseBUnownedHost(HostExternalHelper) },
      prepareRunAndLog(runHost, 1, boolFlag, constRpFlag),
      all { assure.lastRefusal == AssureRefusalNone, refuseEnvSeedInReleaseWrapExit4 },
      all { assure.lastRefusal == AssureRefusalNone, refuseReplayedBlindingSlot(slotChoice) },
      makeWrapProof(runHost, 1, devChoice, boolFlag, envSeedFlag, slotChoice),
      all { assure.lastRefusal == AssureRefusalNone, refuseSettleTestSeedProof(1) },
      settleWrapProofAndScan(1),
      settleWrapProofAndScan(2),
      checkAppKeyD3(1, 16346, appTermBits, appGrindBits),
      all { assure.lastRefusal == AssureRefusalNone, refuseLowD3AppKey(995, 1005) },
      auditCratesAndWrapBoundsPass(candK, plonkDepFlag),
      all { assure.lastRefusal == AssureRefusalNone, refusePlonkInStarkOrWrapAboveK21(22, true) },
      all { assure.lastRefusal == AssureRefusalNone, refuseWrapKeyFixingLTranscript },
      runG3TranscriptCheck(g3Legs),
      completeT12ChecksAndFixes,
      recordGateCheckFailure,
      all { assure.lastRefusal == AssureRefusalNone, refuseSecurityGateEvaluation },
      writePassingGateRecordAndQualify,
      landAndAuditCircuitChange(changeValid),
      evaluateLoopRowForT15(fileVerdictFlag),
    }
  }

  // =========================================================================
  // 7. Safety Invariants (`val inv_<ID>` and Master Conjunction `val inv`)
  // =========================================================================

  // ZKE-001: No non-`test-seed` build prepares a run from a constant `r_P`.
  val inv_ZKE_001: bool =
    assure.preparedRuns.forall(r =>
      not(r.builtWithTestSeed) implies (r.rPSource == RandOsCsprng)
    )

  // ZKE-002: No proof from a `test-seed` binary is submitted/settled on Preview.
  val inv_ZKE_002: bool =
    assure.settledCalls.forall(sc => not(sc.builtWithTestSeed))

  // ZKE-003: Settled calls publish ONLY call vector + blinded wrap proof, with 0 hits across all 5 secret classes.
  val inv_ZKE_003: bool =
    assure.settledCalls.forall(sc =>
      sc.publishedOnlyWrapAndCallVec and sc.leakedSecretClasses.size() == 0
    )

  // ZKE-004: Admitted transfers of secret/witness/trace/STARK files stay inside `ownedRoster` with mode `0600`.
  val inv_ZKE_004: bool =
    assure.admittedSecretTransfers.forall(tf =>
      assure.ownedRoster.contains(tf.srcHost)
        and assure.ownedRoster.contains(tf.dstHost)
        and tf.destFileMode == SECRET_FILE_MODE
    )

  // ZKE-005: Every prepared run of a deployed program required `deploy.secret` present and used its `r_P` without redrawing.
  val inv_ZKE_005: bool =
    assure.preparedRuns.forall(r =>
      r.deploySecretWasPresent and (not(r.builtWithTestSeed) implies r.usedRP == assure.deployedRP)
    )

  // ZKE-006: `Debug` formatting of blinder types prints no word of `r_P` or input salt into logs.
  val inv_ZKE_006: bool =
    assure.debugLogs.forall(lg => not(lg.containsRPWord) and not(lg.containsSaltWord))

  // ZKE-007: Distinct wrap proof invocations (across and within CPU/GPU backends) produce pairwise-distinct byte strings.
  val inv_ZKE_007: bool =
    assure.wrapProofs.forall(p1 =>
      p1.ledger9101Verified and assure.wrapProofs.forall(p2 =>
        (p1.proofSeq != p2.proofSeq) implies (p1.proofBytesDigest != p2.proofBytesDigest)
      )
    )

  // SSG-001: Every qualified per-program R_SL app key has >= 100.0 bits on all terms and >= 101.0 bits on grindable terms.
  val inv_SSG_001: bool =
    assure.checkedAppKeys.forall(ak =>
      ak.minTermDeciBits >= MIN_D3_TERM_DECIBITS
        and ak.minGrindableDeciBits >= MIN_D3_GRINDABLE_DECIBITS
    )

  // SSG-002: Passing T25 crate gate guarantees `sherlock-leaf` and `sherlock-lfr` have no PLONK/KZG dependency.
  val inv_SSG_002: bool =
    assure.t25CrateGatePassed implies not(assure.starkCratesHavePlonkOrKzg)

  // SSG-003: Security gate never passes while an unfixed CONFIRMED counterexample exists.
  val inv_SSG_003: bool =
    assure.gatePassed implies (assure.unfixedConfirmedCxCount == 0)

  // SSG-004: Written gate records list every `NOT ATTEMPTED` counterexample row as open.
  val inv_SSG_004: bool =
    assure.gateRecordWritten implies (
      assure.notAttemptedCxIds.forall(cid => assure.gateRecordOpenCxIds.contains(cid))
    )

  // SSG-005: Passed `CHANGES.md` audit guarantees every post-`c4bd312` circuit change has a row naming its Quint lane.
  val inv_SSG_005: bool =
    assure.changesListAuditPassed implies (
      assure.circuitChanges.forall(cc => cc.inChangesMd and cc.laneNamed)
    )

  // SSG-006: Key manifest is `KeyQualified` iff the gate record passed on the active key manifest (`G1..G5` + `FP-8`).
  val inv_SSG_006: bool = and {
    (assure.manifestStatus == KeyQualified) implies (
      assure.gatePassed
        and assure.gateRecordWritten
        and assure.gateRecordManifestSha == assure.activeManifestSha
        and assure.g5MappedManifestSha == assure.activeManifestSha
    ),
    assure.gateRecordWritten implies (
      assure.g3Passed and assure.g1g2g4Fp8LedgerPassed and assure.g5MappedManifestSha == assure.activeManifestSha
    ),
  }

  // SSG-007: Run records written before the security gate passes on the active key set report `SecNotQualified`.
  val inv_SSG_007: bool =
    assure.preparedRuns.forall(r =>
      not(r.gateWasPassedAtWrite) implies (r.securityClaimInRecord == SecNotQualified)
    )

  // SQG-014: G3 passes ONLY when all 5 transcript legs match on the rekeyed keys.
  val inv_SQG_014: bool =
    assure.g3Passed implies (assure.g3MatchingLegsCount == G3_TRANSCRIPT_LEGS)

  // SQG-015: When gate checks fail twice (`gateFailureStreak >= 2`), row T12 stops and cannot pass.
  val inv_SQG_015: bool =
    ((assure.gateFailureStreak >= MAX_GATE_FAILURES_AFK) implies assure.t12RowStopped)
      and (assure.t12RowStopped implies not(assure.gatePassed))

  // SQG-016: Security gate never passes while any RUNBOOK step is `UNVERIFIED`.
  val inv_SQG_016: bool =
    assure.gatePassed implies (assure.unverifiedRunbookSteps == 0)

  // ZK-012: `deploy.secret` used by any prepared run has mode `0600` and `r_P` drawn once from OS CSPRNG.
  val inv_ZK_012: bool =
    assure.preparedRuns.forall(r => r.deploySecretMode == SECRET_FILE_MODE)

  // ZK-013: Distinct prepared runs of the same input draw distinct salts and yield distinct `claim_pub` values.
  val inv_ZK_013: bool =
    assure.preparedRuns.forall(r1 =>
      assure.preparedRuns.forall(r2 =>
        (r1.runSeq != r2.runSeq and r1.inputId == r2.inputId) implies (
          r1.inputSalt != r2.inputSalt and r1.claimPub != r2.claimPub
        )
      )
    )

  // ZK-014: Non-`test-seed` wrap builds never produce a proof when seed env vars (`GW_DET`, etc.) are passed.
  val inv_ZK_014: bool =
    assure.wrapProofs.forall(wp =>
      not(wp.builtWithTestSeed) implies not(wp.envSeedPassed)
    )

  // ZK-015: Two wrap proofs of the same statement differ and both verify under ledger 9.1.0.1.
  val inv_ZK_015: bool = inv_ZKE_007

  // ZK-016: Blinding slots are one-use; no two wrap proofs consume the same `blindingSlotId`.
  val inv_ZK_016: bool =
    assure.wrapProofs.forall(p1 =>
      assure.wrapProofs.forall(p2 =>
        (p1.proofSeq != p2.proofSeq) implies (p1.blindingSlotId != p2.blindingSlotId)
      )
    )

  // ZK-017: GPU wrap proofs on `odin` / `huginn` draw blinders from `OsRng` (unless in explicit `test-seed` mode).
  val inv_ZK_017: bool =
    assure.wrapProofs.forall(wp =>
      (wp.device == WrapGpu and not(wp.builtWithTestSeed)) implies wp.usedOsRng
    )

  // ZK-018: Case B is refused; every prepared run and wrap proof executes on a host in `ownedRoster`.
  val inv_ZK_018: bool =
    assure.preparedRuns.forall(r => assure.ownedRoster.contains(r.hostBox))
      and assure.wrapProofs.forall(w => assure.ownedRoster.contains(w.hostBox))

  // LPC-004: Closed post-`c4bd312` circuit changes ran a Carbon Quint lane and filed a lead verdict.
  val inv_LPC_004: bool =
    assure.circuitChanges.forall(cc =>
      cc.closed implies (cc.carbonQuintRun and cc.leadVerdictFiled)
    )

  // LPC-016: No DONE row counts as DONE for T15 without a filed lead verdict in `qa/gemini-quint`.
  val inv_LPC_016: bool =
    assure.loopRows.forall(lr => lr.countedDoneForT15 implies lr.hasLeadVerdict)

  // Adopted requirements (`CPR-001`, `DG-001`, `SQG-001`, `ZK-001`, `ZK-002`, `ZK-004`, `ZK-007`)
  val inv_ADOPTED_ASSURANCE: bool = and {
    // CPR-001 & DG-001: Wrap circuit log-size k in {19, 20, 21} (never >= 22)
    assure.t25CrateGatePassed implies (assure.wrapKeyLogK >= MIN_WRAP_K and assure.wrapKeyLogK <= MAX_WRAP_K),
    // ZK-001: Every prover run and secret transfer stays on owned machines
    inv_ZK_018 and inv_ZKE_004,
    // ZK-002, ZK-004, ZK-007: Only wrap + call vector published; no L transcript public; no plain prog_d in anchor
    assure.settledCalls.forall(sc =>
      sc.publishedOnlyWrapAndCallVec
        and not(sc.lTranscriptPublic)
        and not(sc.anchorHoldsPlainProgD)
    ),
    // SQG-001: Proven 100-bit security claimed only after G1..G4 + G5 pass
    inv_SSG_006 and inv_SSG_007,
  }

  // Master safety conjunction
  val inv: bool = and {
    inv_ZKE_001,
    inv_ZKE_002,
    inv_ZKE_003,
    inv_ZKE_004,
    inv_ZKE_005,
    inv_ZKE_006,
    inv_ZKE_007,
    inv_SSG_001,
    inv_SSG_002,
    inv_SSG_003,
    inv_SSG_004,
    inv_SSG_005,
    inv_SSG_006,
    inv_SSG_007,
    inv_SQG_014,
    inv_SQG_015,
    inv_SQG_016,
    inv_ZK_012,
    inv_ZK_013,
    inv_ZK_014,
    inv_ZK_015,
    inv_ZK_016,
    inv_ZK_017,
    inv_ZK_018,
    inv_LPC_004,
    inv_LPC_016,
    inv_ADOPTED_ASSURANCE,
  }

  // =========================================================================
  // 8. Non-Vacuity Reachability Witnesses (Negated Goals: MUST Exit 1 in Main)
  // =========================================================================

  val reach_secret_transferred_and_odin_prepared: bool = not(
    assure.admittedSecretTransfers.size() > 0
      and assure.preparedRuns.exists(r => r.hostBox == HostOdin)
  )

  val reach_wrap_proved_and_settled_clean: bool = not(
    assure.settledCalls.size() >= 1
  )

  val reach_two_runs_distinct_salts_and_claims: bool = not(
    assure.preparedRuns.size() >= 2
  )

  val reach_t12_gate_qualified_with_open_cx_listed: bool = not(
    assure.gatePassed
      and assure.manifestStatus == KeyQualified
      and assure.gateRecordOpenCxIds.size() == NOT_ATTEMPTED_INIT_COUNT
  )

  val reach_t12_stopped_on_double_failure: bool = not(
    assure.t12RowStopped
  )

  val reach_app_d3_and_crate_audit_and_verdict: bool = not(
    assure.checkedAppKeys.size() > 0
      and assure.t25CrateGatePassed
      and assure.loopRows.exists(lr => lr.countedDoneForT15)
  )

  val reach_exit4_and_gate_refusals: bool = not(
    assure.lastExitCode == EXIT_CODE_SEED_REFUSED
      or match assure.lastRefusal {
        | AssureRefusalGateBlockedConfirmedCx(_) => true
        | AssureRefusalSecretTransferOutsideRoster(_) => true
        | _ => false
      }
  )
}

// ===========================================================================
// 9. Concrete Instances: Main, 2 Edge Instances, and 24 Guard/Binding Mutants
// ===========================================================================

module assurance_main {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4,
    SECRET_FILE_MODE = 600,
    MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010,
    MIN_WRAP_K = 19,
    MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5,
    MAX_GATE_FAILURES_AFK = 2,
    NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 0
  ).*
}

// Edge Instance 1: Minimum legal NOT ATTEMPTED count (1) and minimum wrap k = 19 tested via constants
module assurance_edge_min {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4,
    SECRET_FILE_MODE = 600,
    MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010,
    MIN_WRAP_K = 19,
    MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5,
    MAX_GATE_FAILURES_AFK = 2,
    NOT_ATTEMPTED_INIT_COUNT = 1,
    MUTANT_ID = 0
  ).*
}

// Edge Instance 2: Exact boundary limits (`MIN_D3_TERM_DECIBITS = 1000`, `MIN_D3_GRINDABLE_DECIBITS = 1010`, `MAX_WRAP_K = 21`)
module assurance_edge_at_limit {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4,
    SECRET_FILE_MODE = 600,
    MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010,
    MIN_WRAP_K = 19,
    MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5,
    MAX_GATE_FAILURES_AFK = 2,
    NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 0
  ).*
}

// --- 24 Guard / Binding Mutant Instances (`assurance_m_<name>`) ---

module assurance_m_zke001_const_rp_without_feature {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 1
  ).*
}

module assurance_m_zke002_settle_test_seed {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 2
  ).*
}

module assurance_m_zke003_publish_secret_class {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 3
  ).*
}

module assurance_m_zke004_transfer_secret_outside_roster {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 4
  ).*
}

module assurance_m_zke005_redraw_rp_when_missing {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 5
  ).*
}

module assurance_m_zke006_debug_prints_secret {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 6
  ).*
}

module assurance_m_zke007_deterministic_wrap_blinder {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 7
  ).*
}

module assurance_m_ssg001_accept_low_d3_app_key {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 8
  ).*
}

module assurance_m_ssg002_allow_plonk_in_stark_crate {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 9
  ).*
}

module assurance_m_ssg003_pass_gate_with_confirmed_cx {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 10
  ).*
}

module assurance_m_ssg004_omit_not_attempted_rows {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 11
  ).*
}

module assurance_m_ssg005_missing_circuit_change_row {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 12
  ).*
}

module assurance_m_ssg006_qualify_on_stale_g5 {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 13
  ).*
}

module assurance_m_ssg007_claim_qualified_before_gate {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 14
  ).*
}

module assurance_m_sqg014_pass_g3_on_leg_mismatch {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 15
  ).*
}

module assurance_m_sqg015_ignore_double_gate_fail {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 16
  ).*
}

module assurance_m_sqg016_pass_with_unverified_step {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 17
  ).*
}

module assurance_m_zk012_accept_bad_secret_mode {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 18
  ).*
}

module assurance_m_zk013_reuse_run_salt {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 19
  ).*
}

module assurance_m_zk014_allow_gw_det_in_release {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 20
  ).*
}

module assurance_m_zk016_allow_replayed_slot {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 21
  ).*
}

module assurance_m_zk018_allow_case_b_unowned_job {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 22
  ).*
}

module assurance_m_lpc016_count_done_without_verdict {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 23
  ).*
}

module assurance_m_cpr001_allow_wrap_k22 {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4, SECRET_FILE_MODE = 600, MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010, MIN_WRAP_K = 19, MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5, MAX_GATE_FAILURES_AFK = 2, NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 24
  ).*
}
