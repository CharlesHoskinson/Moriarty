// Quint formal specification for Sherlock E2E Bounded Context: Key Management
// Lane: e2e-key-management
// Own IDs: UKS-001 .. UKS-010 (sherlock-e2e-universal-keys)
// Completion IDs: RKY-001, RKY-002, RKY-005 (sherlock-rekey)
// Adopted IDs modelled here: SPL-005 (sherlock-shard-plan), SQG-011 (sherlock-security-gate)

module key_management {
  // =========================================================================
  // 1. Module Constants (Bounds & Mutant Selector)
  // =========================================================================

  const NODE_ROWS_REQUIRED: int          // UKS-002: 7 node rows in keys_d
  const FOREIGN_ROWS_COUNT: int          // UKS-002: foreign rows in keys_d (>= 1; 2 on T10m/rekey)
  const KEYDAG_IDX: int                  // UKS-003: path C DAG leaf index (3)
  const RSL_LEAF_IDX_CFG: int            // UKS-003: configured R_SL chip set 3 leaf index (4 != 3)
  const REQUIRED_LIST_ROW_COUNT: int     // UKS-010: 6 mandatory key-changing rows in LIST.md
  const REQUIRED_GATE_COUNT: int         // UKS-006: 12 regression gates rerun on new keys
  const SECURITY_GATE_CHECK_COUNT: int   // UKS-007, SQG-011: 6 checks (G1..G5 and FP-8)
  const MIN_TERM_DECIBITS: int           // RKY-005: 1000 (= 100.0 bits, in tenths of a bit)
  const MIN_GRINDABLE_DECIBITS: int      // RKY-005: 1010 (= 101.0 bits, in tenths of a bit)
  const MAX_WRAP_K: int                  // Context wrap bound: k <= 21
  const MAX_WRAP_VK_BYTES: int           // Context wrap bound: VK <= 50,000 bytes
  const DEFAULT_NON_GRIND_DECIBITS: int  // Honest D3 non-grindable minimum term (e.g. 1025 = 102.5 bits)
  const DEFAULT_GRIND_DECIBITS: int      // Honest D3 grindable minimum term (e.g. 1030 = 103.0 bits)
  const DEFAULT_WRAP_VK_BYTES: int       // Honest wrap VK byte size (e.g. 1325 bytes)
  const MUTANT_ID: int                   // 0 = sound; 1..15 = mutant instances

  // Mutant ID tags (typed integer constants, avoiding string guards)
  pure val MUT_NONE: int                      = 0
  pure val MUT_UKS001_SPLIT_KEYSET: int       = 1
  pure val MUT_UKS002_SHORT_KEYS_D: int       = 2
  pure val MUT_UKS003_RSL_IDX_COLLISION: int  = 3
  pure val MUT_UKS005_STALE_BOX_PROVES: int   = 4
  pure val MUT_UKS006_SKIP_FAILED_GATE: int   = 5
  pure val MUT_UKS007_PREMATURE_QUALIFIED: int = 6
  pure val MUT_UKS008_SETUP_MUTATES_KEY: int  = 7
  pure val MUT_UKS009_NON_DESCENDANT: int     = 8
  pure val MUT_UKS010_MISSING_LIST_ROW: int   = 9
  pure val MUT_RKY002_UNCOMMITTED_ROW: int    = 10
  pure val MUT_RKY005_GRINDABLE_100_9: int    = 11
  pure val MUT_SPL005_UNLISTED_CHILD_KEY: int = 12
  pure val MUT_SQG011_FREEZE_BEFORE_GATE: int = 13
  pure val MUT_UKS004_MANIFEST_UNBINDS: int   = 14
  pure val MUT_RKY001_STALE_WRAP_KEY: int     = 15

  // =========================================================================
  // 2. Domain Enums & Record Types
  // =========================================================================

  type ProgramId =
    | ProgT02Increment
    | ProgD04Withdraw

  type BoxId =
    | BoxMuninn
    | BoxHuginn
    | BoxOdin

  type ListRowKind =
    | RowFp3
    | RowAnchorSlot15
    | RowCanonicalRange
    | RowRslStmtBind
    | RowRslKeyLeafIdx
    | RowEraCheck
    | RowPostRekeyExtra

  type ManifestStatusMark =
    | MarkUnwritten
    | MarkPrototype
    | MarkQualified

  type RekeyPhase =
    | PhaseRekeyIdle
    | PhaseKeygenComplete
    | PhaseRekeyGateFailed
    | PhaseRekeyDone

  type RefusalRecord =
    | RefusalNone
    | RefusalMissingListRows(Set[ListRowKind])
    | RefusalUncommittedRow(ListRowKind)
    | RefusalNonDescendantTree
    | RefusalRekeyGateFailed(int)
    | RefusalD3SecurityFailed((int, int))
    | RefusalCollidingRslLeafIdx((int, int))
    | RefusalManifestMismatch((BoxId, int, int))
    | RefusalSetupKeyTamper(BoxId)
    | RefusalUnlistedChildKey((ProgramId, int))
    | RefusalFreezeBeforeSecurityGate

  type UniversalKeySetRec = {
    epoch: int,
    internalRecKeyId: int,
    lKeyId: int,
    lfrKeyId: int,
    wrapKeyId: int,
    lKeyListSha: int,
    lfrKeyListSha: int,
    wrapKeyListSha: int,
    wrapLogK: int,
    wrapVkBytes: int,
  }

  type KeysDRec = {
    built: bool,
    nodeRowCount: int,
    row4NwKey: int,
    row5Nw2Key: int,
    row6N3Nw2Key: int,
    foreignRowCount: int,
    keysDDigest: int,
  }

  type ManifestRec = {
    written: bool,
    manifestSha256: int,
    internalRecSha: int,
    lKeySha: int,
    lfrKeySha: int,
    wrapKeySha: int,
    keysDDigest: int,
    listSha256: int,
    openvmCommit: int,
    starkBackendCommit: int,
    moriartyCommit: int,
    statusMark: ManifestStatusMark,
    holdsPerProgramKey: bool,
  }

  type ProgramKeyRootRec = {
    built: bool,
    keyRootDigest: int,
    chip0Idx: int,
    chip1Idx: int,
    chip2Idx: int,
    hasRslChipSet3: bool,
    rslLeafIdx: int,
    keydagIdx: int,
    rslKeyComm: int,
    keydagComm: int,
  }

  type BoxStateRec = {
    installedManifestSha: int,
    keyFileCount: int,
    selectedShape: int,
    planD: int,
    proofEnteringValue: int,
    setupCompleted: bool,
    preSetupManifestSha: int,
    preSetupKeyFileCount: int,
    preSetupShape: int,
    preSetupPlanD: int,
    preSetupProofVal: int,
    proofRunning: bool,
    activeProg: ProgramId,
  }

  type SettledProofRec = {
    progId: ProgramId,
    usedManifestSha: int,
    usedInternalRecKey: int,
    usedLKey: int,
    usedLfrKey: int,
    usedWrapKey: int,
    usedKeyRootDigest: int,
    usedPathC: bool,
    openedChipLeafIdx: int,
    openedDagLeafIdx: int,
    childKeyComm: int,
    publicX: int,
    expectedX: int,
  }

