// -*- mode: Bluespec; -*-
// Independent Quint specification for bounded context Settlement (lane e2e-settlement).
// Sources:
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-ledger-contract/spec.md
//     (SLC-001..012)
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-batched-settlement/spec.md
//     (BAT-001..012)
//   - openspec/changes/sherlock-completion-20261115/specs/sherlock-suite-settlement/spec.md
//     (STL-001..006)
//   - openspec/changes/sherlock-modular-leaf-20261111/specs/sherlock-ledger-contract/spec.md
//     (adopted LDG-001..005)
//   - openspec/changes/sherlock-processor-final-20261111/specs/sherlock-zero-knowledge/spec.md
//     (adopted ZK-006, ZK-011)
//   - openspec/changes/sherlock-e2e-prototype-20261115/architecture/DDD.md
//     (Settlement rows in sections 1..4 and 6..8)

module settlement {
  // =========================================================================
  // Constants (numeric bounds and protocol parameters from the EARS specs)
  // =========================================================================

  // SLC-001: SherlockLedger exports exactly 4 entries.
  const CONTRACT_ENTRY_COUNT: int
  // SLC-002, STL-003: sealed top-level ledger slot holding c_p (15).
  const ANCHOR_SLOT: int
  // SLC-006: StatementPublics holds 4 public words (c_p, input_pub, outcome_pub, claim_pub).
  const STATEMENT_PUBLIC_WORDS: int
  // SLC-008, ZK-006: 8-word CSPRNG run salt inside input_d.
  const RUN_SALT_WORDS: int
  // BAT-001: maximum open duration of an epoch in seconds (3,600 s).
  const EPOCH_WINDOW_S: int
  // BAT-001: maximum claims in one epoch (1,048,576 = 2^20 in main).
  const MAX_CLAIMS_PER_EPOCH: int
  // BAT-005: maximum chunk count per entry at every TTL (K_MAX = 2).
  const K_MAX_CHUNKS: int
  // BAT-006: default Preview intent TTL when unset by user (3,600 s).
  const DEFAULT_INTENT_TTL_S: int
  // BAT-007, LDG-004, LDG-005: binary Merkle claims tree depth (D_C = 20).
  const CLAIMS_TREE_DEPTH: int
  // BAT-008: off-chain published leaf byte width per claim (32 bytes).
  const CLAIM_LEAF_BYTES: int
  // BAT-010: collection proof time budget in seconds (176 s ~ 175.5 s warm).
  const COLLECTION_PROOF_TIME_S: int
  // BAT-010, BAT-011: transaction inclusion time budget in seconds (60 s).
  const INCLUSION_TIME_S: int
  // BAT-010: one block interval margin in seconds (6 s).
  const BLOCK_INTERVAL_S: int
  // Mutant selector: 0 = honest specification; 1..25 = individual guard mutants.
  const MUTANT_ID: int

  // =========================================================================
  // Mutant identifiers
  // =========================================================================

  pure val MUT_NONE: int = 0
  pure val MUT_SLC001_OPEN_CLAIM: int = 1
  pure val MUT_SLC002_UNSEALED_ANCHOR: int = 2
  pure val MUT_SLC003_WRONG_WRITE_VK: int = 3
  pure val MUT_SLC004_WRAP_READ_VK: int = 4
  pure val MUT_SLC005_CROSS_PROG_SETTLE: int = 5
  pure val MUT_SLC008_UNSALTED_INPUT: int = 6
  pure val MUT_SLC009_IGNORE_LANE_MISMATCH: int = 7
  pure val MUT_SLC010_SETTLE_NATIVE_REFUSED: int = 8
  pure val MUT_SLC011_REUSE_DIRTY_INSTANCE: int = 9
  pure val MUT_SLC012_SETTLE_UNBOUND_STMT: int = 10
  pure val MUT_BAT001_LATE_EPOCH_CLOSE: int = 11
  pure val MUT_BAT002_OPERATOR_ACCEPT_RUN: int = 12
  pure val MUT_BAT005_ALLOW_3_CHUNKS: int = 13
  pure val MUT_BAT006_DEFAULT_TTL_14400: int = 14
  pure val MUT_BAT007_NO_SUBMITTER_PATHS: int = 15
  pure val MUT_BAT008_WITHHOLD_LIST: int = 16
  pure val MUT_BAT009_COLL_WITHOUT_L_KEY: int = 17
  pure val MUT_BAT010_EPOCH_PAST_ERA_END: int = 18
  pure val MUT_BAT011_LATE_ERA_RUN: int = 19
  pure val MUT_BAT012_OMIT_OPEN_T15: int = 20
  pure val MUT_STL003_BAD_VK_READBACK: int = 21
  pure val MUT_LDG002_REPLAY_CLAIM: int = 22
  pure val MUT_LDG003_STALE_REG_OR_DUP_ROOT: int = 23
  pure val MUT_LDG004_ROOT_NULLIFIER_DOUBLE_USE: int = 24
  pure val MUT_STL005_HIDE_MISMATCH: int = 25

  // =========================================================================
  // Domain types
  // =========================================================================

  type SuiteCallClass =
    | ClassPass
    | ClassRefusedAsNative
    | ClassMismatch
    | ClassBlocked

  type ProgramLedgerRecord = {
    instanceId: int,
    progId: int,
    exportedEntries: Set[str],
    anchorSlot: int,
    anchorSealedInConstructor: bool,
    anchorCP: int,
    writeEntriesVkId: int,
    readEntriesKeepNativeVk: bool,
    isT25PreviewDeploy: bool,
    readbackVkMatchesExpected: bool
  }

  type BuiltArgsRecord = {
    argId: int,
    progId: int,
    cP: int,
    inputD: int,
    outcomeD: int,
    inputPub: int,
    outcomePub: int,
    claimPub: int,
    publicWordCount: int,
    fuelVal: int,
    stackLimitVal: int,
    saltWordCount: int,
    saltLeakedPublic: bool,
    callVectorLanesMatch: bool,
    nativeRefused: bool,
    leafStmtBound: bool,
    isOwnerSubmitter: bool,
    argsFileWritten: bool
  }

  type SettledRunRecord = {
    claimPub: int,
    cP: int,
    inputPub: int,
    outcomePub: int,
    settledLedgerProgId: int,
    callProgId: int,
    acceptCount: int,
    wasNativeRefused: bool,
    wasLeafBound: bool,
    wasOwnerSubmitted: bool
  }

  type ClosedEpochRecord = {
    epochId: int,
    firstJobTimeS: int,
    closeTimeS: int,
    claimCount: int,
    claimsRoot: int,
    memberClaims: Set[int],
    pathsDelivered: bool,
    deliveredSiblingCount: int,
    leafListPublishedOffChain: bool,
    leafBytesPerClaim: int,
    eraEndAnnouncedAtClose: bool,
    eraEndEffAtCloseS: int,
    collectionSubmitted: bool,
    operatorAcceptRunSent: bool
  }

  type AcceptedCollectionRecord = {
    claimsRoot: int,
    regPubValidInHistory: bool,
    acceptCount: int,
    perClaimLedgerWrites: int,
    leakedShapeOrSetRoot: bool,
    lCollectionKeyPresentAtSubmit: bool
  }

  type ChunkPlanRecord = {
    entryId: int,
    neededChunks: int,
    intentTtlS: int,
    admitted: bool
  }

  type PreviewIntentRecord = {
    intentId: int,
    userGaveTtl: bool,
    assignedTtlS: int
  }

  type SuiteReplayRecord = {
    replayId: int,
    progId: int,
    stateEqualsDeployBlockAtStart: bool,
    deployedFreshInstance: bool,
    reusedDirtyInstance: bool
  }

  type EraRunAttemptRecord = {
    runAttemptId: int,
    startTimeS: int,
    proveTimeS: int,
    endEffS: int,
    eraEndWasAnnounced: bool,
    started: bool
  }

  type SuiteClassRecord = {
    suiteCallId: int,
    nativeRefused: bool,
    settledDiffersFromNativeT18: bool,
    assignedClass: SuiteCallClass,
    acceptRunSent: bool
  }

