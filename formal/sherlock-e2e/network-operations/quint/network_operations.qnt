// -*- mode: Bluespec; -*-
// Independent Quint specification for bounded context Network Operations (lane e2e-network-operations).
// Sources:
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-preview-deployment/spec.md (PVW-001..013)
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-wallet/spec.md (WAL-001..010)
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-sdk-cli/spec.md (SCL-001..010)
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-observability/spec.md (OBS-001..012)
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-recovery/spec.md (FRC-001..015 + FRC-016)
//   - openspec/changes/sherlock-completion-20261115/specs/sherlock-loop-completion/spec.md (LPC-008..011, LPC-015, LPC-020)
//   - openspec/changes/sherlock-vm-consensus-20261107/specs/sherlock-delivery/spec.md (adopted SWF-009)

module network_operations {
  // =========================================================================
  // Constants (numeric bounds, ports, modes, timeouts, and exit codes)
  // =========================================================================

  // PVW-001, PVW-002: pinned Preview genesis identifier (0x3c096de2...3796).
  const PINNED_PREVIEW_GENESIS: int
  // PVW-002: local devnet genesis identifier (undeployed1).
  const LOCAL_DEVNET_GENESIS: int
  // PVW-004: Preview proof-server localhost TCP port (6341).
  const PREVIEW_PROOF_SERVER_PORT: int
  // PVW-004: e6 local devnet proof-server TCP port left untouched (6340).
  const DEVNET_PROOF_SERVER_PORT: int
  // PVW-006: number of Preview tamper controls (R1, R2, R4, R3 => 4).
  const TAMPER_CONTROL_COUNT: int
  // PVW-008: Preview block inclusion timeout in seconds (600 s).
  const INBLOCK_TIMEOUT_S: int
  // WAL-010: strictest octal integer bound on wallet directory permissions (700).
  const MAX_WALLET_DIR_MODE: int
  // WAL-010, WAL-001: strictest octal integer bound on wallet phrase permissions (600).
  const MAX_PHRASE_FILE_MODE: int
  // SCL-002, PVW-012: exit status for a failed verdict (4).
  const EXIT_VERDICT_FAIL: int
  // SCL-007: strict upper bound on T10A_TTL_S in seconds (1,209,600 s).
  const MAX_TTL_EXCLUSIVE_S: int
  // FRC-003: minimum remaining TTL in seconds required to reuse a saved call (300 s = 5 min).
  const MIN_REUSE_TTL_LEFT_S: int
  // FRC-004: maximum total balancing attempts on transport errors (4).
  const MAX_BALANCE_TRANSPORT_TRIES: int
  // FRC-009: consecutive gate failures that stop a loop row (2).
  const MAX_CONSECUTIVE_GATE_FAILS: int
  // Mutant selector: 0 = honest specification; 1..32 = individual guard mutants.
  const MUTANT_ID: int

  // =========================================================================
  // Mutant identifiers
  // =========================================================================

  pure val MUT_NONE: int = 0
  pure val MUT_PVW001_WRONG_GENESIS_WALLET: int = 1
  pure val MUT_PVW002_SEND_TO_MAINNET: int = 2
  pure val MUT_PVW003_INITIAL_FEE_PARAMS: int = 3
  pure val MUT_PVW004_TOUCH_PORT_6340: int = 4
  pure val MUT_PVW007_PASS_NO_NODE_VERDICT: int = 5
  pure val MUT_PVW008_TIMEOUT_AS_REFUSAL: int = 6
  pure val MUT_PVW010_FINAL_WITHOUT_GPU_BOX: int = 7
  pure val MUT_PVW011_ACCEPT_STALE_VALIDIUM: int = 8
  pure val MUT_PVW012_PASS_INCLUDED_CONTROL: int = 9
  pure val MUT_PVW013_DEPLOY_MISSING_GATE: int = 10
  pure val MUT_WAL002_UNSCRUBBED_SECRET: int = 11
  pure val MUT_WAL003_UNPINNED_WALLET: int = 12
  pure val MUT_WAL004_CONCURRENT_SENDERS: int = 13
  pure val MUT_WAL005_PRIVATE_IN_RECEIPTS: int = 14
  pure val MUT_WAL007_SUBMIT_FROM_PROVER: int = 15
  pure val MUT_WAL009_OVERWRITE_WALLET: int = 16
  pure val MUT_WAL010_WIDE_MODE_READ: int = 17
  pure val MUT_SCL002_WRONG_EXIT_CODE: int = 18
  pure val MUT_SCL003_DRYRUN_NETWORK: int = 19
  pure val MUT_SCL004_MISMATCHED_VK_FOLDER: int = 20
  pure val MUT_SCL007_OUT_OF_RANGE_TTL: int = 21
  pure val MUT_OBS003_DEAD_WALKER_OR_LEAK: int = 22
  pure val MUT_FRC001_MARK_FAILED_STEP_OK: int = 23
  pure val MUT_FRC002_DOUBLE_DEPLOY: int = 24
  pure val MUT_FRC004_EXCESS_OR_BAD_RETRY: int = 25
  pure val MUT_FRC005_REUSE_REFUSED_CALL: int = 26
  pure val MUT_FRC006_REFUSALS_NO_READBACK: int = 27
  pure val MUT_FRC011_PUSH_BEFORE_T15: int = 28
  pure val MUT_FRC013_RESUBMIT_UNKNOWN: int = 29
  pure val MUT_FRC014_REUSE_CORRUPT_FILE: int = 30
  pure val MUT_FRC003_REUSE_LOW_TTL: int = 31
  pure val MUT_SWF009_UNAUTH_MAINNET: int = 32

  // =========================================================================
  // Domain types
  // =========================================================================

  type ControlOutcome =
    | CtrlIncluded
    | CtrlNotIncludedTimeout
    | CtrlRefusedByNode
    | CtrlNoNodeVerdict

  type RunVerdictState =
    | VerdictPending
    | VerdictPassed
    | VerdictFailed
    | VerdictError

  type KeyGateRecord = {
    rky003RowsOk: bool,
    rky004VkBytesOk: bool,
    rky005Security100BitOk: bool,
    t12GatePass: bool,
    zk012To017Pass: bool,
    wrp012FiveChecksPass: bool
  }

  type SavedCallArtifact = {
    present: bool,
    ttlLeftS: int,
    markedRefused: bool,
    inclusionUnknown: bool,
    includedOnChain: bool,
    sha256Valid: bool,
    vkMatchesFolder: bool,
    genesisMatchesFolder: bool
  }