  type KeyMgmtState = {
    // Rekey list (`~/.cache/rekey-t11/LIST.md`) and keygen git tree state
    currentListRows: Set[ListRowKind],
    uncommittedRows: Set[ListRowKind],
    treeDescendantOfAllRows: bool,
    currentListSha: int,
    treeOpenvmCommit: int,
    treeStarkBackendCommit: int,
    treeMoriartyCommit: int,

    // Snapshot recorded at the most recent keygen run
    keygenEpoch: int,
    rekeyPhase: RekeyPhase,
    lastKeygenListRows: Set[ListRowKind],
    lastKeygenListSha: int,
    lastKeygenAllRowsCommitted: bool,
    lastKeygenTreeWasDescendant: bool,
    secondKeygenNeeded: bool,

    // Generated universal key set, `keys_d`, and universal key manifest
    universalKeySet: UniversalKeySetRec,
    keysD: KeysDRec,
    manifest: ManifestRec,

    // Post-keygen 12-gate rerun (UKS-006) & D3 100/101-bit security check (RKY-005)
    gateRunKeygenEpoch: int,
    gatesRerunCount: int,
    gatesPassedCount: int,
    d3CheckedEpoch: int,
    d3MinTermDeciBits: int,
    d3MinGrindableDeciBits: int,

    // T12 security gate (G1..G5 + FP-8) & freeze status (UKS-007, SQG-011)
    secGateKeygenEpoch: int,
    secGatePassedChecks: Set[int],
    universalKeysFrozen: bool,

    // Per-program `key_root` trees (UKS-001, UKS-003, SPL-005)
    programKeyRoots: ProgramId -> ProgramKeyRootRec,

    // Fleet boxes (muninn, huginn, odin) & settled Compact suite proofs
    boxes: BoxId -> BoxStateRec,
    settledProofs: Set[SettledProofRec],

    // Observable refusal record
    lastRefusal: RefusalRecord,
  }

  // =========================================================================
  // 3. Domain Sets & Pure Helper Functions
  // =========================================================================

  pure val ALL_PROGRAMS: Set[ProgramId] = Set(ProgT02Increment, ProgD04Withdraw)
  pure val ALL_BOXES: Set[BoxId] = Set(BoxMuninn, BoxHuginn, BoxOdin)

  // The 6 key-changing rows mandated by UKS-010
  pure val MANDATORY_LIST_ROWS: Set[ListRowKind] = Set(
    RowFp3,
    RowAnchorSlot15,
    RowCanonicalRange,
    RowRslStmtBind,
    RowRslKeyLeafIdx,
    RowEraCheck
  )

  pure val T10M_OLD_MANIFEST_SHA: int = 100

  pure def progBaseCode(p: ProgramId): int =
    match p {
      | ProgT02Increment => 1
      | ProgD04Withdraw  => 2
    }

  pure def rowWeight(r: ListRowKind): int =
    match r {
      | RowFp3            => 1
      | RowAnchorSlot15   => 2
      | RowCanonicalRange => 4
      | RowRslStmtBind    => 8
      | RowRslKeyLeafIdx  => 16
      | RowEraCheck       => 32
      | RowPostRekeyExtra => 64
    }

  // Deterministic abstract sha256 of `LIST.md` from its rows
  pure def computeListSha(rows: Set[ListRowKind]): int =
    200 + rows.fold(0, (acc, r) => acc + rowWeight(r))

  // Deterministic abstract sha256 of the universal key manifest (UKS-004)
  pure def computeManifestSha(
    ep: int,
    kDSig: int,
    listSig: int,
    ovmC: int,
    sbC: int,
    morC: int
  ): int =
    1000 * ep + kDSig + listSig + ovmC + sbC + morC

  // Check whether all 5 leaf positions in `key_root` (chip sets 0..2, R_SL set 3, KEYDAG_IDX) are distinct
  pure def fiveLeafIndicesDistinct(c0: int, c1: int, c2: int, rslIdx: int, dagIdx: int): bool =
    Set(c0, c1, c2, rslIdx, dagIdx).size() == 5

  // Has the T12 security gate (G1..G5 and FP-8) passed on the active key set?
  pure def securityGateAllPassed(s: KeyMgmtState): bool =
    s.keygenEpoch > 0
      and s.secGateKeygenEpoch == s.keygenEpoch
      and s.secGatePassedChecks.size() == SECURITY_GATE_CHECK_COUNT

  // Guard: can the rekey runner start keygen? (UKS-009, UKS-010, RKY-002)
  pure def missingMandatoryRows(s: KeyMgmtState): Set[ListRowKind] =
    MANDATORY_LIST_ROWS.exclude(s.currentListRows)

  pure def canStartKeygen(s: KeyMgmtState, mutId: int): bool =
    val rowsOk =
      if (mutId == MUT_UKS010_MISSING_LIST_ROW) true
      else missingMandatoryRows(s).size() == 0
    val commitsOk =
      if (mutId == MUT_RKY002_UNCOMMITTED_ROW) true
      else s.uncommittedRows.intersect(s.currentListRows).size() == 0
    val ancestorOk =
      if (mutId == MUT_UKS009_NON_DESCENDANT) true
      else s.treeDescendantOfAllRows
    rowsOk and commitsOk and ancestorOk

  // Pure transition for `startAndRunKeygen` (RKY-001, UKS-002, UKS-004, UKS-007)
  pure def applyKeygen(
    s: KeyMgmtState,
    nodeRowsToBuild: int,
    foreignRowsCnt: int,
    wrapK: int,
    wrapVkB: int,
    mutId: int
  ): KeyMgmtState =
    val nextEp = s.keygenEpoch + 1
    val lSha = s.currentListSha
    val uks: UniversalKeySetRec = {
      epoch: nextEp,
      internalRecKeyId: 10 * nextEp + 1,
      lKeyId: 10 * nextEp + 2,
      lfrKeyId: 10 * nextEp + 3,
      wrapKeyId: 10 * nextEp + 4,
      lKeyListSha: lSha,
      lfrKeyListSha: lSha,
      wrapKeyListSha: if (mutId == MUT_RKY001_STALE_WRAP_KEY) T10M_OLD_MANIFEST_SHA else lSha,
      wrapLogK: wrapK,
      wrapVkBytes: wrapVkB,
    }
    val kdDigest = 500 + 10 * nextEp + nodeRowsToBuild + foreignRowsCnt
    val kd: KeysDRec = {
      built: true,
      nodeRowCount: nodeRowsToBuild,
      row4NwKey: if (nodeRowsToBuild >= 7) (40 + nextEp) else 0,
      row5Nw2Key: if (nodeRowsToBuild >= 7) (50 + nextEp) else 0,
      row6N3Nw2Key: if (nodeRowsToBuild >= 7) (60 + nextEp) else 0,
      foreignRowCount: foreignRowsCnt,
      keysDDigest: kdDigest,
    }
    val mSha = computeManifestSha(
      nextEp,
      kdDigest,
      lSha,
      s.treeOpenvmCommit,
      s.treeStarkBackendCommit,
      s.treeMoriartyCommit
    )
    val mf: ManifestRec = {
      written: true,
      manifestSha256: mSha,
      internalRecSha: uks.internalRecKeyId,
      lKeySha: uks.lKeyId,
      lfrKeySha: uks.lfrKeyId,
      wrapKeySha: uks.wrapKeyId,
      keysDDigest: kdDigest,
      listSha256: lSha,
      openvmCommit: if (mutId == MUT_UKS004_MANIFEST_UNBINDS) 0 else s.treeOpenvmCommit,
      starkBackendCommit: s.treeStarkBackendCommit,
      moriartyCommit: s.treeMoriartyCommit,
      statusMark: MarkPrototype,
      holdsPerProgramKey: (mutId == MUT_UKS004_MANIFEST_UNBINDS),
    }
    {
      ...s,
      keygenEpoch: nextEp,
      rekeyPhase: PhaseKeygenComplete,
      lastKeygenListRows: s.currentListRows,
      lastKeygenListSha: lSha,
      lastKeygenAllRowsCommitted: s.uncommittedRows.intersect(s.currentListRows).size() == 0,
      lastKeygenTreeWasDescendant: s.treeDescendantOfAllRows,
      secondKeygenNeeded: false,
      universalKeySet: uks,
      keysD: kd,
      manifest: mf,
      settledProofs: Set(),
      universalKeysFrozen: false,
      lastRefusal: RefusalNone,
    }