  type SettleState = {
    nowS: int,
    universalWrapVkId: int,
    rekeyAndT12Passed: bool,
    hasLCollectionKey: bool,
    eraEndAnnounced: bool,
    eraEndEffS: int,

    nextInstanceId: int,
    ledgers: int -> ProgramLedgerRecord,
    knownLedgerProgIds: Set[int],

    builtArgs: int -> BuiltArgsRecord,
    knownArgIds: Set[int],

    settledRuns: int -> SettledRunRecord,
    settledClaimPubs: Set[int],

    epochOpen: bool,
    openEpochId: int,
    openEpochFirstJobS: int,
    openEpochClaims: Set[int],
    closedEpochs: Set[ClosedEpochRecord],

    acceptedCollections: int -> AcceptedCollectionRecord,
    acceptedRoots: Set[int],

    consumerUsedClaims: Set[int],
    consumerDoubleUseOccurred: bool,
    consumerLedgerWritesOnUse: int,

    chunkPlans: Set[ChunkPlanRecord],
    previewIntents: Set[PreviewIntentRecord],
    suiteReplays: Set[SuiteReplayRecord],
    eraRunAttempts: Set[EraRunAttemptRecord],
    suiteClassifications: Set[SuiteClassRecord],

    t15FreezeWritten: bool,
    t15ListsCollectionOpen: bool,
    lastRefusal: str
  }

  // =========================================================================
  // Observable state variables
  // =========================================================================

  var stl: SettleState
  var lastOp: str

  // =========================================================================
  // Pure constants & cryptographic/encoding domain helpers
  // =========================================================================

  pure val PROTOTYPE_ENTRIES: Set[str] = Set(
    "acceptRun",
    "acceptCollection",
    "rootAccepted",
    "runAccepted"
  )

  // Abstract injective hashes matching ZK-006, ZK-011, SLC-006..008, LDG-002.
  pure def deriveCP(progId: int, deploySaltRp: int): int =
    progId * 1000 + deploySaltRp

  pure def deriveInputD(runSalt: int, tapeValD: int, fuelVal: int, stackLim: int): int =
    runSalt * 100000 + tapeValD * 100 + fuelVal * 10 + stackLim

  pure def deriveInputPub(inputD: int): int =
    7000000 + inputD

  // SLC-007: Halted([comm, E_0 .. E_(L_e-1)]) in call order.
  pure def deriveOutcomeD(commWord: int, effectSeqDigest: int): int =
    commWord * 1000 + effectSeqDigest

  // ZK-011: outcome_pub = PoseidonFr_4(SHK_OUTCOME, pack(P2(DOM_OUTHIDE, input_d, outcome_d))).
  pure def deriveOutcomePub(inputD: int, outcomeD: int): int =
    9000000 + inputD * 31 + outcomeD

  // LDG-002: claim_pub = transientHash([C_P, input_pub, outcome_pub]).
  pure def deriveClaimPub(cP: int, inputPub: int, outcomePub: int): int =
    cP * 17 + inputPub * 13 + outcomePub

  // BAT-010: latest epoch close time before an announced era end end_eff.
  pure def eraEpochCloseCutoff(endEff: int): int =
    endEff - (COLLECTION_PROOF_TIME_S + INCLUSION_TIME_S + BLOCK_INTERVAL_S)

  // =========================================================================
  // Guard predicates (exposed for scenario tests in settlement_test.qnt without .fail())
  // =========================================================================

  pure def canDeployLedger(
    s: SettleState,
    pId: int,
    forT25Preview: bool,
    readbackMatches: bool
  ): bool = {
    val t25GatesOk = not(forT25Preview) or s.rekeyAndT12Passed
    val readbackOk =
      (MUTANT_ID == MUT_STL003_BAD_VK_READBACK) or not(forT25Preview) or readbackMatches
    not(s.knownLedgerProgIds.contains(pId)) and t25GatesOk and readbackOk
  }

  pure def canBuildAcceptArgs(
    lanesMatch: bool,
    saltWords: int,
    fuelVal: int,
    stackLim: int,
    saltLeaked: bool
  ): bool = {
    val lanesOk = (MUTANT_ID == MUT_SLC009_IGNORE_LANE_MISMATCH) or lanesMatch
    val saltFuelOk =
      (MUTANT_ID == MUT_SLC008_UNSALTED_INPUT) or
      (saltWords == RUN_SALT_WORDS and fuelVal == 0 and stackLim == 0 and not(saltLeaked))
    lanesOk and saltFuelOk
  }

  pure def canAcceptRun(s: SettleState, ledgerProgId: int, argId: int): bool =
    s.knownLedgerProgIds.contains(ledgerProgId) and
    s.knownArgIds.contains(argId) and {
      val ldg = s.ledgers.get(ledgerProgId)
      val a = s.builtArgs.get(argId)
      val anchorMatchOk =
        (MUTANT_ID == MUT_SLC005_CROSS_PROG_SETTLE) or (ldg.anchorCP == a.cP)
      val notNativeRefusedOk =
        (MUTANT_ID == MUT_SLC010_SETTLE_NATIVE_REFUSED) or not(a.nativeRefused)
      val leafBoundOk =
        (MUTANT_ID == MUT_SLC012_SETTLE_UNBOUND_STMT) or a.leafStmtBound
      val ownerRoleOk =
        (MUTANT_ID == MUT_BAT002_OPERATOR_ACCEPT_RUN) or a.isOwnerSubmitter
      val freshClaimOk =
        (MUTANT_ID == MUT_LDG002_REPLAY_CLAIM) or not(s.settledClaimPubs.contains(a.claimPub))
      a.argsFileWritten and
        anchorMatchOk and
        notNativeRefusedOk and
        leafBoundOk and
        ownerRoleOk and
        freshClaimOk
    }

  pure def canPlanEntryChunks(neededChunks: int, ttlS: int): bool =
    ttlS > 0 and (
      if (MUTANT_ID == MUT_BAT005_ALLOW_3_CHUNKS)
        neededChunks <= ttlS / 1755
      else
        (neededChunks >= 1 and neededChunks <= K_MAX_CHUNKS)
    )

  pure def canAcceptCollection(s: SettleState, rootVal: int, regPubValid: bool): bool = {
    val epochExists = s.closedEpochs.exists(e => e.claimsRoot == rootVal)
    val lKeyOk = (MUTANT_ID == MUT_BAT009_COLL_WITHOUT_L_KEY) or s.hasLCollectionKey
    val regAndFreshOk =
      (MUTANT_ID == MUT_LDG003_STALE_REG_OR_DUP_ROOT) or
      (regPubValid and not(s.acceptedRoots.contains(rootVal)))
    epochExists and lKeyOk and regAndFreshOk
  }

  pure def canStartEraRun(s: SettleState, proveTimeS: int): bool =
    proveTimeS > 0 and (
      if (not(s.eraEndAnnounced)) true
      else if (MUTANT_ID == MUT_BAT011_LATE_ERA_RUN) true
      else (s.nowS + proveTimeS + INCLUSION_TIME_S < s.eraEndEffS)
    )

  pure def canConsumerUseClaim(s: SettleState, claimPub: int, viaCollection: bool): bool = {
    val inRuns = s.settledClaimPubs.contains(claimPub)
    val inAcceptedCollectionWithPublishedList =
      s.closedEpochs.exists(e =>
        e.memberClaims.contains(claimPub) and
        e.leafListPublishedOffChain and
        s.acceptedRoots.contains(e.claimsRoot)
      )
    val sourceValid = if (viaCollection) inAcceptedCollectionWithPublishedList else inRuns
    val unusedOk =
      (MUTANT_ID == MUT_LDG004_ROOT_NULLIFIER_DOUBLE_USE) or
      not(s.consumerUsedClaims.contains(claimPub))
    sourceValid and unusedOk
  }

  // =========================================================================
  // Pure consistency checks & spec-gap witness predicates
  // =========================================================================

  // BAT-003: 24 epochs of 3,600 s per 86,400 s day <= 492 unbatched per-job txs.
  pure val consistency_bat003_daily_batch_rates: bool =
    24 * 3600 == 86400 and 24 < 492