  type NetOpsState = {
    // Chain, proof-server, and TTL configuration (PVW-001..004, SCL-003, SCL-004, SCL-007, SCL-010)
    rpcGenesis: int,
    configuredTtlS: int,
    ttlAccepted: bool,
    folderRecordedVkId: int,
    cliVkId: int,
    folderAccepted: bool,
    proofServerPort: int,
    devnetPort6340Untouched: bool,
    usedChainFeeParams: bool,
    sentChainGenesises: Set[int],
    dryRunExecuted: bool,
    dryRunNetworkConnections: int,
    dryRunTxSent: int,
    outsideFactoryCleanConfigOk: bool,
    sdkRootCheckStoppedCleanly: bool,

    // Wallet security, permissions, locking, and host isolation (WAL-001..010)
    walletDirMode: int,
    phraseFileMode: int,
    phraseExistsInDir: bool,
    existingWalletOverwritten: bool,
    phraseReadInProcess: bool,
    phraseExposedToShellOrEnv: bool,
    derivedWalletAddr: int,
    pinnedWalletAddr: int,
    walletStarted: bool,
    activeWalletSenders: int,
    submittedFromWalletHost: bool,
    unscrubbedSecretWritten: bool,
    nonAllowlistFieldInReceipts: bool,
    keyMaterialInBalanceFile: bool,
    secretsInRepoCount: int,

    // Deploy, honest call, tamper controls, and final prototype gates (PVW-005..013, LPC-008, LPC-009)
    t25KeyGate: KeyGateRecord,
    finalPrototypeDeployed: bool,
    finalCallProvedOnOwnedGpuBox: bool,
    deploySendCount: int,
    deployConfirmed: bool,
    honestCallIncluded: bool,
    honestCallReadbackPresent: bool,
    refusalsStepStarted: bool,
    submittedControlCount: int,
    controlOutcome: ControlOutcome,
    verdictState: RunVerdictState,
    runnerExitCode: int,

    // Validium demo state (PVW-011, LPC-009)
    validiumExpectedProgCP: int,
    validiumStoredRoot: int,
    validiumAcceptedBatches: int,
    validiumAcceptedBadBatch: bool,

    // Observability: receipts, manifest, privacy scanner, wallet error chain (OBS-001..008)
    manifestSha256Written: bool,
    includedReceiptComplete: bool,
    refusedReceiptComplete: bool,
    privacyScanPassed: bool,
    forbiddenPrivateWordOnSurface: bool,
    controlWordsPresentOnSurface: bool,
    walletFailureChainRecorded: bool,

    // Recovery & loop durability (FRC-001..016, LPC-015)
    stepOkMarkers: Set[str],
    failedStepsMarkedOk: Set[str],
    savedCall: SavedCallArtifact,
    reusedLowTtlCall: bool,
    reusedRefusedCall: bool,
    reusedCorruptOrMismatchedCall: bool,
    resubmittedWhileInclusionUnknown: bool,
    suiteSkippedSettledCallsOk: bool,
    balanceIsTransportErr: bool,
    balanceAttemptCount: int,
    resyncedBeforeEachRetry: bool,
    rowConsecutiveGateFails: int,
    rowStoppedOnDoubleFail: bool,
    otherRowsStillRunning: bool,
    t15OwnerSigned: bool,
    fp3Passed: bool,
    pushedBranches: Set[str],

    // Adopted SWF-009 Mainnet authorization state
    mainnetOpRan: bool,
    mainnetAll4PreconditionsMet: bool,
    mainnetExplicitPerOpOwnerAuth: bool,
    mainnetAutoRetried: bool,

    lastStopReason: str
  }

  // =========================================================================
  // Observable state variables
  // =========================================================================

  var net: NetOpsState
  var lastOp: str

  // =========================================================================
  // Pure constants & helper predicates
  // =========================================================================

  pure val PREVIEW_STEPS: List[str] = List(
    "check",
    "balance-before",
    "deploy",
    "call",
    "refusals",
    "balance-after",
    "receipts"
  )

  pure val SDK_COMMANDS: Set[str] = Set(
    "sherlock_plan",
    "lfr_call",
    "wrap",
    "sherlock-wrap-ledger-check",
    "accept_args.py",
    "t10a-ledger-run.mjs",
    "t17.sh"
  )

  pure val SUITE_DRIVER_COMMANDS: Set[str] = Set("run", "selftest")

  pure def fullKeyGatePassed(g: KeyGateRecord): bool =
    g.rky003RowsOk and
    g.rky004VkBytesOk and
    g.rky005Security100BitOk and
    g.t12GatePass and
    g.zk012To017Pass and
    g.wrp012FiveChecksPass

  pure def isLegalTargetChain(genesisId: int): bool =
    genesisId == LOCAL_DEVNET_GENESIS or genesisId == PINNED_PREVIEW_GENESIS

  pure def canStartWallet(s: NetOpsState, rpcGen: int, dirMode: int, fileMode: int, derivAddr: int): bool = {
    val genesisOk =
      (MUTANT_ID == MUT_PVW001_WRONG_GENESIS_WALLET) or (rpcGen == PINNED_PREVIEW_GENESIS)
    val permsOk =
      (MUTANT_ID == MUT_WAL010_WIDE_MODE_READ) or
      (dirMode <= MAX_WALLET_DIR_MODE and fileMode <= MAX_PHRASE_FILE_MODE)
    val addrOk =
      (MUTANT_ID == MUT_WAL003_UNPINNED_WALLET) or (derivAddr == s.pinnedWalletAddr)
    genesisOk and permsOk and addrOk
  }

  pure def canAcceptTtl(ttlS: int): bool =
    if (MUTANT_ID == MUT_SCL007_OUT_OF_RANGE_TTL) true
    else (ttlS > 0 and ttlS < MAX_TTL_EXCLUSIVE_S)

  pure def canAcceptStateFolder(recordedVk: int, argVk: int): bool =
    (MUTANT_ID == MUT_SCL004_MISMATCHED_VK_FOLDER) or (recordedVk == argVk)

  pure def canDeployFinalPrototype(s: NetOpsState): bool =
    if (MUTANT_ID == MUT_PVW013_DEPLOY_MISSING_GATE) s.t25KeyGate.t12GatePass
    else fullKeyGatePassed(s.t25KeyGate)

  pure def canReuseSavedCall(art: SavedCallArtifact): bool = {
    val ttlOk =
      (MUTANT_ID == MUT_FRC003_REUSE_LOW_TTL) or (art.ttlLeftS > MIN_REUSE_TTL_LEFT_S)
    val notRefusedOk = (MUTANT_ID == MUT_FRC005_REUSE_REFUSED_CALL) or not(art.markedRefused)
    val integrityOk =
      (MUTANT_ID == MUT_FRC014_REUSE_CORRUPT_FILE) or
      (art.sha256Valid and art.vkMatchesFolder and art.genesisMatchesFolder)
    art.present and
      not(art.includedOnChain) and
      not(art.inclusionUnknown) and
      ttlOk and
      notRefusedOk and
      integrityOk
  }

  pure def canPushBranch(s: NetOpsState, branchName: str): bool =
    if (MUTANT_ID == MUT_FRC011_PUSH_BEFORE_T15) true
    else if (s.t15OwnerSigned) true
    else (branchName == "t10q" and s.fp3Passed)

  pure def canRunMainnetT4Op(all4Ok: bool, ownerAuth: bool, autoRetried: bool): bool =
    if (MUTANT_ID == MUT_SWF009_UNAUTH_MAINNET) true
    else (all4Ok and ownerAuth and not(autoRetried))

  // =========================================================================
  // Pure consistency & spec-gap witness predicates
  // =========================================================================