  // Check whether `(leafIdx, keyComm)` is listed in a built `ProgramKeyRootRec`
  pure def isChildKeyListedInRoot(pkr: ProgramKeyRootRec, leafIdx: int, keyComm: int): bool =
    pkr.built and (
      (leafIdx == pkr.chip0Idx and keyComm == pkr.keyRootDigest + 10)
      or (leafIdx == pkr.chip1Idx and keyComm == pkr.keyRootDigest + 11)
      or (leafIdx == pkr.chip2Idx and keyComm == pkr.keyRootDigest + 12)
      or (pkr.hasRslChipSet3 and leafIdx == pkr.rslLeafIdx and keyComm == pkr.rslKeyComm)
    )

  // Pure guard helpers for box distribution, `key_root` build, proof start/verify, and key freeze
  pure def canDistributeKeysToBox(s: KeyMgmtState, b: BoxId): bool =
    val bx = s.boxes.get(b)
    s.manifest.written
      and s.rekeyPhase == PhaseRekeyDone
      and bx.installedManifestSha != s.manifest.manifestSha256
      and not(bx.proofRunning)

  pure def canBuildProgramKeyRoot(s: KeyMgmtState, p: ProgramId, rslIdx: int, daggerIdx: int, mutId: int): bool =
    val cur = s.programKeyRoots.get(p)
    val leafOk =
      if (mutId == MUT_UKS003_RSL_IDX_COLLISION) true
      else fiveLeafIndicesDistinct(0, 1, 2, rslIdx, daggerIdx)
    not(cur.built) and leafOk

  pure def canStartProofOnBox(s: KeyMgmtState, b: BoxId, p: ProgramId, mutId: int): bool =
    val bx = s.boxes.get(b)
    val pkr = s.programKeyRoots.get(p)
    val manifestMatches =
      if (mutId == MUT_UKS005_STALE_BOX_PROVES)
        s.manifest.written
      else
        (s.manifest.written and s.rekeyPhase == PhaseRekeyDone and bx.installedManifestSha == s.manifest.manifestSha256)
    manifestMatches and pkr.built and not(bx.proofRunning)

  pure def canVerifyFullTree(
    s: KeyMgmtState,
    b: BoxId,
    p: ProgramId,
    useForeignKeySet: bool,
    useUnlistedKey: bool,
    mutId: int
  ): bool =
    val bx = s.boxes.get(b)
    val pkr = s.programKeyRoots.get(p)
    val childComm = if (useUnlistedKey) 9999 else pkr.rslKeyComm
    val inclusionOk =
      if (mutId == MUT_SPL005_UNLISTED_CHILD_KEY) pkr.built
      else isChildKeyListedInRoot(pkr, pkr.rslLeafIdx, childComm)
    val keySetForeignAllowed = (mutId == MUT_UKS001_SPLIT_KEYSET) or not(useForeignKeySet)
    bx.proofRunning and bx.activeProg == p and s.manifest.written and inclusionOk and keySetForeignAllowed

  pure def canFreezeUniversalKeys(s: KeyMgmtState, mutId: int): bool =
    val gateOk =
      if (mutId == MUT_SQG011_FREEZE_BEFORE_GATE) s.manifest.written
      else securityGateAllPassed(s)
    gateOk and not(s.universalKeysFrozen)

  // =========================================================================
  // 4. State Variables & Initialization
  // =========================================================================

  var km: KeyMgmtState
  var lastOp: str