  // BAT-004 & STL-006: T23 corpus arithmetic (33 programs, 106 circuits, 208 calls = 145 + 62 + 1).
  pure val consistency_bat004_t23_counts: bool =
    208 - 62 - 1 == 145 and 33 <= 106 and 145 <= 208

  val consistency_constants: bool =
    CONTRACT_ENTRY_COUNT == 4 and
    PROTOTYPE_ENTRIES.size() == CONTRACT_ENTRY_COUNT and
    not(PROTOTYPE_ENTRIES.contains("openClaim")) and
    not(PROTOTYPE_ENTRIES.contains("descend")) and
    ANCHOR_SLOT == 15 and
    STATEMENT_PUBLIC_WORDS == 4 and
    RUN_SALT_WORDS == 8 and
    EPOCH_WINDOW_S >= COLLECTION_PROOF_TIME_S + INCLUSION_TIME_S + BLOCK_INTERVAL_S and
    MAX_CLAIMS_PER_EPOCH >= 1 and
    MAX_CLAIMS_PER_EPOCH <= 1048576 and
    K_MAX_CHUNKS >= 1 and
    K_MAX_CHUNKS <= 2 and
    DEFAULT_INTENT_TTL_S == 3600 and
    CLAIMS_TREE_DEPTH == 20 and
    CLAIM_LEAF_BYTES == 32 and
    COLLECTION_PROOF_TIME_S > 0 and
    INCLUSION_TIME_S > 0 and
    BLOCK_INTERVAL_S > 0 and
    consistency_bat003_daily_batch_rates and
    consistency_bat004_t23_counts

  // Finding Q-settlement-1 witness: LDG-001 mandates a single contract ("Two contracts with separate
  // ledgers SHALL NOT be used"), whereas SLC-005 + BAT-004 deploy up to 106 separate per-program ledgers.
  pure val gap_q_stl_1_ldg001_single_vs_slc005_multi_ledger: bool = {
    val ldg001MaxContracts = 1
    val slc005ProgramDeploys = 33
    val bat004CircuitDeploys = 106
    ldg001MaxContracts < slc005ProgramDeploys and slc005ProgramDeploys < bat004CircuitDeploys
  }

  // Finding Q-settlement-2 witness: on chunk 2 of a 2-chunk call, lane 10 carries lock A_1 != c_p,
  // so checking c_p == callVector[10] in SLC-009 raises a false Mismatch on chunk 2.
  pure val gap_q_stl_2_chunk2_lane10_lock_vs_cp: bool = {
    val cP = deriveCP(1, 7)
    val lockA1 = 555555
    cP != lockA1
  }

  // =========================================================================
  // Initialization
  // =========================================================================