  val consistency_constants: bool =
    PINNED_PREVIEW_GENESIS == 3096 and
    LOCAL_DEVNET_GENESIS == 1001 and
    PINNED_PREVIEW_GENESIS != LOCAL_DEVNET_GENESIS and
    PREVIEW_PROOF_SERVER_PORT == 6341 and
    DEVNET_PROOF_SERVER_PORT == 6340 and
    PREVIEW_PROOF_SERVER_PORT != DEVNET_PROOF_SERVER_PORT and
    PREVIEW_STEPS.length() == 7 and
    SDK_COMMANDS.size() == 7 and
    SUITE_DRIVER_COMMANDS.size() == 2 and
    TAMPER_CONTROL_COUNT == 4 and
    INBLOCK_TIMEOUT_S >= 60 and
    INBLOCK_TIMEOUT_S <= 600 and
    MAX_WALLET_DIR_MODE >= 500 and
    MAX_WALLET_DIR_MODE <= 700 and
    MAX_PHRASE_FILE_MODE >= 400 and
    MAX_PHRASE_FILE_MODE <= 600 and
    EXIT_VERDICT_FAIL == 4 and
    MAX_TTL_EXCLUSIVE_S == 1209600 and
    MIN_REUSE_TTL_LEFT_S == 300 and
    MIN_REUSE_TTL_LEFT_S < MAX_TTL_EXCLUSIVE_S and
    MAX_BALANCE_TRANSPORT_TRIES >= 1 and
    MAX_BALANCE_TRANSPORT_TRIES <= 4 and
    MAX_CONSECUTIVE_GATE_FAILS == 2

  // Finding Q-network-operations-3 witness: a submitted call with 3,600 s TTL that times out InBlock
  // at 600 s (PVW-008) still has 3,000 s (> 300 s) of live mempool TTL; once its TTL finally expires
  // so non-inclusion is certain (FRC-013), ttlLeftS == 0 <= 300 s, making FRC-003 reuse impossible.
  val gap_q_netops_3_timeout_vs_ttl_expiry_reuse: bool = {
    val defaultTtlS = 3600
    val afterInblockTimeoutLeftS = defaultTtlS - INBLOCK_TIMEOUT_S
    val afterTrueExpiryLeftS = 0
    afterInblockTimeoutLeftS > MIN_REUSE_TTL_LEFT_S and
      not(afterTrueExpiryLeftS > MIN_REUSE_TTL_LEFT_S)
  }

  // Finding Q-network-operations-4 witness: T12 passing alone satisfies literal PVW-009 while
  // failing PVW-013 when RKY-005 (100-bit D3) or WRP-012 is missing.
  pure val gap_q_netops_4_pvw009_vs_pvw013: bool = {
    val partialGate: KeyGateRecord = {
      rky003RowsOk: true,
      rky004VkBytesOk: true,
      rky005Security100BitOk: false,
      t12GatePass: true,
      zk012To017Pass: true,
      wrp012FiveChecksPass: false
    }
    partialGate.t12GatePass and not(fullKeyGatePassed(partialGate))
  }

  // =========================================================================
  // Initialization
  // =========================================================================