  action init: bool =
    val initRows = MANDATORY_LIST_ROWS
    val initListSha = computeListSha(initRows)
    val emptyProgRoot: ProgramKeyRootRec = {
      built: false,
      keyRootDigest: 0,
      chip0Idx: 0,
      chip1Idx: 1,
      chip2Idx: 2,
      hasRslChipSet3: false,
      rslLeafIdx: 0,
      keydagIdx: KEYDAG_IDX,
      rslKeyComm: 0,
      keydagComm: 0,
    }
    val initBox: BoxStateRec = {
      installedManifestSha: T10M_OLD_MANIFEST_SHA,
      keyFileCount: 4,
      selectedShape: 1,
      planD: 700,
      proofEnteringValue: 42,
      setupCompleted: false,
      preSetupManifestSha: T10M_OLD_MANIFEST_SHA,
      preSetupKeyFileCount: 4,
      preSetupShape: 1,
      preSetupPlanD: 700,
      preSetupProofVal: 42,
      proofRunning: false,
      activeProg: ProgT02Increment,
    }
    all {
      km' = {
        currentListRows: initRows,
        uncommittedRows: Set(),
        treeDescendantOfAllRows: true,
        currentListSha: initListSha,
        treeOpenvmCommit: 11,
        treeStarkBackendCommit: 22,
        treeMoriartyCommit: 33,
        keygenEpoch: 0,
        rekeyPhase: PhaseRekeyIdle,
        lastKeygenListRows: Set(),
        lastKeygenListSha: 0,
        lastKeygenAllRowsCommitted: true,
        lastKeygenTreeWasDescendant: true,
        secondKeygenNeeded: false,
        universalKeySet: {
          epoch: 0,
          internalRecKeyId: 0,
          lKeyId: 0,
          lfrKeyId: 0,
          wrapKeyId: 0,
          lKeyListSha: 0,
          lfrKeyListSha: 0,
          wrapKeyListSha: 0,
          wrapLogK: 0,
          wrapVkBytes: 0,
        },
        keysD: {
          built: false,
          nodeRowCount: 0,
          row4NwKey: 0,
          row5Nw2Key: 0,
          row6N3Nw2Key: 0,
          foreignRowCount: 0,
          keysDDigest: 0,
        },
        manifest: {
          written: false,
          manifestSha256: 0,
          internalRecSha: 0,
          lKeySha: 0,
          lfrKeySha: 0,
          wrapKeySha: 0,
          keysDDigest: 0,
          listSha256: 0,
          openvmCommit: 0,
          starkBackendCommit: 0,
          moriartyCommit: 0,
          statusMark: MarkUnwritten,
          holdsPerProgramKey: false,
        },
        gateRunKeygenEpoch: 0,
        gatesRerunCount: 0,
        gatesPassedCount: 0,
        d3CheckedEpoch: 0,
        d3MinTermDeciBits: 0,
        d3MinGrindableDeciBits: 0,
        secGateKeygenEpoch: 0,
        secGatePassedChecks: Set(),
        universalKeysFrozen: false,
        programKeyRoots: ALL_PROGRAMS.mapBy(_ => emptyProgRoot),
        boxes: ALL_BOXES.mapBy(_ => initBox),
        settledProofs: Set(),
        lastRefusal: RefusalNone,
      },
      lastOp' = "init",
    }

  // =========================================================================
  // 5. Actions (Each Tagged with Its EARS Requirement IDs)
  // =========================================================================

  // Implements test/precondition inputs for UKS-010, RKY-001, UKS-004:
  // Remove a mandatory row (e.g. RowFp3) before rekey, or append RowPostRekeyExtra after keygen.
  action dropListRowBeforeRekey(r: ListRowKind): bool = all {
    km.keygenEpoch == 0,
    km.currentListRows.contains(r),
    val updatedRows = km.currentListRows.exclude(Set(r))
    val updatedSha = computeListSha(updatedRows)
    km' = {
      ...km,
      currentListRows: updatedRows,
      currentListSha: updatedSha,
    },
    lastOp' = "dropListRowBeforeRekey",
  }

  // Implements RKY-001, UKS-004 (Scenario: A second keygen / row added after keygen)
  action appendRowAfterKeygen: bool = all {
    km.keygenEpoch > 0,
    not(km.currentListRows.contains(RowPostRekeyExtra)),
    val updatedRows = km.currentListRows.union(Set(RowPostRekeyExtra))
    val updatedSha = computeListSha(updatedRows)
    km' = {
      ...km,
      currentListRows: updatedRows,
      currentListSha: updatedSha,
      secondKeygenNeeded: (updatedSha != km.manifest.listSha256),
    },
    lastOp' = "appendRowAfterKeygen",
  }

  // Implements precondition setup for RKY-002 (Scenario: T12-SLOT15 uncommitted)
  action markRowUncommitted(r: ListRowKind): bool = all {
    km.keygenEpoch == 0,
    km.currentListRows.contains(r),
    not(km.uncommittedRows.contains(r)),
    km' = { ...km, uncommittedRows: km.uncommittedRows.union(Set(r)) },
    lastOp' = "markRowUncommitted",
  }

  // Implements precondition setup for UKS-009 (Scenario: keygen tree missing ancestor commit)
  action checkoutNonDescendantTree: bool = all {
    km.keygenEpoch == 0,
    km.treeDescendantOfAllRows,
    km' = { ...km, treeDescendantOfAllRows: false },
    lastOp' = "checkoutNonDescendantTree",
  }

  // Restore clean integration branch & complete committed LIST.md
  action prepareValidListAndTree: bool = all {
    missingMandatoryRows(km).size() > 0
      or km.uncommittedRows.size() > 0
      or not(km.treeDescendantOfAllRows),
    val restoredRows = km.currentListRows.union(MANDATORY_LIST_ROWS)
    val restoredSha = computeListSha(restoredRows)
    km' = {
      ...km,
      currentListRows: restoredRows,
      uncommittedRows: Set(),
      treeDescendantOfAllRows: true,
      currentListSha: restoredSha,
      secondKeygenNeeded: if (km.keygenEpoch > 0) (restoredSha != km.manifest.listSha256) else false,
    },
    lastOp' = "prepareValidListAndTree",
  }

  // Implements UKS-010: Refuse to start rekey when LIST.md lacks any of the 6 mandatory rows
  action refuseRekeyMissingRows: bool =
    val missing = missingMandatoryRows(km)
    all {
      missing.size() > 0,
      km' = { ...km, lastRefusal: RefusalMissingListRows(missing) },
      lastOp' = "refuseRekeyMissingRows",
    }

  // Implements RKY-002: Refuse to start rekey when a list row has no commit on its branch
  action refuseRekeyUncommittedRow(r: ListRowKind): bool = all {
    km.currentListRows.contains(r),
    km.uncommittedRows.contains(r),
    km' = { ...km, lastRefusal: RefusalUncommittedRow(r) },
    lastOp' = "refuseRekeyUncommittedRow",
  }

  // Implements UKS-009: Refuse to start rekey when keygen tree is not a descendant of all list commits
  action refuseRekeyNonDescendantTree: bool = all {
    not(km.treeDescendantOfAllRows),
    km' = { ...km, lastRefusal: RefusalNonDescendantTree },
    lastOp' = "refuseRekeyNonDescendantTree",
  }

  // Implements RKY-001, UKS-001, UKS-002, UKS-004, UKS-007, UKS-009, UKS-010, RKY-002:
  // Run single keygen from LIST.md, build 7-node-row keys_d, and write prototype key manifest.
  action startAndRunKeygen(nodeRowsChoice: int): bool = all {
    km.keygenEpoch < 2,
    not(km.universalKeysFrozen),
    ALL_BOXES.forall(b => not(km.boxes.get(b).proofRunning)),
    km.keygenEpoch == 0 or km.secondKeygenNeeded,
    canStartKeygen(km, MUTANT_ID),
    if (MUTANT_ID == MUT_UKS002_SHORT_KEYS_D)
      Set(NODE_ROWS_REQUIRED, 4).contains(nodeRowsChoice)
    else
      nodeRowsChoice == NODE_ROWS_REQUIRED,
    km' = applyKeygen(km, nodeRowsChoice, FOREIGN_ROWS_COUNT, MAX_WRAP_K, DEFAULT_WRAP_VK_BYTES, MUTANT_ID),
    lastOp' = "startAndRunKeygen",
  }

  // Implements UKS-006, RKY-005: Rerun all 12 gates and D3 security check on new keys -> PASS (RekeyDone)
  action runGatesAndD3Pass: bool = all {
    km.keygenEpoch > 0,
    km.rekeyPhase == PhaseKeygenComplete,
    DEFAULT_NON_GRIND_DECIBITS >= MIN_TERM_DECIBITS,
    DEFAULT_GRIND_DECIBITS >= MIN_GRINDABLE_DECIBITS,
    km.universalKeySet.wrapVkBytes <= MAX_WRAP_VK_BYTES,
    km.universalKeySet.wrapLogK <= MAX_WRAP_K,
    km' = {
      ...km,
      rekeyPhase: PhaseRekeyDone,
      gateRunKeygenEpoch: km.keygenEpoch,
      gatesRerunCount: REQUIRED_GATE_COUNT,
      gatesPassedCount: REQUIRED_GATE_COUNT,
      d3CheckedEpoch: km.keygenEpoch,
      d3MinTermDeciBits: DEFAULT_NON_GRIND_DECIBITS,
      d3MinGrindableDeciBits: DEFAULT_GRIND_DECIBITS,
      lastRefusal: RefusalNone,
    },
    lastOp' = "runGatesAndD3Pass",
  }

  // Implements UKS-006 (Scenario: failing x7 gate -> rekey is not DONE and no box receives keys)
  action runGatesFailX7: bool = all {
    km.keygenEpoch > 0,
    km.rekeyPhase == PhaseKeygenComplete,
    val nextPhase =
      if (MUTANT_ID == MUT_UKS006_SKIP_FAILED_GATE) PhaseRekeyDone else PhaseRekeyGateFailed
    km' = {
      ...km,
      rekeyPhase: nextPhase,
      gateRunKeygenEpoch: km.keygenEpoch,
      gatesRerunCount: REQUIRED_GATE_COUNT,
      gatesPassedCount: REQUIRED_GATE_COUNT - 1,
      d3CheckedEpoch: km.keygenEpoch,
      d3MinTermDeciBits: DEFAULT_NON_GRIND_DECIBITS,
      d3MinGrindableDeciBits: DEFAULT_GRIND_DECIBITS,
      lastRefusal: RefusalRekeyGateFailed(7),
    },
    lastOp' = "runGatesFailX7",
  }

  // Implements RKY-005 (Scenario: grindable term at 100.9 bits = 1009 deci-bits -> gate fails)
  action runD3FailGrindable100Point9: bool = all {
    km.keygenEpoch > 0,
    km.rekeyPhase == PhaseKeygenComplete,
    val nonGrind = DEFAULT_NON_GRIND_DECIBITS
    val grind1009 = 1009
    val nextPhase =
      if (MUTANT_ID == MUT_RKY005_GRINDABLE_100_9 and nonGrind >= MIN_TERM_DECIBITS and grind1009 >= 1000)
        PhaseRekeyDone
      else
        PhaseRekeyGateFailed
    km' = {
      ...km,
      rekeyPhase: nextPhase,
      gateRunKeygenEpoch: km.keygenEpoch,
      gatesRerunCount: REQUIRED_GATE_COUNT,
      gatesPassedCount: REQUIRED_GATE_COUNT,
      d3CheckedEpoch: km.keygenEpoch,
      d3MinTermDeciBits: nonGrind,
      d3MinGrindableDeciBits: grind1009,
      lastRefusal: RefusalD3SecurityFailed((nonGrind, grind1009)),
    },
    lastOp' = "runD3FailGrindable100Point9",
  }

  // Implements UKS-006, UKS-005: Transfer new universal keys & manifest to a box only when rekey is DONE
  action distributeKeysToBox(b: BoxId): bool =
    val bx = km.boxes.get(b)
    all {
      canDistributeKeysToBox(km, b),
      val updatedBox: BoxStateRec = {
        ...bx,
        installedManifestSha: km.manifest.manifestSha256,
        setupCompleted: false,
        preSetupManifestSha: km.manifest.manifestSha256,
      }
      km' = {
        ...km,
        boxes: km.boxes.put(b, updatedBox),
        lastRefusal: RefusalNone,
      },
      lastOp' = "distributeKeysToBox",
    }

  // Implements UKS-008: Setup mode runs on a box and makes, changes and selects no key, shape, plan_d or proof value
  action runSetupModeClean(b: BoxId): bool =
    val bx = km.boxes.get(b)
    all {
      not(bx.setupCompleted),
      not(bx.proofRunning),
      val snapped: BoxStateRec = {
        ...bx,
        preSetupManifestSha: bx.installedManifestSha,
        preSetupKeyFileCount: bx.keyFileCount,
        preSetupShape: bx.selectedShape,
        preSetupPlanD: bx.planD,
        preSetupProofVal: bx.proofEnteringValue,
        setupCompleted: true,
      }
      val postBox: BoxStateRec =
        if (MUTANT_ID == MUT_UKS008_SETUP_MUTATES_KEY)
          { ...snapped, keyFileCount: bx.keyFileCount + 1, planD: bx.planD + 1 }
        else
          snapped
      km' = {
        ...km,
        boxes: km.boxes.put(b, postBox),
        lastRefusal: RefusalNone,
      },
      lastOp' = "runSetupModeClean",
    }

  // Implements UKS-008 Verification: key tamper attempted during setup -> setup refuses/aborts
  action abortSetupOnKeyTamper(b: BoxId): bool =
    val bx = km.boxes.get(b)
    all {
      not(bx.proofRunning),
      km' = {
        ...km,
        boxes: km.boxes.put(b, { ...bx, setupCompleted: false }),
        lastRefusal: RefusalSetupKeyTamper(b),
      },
      lastOp' = "abortSetupOnKeyTamper",
    }

  // Implements UKS-001, UKS-003: Build per-program `key_root` with chip sets 0..3 and KEYDAG_IDX
  // at 5 distinct leaf positions (`rslIdx != KEYDAG_IDX` and disjoint from 0..2).
  action buildProgramKeyRoot(p: ProgramId, rslIdx: int): bool =
    val rootDig = 800 + 100 * progBaseCode(p) + rslIdx
    all {
      canBuildProgramKeyRoot(km, p, rslIdx, KEYDAG_IDX, MUTANT_ID),
      val nextRoot: ProgramKeyRootRec = {
        built: true,
        keyRootDigest: rootDig,
        chip0Idx: 0,
        chip1Idx: 1,
        chip2Idx: 2,
        hasRslChipSet3: true,
        rslLeafIdx: rslIdx,
        keydagIdx: KEYDAG_IDX,
        rslKeyComm: rootDig + 13,
        keydagComm: rootDig + 99,
      }
      km' = {
        ...km,
        programKeyRoots: km.programKeyRoots.put(p, nextRoot),
        lastRefusal: RefusalNone,
      },
      lastOp' = "buildProgramKeyRoot",
    }

  // Implements UKS-003: Refuse colliding R_SL leaf index (`rslIdx == KEYDAG_IDX` or colliding with 0..2)
  action refuseCollidingRslLeafIdx(rslIdx: int): bool = all {
    not(fiveLeafIndicesDistinct(0, 1, 2, rslIdx, KEYDAG_IDX)),
    km' = {
      ...km,
      lastRefusal: RefusalCollidingRslLeafIdx((rslIdx, KEYDAG_IDX)),
    },
    lastOp' = "refuseCollidingRslLeafIdx",
  }

  // Implements UKS-005 (Scenario: huginn holds T10m keys -> prover refuses job and names both digests)
  action refuseStartProofManifestMismatch(b: BoxId): bool =
    val bx = km.boxes.get(b)
    all {
      km.manifest.written,
      bx.installedManifestSha != km.manifest.manifestSha256,
      km' = {
        ...km,
        lastRefusal: RefusalManifestMismatch((b, bx.installedManifestSha, km.manifest.manifestSha256)),
      },
      lastOp' = "refuseStartProofManifestMismatch",
    }

  // Implements UKS-005: Prover starts a proof on a box ONLY when box manifest equals the rekey manifest
  action startProofOnBox(b: BoxId, p: ProgramId): bool =
    val bx = km.boxes.get(b)
    all {
      canStartProofOnBox(km, b, p, MUTANT_ID),
      km' = {
        ...km,
        boxes: km.boxes.put(b, { ...bx, proofRunning: true, activeProg: p }),
        lastRefusal: RefusalNone,
      },
      lastOp' = "startProofOnBox",
    }

  // Implements UKS-001, SPL-005 (Full tree: N1 checks child key commitment into `key_root` and forwards root)
  action verifyAndSettleFullTree(b: BoxId, p: ProgramId, useForeignKeySet: bool, useUnlistedKey: bool): bool =
    val bx = km.boxes.get(b)
    val pkr = km.programKeyRoots.get(p)
    val leafIdx = pkr.rslLeafIdx
    val childComm = if (useUnlistedKey) 9999 else pkr.rslKeyComm
    val mSha = if (useForeignKeySet) (km.manifest.manifestSha256 + 77) else km.manifest.manifestSha256
    val expX = pkr.keydagComm
    all {
      canVerifyFullTree(km, b, p, useForeignKeySet, useUnlistedKey, MUTANT_ID),
      val rec: SettledProofRec = {
        progId: p,
        usedManifestSha: mSha,
        usedInternalRecKey: km.universalKeySet.internalRecKeyId,
        usedLKey: km.universalKeySet.lKeyId,
        usedLfrKey: km.universalKeySet.lfrKeyId,
        usedWrapKey: km.universalKeySet.wrapKeyId,
        usedKeyRootDigest: pkr.keyRootDigest,
        usedPathC: false,
        openedChipLeafIdx: leafIdx,
        openedDagLeafIdx: -1,
        childKeyComm: childComm,
        publicX: expX,
        expectedX: expX,
      }
      km' = {
        ...km,
        boxes: km.boxes.put(b, { ...bx, proofRunning: false }),
        settledProofs: km.settledProofs.union(Set(rec)),
        lastRefusal: RefusalNone,
      },
      lastOp' = "verifyAndSettleFullTree",
    }

  // Implements UKS-001, UKS-003, SPL-005 (Path C: `L` opens chip set 3 leaf and KEYDAG_IDX at distinct
  // positions, exposes public X, and wrap checks X)
  action verifyAndSettlePathC(b: BoxId, p: ProgramId): bool =
    val bx = km.boxes.get(b)
    val pkr = km.programKeyRoots.get(p)
    val expX = pkr.keydagComm
    all {
      bx.proofRunning,
      bx.activeProg == p,
      km.manifest.written,
      isChildKeyListedInRoot(pkr, pkr.rslLeafIdx, pkr.rslKeyComm),
      pkr.rslLeafIdx != pkr.keydagIdx,
      val rec: SettledProofRec = {
        progId: p,
        usedManifestSha: km.manifest.manifestSha256,
        usedInternalRecKey: km.universalKeySet.internalRecKeyId,
        usedLKey: km.universalKeySet.lKeyId,
        usedLfrKey: km.universalKeySet.lfrKeyId,
        usedWrapKey: km.universalKeySet.wrapKeyId,
        usedKeyRootDigest: pkr.keyRootDigest,
        usedPathC: true,
        openedChipLeafIdx: pkr.rslLeafIdx,
        openedDagLeafIdx: pkr.keydagIdx,
        childKeyComm: pkr.rslKeyComm,
        publicX: expX,
        expectedX: expX,
      }
      km' = {
        ...km,
        boxes: km.boxes.put(b, { ...bx, proofRunning: false }),
        settledProofs: km.settledProofs.union(Set(rec)),
        lastRefusal: RefusalNone,
      },
      lastOp' = "verifyAndSettlePathC",
    }

  // Implements SPL-005 (Scenario: Child under an unlisted key -> node / wrap refuses)
  action refuseUnlistedChildKey(p: ProgramId): bool =
    val pkr = km.programKeyRoots.get(p)
    val badComm = 9999
    all {
      pkr.built,
      not(isChildKeyListedInRoot(pkr, pkr.rslLeafIdx, badComm)),
      km' = {
        ...km,
        lastRefusal: RefusalUnlistedChildKey((p, badComm)),
      },
      lastOp' = "refuseUnlistedChildKey",
    }

  // Implements UKS-007: T12 security gate (G1..G5 and FP-8) passes on the new key set
  action passSecurityGateAllChecks: bool = all {
    km.rekeyPhase == PhaseRekeyDone,
    km.manifest.written,
    not(securityGateAllPassed(km)),
    km' = {
      ...km,
      secGateKeygenEpoch: km.keygenEpoch,
      secGatePassedChecks: 1.to(SECURITY_GATE_CHECK_COUNT),
      manifest: { ...km.manifest, statusMark: MarkQualified },
      lastRefusal: RefusalNone,
    },
    lastOp' = "passSecurityGateAllChecks",
  }

  // Mutant-only premature qualification trigger for UKS-007 (`MUT_UKS007_PREMATURE_QUALIFIED`)
  action markManifestQualifiedPrematurely: bool = all {
    MUTANT_ID == MUT_UKS007_PREMATURE_QUALIFIED,
    km.manifest.written,
    not(securityGateAllPassed(km)),
    km.manifest.statusMark == MarkPrototype,
    km' = {
      ...km,
      manifest: { ...km.manifest, statusMark: MarkQualified },
    },
    lastOp' = "markManifestQualifiedPrematurely",
  }

  // Implements SQG-011 (Scenario: Freeze request before the gate -> freeze is refused)
  action refuseFreezeBeforeGate: bool = all {
    km.manifest.written,
    not(securityGateAllPassed(km)),
    km' = {
      ...km,
      lastRefusal: RefusalFreezeBeforeSecurityGate,
    },
    lastOp' = "refuseFreezeBeforeGate",
  }

  // Implements SQG-011: Freeze universal keys ONLY after G1..G4 pass with G5 and FP-8 closes
  action freezeUniversalKeys: bool = all {
    canFreezeUniversalKeys(km, MUTANT_ID),
    km' = {
      ...km,
      universalKeysFrozen: true,
      lastRefusal: RefusalNone,
    },
    lastOp' = "freezeUniversalKeys",
  }

  // =========================================================================
  // 6. Nondeterministic `step` Relation
  // =========================================================================

  action step: bool = {
    nondet boxChoice = ALL_BOXES.oneOf()
    nondet progChoice = ALL_PROGRAMS.oneOf()
    nondet rowChoice = MANDATORY_LIST_ROWS.oneOf()
    nondet rslIdxMut = Set(KEYDAG_IDX, RSL_LEAF_IDX_CFG).oneOf()
    nondet nodeRowsMut = Set(NODE_ROWS_REQUIRED, 4).oneOf()
    nondet badKeyMut = Set(false, true).oneOf()
    nondet foreignKeyMut = Set(false, true).oneOf()
    val rslChoice = if (MUTANT_ID == MUT_UKS003_RSL_IDX_COLLISION) rslIdxMut else RSL_LEAF_IDX_CFG
    val nodeChoice = if (MUTANT_ID == MUT_UKS002_SHORT_KEYS_D) nodeRowsMut else NODE_ROWS_REQUIRED
    val foreignChoice = if (MUTANT_ID == MUT_UKS001_SPLIT_KEYSET) foreignKeyMut else false
    val badKeyChoice = if (MUTANT_ID == MUT_SPL005_UNLISTED_CHILD_KEY) badKeyMut else false
    any {
      all { lastOp == "init", dropListRowBeforeRekey(rowChoice) },
      all { lastOp == "init", markRowUncommitted(rowChoice) },
      all { lastOp == "init", checkoutNonDescendantTree },
      prepareValidListAndTree,
      all { km.lastRefusal == RefusalNone, refuseRekeyMissingRows },
      all { km.lastRefusal == RefusalNone, refuseRekeyUncommittedRow(rowChoice) },
      all { km.lastRefusal == RefusalNone, refuseRekeyNonDescendantTree },
      startAndRunKeygen(nodeChoice),
      runGatesAndD3Pass,
      runGatesFailX7,
      runD3FailGrindable100Point9,
      distributeKeysToBox(BoxMuninn),
      distributeKeysToBox(BoxHuginn),
      distributeKeysToBox(BoxOdin),
      runSetupModeClean(boxChoice),
      all { km.boxes.get(boxChoice).setupCompleted, km.lastRefusal == RefusalNone, abortSetupOnKeyTamper(boxChoice) },
      buildProgramKeyRoot(ProgT02Increment, rslChoice),
      buildProgramKeyRoot(ProgD04Withdraw, rslChoice),
      all { km.lastRefusal == RefusalNone, not(km.programKeyRoots.get(progChoice).built), refuseCollidingRslLeafIdx(KEYDAG_IDX) },
      all { km.lastRefusal == RefusalNone, refuseStartProofManifestMismatch(boxChoice) },
      startProofOnBox(BoxMuninn, ProgT02Increment),
      startProofOnBox(BoxMuninn, ProgD04Withdraw),
      startProofOnBox(BoxHuginn, ProgT02Increment),
      startProofOnBox(BoxHuginn, ProgD04Withdraw),
      startProofOnBox(BoxOdin, ProgT02Increment),
      startProofOnBox(BoxOdin, ProgD04Withdraw),
      verifyAndSettleFullTree(BoxMuninn, km.boxes.get(BoxMuninn).activeProg, foreignChoice, badKeyChoice),
      verifyAndSettleFullTree(BoxHuginn, km.boxes.get(BoxHuginn).activeProg, foreignChoice, badKeyChoice),
      verifyAndSettleFullTree(BoxOdin, km.boxes.get(BoxOdin).activeProg, foreignChoice, badKeyChoice),
      verifyAndSettlePathC(BoxMuninn, km.boxes.get(BoxMuninn).activeProg),
      verifyAndSettlePathC(BoxHuginn, km.boxes.get(BoxHuginn).activeProg),
      verifyAndSettlePathC(BoxOdin, km.boxes.get(BoxOdin).activeProg),
      all { km.lastRefusal == RefusalNone, refuseUnlistedChildKey(progChoice) },
      passSecurityGateAllChecks,
      markManifestQualifiedPrematurely,
      all { km.lastRefusal == RefusalNone, refuseFreezeBeforeGate },
      freezeUniversalKeys,
      appendRowAfterKeygen,
    }
  }

  // =========================================================================
  // 7. Safety Invariants (`val inv_<ID>` and Master Conjunction `val inv`)
  // =========================================================================

  // Pure cross-constant consistency check
  val consistency_constants: bool = and {
    NODE_ROWS_REQUIRED == 7,
    FOREIGN_ROWS_COUNT >= 1,
    fiveLeafIndicesDistinct(0, 1, 2, RSL_LEAF_IDX_CFG, KEYDAG_IDX),
    REQUIRED_LIST_ROW_COUNT == 6,
    REQUIRED_GATE_COUNT == 12,
    SECURITY_GATE_CHECK_COUNT == 6,
    MIN_TERM_DECIBITS >= 1000,
    MIN_GRINDABLE_DECIBITS >= 1010,
    MAX_WRAP_K <= 21,
    MAX_WRAP_VK_BYTES <= 50000,
  }

  // UKS-001: One universal key set (1 internal-rec, 1 L, 1 L_Fr, 1 wrap) for every Compact suite program;
  //          distinct programs have distinct `key_root` values.
  val inv_UKS_001: bool = and {
    (km.keygenEpoch > 0) implies (
      km.universalKeySet.internalRecKeyId > 0
        and km.universalKeySet.lKeyId > 0
        and km.universalKeySet.lfrKeyId > 0
        and km.universalKeySet.wrapKeyId > 0
    ),
    km.settledProofs.forall(p1 =>
      p1.usedManifestSha == km.manifest.manifestSha256
        and km.settledProofs.forall(p2 =>
          (p1.usedManifestSha == p2.usedManifestSha)
            and ((p1.progId != p2.progId) implies (p1.usedKeyRootDigest != p2.usedKeyRootDigest))
        )
    ),
  }

  // UKS-002: `keys_d` is built from 7 node rows (rows 4, 5, 6 holding NW, NW2, N3 over NW2) and foreign rows.
  val inv_UKS_002: bool =
    km.keysD.built implies (
      km.keysD.nodeRowCount == NODE_ROWS_REQUIRED
        and km.keysD.row4NwKey > 0
        and km.keysD.row5Nw2Key > 0
        and km.keysD.row6N3Nw2Key > 0
        and Set(km.keysD.row4NwKey, km.keysD.row5Nw2Key, km.keysD.row6N3Nw2Key).size() == 3
        and km.keysD.foreignRowCount >= 1
    )

  // UKS-003: R_SL chip set 3 key leaf index differs from `KEYDAG_IDX` and from chip sets 0..2 (5 distinct positions).
  val inv_UKS_003: bool =
    ALL_PROGRAMS.forall(p =>
      val pkr = km.programKeyRoots.get(p)
      (pkr.built and pkr.hasRslChipSet3) implies (
        pkr.rslLeafIdx != pkr.keydagIdx
          and fiveLeafIndicesDistinct(pkr.chip0Idx, pkr.chip1Idx, pkr.chip2Idx, pkr.rslLeafIdx, pkr.keydagIdx)
      )
    )

  // UKS-004: Rekey writes a universal key manifest holding sha256 of every universal key file, `keys_d`,
  //          sha256 of `LIST.md`, and the 3 repo commits, and holding no per-program key.
  val inv_UKS_004: bool =
    km.manifest.written implies (
      km.manifest.internalRecSha == km.universalKeySet.internalRecKeyId
        and km.manifest.lKeySha == km.universalKeySet.lKeyId
        and km.manifest.lfrKeySha == km.universalKeySet.lfrKeyId
        and km.manifest.wrapKeySha == km.universalKeySet.wrapKeyId
        and km.manifest.keysDDigest == km.keysD.keysDDigest
        and km.manifest.listSha256 == km.lastKeygenListSha
        and km.manifest.openvmCommit > 0
        and km.manifest.starkBackendCommit > 0
        and km.manifest.moriartyCommit > 0
        and not(km.manifest.holdsPerProgramKey)
        and ((km.currentListSha != km.manifest.listSha256) implies km.secondKeygenNeeded)
    )

  // UKS-005: A box whose key manifest differs from the rekey manifest never starts/runs a proof.
  val inv_UKS_005: bool =
    ALL_BOXES.forall(b =>
      val bx = km.boxes.get(b)
      bx.proofRunning implies (
        km.manifest.written
          and bx.installedManifestSha == km.manifest.manifestSha256
      )
    )

  // UKS-006: Rekey is DONE and distributes new keys to boxes ONLY after all 12 gates rerun and pass on the new keys.
  val inv_UKS_006: bool = and {
    (km.rekeyPhase == PhaseRekeyDone) implies (
      km.gateRunKeygenEpoch == km.keygenEpoch
        and km.gatesRerunCount == REQUIRED_GATE_COUNT
        and km.gatesPassedCount == REQUIRED_GATE_COUNT
    ),
    ALL_BOXES.forall(b =>
      val bx = km.boxes.get(b)
      (km.manifest.written and bx.installedManifestSha == km.manifest.manifestSha256)
        implies (km.rekeyPhase == PhaseRekeyDone)
    ),
  }

  // UKS-007: While the T12 security gate (G1..G5 and FP-8) has not passed on a key set, its manifest marks `prototype`.
  val inv_UKS_007: bool =
    (km.manifest.written and not(securityGateAllPassed(km)))
      implies (km.manifest.statusMark == MarkPrototype)

  // UKS-008: Setup mode makes, changes and selects no key, shape, `plan_d` or proof-entering value.
  val inv_UKS_008: bool =
    ALL_BOXES.forall(b =>
      val bx = km.boxes.get(b)
      bx.setupCompleted implies (
        bx.installedManifestSha == bx.preSetupManifestSha
          and bx.keyFileCount == bx.preSetupKeyFileCount
          and bx.selectedShape == bx.preSetupShape
          and bx.planD == bx.preSetupPlanD
          and bx.proofEnteringValue == bx.preSetupProofVal
      )
    )

  // UKS-009: Rekey never starts on a keygen tree that is not a descendant of every commit in `LIST.md`.
  val inv_UKS_009: bool =
    (km.keygenEpoch > 0) implies km.lastKeygenTreeWasDescendant

  // UKS-010: Rekey never starts if `LIST.md` lacks any of the 6 mandatory key-changing rows.
  val inv_UKS_010: bool =
    (km.keygenEpoch > 0) implies (
      MANDATORY_LIST_ROWS.forall(r => km.lastKeygenListRows.contains(r))
    )

  // RKY-001: Rekey regenerates L, L_Fr and wrap keys once from `LIST.md`; a later row change marks secondKeygenNeeded.
  val inv_RKY_001: bool =
    (km.keygenEpoch > 0) implies (
      km.universalKeySet.lKeyListSha == km.lastKeygenListSha
        and km.universalKeySet.lfrKeyListSha == km.lastKeygenListSha
        and km.universalKeySet.wrapKeyListSha == km.lastKeygenListSha
        and ((km.currentListRows != km.lastKeygenListRows) implies km.secondKeygenNeeded)
    )

  // RKY-002: Rekey never starts if any row of `LIST.md` has no commit on its branch.
  val inv_RKY_002: bool =
    (km.keygenEpoch > 0) implies km.lastKeygenAllRowsCommitted

  // RKY-005: Completed rekey requires D3 >= 100.0 bits on every term and >= 101.0 bits on every grindable term.
  val inv_RKY_005: bool =
    (km.rekeyPhase == PhaseRekeyDone) implies (
      km.d3CheckedEpoch == km.keygenEpoch
        and km.d3MinTermDeciBits >= MIN_TERM_DECIBITS
        and km.d3MinGrindableDeciBits >= MIN_GRINDABLE_DECIBITS
        and km.universalKeySet.wrapLogK <= MAX_WRAP_K
        and km.universalKeySet.wrapVkBytes <= MAX_WRAP_VK_BYTES
    )

  // Adopted SPL-005: Every accepted proof verified its child key commitment against `key_root`
  //                  (and on Path C opened distinct leaf indices and checked public `X`).
  val inv_SPL_005: bool =
    km.settledProofs.forall(sp =>
      val pkr = km.programKeyRoots.get(sp.progId)
      isChildKeyListedInRoot(pkr, sp.openedChipLeafIdx, sp.childKeyComm)
        and sp.usedKeyRootDigest == pkr.keyRootDigest
        and (sp.usedPathC implies (
          sp.openedDagLeafIdx == pkr.keydagIdx
            and sp.openedChipLeafIdx != sp.openedDagLeafIdx
            and sp.publicX == sp.expectedX
        ))
    )

  // Adopted SQG-011: No universal key freezes before G1..G4 pass with G5 and FP-8 closes.
  val inv_SQG_011: bool =
    km.universalKeysFrozen implies securityGateAllPassed(km)

  // Master safety conjunction (`val inv` per PROMPT.md item 2)
  val inv: bool = and {
    inv_UKS_001,
    inv_UKS_002,
    inv_UKS_003,
    inv_UKS_004,
    inv_UKS_005,
    inv_UKS_006,
    inv_UKS_007,
    inv_UKS_008,
    inv_UKS_009,
    inv_UKS_010,
    inv_RKY_001,
    inv_RKY_002,
    inv_RKY_005,
    inv_SPL_005,
    inv_SQG_011,
  }

  // =========================================================================
  // 8. Non-Vacuity Witnesses (Negated Goals: MUST Violate / Exit 1 in Main)
  // =========================================================================

  val reach_rekey_done_and_distributed: bool = not(
    km.rekeyPhase == PhaseRekeyDone
      and ALL_BOXES.exists(b => km.boxes.get(b).installedManifestSha == km.manifest.manifestSha256)
  )

  val reach_two_suite_programs_settled: bool = not(
    km.settledProofs.exists(p1 => p1.progId == ProgT02Increment)
      and km.settledProofs.exists(p2 => p2.progId == ProgD04Withdraw and p2.usedPathC)
  )

  val reach_manifest_mismatch_refused: bool = not(
    match km.lastRefusal {
      | RefusalManifestMismatch(_) => true
      | _ => false
    }
  )

  val reach_gate_or_d3_failure_refused: bool = not(
    km.rekeyPhase == PhaseRekeyGateFailed
  )

  val reach_pre_rekey_guard_refused: bool = not(
    match km.lastRefusal {
      | RefusalMissingListRows(_) => true
      | RefusalUncommittedRow(_)  => true
      | RefusalNonDescendantTree  => true
      | _ => false
    }
  )

  val reach_qualified_and_frozen: bool = not(
    km.manifest.statusMark == MarkQualified and km.universalKeysFrozen
  )

  val reach_second_keygen_triggered: bool = not(
    km.keygenEpoch >= 1 and km.secondKeygenNeeded
  )
}