  action init = all {
    stl' = {
      nowS: 100,
      universalWrapVkId: 1,
      rekeyAndT12Passed: false,
      hasLCollectionKey: false,
      eraEndAnnounced: false,
      eraEndEffS: 0,
      nextInstanceId: 1,
      ledgers: Map(),
      knownLedgerProgIds: Set(),
      builtArgs: Map(),
      knownArgIds: Set(),
      settledRuns: Map(),
      settledClaimPubs: Set(),
      epochOpen: false,
      openEpochId: 1,
      openEpochFirstJobS: 0,
      openEpochClaims: Set(),
      closedEpochs: Set(),
      acceptedCollections: Map(),
      acceptedRoots: Set(),
      consumerUsedClaims: Set(),
      consumerDoubleUseOccurred: false,
      consumerLedgerWritesOnUse: 0,
      chunkPlans: Set(),
      previewIntents: Set(),
      suiteReplays: Set(),
      eraRunAttempts: Set(),
      suiteClassifications: Set(),
      t15FreezeWritten: false,
      t15ListsCollectionOpen: false,
      lastRefusal: "none"
    },
    lastOp' = "init"
  }

  // =========================================================================
  // Parameterized actions
  // =========================================================================

  // Implements STL-003: record completion of rekey and T12 security gate enabling Preview redeploy.
  action doPassRekeyAndT12: bool = all {
    not(stl.rekeyAndT12Passed),
    stl' = {
      ...stl,
      rekeyAndT12Passed: true
    },
    lastOp' = "passRekeyAndT12"
  }

  // Implements BAT-009: install L collection form key into the universal key set (row T8c).
  action doEnableLCollectionKey: bool = all {
    not(stl.hasLCollectionKey),
    stl' = {
      ...stl,
      hasLCollectionKey: true
    },
    lastOp' = "enableLCollectionKey"
  }

  // Implements SLC-001..005, STL-003: deploy a SherlockLedger for progId with sealed slot-15 anchor c_p,
  // universal wrap VK on write entries, and native keys on read entries.
  action doDeployProgramLedger(
    pId: int,
    forT25Preview: bool,
    readbackMatches: bool,
    mutTrig: bool
  ): bool = all {
    canDeployLedger(stl, pId, forT25Preview, readbackMatches),
    val entries =
      if (MUTANT_ID == MUT_SLC001_OPEN_CLAIM and mutTrig)
        PROTOTYPE_ENTRIES.union(Set("openClaim"))
      else
        PROTOTYPE_ENTRIES
    val slotIdx =
      if (MUTANT_ID == MUT_SLC002_UNSEALED_ANCHOR and mutTrig) 0 else ANCHOR_SLOT
    val sealedFlag =
      not(MUTANT_ID == MUT_SLC002_UNSEALED_ANCHOR and mutTrig)
    val writeVk =
      if (MUTANT_ID == MUT_SLC003_WRONG_WRITE_VK and mutTrig) 99 else stl.universalWrapVkId
    val nativeReads =
      not(MUTANT_ID == MUT_SLC004_WRAP_READ_VK and mutTrig)
    val ldgRec: ProgramLedgerRecord = {
      instanceId: stl.nextInstanceId,
      progId: pId,
      exportedEntries: entries,
      anchorSlot: slotIdx,
      anchorSealedInConstructor: sealedFlag,
      anchorCP: deriveCP(pId, 7),
      writeEntriesVkId: writeVk,
      readEntriesKeepNativeVk: nativeReads,
      isT25PreviewDeploy: forT25Preview,
      readbackVkMatchesExpected: readbackMatches
    }
    stl' = {
      ...stl,
      nextInstanceId: stl.nextInstanceId + 1,
      ledgers: stl.ledgers.put(pId, ldgRec),
      knownLedgerProgIds: stl.knownLedgerProgIds.union(Set(pId))
    },
    lastOp' = "deployProgramLedger"
  }

  // Implements STL-003 Scenario: stop when installed VK readback differs from new wrap VK.
  action doRefuseInstalledVkReadbackMismatch: bool = all {
    stl.rekeyAndT12Passed,
    stl' = {
      ...stl,
      lastRefusal: "installed_vk_readback_mismatch"
    },
    lastOp' = "refuseInstalledVkReadbackMismatch"
  }

  // Implements SLC-006..009, STL-001, ZK-006, ZK-011:
  // Map sherlock-accept-bundle/1 on the proving box into StatementPublics {c_p, input_pub, outcome_pub, claim_pub}.
  action doBuildAcceptArgs(
    argId: int,
    pId: int,
    runSalt: int,
    tapeValD: int,
    commWord: int,
    effectSeqDigest: int,
    lanesMatch: bool,
    nativeRefusedFlag: bool,
    leafBoundFlag: bool,
    ownerSubmitterFlag: bool,
    mutUnsalted: bool
  ): bool = {
    val usedSaltWords =
      if (MUTANT_ID == MUT_SLC008_UNSALTED_INPUT and mutUnsalted) 0 else RUN_SALT_WORDS
    val usedFuel =
      if (MUTANT_ID == MUT_SLC008_UNSALTED_INPUT and mutUnsalted) 10 else 0
    val usedLeak =
      (MUTANT_ID == MUT_SLC008_UNSALTED_INPUT and mutUnsalted)
    all {
      not(stl.knownArgIds.contains(argId)),
      canBuildAcceptArgs(lanesMatch, usedSaltWords, usedFuel, 0, usedLeak),
      val cP = deriveCP(pId, 7)
      val inD = deriveInputD(runSalt, tapeValD, usedFuel, 0)
      val outD = deriveOutcomeD(commWord, effectSeqDigest)
      val inPub = deriveInputPub(inD)
      val outPub = deriveOutcomePub(inD, outD)
      val clmPub = deriveClaimPub(cP, inPub, outPub)
      val rec: BuiltArgsRecord = {
        argId: argId,
        progId: pId,
        cP: cP,
        inputD: inD,
        outcomeD: outD,
        inputPub: inPub,
        outcomePub: outPub,
        claimPub: clmPub,
        publicWordCount: STATEMENT_PUBLIC_WORDS,
        fuelVal: usedFuel,
        stackLimitVal: 0,
        saltWordCount: usedSaltWords,
        saltLeakedPublic: usedLeak,
        callVectorLanesMatch: lanesMatch,
        nativeRefused: nativeRefusedFlag,
        leafStmtBound: leafBoundFlag,
        isOwnerSubmitter: ownerSubmitterFlag,
        argsFileWritten: true
      }
      stl' = {
        ...stl,
        builtArgs: stl.builtArgs.put(argId, rec),
        knownArgIds: stl.knownArgIds.union(Set(argId))
      },
      lastOp' = "buildAcceptArgs"
    }
  }

  // Implements SLC-009: raise Mismatch and write no argument file when lanes 10/20/34/39 differ.
  action doRefuseLaneMismatch: bool = all {
    stl' = {
      ...stl,
      lastRefusal: "Mismatch"
    },
    lastOp' = "refuseLaneMismatch"
  }

  // Implements SLC-005, SLC-010, SLC-012, BAT-002, STL-004, LDG-001, LDG-002:
  // Settle an owner-submitted run via acceptRun(c_p, input_pub, outcome_pub), setting runs[claim_pub] = outcome_pub.
  action doAcceptRun(ledgerProgId: int, argId: int): bool = all {
    canAcceptRun(stl, ledgerProgId, argId),
    val a = stl.builtArgs.get(argId)
    val prevCnt =
      if (stl.settledClaimPubs.contains(a.claimPub))
        stl.settledRuns.get(a.claimPub).acceptCount
      else
        0
    val runRec: SettledRunRecord = {
      claimPub: a.claimPub,
      cP: a.cP,
      inputPub: a.inputPub,
      outcomePub: a.outcomePub,
      settledLedgerProgId: ledgerProgId,
      callProgId: a.progId,
      acceptCount: prevCnt + 1,
      wasNativeRefused: a.nativeRefused,
      wasLeafBound: a.leafStmtBound,
      wasOwnerSubmitted: a.isOwnerSubmitter
    }
    stl' = {
      ...stl,
      settledRuns: stl.settledRuns.put(a.claimPub, runRec),
      settledClaimPubs: stl.settledClaimPubs.union(Set(a.claimPub))
    },
    lastOp' = "acceptRun"
  }

  // Implements SLC-005, SLC-010, SLC-012, BAT-002, LDG-002: explicit refusals of invalid acceptRun attempts.
  action doRefuseAcceptRun(reasonTag: str): bool = all {
    stl' = {
      ...stl,
      lastRefusal: reasonTag
    },
    lastOp' = "refuseAcceptRun"
  }

  // Implements BAT-001, BAT-002: route an operator-submitted proved job into the open epoch collection
  // (opening the epoch at nowS if closed; auto-closing if claim count hits MAX_CLAIMS_PER_EPOCH).
  action doArriveOperatorJob(jobClaimPub: int): bool = all {
    not(stl.eraEndAnnounced and stl.nowS >= eraEpochCloseCutoff(stl.eraEndEffS)),
    val startS = if (stl.epochOpen) stl.openEpochFirstJobS else stl.nowS
    val updatedClaims = stl.openEpochClaims.union(Set(jobClaimPub))
    val hitsCap = (updatedClaims.size() >= MAX_CLAIMS_PER_EPOCH)
    val closedRec: ClosedEpochRecord = {
      epochId: stl.openEpochId,
      firstJobTimeS: startS,
      closeTimeS: stl.nowS,
      claimCount: updatedClaims.size(),
      claimsRoot: stl.openEpochId * 10000 + updatedClaims.size(),
      memberClaims: updatedClaims,
      pathsDelivered: true,
      deliveredSiblingCount: CLAIMS_TREE_DEPTH,
      leafListPublishedOffChain: true,
      leafBytesPerClaim: CLAIM_LEAF_BYTES,
      eraEndAnnouncedAtClose: stl.eraEndAnnounced,
      eraEndEffAtCloseS: stl.eraEndEffS,
      collectionSubmitted: false,
      operatorAcceptRunSent: false
    }
    stl' =
      if (hitsCap) {
        ...stl,
        epochOpen: false,
        openEpochId: stl.openEpochId + 1,
        openEpochFirstJobS: 0,
        openEpochClaims: Set(),
        closedEpochs: stl.closedEpochs.union(Set(closedRec))
      } else {
        ...stl,
        epochOpen: true,
        openEpochFirstJobS: startS,
        openEpochClaims: updatedClaims
      },
    lastOp' = "arriveOperatorJob"
  }

  // Implements BAT-001, BAT-007, BAT-008, BAT-010, LDG-005:
  // Close the open epoch (at <= firstJobTime + 3,600 s and <= era cutoff), delivering 20-sibling paths
  // to each submitter and publishing the 32-byte-per-claim leaf list off chain.
  action doCloseEpoch(advanceToDeadline: bool, mutTrig: bool): bool = all {
    stl.epochOpen,
    stl.openEpochClaims.size() >= 1,
    val windowDeadline = stl.openEpochFirstJobS + EPOCH_WINDOW_S
    val effectiveDeadline =
      if (stl.eraEndAnnounced and eraEpochCloseCutoff(stl.eraEndEffS) < windowDeadline)
        eraEpochCloseCutoff(stl.eraEndEffS)
      else
        windowDeadline
    val baseCloseS = if (advanceToDeadline) effectiveDeadline else stl.nowS
    val usedCloseS =
      if (MUTANT_ID == MUT_BAT001_LATE_EPOCH_CLOSE and mutTrig)
        windowDeadline + 100
      else if (MUTANT_ID == MUT_BAT010_EPOCH_PAST_ERA_END and stl.eraEndAnnounced and mutTrig)
        stl.eraEndEffS
      else
        baseCloseS
    all {
      usedCloseS >= stl.nowS,
      (MUTANT_ID == MUT_BAT001_LATE_EPOCH_CLOSE and mutTrig) or
        (MUTANT_ID == MUT_BAT010_EPOCH_PAST_ERA_END and stl.eraEndAnnounced and mutTrig) or
        (usedCloseS <= effectiveDeadline),
      val pathsOk = not(MUTANT_ID == MUT_BAT007_NO_SUBMITTER_PATHS and mutTrig)
      val sibCnt = if (pathsOk) CLAIMS_TREE_DEPTH else 0
      val pubOk = not(MUTANT_ID == MUT_BAT008_WITHHOLD_LIST and mutTrig)
      val leafB = if (pubOk) CLAIM_LEAF_BYTES else 0
      val closedRec: ClosedEpochRecord = {
        epochId: stl.openEpochId,
        firstJobTimeS: stl.openEpochFirstJobS,
        closeTimeS: usedCloseS,
        claimCount: stl.openEpochClaims.size(),
        claimsRoot: stl.openEpochId * 10000 + stl.openEpochClaims.size(),
        memberClaims: stl.openEpochClaims,
        pathsDelivered: pathsOk,
        deliveredSiblingCount: sibCnt,
        leafListPublishedOffChain: pubOk,
        leafBytesPerClaim: leafB,
        eraEndAnnouncedAtClose: stl.eraEndAnnounced,
        eraEndEffAtCloseS: stl.eraEndEffS,
        collectionSubmitted: false,
        operatorAcceptRunSent: false
      }
      stl' = {
        ...stl,
        nowS: usedCloseS,
        epochOpen: false,
        openEpochId: stl.openEpochId + 1,
        openEpochFirstJobS: 0,
        openEpochClaims: Set(),
        closedEpochs: stl.closedEpochs.union(Set(closedRec))
      },
      lastOp' = "closeEpoch"
    }
  }

  // Implements BAT-009, LDG-001, LDG-003:
  // Submit acceptCollection(reg_pub, claims_root) once L collection key exists and reg_pub is valid;
  // writes one root into claim_roots with zero per-claim writes and leaves settledRuns unchanged.
  action doAcceptCollection(rootVal: int, regPubValid: bool): bool = all {
    canAcceptCollection(stl, rootVal, regPubValid),
    val prevCnt =
      if (stl.acceptedRoots.contains(rootVal))
        stl.acceptedCollections.get(rootVal).acceptCount
      else
        0
    val collRec: AcceptedCollectionRecord = {
      claimsRoot: rootVal,
      regPubValidInHistory: regPubValid,
      acceptCount: prevCnt + 1,
      perClaimLedgerWrites: 0,
      leakedShapeOrSetRoot: false,
      lCollectionKeyPresentAtSubmit: stl.hasLCollectionKey
    }
    stl' = {
      ...stl,
      acceptedCollections: stl.acceptedCollections.put(rootVal, collRec),
      acceptedRoots: stl.acceptedRoots.union(Set(rootVal))
    },
    lastOp' = "acceptCollection"
  }

  // Implements BAT-009: refuse acceptCollection while universal key set lacks L collection form key.
  action doRefuseCollectionWithoutLKey: bool = all {
    not(stl.hasLCollectionKey),
    stl' = {
      ...stl,
      lastRefusal: "missing_l_collection_form_key"
    },
    lastOp' = "refuseCollectionWithoutLKey"
  }

  // Implements BAT-007, BAT-008, LDG-001, LDG-004, LDG-005:
  // Consumer contract uses a claim (either from runs or by 20-sibling path to rootAccepted)
  // with zero writes to SherlockLedger and a claim-keyed nullifier preventing double use across sources.
  action doConsumerUseClaim(claimPub: int, viaCollection: bool): bool = all {
    canConsumerUseClaim(stl, claimPub, viaCollection),
    val wasAlreadyUsed = stl.consumerUsedClaims.contains(claimPub)
    stl' = {
      ...stl,
      consumerUsedClaims: stl.consumerUsedClaims.union(Set(claimPub)),
      consumerDoubleUseOccurred: stl.consumerDoubleUseOccurred or wasAlreadyUsed,
      consumerLedgerWritesOnUse: 0
    },
    lastOp' = "consumerUseClaim"
  }

  // Implements BAT-005: plan tool checks chunk count <= K_MAX = 2 at every intent TTL.
  action doPlanEntryChunks(entryId: int, neededChunks: int, ttlS: int): bool = all {
    canPlanEntryChunks(neededChunks, ttlS),
    val rec: ChunkPlanRecord = {
      entryId: entryId,
      neededChunks: neededChunks,
      intentTtlS: ttlS,
      admitted: true
    }
    stl' = {
      ...stl,
      chunkPlans: stl.chunkPlans.union(Set(rec))
    },
    lastOp' = "planEntryChunks"
  }

  // Implements BAT-005 Scenario: refuse entry needing > 2 chunks even at TTL = 14,400 s.
  action doRefuseOverChunkedEntry(entryId: int, neededChunks: int, ttlS: int): bool = all {
    neededChunks > K_MAX_CHUNKS,
    ttlS > 0,
    stl' = {
      ...stl,
      lastRefusal: "chunk_cap_K_MAX_2_exceeded"
    },
    lastOp' = "refuseOverChunkedEntry"
  }

  // Implements BAT-006: settle runner builds Preview intent with default TTL = 3,600 s when unset.
  action doBuildPreviewIntent(intentId: int, userGaveTtl: bool, explicitTtlS: int, mutTrig: bool): bool = all {
    val chosenTtl =
      if (userGaveTtl) explicitTtlS
      else if (MUTANT_ID == MUT_BAT006_DEFAULT_TTL_14400 and mutTrig) 14400
      else DEFAULT_INTENT_TTL_S
    all {
      chosenTtl > 0,
      val rec: PreviewIntentRecord = {
        intentId: intentId,
        userGaveTtl: userGaveTtl,
        assignedTtlS: chosenTtl
      }
      stl' = {
        ...stl,
        previewIntents: stl.previewIntents.union(Set(rec))
      },
      lastOp' = "buildPreviewIntent"
    }
  }

  // Implements SLC-011: deploy a fresh instance when replay starts on state != deploy-block state;
  // keep instance when state equals deploy-block state.
  action doStartSuiteReplay(replayId: int, pId: int, stateEqDeployBlock: bool, mutTrig: bool): bool = all {
    val deployFresh =
      if (stateEqDeployBlock) false
      else not(MUTANT_ID == MUT_SLC011_REUSE_DIRTY_INSTANCE and mutTrig)
    val rec: SuiteReplayRecord = {
      replayId: replayId,
      progId: pId,
      stateEqualsDeployBlockAtStart: stateEqDeployBlock,
      deployedFreshInstance: deployFresh,
      reusedDirtyInstance: (not(stateEqDeployBlock) and not(deployFresh))
    }
    stl' = {
      ...stl,
      nextInstanceId: if (deployFresh) stl.nextInstanceId + 1 else stl.nextInstanceId,
      suiteReplays: stl.suiteReplays.union(Set(rec))
    },
    lastOp' = "startSuiteReplay"
  }

  // Implements BAT-010, BAT-011: announce era end end_eff (strictly beyond current epoch cutoff).
  action doAnnounceEraEnd(endEffVal: int): bool = all {
    not(stl.eraEndAnnounced),
    eraEpochCloseCutoff(endEffVal) >= stl.nowS,
    (stl.epochOpen implies stl.openEpochFirstJobS <= eraEpochCloseCutoff(endEffVal)),
    stl' = {
      ...stl,
      eraEndAnnounced: true,
      eraEndEffS: endEffVal
    },
    lastOp' = "announceEraEnd"
  }

  // Implements BAT-011: start a new run in the accepting era only when now + prove + incl < end_eff.
  action doStartEraRun(runAttemptId: int, proveTimeS: int): bool = all {
    canStartEraRun(stl, proveTimeS),
    val rec: EraRunAttemptRecord = {
      runAttemptId: runAttemptId,
      startTimeS: stl.nowS,
      proveTimeS: proveTimeS,
      endEffS: stl.eraEndEffS,
      eraEndWasAnnounced: stl.eraEndAnnounced,
      started: true
    }
    stl' = {
      ...stl,
      eraRunAttempts: stl.eraRunAttempts.union(Set(rec))
    },
    lastOp' = "startEraRun"
  }

  // Implements BAT-011 Scenario: refuse a run whose proving + inclusion time lands at or after end_eff.
  action doRefuseLateEraRun(proveTimeS: int): bool = all {
    stl.eraEndAnnounced,
    stl.nowS + proveTimeS + INCLUSION_TIME_S >= stl.eraEndEffS,
    stl' = {
      ...stl,
      lastRefusal: "run_outlives_era_end_eff"
    },
    lastOp' = "refuseLateEraRun"
  }

  // Implements BAT-012: write T15 freeze record; if no acceptCollection receipt exists, list collection as open.
  action doWriteT15FreezeRecord(mutTrig: bool): bool = all {
    val noCollectionReceipt = (stl.acceptedRoots.size() == 0)
    val listOpen =
      if (noCollectionReceipt)
        not(MUTANT_ID == MUT_BAT012_OMIT_OPEN_T15 and mutTrig)
      else
        false
    stl' = {
      ...stl,
      t15FreezeWritten: true,
      t15ListsCollectionOpen: listOpen
    },
    lastOp' = "writeT15FreezeRecord"
  }

  // Implements SLC-010, STL-005: classify a suite call against the native T18(a) Preview transcript.
  action doClassifySuiteCall(
    suiteCallId: int,
    nativeRefusedFlag: bool,
    differsFromNativeT18: bool,
    mutTrig: bool
  ): bool = all {
    val assigned =
      if (nativeRefusedFlag) ClassRefusedAsNative
      else if (differsFromNativeT18)
        (if (MUTANT_ID == MUT_STL005_HIDE_MISMATCH and mutTrig) ClassPass else ClassMismatch)
      else ClassPass
    val rec: SuiteClassRecord = {
      suiteCallId: suiteCallId,
      nativeRefused: nativeRefusedFlag,
      settledDiffersFromNativeT18: (not(nativeRefusedFlag) and differsFromNativeT18),
      assignedClass: assigned,
      acceptRunSent: false
    }
    stl' = {
      ...stl,
      suiteClassifications: stl.suiteClassifications.union(Set(rec))
    },
    lastOp' = "classifySuiteCall"
  }

  // =========================================================================
  // Nondeterministic step relation
  // =========================================================================

  action step = any {
    doPassRekeyAndT12,
    doEnableLCollectionKey,

    // Ledger deployments & readback refusal (SLC-001..005, STL-003)
    {
      nondet pId = Set(1, 2).oneOf()
      nondet t25Flag = Set(false, true).oneOf()
      val rbPool =
        if (MUTANT_ID == MUT_STL003_BAD_VK_READBACK) Set(false, true)
        else Set(true)
      nondet rbOk = rbPool.oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doDeployProgramLedger(pId, t25Flag, rbOk, mutTrig)
    },
    doRefuseInstalledVkReadbackMismatch,

    // Argument builder & lane check (SLC-006..009, STL-001)
    {
      nondet aId = Set(1, 2).oneOf()
      nondet pId = Set(1, 2).oneOf()
      val lanesPool =
        if (MUTANT_ID == MUT_SLC009_IGNORE_LANE_MISMATCH) Set(false, true)
        else Set(true)
      nondet lOk = lanesPool.oneOf()
      val natRefPool =
        if (MUTANT_ID == MUT_SLC010_SETTLE_NATIVE_REFUSED) Set(true, false)
        else Set(false)
      nondet nRef = natRefPool.oneOf()
      val boundPool =
        if (MUTANT_ID == MUT_SLC012_SETTLE_UNBOUND_STMT) Set(false, true)
        else Set(true)
      nondet bOk = boundPool.oneOf()
      val ownerPool =
        if (MUTANT_ID == MUT_BAT002_OPERATOR_ACCEPT_RUN) Set(false, true)
        else Set(true)
      nondet ownOk = ownerPool.oneOf()
      nondet mutUnsalt = Set(true, false).oneOf()
      doBuildAcceptArgs(aId, pId, aId + 10, 50, 7, 9, lOk, nRef, bOk, ownOk, mutUnsalt)
    },
    doRefuseLaneMismatch,

    // Run settlement & refusals (SLC-005, SLC-010, SLC-012, BAT-002, STL-004, LDG-002)
    {
      nondet lPid = Set(1, 2).oneOf()
      nondet aId = Set(1, 2).oneOf()
      doAcceptRun(lPid, aId)
    },
    {
      nondet rTag = Set("native_refused", "unbound_leaf_statement", "operator_must_batch").oneOf()
      doRefuseAcceptRun(rTag)
    },

    // Epoch operator batching & collection settlement (BAT-001, BAT-002, BAT-007..010, LDG-003..005)
    {
      nondet jClaim = Set(701, 702).oneOf()
      doArriveOperatorJob(jClaim)
    },
    {
      nondet adv = Set(true, false).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doCloseEpoch(adv, mutTrig)
    },
    {
      nondet rCandidate = Set(10001, 10002, 20001, 20002).oneOf()
      val regPool =
        if (MUTANT_ID == MUT_LDG003_STALE_REG_OR_DUP_ROOT) Set(false, true)
        else Set(true)
      nondet regOk = regPool.oneOf()
      doAcceptCollection(rCandidate, regOk)
    },
    doRefuseCollectionWithoutLKey,

    // Consumer membership / run claim discharge (BAT-007, BAT-008, LDG-004, LDG-005)
    {
      val candidateClaims = stl.settledClaimPubs.union(Set(701, 702))
      nondet viaColl = Set(false, true).oneOf()
      all {
        candidateClaims.size() > 0,
        nondet cPub = candidateClaims.oneOf()
        doConsumerUseClaim(cPub, viaColl)
      }
    },

    // Chunk planning, Preview TTL, instance reuse, era bounds, T15 freeze, suite classification
    {
      val chunkPool =
        if (MUTANT_ID == MUT_BAT005_ALLOW_3_CHUNKS) Set(1, K_MAX_CHUNKS, 3)
        else Set(1, K_MAX_CHUNKS)
      nondet nChunks = chunkPool.oneOf()
      doPlanEntryChunks(1, nChunks, 14400)
    },
    doRefuseOverChunkedEntry(2, 3, 14400),
    {
      nondet userTtl = Set(false, true).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doBuildPreviewIntent(1, userTtl, 7200, mutTrig)
    },
    {
      nondet eqDeploy = Set(false, true).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doStartSuiteReplay(1, 1, eqDeploy, mutTrig)
    },
    doAnnounceEraEnd(stl.nowS + EPOCH_WINDOW_S + 300),
    {
      val pTimePool =
        if (MUTANT_ID == MUT_BAT011_LATE_ERA_RUN) Set(100, 5000)
        else Set(100)
      nondet pTime = pTimePool.oneOf()
      doStartEraRun(1, pTime)
    },
    doRefuseLateEraRun(5000),
    {
      nondet mutTrig = Set(true, false).oneOf()
      doWriteT15FreezeRecord(mutTrig)
    },
    {
      nondet natRef = Set(false, true).oneOf()
      nondet diffT18 = Set(false, true).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doClassifySuiteCall(1, natRef, diffT18, mutTrig)
    }
  }

  // =========================================================================
  // Safety invariants (one inv_<ID> per expressible requirement)
  // =========================================================================

  // SLC-001 (and LDG-004): Every deployed SherlockLedger exports exactly the 4 prototype entries
  // {acceptRun, acceptCollection, rootAccepted, runAccepted} (no openClaim, no descend).
  val inv_SLC_001: bool =
    stl.knownLedgerProgIds.forall(pId => {
      val l = stl.ledgers.get(pId)
      l.exportedEntries == PROTOTYPE_ENTRIES and
      l.exportedEntries.size() == CONTRACT_ENTRY_COUNT
    })

  // SLC-002: Every deployed SherlockLedger holds the anchor in slot 15 sealed by the constructor.
  val inv_SLC_002: bool =
    stl.knownLedgerProgIds.forall(pId => {
      val l = stl.ledgers.get(pId)
      l.anchorSlot == ANCHOR_SLOT and l.anchorSealedInConstructor
    })

  // SLC-003: Write entries (acceptRun, acceptCollection) hold the universal wrap verifying key.
  val inv_SLC_003: bool =
    stl.knownLedgerProgIds.forall(pId =>
      stl.ledgers.get(pId).writeEntriesVkId == stl.universalWrapVkId
    )

  // SLC-004: Read entries (rootAccepted, runAccepted) hold their compiled native keys (not wrap VK).
  val inv_SLC_004: bool =
    stl.knownLedgerProgIds.forall(pId =>
      stl.ledgers.get(pId).readEntriesKeepNativeVk
    )

  // SLC-005: Constructor anchor equals c_p of the ledger's program, and no call of program B settles on program A's ledger.
  val inv_SLC_005: bool =
    stl.knownLedgerProgIds.forall(pId =>
      stl.ledgers.get(pId).anchorCP == deriveCP(pId, 7)
    ) and
    stl.settledClaimPubs.forall(clm => {
      val r = stl.settledRuns.get(clm)
      stl.knownLedgerProgIds.contains(r.settledLedgerProgId) and
      stl.ledgers.get(r.settledLedgerProgId).anchorCP == r.cP and
      r.settledLedgerProgId == r.callProgId
    })

  // SLC-006 (and STL-001): Every built StatementPublics carries 4 public words with consistent claim_pub.
  val inv_SLC_006: bool =
    stl.knownArgIds.forall(aId => {
      val a = stl.builtArgs.get(aId)
      a.publicWordCount == STATEMENT_PUBLIC_WORDS and
      a.inputPub == deriveInputPub(a.inputD) and
      a.outcomePub == deriveOutcomePub(a.inputD, a.outcomeD) and
      a.claimPub == deriveClaimPub(a.cP, a.inputPub, a.outcomePub)
    })

  // SLC-008 (and ZK-006, ZK-011): Every built Mycroft call input has fuel 0, stack_limit 0,
  // an 8-word run salt, and never leaks the salt into public arguments.
  val inv_SLC_008: bool =
    stl.knownArgIds.forall(aId => {
      val a = stl.builtArgs.get(aId)
      a.fuelVal == 0 and
      a.stackLimitVal == 0 and
      a.saltWordCount == RUN_SALT_WORDS and
      not(a.saltLeakedPublic)
    })

  // SLC-009: No argument file is written when recomputed public words differ from call-vector lanes 10, 20, 34, 39.
  val inv_SLC_009: bool =
    stl.knownArgIds.forall(aId => {
      val a = stl.builtArgs.get(aId)
      a.argsFileWritten implies a.callVectorLanesMatch
    })

  // SLC-010: A call refused by the native transcript never sends acceptRun.
  val inv_SLC_010: bool =
    stl.settledClaimPubs.forall(clm =>
      not(stl.settledRuns.get(clm).wasNativeRefused)
    ) and
    stl.suiteClassifications.forall(sc =>
      sc.nativeRefused implies not(sc.acceptRunSent)
    )

  // SLC-011: Replay starting on state != deploy-block state deploys a new instance and never reuses the mutated one.
  val inv_SLC_011: bool =
    stl.suiteReplays.forall(sr =>
      (not(sr.stateEqualsDeployBlockAtStart) implies sr.deployedFreshInstance) and
      not(sr.reusedDirtyInstance)
    )

  // SLC-012: No live settle occurs for a call whose leaf statement fails to bind prog_d, input_d, and outcome_d.
  val inv_SLC_012: bool =
    stl.settledClaimPubs.forall(clm =>
      stl.settledRuns.get(clm).wasLeafBound
    )

  // BAT-001: Every closed epoch is non-empty, holds at most MAX_CLAIMS_PER_EPOCH claims,
  // and closed at or before firstJobTimeS + EPOCH_WINDOW_S.
  val inv_BAT_001: bool =
    (stl.epochOpen implies (
      stl.openEpochClaims.size() >= 1 and
      stl.openEpochClaims.size() < MAX_CLAIMS_PER_EPOCH and
      stl.nowS <= stl.openEpochFirstJobS + EPOCH_WINDOW_S
    )) and
    stl.closedEpochs.forall(e =>
      e.claimCount >= 1 and
      e.claimCount <= MAX_CLAIMS_PER_EPOCH and
      e.closeTimeS >= e.firstJobTimeS and
      e.closeTimeS <= e.firstJobTimeS + EPOCH_WINDOW_S
    )

  // BAT-002: Operator-submitted jobs route to epoch collections and never settle via per-job acceptRun.
  val inv_BAT_002: bool =
    stl.settledClaimPubs.forall(clm =>
      stl.settledRuns.get(clm).wasOwnerSubmitted
    ) and
    stl.closedEpochs.forall(e => not(e.operatorAcceptRunSent))

  // BAT-005: No admitted entry plan exceeds K_MAX = 2 chunks at any intent TTL.
  val inv_BAT_005: bool =
    stl.chunkPlans.forall(cp =>
      cp.admitted implies (cp.neededChunks >= 1 and cp.neededChunks <= K_MAX_CHUNKS)
    )

  // BAT-006: Preview intents built with no user TTL use DEFAULT_INTENT_TTL_S = 3,600 s.
  val inv_BAT_006: bool =
    stl.previewIntents.forall(pi =>
      not(pi.userGaveTtl) implies (pi.assignedTtlS == DEFAULT_INTENT_TTL_S)
    )

  // BAT-007: Every closed epoch delivers slot index and 20 Merkle sibling words to each job submitter.
  val inv_BAT_007: bool =
    stl.closedEpochs.forall(e =>
      e.pathsDelivered and e.deliveredSiblingCount == CLAIMS_TREE_DEPTH
    )

  // BAT-008: Every closed epoch publishes the off-chain leaf list at 32 bytes per claim.
  val inv_BAT_008: bool =
    stl.closedEpochs.forall(e =>
      e.leafListPublishedOffChain and e.leafBytesPerClaim == CLAIM_LEAF_BYTES
    )

  // BAT-009: No acceptCollection is submitted while the universal key set lacks the L collection form key.
  val inv_BAT_009: bool =
    stl.acceptedRoots.forall(rt =>
      stl.acceptedCollections.get(rt).lCollectionKeyPresentAtSubmit and stl.hasLCollectionKey
    )

  // BAT-010: When era end end_eff is announced, every epoch closes at or before
  // end_eff - (COLLECTION_PROOF_TIME_S + INCLUSION_TIME_S + BLOCK_INTERVAL_S) so its root lands strictly before end_eff.
  val inv_BAT_010: bool =
    (stl.eraEndAnnounced and stl.epochOpen implies (
      stl.nowS <= eraEpochCloseCutoff(stl.eraEndEffS)
    )) and
    stl.closedEpochs.forall(e =>
      e.eraEndAnnouncedAtClose implies (
        e.closeTimeS <= eraEpochCloseCutoff(e.eraEndEffAtCloseS)
      )
    )

  // BAT-011: No run starts for an era with announced end_eff if start + prove + inclusion >= end_eff.
  val inv_BAT_011: bool =
    stl.eraRunAttempts.forall(ra =>
      (ra.started and ra.eraEndWasAnnounced) implies (
        ra.startTimeS + ra.proveTimeS + INCLUSION_TIME_S < ra.endEffS
      )
    )

  // BAT-012: If T15 freeze record has no acceptCollection receipt, batched collection settlement is listed as open.
  val inv_BAT_012: bool =
    (stl.t15FreezeWritten and stl.acceptedRoots.size() == 0) implies stl.t15ListsCollectionOpen

  // STL-003: Preview T25 redeploy requires rekey + T12 pass, anchor in slot 15, and matching readback VK.
  val inv_STL_003: bool =
    stl.knownLedgerProgIds.forall(pId => {
      val l = stl.ledgers.get(pId)
      l.isT25PreviewDeploy implies (
        stl.rekeyAndT12Passed and
        l.anchorSlot == ANCHOR_SLOT and
        l.writeEntriesVkId == stl.universalWrapVkId and
        l.readbackVkMatchesExpected
      )
    })

  // STL-004 & Adopted LDG-002: Every settled run records runs[claim_pub] = outcome_pub and settles at most once.
  val inv_STL_004: bool =
    stl.settledClaimPubs.forall(clm => {
      val r = stl.settledRuns.get(clm)
      r.claimPub == clm and
      r.claimPub == deriveClaimPub(r.cP, r.inputPub, r.outcomePub) and
      r.acceptCount == 1
    })

  val inv_LDG_002: bool = inv_STL_004 and inv_BAT_002

  // STL-005: Any settled suite call differing from the native T18(a) Preview transcript is recorded as ClassMismatch,
  // and any natively refused call is recorded as ClassRefusedAsNative.
  val inv_STL_005: bool =
    stl.suiteClassifications.forall(sc =>
      (sc.nativeRefused implies sc.assignedClass == ClassRefusedAsNative) and
      (sc.settledDiffersFromNativeT18 implies sc.assignedClass == ClassMismatch)
    )

  // Adopted LDG-003 & LDG-001: acceptCollection checks reg_pub against registry history, accepts each
  // claims_root at most once, makes zero per-claim ledger writes, and exposes no shape_d / set_root.
  val inv_LDG_003: bool =
    stl.acceptedRoots.forall(rt => {
      val ac = stl.acceptedCollections.get(rt)
      ac.regPubValidInHistory and
      ac.acceptCount == 1 and
      ac.perClaimLedgerWrites == 0 and
      not(ac.leakedShapeOrSetRoot)
    })

  // Adopted LDG-004 & LDG-005: Consumer uses only claims settled in runs or in an accepted claims_root
  // with published off-chain leaf list, makes zero writes to SherlockLedger, and never double-uses a claim.
  val inv_LDG_004: bool =
    not(stl.consumerDoubleUseOccurred) and
    stl.consumerLedgerWritesOnUse == 0 and
    stl.consumerUsedClaims.forall(clm =>
      stl.settledClaimPubs.contains(clm) or
      stl.closedEpochs.exists(e =>
        e.memberClaims.contains(clm) and
        e.leafListPublishedOffChain and
        stl.acceptedRoots.contains(e.claimsRoot)
      )
    )

  // Master safety invariant.
  val inv: bool =
    consistency_constants and
    inv_SLC_001 and
    inv_SLC_002 and
    inv_SLC_003 and
    inv_SLC_004 and
    inv_SLC_005 and
    inv_SLC_006 and
    inv_SLC_008 and
    inv_SLC_009 and
    inv_SLC_010 and
    inv_SLC_011 and
    inv_SLC_012 and
    inv_BAT_001 and
    inv_BAT_002 and
    inv_BAT_005 and
    inv_BAT_006 and
    inv_BAT_007 and
    inv_BAT_008 and
    inv_BAT_009 and
    inv_BAT_010 and
    inv_BAT_011 and
    inv_BAT_012 and
    inv_STL_003 and
    inv_STL_004 and
    inv_STL_005 and
    inv_LDG_002 and
    inv_LDG_003 and
    inv_LDG_004

  // =========================================================================
  // Reachability witnesses (negated goals — MUST be violated under settlement_main)
  // =========================================================================

  // Witness 1: Deploy ledger, build StatementPublics, settle via acceptRun, and discharge claim in consumer.
  val reach_accept_run_and_consumer_use: bool = not(
    stl.settledClaimPubs.size() >= 1 and
    stl.consumerUsedClaims.size() >= 1
  )

  // Witness 2: Operator batches jobs into an epoch, closes with 20-sibling paths & 32B list,
  // enables L collection key, and settles via acceptCollection.
  val reach_epoch_close_and_accept_collection: bool = not(
    stl.closedEpochs.size() >= 1 and
    stl.acceptedRoots.size() >= 1
  )

  // Witness 3: Era end announced, epoch closes before era cutoff, and late era run is refused.
  val reach_era_end_cutoff_and_late_run_refusal: bool = not(
    stl.eraEndAnnounced and
    stl.lastRefusal == "run_outlives_era_end_eff"
  )

  // Witness 4: Chunk cap refusal (> 2 chunks at 14,400 s), default 3,600 s Preview TTL, and dirty-state redeploy.
  val reach_chunk_cap_default_ttl_and_dirty_redeploy: bool = not(
    stl.lastRefusal == "chunk_cap_K_MAX_2_exceeded" and
    stl.previewIntents.exists(pi => not(pi.userGaveTtl) and pi.assignedTtlS == DEFAULT_INTENT_TTL_S) and
    stl.suiteReplays.exists(sr => not(sr.stateEqualsDeployBlockAtStart) and sr.deployedFreshInstance)
  )

  // Witness 5: Rekey + T12 pass, Preview slot-15 ledger deploys, and T15 freeze lists collection open when uncollected.
  val reach_t25_preview_deploy_and_t15_open_record: bool = not(
    stl.knownLedgerProgIds.exists(pId => stl.ledgers.get(pId).isT25PreviewDeploy) and
    stl.t15FreezeWritten and
    stl.t15ListsCollectionOpen
  )

  // Witness 6: Suite call classification covers both refused-as-native and outcome mismatch.
  val reach_suite_mismatch_and_refused_classes: bool = not(
    stl.suiteClassifications.exists(sc => sc.assignedClass == ClassRefusedAsNative) and
    stl.suiteClassifications.exists(sc => sc.assignedClass == ClassMismatch)
  )
}