  action init = all {
    net' = {
      rpcGenesis: PINNED_PREVIEW_GENESIS,
      configuredTtlS: 3600,
      ttlAccepted: true,
      folderRecordedVkId: 1,
      cliVkId: 1,
      folderAccepted: true,
      proofServerPort: PREVIEW_PROOF_SERVER_PORT,
      devnetPort6340Untouched: true,
      usedChainFeeParams: true,
      sentChainGenesises: Set(),
      dryRunExecuted: false,
      dryRunNetworkConnections: 0,
      dryRunTxSent: 0,
      outsideFactoryCleanConfigOk: true,
      sdkRootCheckStoppedCleanly: false,

      walletDirMode: MAX_WALLET_DIR_MODE,
      phraseFileMode: MAX_PHRASE_FILE_MODE,
      phraseExistsInDir: true,
      existingWalletOverwritten: false,
      phraseReadInProcess: false,
      phraseExposedToShellOrEnv: false,
      derivedWalletAddr: 77,
      pinnedWalletAddr: 77,
      walletStarted: false,
      activeWalletSenders: 0,
      submittedFromWalletHost: true,
      unscrubbedSecretWritten: false,
      nonAllowlistFieldInReceipts: false,
      keyMaterialInBalanceFile: false,
      secretsInRepoCount: 0,

      t25KeyGate: {
        rky003RowsOk: false,
        rky004VkBytesOk: false,
        rky005Security100BitOk: false,
        t12GatePass: false,
        zk012To017Pass: false,
        wrp012FiveChecksPass: false
      },
      finalPrototypeDeployed: false,
      finalCallProvedOnOwnedGpuBox: false,
      deploySendCount: 0,
      deployConfirmed: false,
      honestCallIncluded: false,
      honestCallReadbackPresent: false,
      refusalsStepStarted: false,
      submittedControlCount: 0,
      controlOutcome: CtrlRefusedByNode,
      verdictState: VerdictPending,
      runnerExitCode: 0,

      validiumExpectedProgCP: 1007,
      validiumStoredRoot: 10,
      validiumAcceptedBatches: 0,
      validiumAcceptedBadBatch: false,

      manifestSha256Written: false,
      includedReceiptComplete: true,
      refusedReceiptComplete: true,
      privacyScanPassed: false,
      forbiddenPrivateWordOnSurface: false,
      controlWordsPresentOnSurface: true,
      walletFailureChainRecorded: true,

      stepOkMarkers: Set(),
      failedStepsMarkedOk: Set(),
      savedCall: {
        present: true,
        ttlLeftS: 2400,
        markedRefused: false,
        inclusionUnknown: false,
        includedOnChain: false,
        sha256Valid: true,
        vkMatchesFolder: true,
        genesisMatchesFolder: true
      },
      reusedLowTtlCall: false,
      reusedRefusedCall: false,
      reusedCorruptOrMismatchedCall: false,
      resubmittedWhileInclusionUnknown: false,
      suiteSkippedSettledCallsOk: true,
      balanceIsTransportErr: true,
      balanceAttemptCount: 1,
      resyncedBeforeEachRetry: true,
      rowConsecutiveGateFails: 0,
      rowStoppedOnDoubleFail: false,
      otherRowsStillRunning: true,
      t15OwnerSigned: false,
      fp3Passed: false,
      pushedBranches: Set(),

      mainnetOpRan: false,
      mainnetAll4PreconditionsMet: false,
      mainnetExplicitPerOpOwnerAuth: false,
      mainnetAutoRetried: false,

      lastStopReason: "none"
    },
    lastOp' = "init"
  }

  // =========================================================================
  // Parameterized actions
  // =========================================================================

  // Implements PVW-004, SCL-004, SCL-007, SCL-009, OBS-006, FRC-001:
  // Run `check` step: verify state-folder VK pin, TTL range, own proof-server on 6341, and write manifest.sha256.
  action doRunCheckStep(ttlVal: int, folderVk: int, cliVk: int, mutPort: bool): bool = all {
    canAcceptTtl(ttlVal),
    canAcceptStateFolder(folderVk, cliVk),
    val touch6340 = (MUTANT_ID == MUT_PVW004_TOUCH_PORT_6340 and mutPort)
    net' = {
      ...net,
      configuredTtlS: ttlVal,
      ttlAccepted: true,
      folderRecordedVkId: folderVk,
      cliVkId: cliVk,
      folderAccepted: true,
      proofServerPort: if (touch6340) DEVNET_PROOF_SERVER_PORT else PREVIEW_PROOF_SERVER_PORT,
      devnetPort6340Untouched: not(touch6340),
      manifestSha256Written: true,
      stepOkMarkers: net.stepOkMarkers.union(Set("check"))
    },
    lastOp' = "runCheckStep"
  }

  // Implements PVW-001, PVW-002, SCL-004, SCL-007, SCL-009, WAL-003, WAL-009, WAL-010:
  // Pre-wallet / configuration stop guards (evaluated before starting the wallet).
  action doStopPreWalletOrConfig(stopReason: str): bool = all {
    not(net.walletStarted),
    not(net.refusalsStepStarted),
    net' = {
      ...net,
      walletStarted: false,
      sdkRootCheckStoppedCleanly: (stopReason == "SHERLOCK_SDK_ROOT_unset"),
      verdictState: VerdictError,
      runnerExitCode: 1,
      lastStopReason: stopReason
    },
    lastOp' = "stopPreWalletOrConfig"
  }

  // Implements WAL-009: wallet setup refuses to overwrite an existing phrase file.
  action doAttemptWalletSetupOverwrite(mutTrig: bool): bool = all {
    net.phraseExistsInDir,
    val overwritten = (MUTANT_ID == MUT_WAL009_OVERWRITE_WALLET and mutTrig)
    net' = {
      ...net,
      existingWalletOverwritten: net.existingWalletOverwritten or overwritten,
      lastStopReason: if (overwritten) net.lastStopReason else "a wallet seed already exists"
    },
    lastOp' = "attemptWalletSetupOverwrite"
  }

  // Implements PVW-001, WAL-001, WAL-003, WAL-004, WAL-006, WAL-010:
  // Check permissions (0700 / 0600), genesis pin, and derived address pin, acquire exclusive flock,
  // read phrase in-process only, and write public balance-before record.
  action doStartWalletAndBalanceBefore(
    rpcGen: int,
    dirMode: int,
    fileMode: int,
    derivAddr: int,
    mutConcurrent: bool
  ): bool = all {
    canStartWallet(net, rpcGen, dirMode, fileMode, derivAddr),
    val senders = if (MUTANT_ID == MUT_WAL004_CONCURRENT_SENDERS and mutConcurrent) 2 else 1
    net' = {
      ...net,
      rpcGenesis: rpcGen,
      walletDirMode: dirMode,
      phraseFileMode: fileMode,
      derivedWalletAddr: derivAddr,
      phraseReadInProcess: true,
      phraseExposedToShellOrEnv: false,
      walletStarted: true,
      activeWalletSenders: senders,
      keyMaterialInBalanceFile: false,
      stepOkMarkers: net.stepOkMarkers.union(Set("balance-before"))
    },
    lastOp' = "startWalletAndBalanceBefore"
  }

  // Implements SCL-003: dry-run executes padding, JS shape check, and privacy scan with 0 network connections.
  action doRunSuiteDryRun(mutNet: bool): bool = all {
    val badNet = (MUTANT_ID == MUT_SCL003_DRYRUN_NETWORK and mutNet)
    net' = {
      ...net,
      dryRunExecuted: true,
      dryRunNetworkConnections: if (badNet) 1 else 0,
      dryRunTxSent: if (badNet) 1 else 0
    },
    lastOp' = "runSuiteDryRun"
  }

  // Implements PVW-002, PVW-003, WAL-007, FRC-001, FRC-002:
  // Send or recover single contract deployment on an allowed chain using chain fee parameters from wallet host.
  action doDeployOrRecoverContract(targetGenesis: int, mutTrig: bool): bool = {
    val chainOk =
      (MUTANT_ID == MUT_PVW002_SEND_TO_MAINNET and mutTrig) or isLegalTargetChain(targetGenesis)
    val alreadySent = (net.deploySendCount >= 1)
    val nextSendCnt =
      if (not(alreadySent)) 1
      else if (MUTANT_ID == MUT_FRC002_DOUBLE_DEPLOY and mutTrig) net.deploySendCount + 1
      else net.deploySendCount
    val chainFees = not(MUTANT_ID == MUT_PVW003_INITIAL_FEE_PARAMS and mutTrig)
    val fromWalletBox = not(MUTANT_ID == MUT_WAL007_SUBMIT_FROM_PROVER and mutTrig)
    all {
      net.walletStarted,
      chainOk,
      net' = {
        ...net,
        sentChainGenesises: net.sentChainGenesises.union(Set(targetGenesis)),
        usedChainFeeParams: chainFees,
        submittedFromWalletHost: fromWalletBox,
        deploySendCount: nextSendCnt,
        deployConfirmed: true,
        stepOkMarkers: net.stepOkMarkers.union(Set("deploy"))
      },
      lastOp' = "deployOrRecoverContract"
    }
  }

  // Implements PVW-009, PVW-010, PVW-013, LPC-008:
  // Populate T25 wrap-key gate record before final prototype deployment, then deploy/prove on owned GPU box.
  action doRecordT25KeyGate(allSixPass: bool): bool = all {
    not(net.finalPrototypeDeployed),
    net' = {
      ...net,
      t25KeyGate: {
        rky003RowsOk: true,
        rky004VkBytesOk: true,
        rky005Security100BitOk: allSixPass,
        t12GatePass: true,
        zk012To017Pass: allSixPass,
        wrp012FiveChecksPass: allSixPass
      }
    },
    lastOp' = "recordT25KeyGate"
  }

  action doDeployAndProveFinalPrototype(mutSkipGpuBox: bool): bool = all {
    canDeployFinalPrototype(net),
    val gpuUsed = not(MUTANT_ID == MUT_PVW010_FINAL_WITHOUT_GPU_BOX and mutSkipGpuBox)
    net' = {
      ...net,
      finalPrototypeDeployed: true,
      finalCallProvedOnOwnedGpuBox: gpuUsed
    },
    lastOp' = "deployAndProveFinalPrototype"
  }

  // Implements OBS-001, OBS-003, OBS-004, WAL-002, WAL-005, FRC-001:
  // Include honest call, run privacy scanner (verifying positive control words fire and no private words leak),
  // record readback, and write scrubbed allow-listed receipts.
  action doIncludeHonestCallAndScan(mutTrig: bool): bool = all {
    net.deployConfirmed,
    val badLeakOrDeadWalker = (MUTANT_ID == MUT_OBS003_DEAD_WALKER_OR_LEAK and mutTrig)
    val badUnscrubbed = (MUTANT_ID == MUT_WAL002_UNSCRUBBED_SECRET and mutTrig)
    val badReceiptAllowlist = (MUTANT_ID == MUT_WAL005_PRIVATE_IN_RECEIPTS and mutTrig)
    net' = {
      ...net,
      honestCallIncluded: true,
      honestCallReadbackPresent: true,
      privacyScanPassed: true,
      forbiddenPrivateWordOnSurface: badLeakOrDeadWalker,
      controlWordsPresentOnSurface: not(badLeakOrDeadWalker),
      unscrubbedSecretWritten: badUnscrubbed,
      nonAllowlistFieldInReceipts: badReceiptAllowlist,
      includedReceiptComplete: true,
      stepOkMarkers: net.stepOkMarkers.union(Set("call", "balance-after", "receipts"))
    },
    lastOp' = "includeHonestCallAndScan"
  }

  // Implements PVW-006, PVW-007, PVW-008, PVW-012, SCL-002, OBS-002, FRC-001, FRC-006:
  // Run the 4 tamper controls (R1, R2, R4, R3) and compute run verdict + exit code (0, 4, or 1).
  action doRunTamperControlsAndVerdict(ctrlOut: ControlOutcome, mutTrig: bool): bool = {
    val readbackOk =
      (MUTANT_ID == MUT_FRC006_REFUSALS_NO_READBACK) or net.honestCallReadbackPresent
    val honestPassed = (ctrlOut == CtrlRefusedByNode)
    val passedVerdict =
      if (honestPassed) true
      else if (MUTANT_ID == MUT_PVW007_PASS_NO_NODE_VERDICT and ctrlOut == CtrlNoNodeVerdict and mutTrig) true
      else if (MUTANT_ID == MUT_PVW008_TIMEOUT_AS_REFUSAL and ctrlOut == CtrlNotIncludedTimeout and mutTrig) true
      else if (MUTANT_ID == MUT_PVW012_PASS_INCLUDED_CONTROL and ctrlOut == CtrlIncluded and mutTrig) true
      else false
    val vState = if (passedVerdict) VerdictPassed else VerdictFailed
    val exitSt =
      if (MUTANT_ID == MUT_SCL002_WRONG_EXIT_CODE and mutTrig)
        (if (passedVerdict) 1 else 0)
      else if (passedVerdict) 0
      else EXIT_VERDICT_FAIL
    val badStepOk =
      (MUTANT_ID == MUT_FRC001_MARK_FAILED_STEP_OK and not(passedVerdict) and mutTrig)
    all {
      readbackOk,
      net' = {
        ...net,
        refusalsStepStarted: true,
        submittedControlCount: TAMPER_CONTROL_COUNT,
        controlOutcome: ctrlOut,
        verdictState: vState,
        runnerExitCode: exitSt,
        refusedReceiptComplete: true,
        stepOkMarkers:
          if (passedVerdict or badStepOk) net.stepOkMarkers.union(Set("refusals"))
          else net.stepOkMarkers,
        failedStepsMarkedOk:
          if (badStepOk) net.failedStepsMarkedOk.union(Set("refusals"))
          else net.failedStepsMarkedOk
      },
      lastOp' = "runTamperControlsAndVerdict"
    }
  }

  // Implements PVW-011, LPC-009: Validium batch submission (accepts valid chained root, refuses stale/wrong prog).
  action doSubmitValidiumBatch(progCP: int, batchInputRoot: int, newRoot: int, mutTrig: bool): bool = {
    val batchValid =
      (progCP == net.validiumExpectedProgCP and batchInputRoot == net.validiumStoredRoot)
    val acceptAnyway = (MUTANT_ID == MUT_PVW011_ACCEPT_STALE_VALIDIUM and not(batchValid) and mutTrig)
    all {
      net' =
        if (batchValid or acceptAnyway) {
          ...net,
          validiumStoredRoot: newRoot,
          validiumAcceptedBatches: net.validiumAcceptedBatches + 1,
          validiumAcceptedBadBatch: net.validiumAcceptedBadBatch or not(batchValid)
        } else {
          ...net,
          lastStopReason: "validium_stale_root_or_wrong_program_refused"
        },
      lastOp' = "submitValidiumBatch"
    }
  }

  // Implements FRC-003, FRC-005, FRC-012, FRC-013, FRC-014:
  // Configure saved call state (refused, corrupted, or unknown inclusion) and test reuse/resubmit guards.
  action doSetSavedCallState(
    ttlLeft: int,
    refusedFlag: bool,
    unknownIncFlag: bool,
    hashOkFlag: bool
  ): bool = all {
    net' = {
      ...net,
      savedCall: {
        ...net.savedCall,
        ttlLeftS: ttlLeft,
        markedRefused: refusedFlag,
        inclusionUnknown: unknownIncFlag,
        sha256Valid: hashOkFlag
      }
    },
    lastOp' = "setSavedCallState"
  }

  action doReuseSavedCall: bool = all {
    canReuseSavedCall(net.savedCall),
    val lowTtl = (net.savedCall.ttlLeftS <= MIN_REUSE_TTL_LEFT_S)
    val refused = net.savedCall.markedRefused
    val corrupt =
      not(net.savedCall.sha256Valid) or
      not(net.savedCall.vkMatchesFolder) or
      not(net.savedCall.genesisMatchesFolder)
    net' = {
      ...net,
      reusedLowTtlCall: net.reusedLowTtlCall or lowTtl,
      reusedRefusedCall: net.reusedRefusedCall or refused,
      reusedCorruptOrMismatchedCall: net.reusedCorruptOrMismatchedCall or corrupt
    },
    lastOp' = "reuseSavedCall"
  }

  action doAttemptResubmitClaim(mutTrig: bool): bool = {
    val allowUnknown = (MUTANT_ID == MUT_FRC013_RESUBMIT_UNKNOWN and mutTrig)
    all {
      allowUnknown or not(net.savedCall.inclusionUnknown),
      net' = {
        ...net,
        resubmittedWhileInclusionUnknown:
          net.resubmittedWhileInclusionUnknown or net.savedCall.inclusionUnknown
      },
      lastOp' = "attemptResubmitClaim"
    }
  }

  // Implements FRC-004, FRC-016, OBS-008:
  // Record wallet balancing failure retries (<= 4 tries with resync on transport error; 1 try on non-transport).
  action doRecordWalletBalanceFailure(isTransport: bool, tries: int, mutTrig: bool): bool = {
    val validRetries =
      if (MUTANT_ID == MUT_FRC004_EXCESS_OR_BAD_RETRY and mutTrig) (tries >= 1)
      else if (isTransport) (tries >= 1 and tries <= MAX_BALANCE_TRANSPORT_TRIES)
      else (tries == 1)
    all {
      validRetries,
      net' = {
        ...net,
        balanceIsTransportErr: isTransport,
        balanceAttemptCount: tries,
        resyncedBeforeEachRetry: isTransport,
        walletFailureChainRecorded: true,
        lastStopReason: if (isTransport) "transport_retries_exhausted" else "non_transport_balance_stopped"
      },
      lastOp' = "recordWalletBalanceFailure"
    }
  }

  // Implements FRC-009: two consecutive gate failures stop that row while other rows continue.
  action doRecordGateFailureOnRow: bool = {
    val nextFails = net.rowConsecutiveGateFails + 1
    val shouldStop = (nextFails >= MAX_CONSECUTIVE_GATE_FAILS)
    all {
      net' = {
        ...net,
        rowConsecutiveGateFails: nextFails,
        rowStoppedOnDoubleFail: net.rowStoppedOnDoubleFail or shouldStop,
        otherRowsStillRunning: true
      },
      lastOp' = "recordGateFailureOnRow"
    }
  }

  // Implements FRC-011, LPC-015: branch push gating on FP-3 (for t10q) and T15 sign-off (for all other branches).
  action doAdvancePushGates(passFp3: bool, signT15: bool): bool = all {
    net' = {
      ...net,
      fp3Passed: net.fp3Passed or passFp3,
      t15OwnerSigned: net.t15OwnerSigned or signT15
    },
    lastOp' = "advancePushGates"
  }

  action doPushBranch(branchName: str): bool = all {
    canPushBranch(net, branchName),
    net' = {
      ...net,
      pushedBranches: net.pushedBranches.union(Set(branchName))
    },
    lastOp' = "pushBranch"
  }

  // Implements adopted SWF-009: T4 Mainnet operation gated on all 4 amendment conditions,
  // per-operation owner+checklist authorization, and zero automatic retries.
  action doRunMainnetT4Operation(all4Ok: bool, ownerAuth: bool, autoRetried: bool): bool = all {
    canRunMainnetT4Op(all4Ok, ownerAuth, autoRetried),
    net' = {
      ...net,
      mainnetOpRan: true,
      mainnetAll4PreconditionsMet: all4Ok,
      mainnetExplicitPerOpOwnerAuth: ownerAuth,
      mainnetAutoRetried: autoRetried
    },
    lastOp' = "runMainnetT4Operation"
  }

  // =========================================================================
  // Nondeterministic step relation
  // =========================================================================

  action step = any {
    // Check step & pre-wallet stops (PVW-001, PVW-004, SCL-004, SCL-007, SCL-009, OBS-006)
    {
      val ttlPool =
        if (MUTANT_ID == MUT_SCL007_OUT_OF_RANGE_TTL) Set(0, MAX_TTL_EXCLUSIVE_S, 2000000)
        else Set(1, 3600, MAX_TTL_EXCLUSIVE_S - 1)
      nondet tVal = ttlPool.oneOf()
      val vkPool =
        if (MUTANT_ID == MUT_SCL004_MISMATCHED_VK_FOLDER) Set(1, 2)
        else Set(1)
      nondet cliV = vkPool.oneOf()
      nondet mutP = Set(true, false).oneOf()
      doRunCheckStep(tVal, 1, cliV, mutP)
    },
    {
      nondet sReason = Set(
        "the RPC genesis is not the pinned Preview genesis",
        "T10A_TTL_S out of range",
        "SHERLOCK_SDK_ROOT_unset"
      ).oneOf()
      doStopPreWalletOrConfig(sReason)
    },

    // Wallet setup, permissions, pin, and balance-before (WAL-001..010)
    {
      nondet mutTrig = Set(true, false).oneOf()
      doAttemptWalletSetupOverwrite(mutTrig)
    },
    {
      val genPool =
        if (MUTANT_ID == MUT_PVW001_WRONG_GENESIS_WALLET) Set(PINNED_PREVIEW_GENESIS, 8888)
        else Set(PINNED_PREVIEW_GENESIS)
      nondet gId = genPool.oneOf()
      val fModePool =
        if (MUTANT_ID == MUT_WAL010_WIDE_MODE_READ) Set(MAX_PHRASE_FILE_MODE, 640)
        else Set(MAX_PHRASE_FILE_MODE)
      nondet fMode = fModePool.oneOf()
      val addrPool =
        if (MUTANT_ID == MUT_WAL003_UNPINNED_WALLET) Set(77, 99)
        else Set(77)
      nondet dAddr = addrPool.oneOf()
      nondet mutConc = Set(true, false).oneOf()
      doStartWalletAndBalanceBefore(gId, MAX_WALLET_DIR_MODE, fMode, dAddr, mutConc)
    },

    // Dry run & contract deploy (PVW-002, PVW-003, SCL-003, WAL-007, FRC-002)
    {
      nondet mutN = Set(true, false).oneOf()
      doRunSuiteDryRun(mutN)
    },
    {
      val chPool =
        if (MUTANT_ID == MUT_PVW002_SEND_TO_MAINNET)
          Set(PINNED_PREVIEW_GENESIS, LOCAL_DEVNET_GENESIS, 9999)
        else
          Set(PINNED_PREVIEW_GENESIS, LOCAL_DEVNET_GENESIS)
      nondet chId = chPool.oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doDeployOrRecoverContract(chId, mutTrig)
    },

    // T25 final deploy gate & GPU box proving (PVW-009, PVW-010, PVW-013, LPC-008)
    {
      nondet fullOk = Set(true, false).oneOf()
      doRecordT25KeyGate(fullOk)
    },
    {
      nondet mutGpu = Set(true, false).oneOf()
      doDeployAndProveFinalPrototype(mutGpu)
    },

    // Honest call, privacy scan, and 4 tamper controls (PVW-006..008, PVW-012, SCL-002, OBS-001..004, FRC-001, FRC-006)
    {
      nondet mutTrig = Set(true, false).oneOf()
      doIncludeHonestCallAndScan(mutTrig)
    },
    {
      nondet cOut = Set(
        CtrlRefusedByNode,
        CtrlIncluded,
        CtrlNotIncludedTimeout,
        CtrlNoNodeVerdict
      ).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doRunTamperControlsAndVerdict(cOut, mutTrig)
    },

    // Validium batches (PVW-011, LPC-009)
    {
      nondet staleOrHonest = Set(true, false).oneOf()
      val inRoot = if (staleOrHonest) net.validiumStoredRoot else 0
      nondet mutTrig = Set(true, false).oneOf()
      doSubmitValidiumBatch(1007, inRoot, net.validiumStoredRoot + 1, mutTrig)
    },

    // Recovery: saved call reuse, unknown inclusion, balance retries, double gate fail, push holds (FRC-003..016, LPC-015)
    {
      nondet ttlL = Set(MIN_REUSE_TTL_LEFT_S, MIN_REUSE_TTL_LEFT_S + 1, 2400).oneOf()
      nondet refFlag = Set(false, true).oneOf()
      nondet unkFlag = Set(false, true).oneOf()
      nondet hOk = Set(true, false).oneOf()
      doSetSavedCallState(ttlL, refFlag, unkFlag, hOk)
    },
    doReuseSavedCall,
    {
      nondet mutTrig = Set(true, false).oneOf()
      doAttemptResubmitClaim(mutTrig)
    },
    {
      nondet isTr = Set(true, false).oneOf()
      val tryPool =
        if (MUTANT_ID == MUT_FRC004_EXCESS_OR_BAD_RETRY)
          Set(1, MAX_BALANCE_TRANSPORT_TRIES + 1)
        else if (isTr)
          Set(1, MAX_BALANCE_TRANSPORT_TRIES)
        else
          Set(1)
      nondet trCnt = tryPool.oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doRecordWalletBalanceFailure(isTr, trCnt, mutTrig)
    },
    doRecordGateFailureOnRow,
    {
      nondet pFp3 = Set(true, false).oneOf()
      nondet sT15 = Set(false, true).oneOf()
      doAdvancePushGates(pFp3, sT15)
    },
    {
      nondet br = Set("t10q", "sherlock-e2e-spec").oneOf()
      doPushBranch(br)
    },
    {
      nondet all4 = Set(true, false).oneOf()
      nondet ownA = Set(true, false).oneOf()
      nondet autoR = Set(false, true).oneOf()
      doRunMainnetT4Operation(all4, ownA, autoR)
    }
  }

  // =========================================================================
  // Safety invariants (one inv_<ID> per expressible stateful requirement)
  // =========================================================================

  val inv_PVW_001: bool =
    net.walletStarted implies (net.rpcGenesis == PINNED_PREVIEW_GENESIS)

  val inv_PVW_002: bool =
    net.sentChainGenesises.forall(g => isLegalTargetChain(g))

  val inv_PVW_003: bool =
    (net.deploySendCount >= 1) implies net.usedChainFeeParams

  val inv_PVW_004: bool =
    net.proofServerPort == PREVIEW_PROOF_SERVER_PORT and net.devnetPort6340Untouched

  val inv_PVW_006: bool =
    net.refusalsStepStarted implies (net.submittedControlCount == TAMPER_CONTROL_COUNT)

  val inv_PVW_007: bool =
    (net.refusalsStepStarted and net.controlOutcome == CtrlNoNodeVerdict) implies
      (net.verdictState == VerdictFailed)

  val inv_PVW_008: bool =
    (net.refusalsStepStarted and net.controlOutcome == CtrlNotIncludedTimeout) implies
      (net.verdictState == VerdictFailed)

  val inv_PVW_009: bool =
    net.finalPrototypeDeployed implies net.t25KeyGate.t12GatePass

  val inv_PVW_010: bool =
    net.finalPrototypeDeployed implies net.finalCallProvedOnOwnedGpuBox

  val inv_PVW_011: bool =
    not(net.validiumAcceptedBadBatch)

  val inv_PVW_012: bool =
    (net.refusalsStepStarted and net.controlOutcome == CtrlIncluded) implies
      (net.verdictState == VerdictFailed and net.runnerExitCode == EXIT_VERDICT_FAIL)

  val inv_PVW_013: bool =
    net.finalPrototypeDeployed implies fullKeyGatePassed(net.t25KeyGate)

  val inv_WAL_001: bool =
    not(net.phraseExposedToShellOrEnv)

  val inv_WAL_002: bool =
    not(net.unscrubbedSecretWritten)

  val inv_WAL_003: bool =
    net.walletStarted implies (net.derivedWalletAddr == net.pinnedWalletAddr)

  val inv_WAL_004: bool =
    net.activeWalletSenders <= 1

  val inv_WAL_005: bool =
    not(net.nonAllowlistFieldInReceipts)

  val inv_WAL_006: bool =
    not(net.keyMaterialInBalanceFile)

  val inv_WAL_007: bool =
    (net.deploySendCount >= 1) implies net.submittedFromWalletHost

  val inv_WAL_008: bool =
    net.secretsInRepoCount == 0

  val inv_WAL_009: bool =
    not(net.existingWalletOverwritten)

  val inv_WAL_010: bool =
    net.phraseReadInProcess implies (
      net.walletDirMode <= MAX_WALLET_DIR_MODE and
      net.phraseFileMode <= MAX_PHRASE_FILE_MODE
    )

  val inv_SCL_002: bool =
    ((net.verdictState == VerdictPassed) implies (net.runnerExitCode == 0)) and
    ((net.verdictState == VerdictFailed) implies (net.runnerExitCode == EXIT_VERDICT_FAIL)) and
    ((net.verdictState == VerdictError) implies (net.runnerExitCode == 1))

  val inv_SCL_003: bool =
    net.dryRunExecuted implies (net.dryRunNetworkConnections == 0 and net.dryRunTxSent == 0)

  val inv_SCL_004: bool =
    net.folderAccepted implies (net.folderRecordedVkId == net.cliVkId)

  val inv_SCL_007: bool =
    net.ttlAccepted implies (net.configuredTtlS > 0 and net.configuredTtlS < MAX_TTL_EXCLUSIVE_S)

  val inv_SCL_010: bool =
    net.outsideFactoryCleanConfigOk

  val inv_OBS_001: bool =
    net.includedReceiptComplete

  val inv_OBS_002: bool =
    net.refusedReceiptComplete

  val inv_OBS_003: bool =
    net.privacyScanPassed implies not(net.forbiddenPrivateWordOnSurface)

  val inv_OBS_004: bool =
    net.privacyScanPassed implies net.controlWordsPresentOnSurface

  val inv_OBS_006: bool =
    net.stepOkMarkers.contains("check") implies net.manifestSha256Written

  val inv_OBS_008: bool =
    net.walletFailureChainRecorded

  val inv_FRC_001: bool =
    net.failedStepsMarkedOk == Set()

  val inv_FRC_002: bool =
    net.deploySendCount <= 1

  val inv_FRC_003: bool =
    not(net.reusedLowTtlCall)

  // Covers FRC-004 (<= 4 transport attempts with resync) and FRC-016 (1 attempt on non-transport error).
  val inv_FRC_004: bool =
    if (net.balanceIsTransportErr)
      (net.balanceAttemptCount >= 1 and net.balanceAttemptCount <= MAX_BALANCE_TRANSPORT_TRIES and net.resyncedBeforeEachRetry)
    else
      (net.balanceAttemptCount == 1)

  val inv_FRC_005: bool =
    not(net.reusedRefusedCall)

  val inv_FRC_006: bool =
    net.refusalsStepStarted implies net.honestCallReadbackPresent

  val inv_FRC_009: bool =
    (net.rowConsecutiveGateFails >= MAX_CONSECUTIVE_GATE_FAILS) implies
      (net.rowStoppedOnDoubleFail and net.otherRowsStillRunning)

  val inv_FRC_011: bool =
    net.pushedBranches.forall(br =>
      (br != "t10q") implies net.t15OwnerSigned
    )

  val inv_LPC_015: bool =
    inv_FRC_011 and
    (net.pushedBranches.contains("t10q") implies (net.fp3Passed or net.t15OwnerSigned))

  val inv_FRC_012: bool =
    net.suiteSkippedSettledCallsOk

  val inv_FRC_013: bool =
    not(net.resubmittedWhileInclusionUnknown)

  val inv_FRC_014: bool =
    not(net.reusedCorruptOrMismatchedCall)

  val inv_SWF_009: bool =
    net.mainnetOpRan implies (
      net.mainnetAll4PreconditionsMet and
      net.mainnetExplicitPerOpOwnerAuth and
      not(net.mainnetAutoRetried)
    )

  // Master safety invariant.
  val inv: bool =
    consistency_constants and
    inv_PVW_001 and
    inv_PVW_002 and
    inv_PVW_003 and
    inv_PVW_004 and
    inv_PVW_006 and
    inv_PVW_007 and
    inv_PVW_008 and
    inv_PVW_009 and
    inv_PVW_010 and
    inv_PVW_011 and
    inv_PVW_012 and
    inv_PVW_013 and
    inv_WAL_001 and
    inv_WAL_002 and
    inv_WAL_003 and
    inv_WAL_004 and
    inv_WAL_005 and
    inv_WAL_006 and
    inv_WAL_007 and
    inv_WAL_008 and
    inv_WAL_009 and
    inv_WAL_010 and
    inv_SCL_002 and
    inv_SCL_003 and
    inv_SCL_004 and
    inv_SCL_007 and
    inv_SCL_010 and
    inv_OBS_001 and
    inv_OBS_002 and
    inv_OBS_003 and
    inv_OBS_004 and
    inv_OBS_006 and
    inv_OBS_008 and
    inv_FRC_001 and
    inv_FRC_002 and
    inv_FRC_003 and
    inv_FRC_004 and
    inv_FRC_005 and
    inv_FRC_006 and
    inv_FRC_009 and
    inv_FRC_011 and
    inv_LPC_015 and
    inv_FRC_012 and
    inv_FRC_013 and
    inv_FRC_014 and
    inv_SWF_009

  // =========================================================================
  // Reachability witnesses (negated goals — MUST be violated under network_operations_main)
  // =========================================================================

  // Witness 1: Full Preview T17 flow succeeds (check -> wallet -> deploy -> call+scan -> 4 controls refused -> exit 0).
  val reach_full_preview_pass_verdict: bool = not(
    net.stepOkMarkers.contains("check") and
    net.stepOkMarkers.contains("deploy") and
    net.stepOkMarkers.contains("call") and
    net.stepOkMarkers.contains("refusals") and
    net.verdictState == VerdictPassed and
    net.runnerExitCode == 0
  )

  // Witness 2: Included tamper control or timeout/no-node-verdict fails verdict with exit code 4.
  val reach_failed_control_verdict_exit_4: bool = not(
    net.refusalsStepStarted and
    net.verdictState == VerdictFailed and
    net.runnerExitCode == EXIT_VERDICT_FAIL
  )

  // Witness 3: All 6 T25 key gates pass and final prototype deploys & proves on owned GPU box.
  val reach_final_t25_deploy_and_gpu_prove: bool = not(
    net.finalPrototypeDeployed and net.finalCallProvedOnOwnedGpuBox
  )

  // Witness 4: Validium demo settles >= 2 chained batches and refuses stale roots (PVW-011, LPC-009).
  val reach_validium_two_chained_batches_and_stale_refusal: bool = not(
    net.validiumAcceptedBatches >= 2 and
    net.lastStopReason == "validium_stale_root_or_wrong_program_refused"
  )

  // Witness 5: Suite dry-run with 0 network connections, wallet no-overwrite, and double-gate row stop.
  val reach_dryrun_no_overwrite_and_double_fail_stop: bool = not(
    net.dryRunExecuted and
    net.lastStopReason == "a wallet seed already exists" and
    net.rowStoppedOnDoubleFail
  )

  // Witness 6: t10q push unlocked after FP-3 while non-t10q branches wait for T15 owner sign-off.
  val reach_fp3_t10q_push_and_balance_failure_stops: bool = not(
    net.pushedBranches.contains("t10q") and
    not(net.t15OwnerSigned)
  )
}