// ===========================================================================
// 9. Concrete Instances: Main, Edge Instances, and 13 Guard Mutants
// ===========================================================================

module key_management_main {
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
  ).*
}

// Edge Instance 1: Smallest legal foreign row count (1) and VK size (1 byte)
module key_management_edge_min {
  import key_management(
    NODE_ROWS_REQUIRED = 7,
    FOREIGN_ROWS_COUNT = 1,
    KEYDAG_IDX = 3,
    RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6,
    REQUIRED_GATE_COUNT = 12,
    SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000,
    MIN_GRINDABLE_DECIBITS = 1010,
    MAX_WRAP_K = 21,
    MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1000,
    DEFAULT_GRIND_DECIBITS = 1010,
    DEFAULT_WRAP_VK_BYTES = 1,
    MUTANT_ID = 0
  ).*
}

// Edge Instance 2: Every numeric bound exact at its spec limit (100.0 bits, 101.0 bits, k=21, VK=50,000 bytes)
module key_management_edge_at_limit {
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
    DEFAULT_NON_GRIND_DECIBITS = 1000,
    DEFAULT_GRIND_DECIBITS = 1010,
    DEFAULT_WRAP_VK_BYTES = 50000,
    MUTANT_ID = 0
  ).*
}

// --- Mutant Instances (`key_management_m_<name>`) ---