// ===========================================================================
// Concrete instances: main, edge instances, and guard mutants
// ===========================================================================

module settlement_main {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4,
    ANCHOR_SLOT = 15,
    STATEMENT_PUBLIC_WORDS = 4,
    RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600,
    MAX_CLAIMS_PER_EPOCH = 1048576,
    K_MAX_CHUNKS = 2,
    DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20,
    CLAIM_LEAF_BYTES = 32,
    COLLECTION_PROOF_TIME_S = 176,
    INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6,
    MUTANT_ID = 0
  ).*
}

// Smallest legal parameters: EPOCH_WINDOW_S = 242 (= 176 + 60 + 6), MAX_CLAIMS_PER_EPOCH = 1, K_MAX_CHUNKS = 1.
module settlement_edge_min {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4,
    ANCHOR_SLOT = 15,
    STATEMENT_PUBLIC_WORDS = 4,
    RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 242,
    MAX_CLAIMS_PER_EPOCH = 1,
    K_MAX_CHUNKS = 1,
    DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20,
    CLAIM_LEAF_BYTES = 32,
    COLLECTION_PROOF_TIME_S = 176,
    INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6,
    MUTANT_ID = 0
  ).*
}

// Tight capacity bound: MAX_CLAIMS_PER_EPOCH = 2 so two operator jobs hit the exact capacity auto-close limit.
module settlement_edge_at_limit {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4,
    ANCHOR_SLOT = 15,
    STATEMENT_PUBLIC_WORDS = 4,
    RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600,
    MAX_CLAIMS_PER_EPOCH = 2,
    K_MAX_CHUNKS = 2,
    DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20,
    CLAIM_LEAF_BYTES = 32,
    COLLECTION_PROOF_TIME_S = 176,
    INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6,
    MUTANT_ID = 0
  ).*
}