// ===========================================================================
// Concrete instances: main, edge instances, and guard mutants
// ===========================================================================

module network_operations_main {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096,
    LOCAL_DEVNET_GENESIS = 1001,
    PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340,
    TAMPER_CONTROL_COUNT = 4,
    INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700,
    MAX_PHRASE_FILE_MODE = 600,
    EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600,
    MIN_REUSE_TTL_LEFT_S = 300,
    MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2,
    MUTANT_ID = 0
  ).*
}

// Smallest legal retry/timeout/permission values: MAX_BALANCE_TRANSPORT_TRIES = 1, INBLOCK_TIMEOUT_S = 60,
// MAX_WALLET_DIR_MODE = 500, MAX_PHRASE_FILE_MODE = 400.
module network_operations_edge_min {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096,
    LOCAL_DEVNET_GENESIS = 1001,
    PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340,
    TAMPER_CONTROL_COUNT = 4,
    INBLOCK_TIMEOUT_S = 60,
    MAX_WALLET_DIR_MODE = 500,
    MAX_PHRASE_FILE_MODE = 400,
    EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600,
    MIN_REUSE_TTL_LEFT_S = 300,
    MAX_BALANCE_TRANSPORT_TRIES = 1,
    MAX_CONSECUTIVE_GATE_FAILS = 2,
    MUTANT_ID = 0
  ).*
}