module key_management_m_uks001_split_keyset {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 1
  ).*
}

module key_management_m_uks002_short_keys_d {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 2
  ).*
}

module key_management_m_uks003_rsl_idx_collision {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 3
  ).*
}

module key_management_m_uks005_stale_box_proves {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 4
  ).*
}

module key_management_m_uks006_skip_failed_gate {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 5
  ).*
}

module key_management_m_uks007_premature_qualified {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 6
  ).*
}

module key_management_m_uks008_setup_mutates_key {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 7
  ).*
}

module key_management_m_uks009_non_descendant_tree {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 8
  ).*
}

module key_management_m_uks010_missing_list_row {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 9
  ).*
}

module key_management_m_rky002_uncommitted_row {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 10
  ).*
}

module key_management_m_rky005_grindable_100_9 {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 11
  ).*
}

module key_management_m_spl005_unlisted_child_key {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 12
  ).*
}

module key_management_m_sqg011_freeze_before_gate {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 13
  ).*
}

module key_management_m_uks004_manifest_unbinds {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 14
  ).*
}

module key_management_m_rky001_stale_wrap_key {
  import key_management(
    NODE_ROWS_REQUIRED = 7, FOREIGN_ROWS_COUNT = 2, KEYDAG_IDX = 3, RSL_LEAF_IDX_CFG = 4,
    REQUIRED_LIST_ROW_COUNT = 6, REQUIRED_GATE_COUNT = 12, SECURITY_GATE_CHECK_COUNT = 6,
    MIN_TERM_DECIBITS = 1000, MIN_GRINDABLE_DECIBITS = 1010, MAX_WRAP_K = 21, MAX_WRAP_VK_BYTES = 50000,
    DEFAULT_NON_GRIND_DECIBITS = 1025, DEFAULT_GRIND_DECIBITS = 1030, DEFAULT_WRAP_VK_BYTES = 1325,
    MUTANT_ID = 15
  ).*
}