// Mutants (each removes or weakens one refusal/binding guard and must break its named invariant)

module settlement_m_slc001_open_claim_entry {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 1
  ).*
}

module settlement_m_slc002_unsealed_anchor {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 2
  ).*
}

module settlement_m_slc003_wrong_write_vk {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 3
  ).*
}

module settlement_m_slc004_wrap_key_on_read_entry {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 4
  ).*
}

module settlement_m_slc005_cross_program_settle {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 5
  ).*
}

module settlement_m_slc008_unsalted_or_nonzero_fuel {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 6
  ).*
}

module settlement_m_slc009_write_args_on_lane_mismatch {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 7
  ).*
}

module settlement_m_slc010_settle_natively_refused {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 8
  ).*
}

module settlement_m_slc011_reuse_dirty_instance {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 9
  ).*
}

module settlement_m_slc012_settle_unbound_stmt {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 10
  ).*
}

module settlement_m_bat001_late_epoch_close {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 11
  ).*
}

module settlement_m_bat002_operator_accept_run {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 12
  ).*
}

module settlement_m_bat005_allow_3_chunks {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 13
  ).*
}

module settlement_m_bat006_default_ttl_14400 {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 14
  ).*
}

module settlement_m_bat007_missing_paths {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 15
  ).*
}

module settlement_m_bat008_unpublish_leaf_list {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 16
  ).*
}

module settlement_m_bat009_collection_without_l_key {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 17
  ).*
}

module settlement_m_bat010_epoch_past_era_cutoff {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 18
  ).*
}

module settlement_m_bat011_start_run_at_era_end {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 19
  ).*
}

module settlement_m_bat012_omit_open_t15 {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 20
  ).*
}

module settlement_m_stl003_mismatched_readback_vk {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 21
  ).*
}

module settlement_m_ldg002_replay_settled_claim {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 22
  ).*
}

module settlement_m_ldg003_stale_reg_or_dup_root {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 23
  ).*
}

module settlement_m_ldg004_root_scoped_double_use {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 24
  ).*
}

module settlement_m_stl005_hide_mismatch {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4, ANCHOR_SLOT = 15, STATEMENT_PUBLIC_WORDS = 4, RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600, MAX_CLAIMS_PER_EPOCH = 1048576, K_MAX_CHUNKS = 2, DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20, CLAIM_LEAF_BYTES = 32, COLLECTION_PROOF_TIME_S = 176, INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6, MUTANT_ID = 25
  ).*
}