// Exact spec ceiling bounds (INBLOCK_TIMEOUT_S = 600, MAX_BALANCE_TRANSPORT_TRIES = 4, MAX_TTL_EXCLUSIVE_S = 1,209,600).
module network_operations_edge_at_limit {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096,
    LOCAL_DEVNET_GENESIS = 1001,
    PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340,
    TAMPER_CONTROL_COUNT = 4,
    INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700,
    MAX_PHRASE_FILE_MODE = 600,
    EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600,
    MIN_REUSE_TTL_LEFT_S = 300,
    MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2,
    MUTANT_ID = 0
  ).*
}

// Mutants (each removes or weakens one refusal/binding guard and must break its named invariant)

module network_operations_m_pvw001_wrong_genesis_wallet {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 1
  ).*
}

module network_operations_m_pvw002_send_to_mainnet {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 2
  ).*
}

module network_operations_m_pvw003_initial_fee_params {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 3
  ).*
}

module network_operations_m_pvw004_touch_port_6340 {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 4
  ).*
}

module network_operations_m_pvw007_pass_no_node_verdict {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 5
  ).*
}

module network_operations_m_pvw008_timeout_as_refusal {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 6
  ).*
}

module network_operations_m_pvw010_final_without_gpu_box {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 7
  ).*
}

module network_operations_m_pvw011_accept_stale_validium {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 8
  ).*
}

module network_operations_m_pvw012_pass_included_control {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 9
  ).*
}

module network_operations_m_pvw013_deploy_missing_gate {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 10
  ).*
}

module network_operations_m_wal002_unscrubbed_secret {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 11
  ).*
}

module network_operations_m_wal003_unpinned_wallet {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 12
  ).*
}

module network_operations_m_wal004_concurrent_senders {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 13
  ).*
}

module network_operations_m_wal005_private_in_receipts {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 14
  ).*
}

module network_operations_m_wal007_submit_from_prover {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 15
  ).*
}

module network_operations_m_wal009_overwrite_wallet {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 16
  ).*
}

module network_operations_m_wal010_wide_mode_read {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 17
  ).*
}

module network_operations_m_scl002_wrong_exit_code {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 18
  ).*
}

module network_operations_m_scl003_dryrun_network {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 19
  ).*
}

module network_operations_m_scl004_mismatched_vk_folder {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 20
  ).*
}

module network_operations_m_scl007_out_of_range_ttl {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 21
  ).*
}

module network_operations_m_obs003_dead_walker_or_leak {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 22
  ).*
}

module network_operations_m_frc001_mark_failed_step_ok {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 23
  ).*
}

module network_operations_m_frc002_double_deploy {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 24
  ).*
}

module network_operations_m_frc004_excess_or_bad_retry {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 25
  ).*
}

module network_operations_m_frc005_reuse_refused_call {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 26
  ).*
}

module network_operations_m_frc006_refusals_no_readback {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 27
  ).*
}

module network_operations_m_frc011_push_before_t15 {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 28
  ).*
}

module network_operations_m_frc013_resubmit_unknown {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 29
  ).*
}

module network_operations_m_frc014_reuse_corrupt_file {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 30
  ).*
}

module network_operations_m_frc003_reuse_low_ttl {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 31
  ).*
}

module network_operations_m_swf009_unauth_mainnet {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096, LOCAL_DEVNET_GENESIS = 1001, PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340, TAMPER_CONTROL_COUNT = 4, INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700, MAX_PHRASE_FILE_MODE = 600, EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600, MIN_REUSE_TTL_LEFT_S = 300, MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2, MUTANT_ID = 32
  ).*
}
