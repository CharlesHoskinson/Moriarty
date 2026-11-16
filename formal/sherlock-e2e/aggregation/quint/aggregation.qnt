// Quint specification for Sherlock E2E Prototype — Bounded Context: Aggregation
// Lane: e2e-aggregation
// Sources: NAG-001..011, GPH-001..008, LPC-001..003, and adopted DG-010, CPR-005, GP-002..004.

module aggregation {
  // =========================================================================
  // 1. Spec Constants (numeric bounds and T8c / key-set capability flags)
  // =========================================================================

  const MERGE_WORK_MAX_COLS: int     // NAG-005, LPC-002: max MERGE-WORK VK columns (5,400)
  const MW_BASE_COLS: int            // NAG-005: post-T10q MERGE-WORK base width (4,988)
  const MW_VALUE_DIGEST_DELTA: int   // NAG-005, LPC-002: value-set CORE delta (+62)
  const MW_STACKS_DELTA: int         // NAG-005, LPC-002: STACKS AIR delta (+71)
  const KEYS_D_NODE_ROWS: int        // UKS-002 / GPH-005: node rows in keys_d (7)
  const KEYS_D_FOREIGN_ROWS: int     // GPH-003: foreign-verifier rows in keys_d (2)
  const GP2_CASE_FLOOR: int          // GPH-007: gp2 gate case count floor (48)
  const X7_CASE_FLOOR: int           // GPH-007: x7 gate case count floor (22)
  const CLAIMS_TREE_DEPTH: int       // GPH-008: SET claims Merkle depth (20)
  const HAS_X8_COMPACT_VERIFIER: bool // GPH-004, LPC-001, LPC-003: X8 in-circuit Compact verifier built
  const HAS_L_COLLECTION_KEY: bool    // GPH-008, LPC-001, LPC-003: L collection form keyed
  const MUTANT: str                  // "none" in sound instances, mutant tag in aggregation_m_*

  // =========================================================================
  // 2. Domain Types (enums and cohesive records — no string manipulation)
  // =========================================================================

  type RuleFamily =
    | FamInterpreted
    | FamRsl

  type StarkField =
    | FieldBabyBear
    | FieldWrong

  type ParamSet =
    | ParamInternal
    | ParamWrong

  type NodeTier =
    | TierN1
    | TierN2
    | TierN3
    | TierMergeWork

  type Gp2Mode =
    | ModeSegMerge
    | ModeSeal
    | ModeSealWithDeps
    | ModeJoin
    | ModeDlStepDescend
    | ModeFvAccept
    | ModePcdRoot

  type StmtKind =
    | StmtSeg
    | StmtClaim
    | StmtPendingDeps
    | StmtSet
    | StmtFv

  type TreeShape =
    | ShapeSingle
    | ShapeBalanced
    | ShapeChain

  type AdapterClass =
    | AdapterNone
    | AdapterCompactZkir3
    | AdapterUnboundRelabel

  type HookFreezeState =
    | HookFrozen
    | HookFreezePending

  type GateVerdict =
    | GateNotRun
    | GatePassed
    | GateFailedCaseMap
    | GateFailedDerived

  type ForgeryKind =
    | ForgeryNone
    | ForgeryWrongNodeParams
    | ForgeryOmitRequiredAir
    | ForgeryWrongKeyRoot
    | ForgeryMixedChipWrongKeyRoot
    | ForgeryMwF01FalseCarry
    | ForgeryAfkF7ShapeCollision
    | ForgeryGp2F42UnboundType
    | ForgeryAfkF9bUnlinkedOneChild
    | ForgeryGp2C21PubvConstRow
    | ForgeryDisjointOrSwappedLink
    | ForgeryWrongLOutcome
    | ForgeryUnprovedDep
    | ForgeryUnlistedFvKey
    | ForgeryUnboundForeignAdapter
    | ForgeryPcdShapeAltersClaim
    | ForgeryForgedSetRoot

  type RefusalCode =
    | RefNone
    | RefMissingRequiredAir
    | RefWrongProgDOpening
    | RefMergeWorkCols
    | RefT10qMwF01
    | RefT10qAfkF7
    | RefT10qGp2F42
    | RefT10qAfkF9b
    | RefGp2C21ConstRow
    | RefWrongOutcomeAtL
    | RefDisjointOrSwappedChildren
    | RefUnprovedOrUnsealedDep
    | RefMixedRegRootJoin
    | RefUnlistedForeignKey
    | RefUnboundCompactAdapter
    | RefForgedSetRootAtL

  type UniversalAggKeys = {
    internalRecVk: int,
    lVk: int,
    lFrVk: int,
    keyManifestSha256: int,
    nodeRowCount: int,
    foreignVkRows: Set[int],
  }

  type MergeWorkKeyRec = {
    mwKeyId: int,
    hasStacksAir: bool,
    hasValueDigest: bool,
    colCount: int,
  }

  type ProgramMeta = {
    progId: int,
    family: RuleFamily,
    progD: int,
    imageD: int,
    planD: int,
    keyRoot: int,
    machineVersion: int,
  }

  type N1ShardSpec = {
    n1Id: int,
    runId: int,
    progId: int,
    family: RuleFamily,
    chipSetId: int,
    hStart: int,
    hEnd: int,
    preD: int,
    postD: int,
    regRoot: int,
    isTerminal: bool,
    outcomeD: int,
    deps: Set[int],
    boxHost: int,
  }

  type RunCatalogEntry = {
    runId: int,
    progId: int,
    family: RuleFamily,
    inputD: int,
    leftN1Id: int,
    rightN1Id: int,
    expectedOutcomeD: int,
    isTerminalRun: bool,
    suspendedAndResumed: bool,
    deps: Set[int],
    regRoot: int,
  }

  type NodeProofRec = {
    nodeId: int,
    tier: NodeTier,
    gp2Mode: Gp2Mode,
    stmtKind: StmtKind,
    runId: int,
    progId: int,
    family: RuleFamily,
    starkField: StarkField,
    paramSet: ParamSet,
    stockVerified: bool,
    verifiedVk: int,
    proofPathId: int,
    proofSha256: int,
    allChildrenHaveRequiredAirs: bool,
    openedProgD: int,
    openedImageD: int,
    openedPlanD: int,
    openedKeyRoot: int,
    openedMachineVer: int,
    forwardedKeyRoot: int,
    chipSetsMerged: Set[int],
    hStart: int,
    hEnd: int,
    preD: int,
    postD: int,
    intervalsContiguous: bool,
    stateDigestsLinked: bool,
    regRoot: int,
    regRootsEqual: bool,
    isTerminal: bool,
    outcomeD: int,
    claimD: int,
    claimSet: Set[int],
    claimsRoot: int,
    pendingDeps: Set[int],
    depsVerifiedInSet: bool,
    depsAllSealedTerminal: bool,
    treeShape: TreeShape,
    shapeD: int,
    shapeUsesPhysState: bool,
    oneChildMode: bool,
    oneChildLinked: bool,
    whiLimbs16Bit: bool,
    shapeBindsCut: bool,
    nominalTypeBound: bool,
    pubvConstRowChecked: bool,
    acceptedForgery: ForgeryKind,
    fvVkRow: int,
    compactVkId: int,
    adapterClass: AdapterClass,
    adapterBoundInCircuit: bool,
    suspendedAndResumed: bool,
  }

  type LRunProofRec = {
    runId: int,
    n3NodeId: int,
    progId: int,
    family: RuleFamily,
    verifiedLVk: int,
    verifiedLFrVk: int,
    lOutcomeD: int,
    claimPub: int,
    suspendedAndResumed: bool,
    stockVerified: bool,
  }

  type LCollectionProofRec = {
    collId: int,
    setNodeId: int,
    verifiedLVk: int,
    regPub: int,
    exposedClaimsRoot: int,
    memberClaims: Set[int],
    allMembershipsVerified: bool,
    treeDepth: int,
    stockVerified: bool,
  }

  type BundleNodeEntry = {
    nodeId: int,
    tier: NodeTier,
    proofPathId: int,
    proofSha256: int,
    pubOutcomeD: int,
  }

  type ProofBundleRec = {
    runId: int,
    hasLeafEntries: bool,
    n1NodeIds: List[int],
    n2NodeId: int,
    n3NodeId: int,
    hasLAndLFrEntries: bool,
    nodeEntries: List[BundleNodeEntry],
    claimPub: int,
    keyManifestSha256: int,
  }

  type GateStatusRec = {
    verdict: GateVerdict,
    gp2Count: int,
    x7Count: int,
    caseMapMatchesBaseline: bool,
    gpTargetsNarrowed: bool,
  }

  type RefusalEvent = {
    code: RefusalCode,
    targetId: int,
  }

  type StepArgs = {
    primaryId: int,
    secondaryId: int,
    forgery: ForgeryKind,
    refusal: RefusalCode,
  }

  type WorldSnap = {
    universalKeys: UniversalAggKeys,
    mwKeyRegistry: int -> MergeWorkKeyRec,
    nodeProofs: int -> NodeProofRec,
    lRunProofs: int -> LRunProofRec,
    lCollectionProofs: int -> LCollectionProofRec,
    proofBundles: int -> ProofBundleRec,
    gateStatus: GateStatusRec,
    refusalLog: Set[RefusalEvent],
  }

  // =========================================================================
  // 3. Fixed Finite Catalogs and Pure Helpers
  // =========================================================================

  pure val UNIV_INTERNAL_REC_VK: int  = 9101
  pure val UNIV_L_VK: int             = 9102
  pure val UNIV_LFR_VK: int           = 9103
  pure val UNIV_MANIFEST_SHA256: int  = 9200
  pure val REGISTERED_COMPACT_VK: int = 701
  pure val UNREGISTERED_COMPACT_VK: int = 799
  pure val VALID_REG_ROOT: int        = 77

  pure val N1_SPEC_IDS: Set[int]   = 1.to(10)
  pure val RUN_IDS: Set[int]       = 1.to(5)
  pure val L_RUN_IDS: Set[int]     = 1.to(4)
  pure val MW_KEY_IDS: Set[int]    = 1.to(4)
  pure val SEAL_RUN_IDS: Set[int]  = Set(1, 2, 3)

  pure val T10Q_FORGERIES: Set[ForgeryKind] = Set(
    ForgeryMwF01FalseCarry,
    ForgeryAfkF7ShapeCollision,
    ForgeryGp2F42UnboundType,
    ForgeryAfkF9bUnlinkedOneChild
  )

  pure def computeProgDigest(img: int, pln: int, kr: int, mv: int): int =
    img * 100000 + pln * 1000 + kr + mv

  pure def computeClaimDigest(progD: int, inputD: int, outcomeD: int): int =
    progD * 10000 + inputD * 10 + (outcomeD % 100)

  pure def computeClaimPub(progD: int, inputD: int, outcomeD: int): int =
    700000 + progD * 100 + (inputD % 100) + (outcomeD % 100)

  pure def computeSetRoot(claims: Set[int]): int =
    claims.fold(30000, (acc, c) => acc + c)

  pure def programCatalog(pId: int): ProgramMeta =
    if (pId == 1) {
      progId: 1, family: FamInterpreted, imageD: 11, planD: 21, keyRoot: 1001, machineVersion: 2,
      progD: computeProgDigest(11, 21, 1001, 2),
    } else if (pId == 2) {
      progId: 2, family: FamRsl, imageD: 12, planD: 22, keyRoot: 1002, machineVersion: 2,
      progD: computeProgDigest(12, 22, 1002, 2),
    } else if (pId == 3) {
      progId: 3, family: FamInterpreted, imageD: 13, planD: 23, keyRoot: 1003, machineVersion: 2,
      progD: computeProgDigest(13, 23, 1003, 2),
    } else {
      progId: 4, family: FamInterpreted, imageD: 14, planD: 24, keyRoot: 1004, machineVersion: 2,
      progD: computeProgDigest(14, 24, 1004, 2),
    }

  // N1 specs covering:
  // 1,2: Run 1 (Prog 1, Interpreted, 3 leaves split [0,8)+[8,12), nonzero regRoot=77 AFK-F6, box 1 huginn)
  // 3,4: Run 2 (Prog 2, FamRsl, chipSet 3, [0,4)+[4,8), regRoot=77)
  // 5,6: Run 3 (Prog 3, Mixed chipSet 0 [0,4) + chipSet 3 [4,8) under ONE keyRoot=1003, regRoot=77)
  // 7,8: Run 4 (Prog 1, same statement as Run 1, suspended on huginn [0,8) & resumed on odin [8,12), regRoot=0)
  // 9:   Run 5 (Prog 4, consumer run [0,4) depending on Run 1's claim, regRoot=77)
  // 10:  Run 6 (Prog 1, incomplete non-terminal segment [0,4), isTerminal=false)
  pure def n1Catalog(id: int): N1ShardSpec =
    val c1 = computeClaimDigest(programCatalog(1).progD, 201, 501)
    if (id == 1) {
      n1Id: 1, runId: 1, progId: 1, family: FamInterpreted, chipSetId: 0,
      hStart: 0, hEnd: 8, preD: 1000, postD: 1008, regRoot: VALID_REG_ROOT,
      isTerminal: false, outcomeD: 0, deps: Set(), boxHost: 1,
    } else if (id == 2) {
      n1Id: 2, runId: 1, progId: 1, family: FamInterpreted, chipSetId: 0,
      hStart: 8, hEnd: 12, preD: 1008, postD: 1012, regRoot: VALID_REG_ROOT,
      isTerminal: true, outcomeD: 501, deps: Set(), boxHost: 1,
    } else if (id == 3) {
      n1Id: 3, runId: 2, progId: 2, family: FamRsl, chipSetId: 3,
      hStart: 0, hEnd: 4, preD: 2000, postD: 2004, regRoot: VALID_REG_ROOT,
      isTerminal: false, outcomeD: 0, deps: Set(), boxHost: 1,
    } else if (id == 4) {
      n1Id: 4, runId: 2, progId: 2, family: FamRsl, chipSetId: 3,
      hStart: 4, hEnd: 8, preD: 2004, postD: 2008, regRoot: VALID_REG_ROOT,
      isTerminal: true, outcomeD: 502, deps: Set(), boxHost: 1,
    } else if (id == 5) {
      n1Id: 5, runId: 3, progId: 3, family: FamInterpreted, chipSetId: 0,
      hStart: 0, hEnd: 4, preD: 3000, postD: 3004, regRoot: VALID_REG_ROOT,
      isTerminal: false, outcomeD: 0, deps: Set(), boxHost: 1,
    } else if (id == 6) {
      n1Id: 6, runId: 3, progId: 3, family: FamInterpreted, chipSetId: 3,
      hStart: 4, hEnd: 8, preD: 3004, postD: 3008, regRoot: VALID_REG_ROOT,
      isTerminal: true, outcomeD: 503, deps: Set(), boxHost: 1,
    } else if (id == 7) {
      n1Id: 7, runId: 4, progId: 1, family: FamInterpreted, chipSetId: 0,
      hStart: 0, hEnd: 8, preD: 1000, postD: 1008, regRoot: 0,
      isTerminal: false, outcomeD: 0, deps: Set(), boxHost: 1,
    } else if (id == 8) {
      n1Id: 8, runId: 4, progId: 1, family: FamInterpreted, chipSetId: 0,
      hStart: 8, hEnd: 12, preD: 1008, postD: 1012, regRoot: 0,
      isTerminal: true, outcomeD: 501, deps: Set(), boxHost: 2,
    } else if (id == 9) {
      n1Id: 9, runId: 5, progId: 4, family: FamInterpreted, chipSetId: 0,
      hStart: 0, hEnd: 4, preD: 5000, postD: 5004, regRoot: VALID_REG_ROOT,
      isTerminal: true, outcomeD: 505, deps: Set(c1), boxHost: 1,
    } else {
      n1Id: 10, runId: 6, progId: 1, family: FamInterpreted, chipSetId: 0,
      hStart: 0, hEnd: 4, preD: 1000, postD: 1004, regRoot: VALID_REG_ROOT,
      isTerminal: false, outcomeD: 0, deps: Set(), boxHost: 1,
    }

  pure def runCatalog(rId: int): RunCatalogEntry =
    val c1 = computeClaimDigest(programCatalog(1).progD, 201, 501)
    if (rId == 1) {
      runId: 1, progId: 1, family: FamInterpreted, inputD: 201,
      leftN1Id: 1, rightN1Id: 2, expectedOutcomeD: 501, isTerminalRun: true,
      suspendedAndResumed: false, deps: Set(), regRoot: VALID_REG_ROOT,
    } else if (rId == 2) {
      runId: 2, progId: 2, family: FamRsl, inputD: 202,
      leftN1Id: 3, rightN1Id: 4, expectedOutcomeD: 502, isTerminalRun: true,
      suspendedAndResumed: false, deps: Set(), regRoot: VALID_REG_ROOT,
    } else if (rId == 3) {
      runId: 3, progId: 3, family: FamInterpreted, inputD: 203,
      leftN1Id: 5, rightN1Id: 6, expectedOutcomeD: 503, isTerminalRun: true,
      suspendedAndResumed: false, deps: Set(), regRoot: VALID_REG_ROOT,
    } else if (rId == 4) {
      runId: 4, progId: 1, family: FamInterpreted, inputD: 201,
      leftN1Id: 7, rightN1Id: 8, expectedOutcomeD: 501, isTerminalRun: true,
      suspendedAndResumed: true, deps: Set(), regRoot: 0,
    } else {
      runId: 5, progId: 4, family: FamInterpreted, inputD: 205,
      leftN1Id: 9, rightN1Id: 9, expectedOutcomeD: 505, isTerminalRun: true,
      suspendedAndResumed: false, deps: Set(c1), regRoot: VALID_REG_ROOT,
    }

  // MERGE-WORK candidate keys:
  // 1: post-T10q inline kernel with STACKS + VALUE DIGEST (MW_BASE_COLS + 62 + 71 = 5,121 in main)
  // 2: exact bound candidate (MERGE_WORK_MAX_COLS = 5,400 in main)
  // 3: 1 column over bound (MERGE_WORK_MAX_COLS + 1 = 5,401 in main, NAG-005 Scenario)
  // 4: pre-T10q 5,417-column candidate (LPC-002 Scenario)
  def mwKeyCandidate(kId: int): MergeWorkKeyRec =
    if (kId == 1) {
      mwKeyId: 1, hasStacksAir: true, hasValueDigest: true,
      colCount: MW_BASE_COLS + MW_VALUE_DIGEST_DELTA + MW_STACKS_DELTA,
    } else if (kId == 2) {
      mwKeyId: 2, hasStacksAir: true, hasValueDigest: true,
      colCount: MERGE_WORK_MAX_COLS,
    } else if (kId == 3) {
      mwKeyId: 3, hasStacksAir: true, hasValueDigest: true,
      colCount: MERGE_WORK_MAX_COLS + 1,
    } else {
      mwKeyId: 4, hasStacksAir: true, hasValueDigest: true,
      colCount: 5417,
    }

  pure def n1NodeId(n1Id: int): int = 10 + n1Id
  pure def n2NodeId(rId: int): int  = 20 + rId
  pure def n3NodeId(rId: int): int  = 30 + rId
  pure def sealNodeId(rId: int): int = 40 + rId

  def genesisUniversalKeys: UniversalAggKeys = {
    internalRecVk: UNIV_INTERNAL_REC_VK,
    lVk: UNIV_L_VK,
    lFrVk: UNIV_LFR_VK,
    keyManifestSha256: UNIV_MANIFEST_SHA256,
    nodeRowCount: KEYS_D_NODE_ROWS,
    foreignVkRows: 1.to(KEYS_D_FOREIGN_ROWS),
  }

  // Default clean NodeProofRec builder to avoid repetitive boilerplate
  pure def baseNodeRec(
    nId: int,
    tr: NodeTier,
    md: Gp2Mode,
    sk: StmtKind,
    rId: int,
    pId: int,
    fam: RuleFamily,
    vk: int
  ): NodeProofRec = {
    nodeId: nId,
    tier: tr,
    gp2Mode: md,
    stmtKind: sk,
    runId: rId,
    progId: pId,
    family: fam,
    starkField: FieldBabyBear,
    paramSet: ParamInternal,
    stockVerified: true,
    verifiedVk: vk,
    proofPathId: 5000 + nId,
    proofSha256: 8000 + nId,
    allChildrenHaveRequiredAirs: true,
    openedProgD: programCatalog(pId).progD,
    openedImageD: programCatalog(pId).imageD,
    openedPlanD: programCatalog(pId).planD,
    openedKeyRoot: programCatalog(pId).keyRoot,
    openedMachineVer: programCatalog(pId).machineVersion,
    forwardedKeyRoot: programCatalog(pId).keyRoot,
    chipSetsMerged: Set(0),
    hStart: 0,
    hEnd: 4,
    preD: 0,
    postD: 0,
    intervalsContiguous: true,
    stateDigestsLinked: true,
    regRoot: 0,
    regRootsEqual: true,
    isTerminal: true,
    outcomeD: 0,
    claimD: 0,
    claimSet: Set(),
    claimsRoot: 0,
    pendingDeps: Set(),
    depsVerifiedInSet: true,
    depsAllSealedTerminal: true,
    treeShape: ShapeSingle,
    shapeD: 7000 + nId,
    shapeUsesPhysState: false,
    oneChildMode: false,
    oneChildLinked: true,
    whiLimbs16Bit: true,
    shapeBindsCut: true,
    nominalTypeBound: true,
    pubvConstRowChecked: true,
    acceptedForgery: ForgeryNone,
    fvVkRow: 0,
    compactVkId: 0,
    adapterClass: AdapterNone,
    adapterBoundInCircuit: true,
    suspendedAndResumed: false,
  }

  // =========================================================================
  // 4. State Variables (observable state + replay driver metadata)
  // =========================================================================

  var lastOp: str
  var lastArgs: StepArgs
  var universalKeys: UniversalAggKeys
  var mwKeyRegistry: int -> MergeWorkKeyRec
  var nodeProofs: int -> NodeProofRec
  var lRunProofs: int -> LRunProofRec
  var lCollectionProofs: int -> LCollectionProofRec
  var proofBundles: int -> ProofBundleRec
  var gateStatus: GateStatusRec
  var refusalLog: Set[RefusalEvent]

  val currentSnap: WorldSnap = {
    universalKeys: universalKeys,
    mwKeyRegistry: mwKeyRegistry,
    nodeProofs: nodeProofs,
    lRunProofs: lRunProofs,
    lCollectionProofs: lCollectionProofs,
    proofBundles: proofBundles,
    gateStatus: gateStatus,
    refusalLog: refusalLog,
  }

  action commitSnap(ws: WorldSnap, opName: str, args: StepArgs): bool = all {
    lastOp'            = opName,
    lastArgs'          = args,
    universalKeys'     = ws.universalKeys,
    mwKeyRegistry'     = ws.mwKeyRegistry,
    nodeProofs'        = ws.nodeProofs,
    lRunProofs'        = ws.lRunProofs,
    lCollectionProofs' = ws.lCollectionProofs,
    proofBundles'      = ws.proofBundles,
    gateStatus'        = ws.gateStatus,
    refusalLog'        = ws.refusalLog,
  }

  // =========================================================================
  // 5. Pure / Functional Guards and State Updates
  // =========================================================================

  // --- MERGE-WORK keygen (NAG-005, LPC-002) ---
  def canAdmitMergeWorkKey(ws: WorldSnap, kId: int): bool =
    val kc = mwKeyCandidate(kId)
    MW_KEY_IDS.contains(kId)
      and not(ws.mwKeyRegistry.keys().contains(kId))
      and ((MUTANT == "allow_mw_over_5400") or (kc.colCount <= MERGE_WORK_MAX_COLS))

  def applyAdmitMergeWorkKey(ws: WorldSnap, kId: int): WorldSnap =
    val kc = mwKeyCandidate(kId)
    { ...ws, mwKeyRegistry: ws.mwKeyRegistry.put(kId, kc) }

  def canRefuseMergeWorkKeyOverCols(ws: WorldSnap, kId: int): bool =
    val kc = mwKeyCandidate(kId)
    val ev = { code: RefMergeWorkCols, targetId: kId }
    MW_KEY_IDS.contains(kId)
      and kc.colCount > MERGE_WORK_MAX_COLS
      and MUTANT != "allow_mw_over_5400"
      and not(ws.refusalLog.contains(ev))

  // --- N1 proving (NAG-001, NAG-002, NAG-003, NAG-009, NAG-011) ---
  def canProveN1(ws: WorldSnap, id: int): bool =
    N1_SPEC_IDS.contains(id)
      and not(ws.nodeProofs.keys().contains(n1NodeId(id)))

  def applyProveN1(ws: WorldSnap, id: int): WorldSnap =
    val sp = n1Catalog(id)
    val nId = n1NodeId(id)
    val base = baseNodeRec(nId, TierN1, ModeSegMerge, StmtSeg, sp.runId, sp.progId, sp.family, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      chipSetsMerged: Set(sp.chipSetId),
      hStart: sp.hStart,
      hEnd: sp.hEnd,
      preD: sp.preD,
      postD: sp.postD,
      regRoot: sp.regRoot,
      isTerminal: sp.isTerminal,
      outcomeD: sp.outcomeD,
      pendingDeps: sp.deps,
      shapeD: 7100 + id * 13 + sp.hEnd,
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(nId, rec) }

  // --- N2 segment merge (NAG-001, NAG-002, NAG-004, NAG-009, GPH-001, GPH-006) ---
  def canProveN2Merge(ws: WorldSnap, rId: int): bool =
    val rc = runCatalog(rId)
    val lNode = n1NodeId(rc.leftN1Id)
    val rNode = n1NodeId(rc.rightN1Id)
    RUN_IDS.contains(rId)
      and not(ws.nodeProofs.keys().contains(n2NodeId(rId)))
      and ws.nodeProofs.keys().contains(lNode)
      and ws.nodeProofs.keys().contains(rNode)
      and (
        rc.leftN1Id == rc.rightN1Id or (
          val lp = ws.nodeProofs.get(lNode)
          val rp = ws.nodeProofs.get(rNode)
          lp.allChildrenHaveRequiredAirs and rp.allChildrenHaveRequiredAirs
            and lp.forwardedKeyRoot == rp.forwardedKeyRoot
            and lp.hEnd == rp.hStart
            and lp.postD == rp.preD
            and lp.regRoot == rp.regRoot
        )
      )

  def applyProveN2Merge(ws: WorldSnap, rId: int): WorldSnap =
    val rc = runCatalog(rId)
    val lp = ws.nodeProofs.get(n1NodeId(rc.leftN1Id))
    val rp = ws.nodeProofs.get(n1NodeId(rc.rightN1Id))
    val nId = n2NodeId(rId)
    val base = baseNodeRec(nId, TierN2, ModeSegMerge, StmtSeg, rId, rc.progId, rc.family, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      forwardedKeyRoot: lp.forwardedKeyRoot,
      chipSetsMerged: lp.chipSetsMerged.union(rp.chipSetsMerged),
      hStart: lp.hStart,
      hEnd: rp.hEnd,
      preD: lp.preD,
      postD: rp.postD,
      intervalsContiguous: (rc.leftN1Id == rc.rightN1Id) or (lp.hEnd == rp.hStart),
      stateDigestsLinked: (rc.leftN1Id == rc.rightN1Id) or (lp.postD == rp.preD),
      regRoot: lp.regRoot,
      regRootsEqual: (lp.regRoot == rp.regRoot),
      isTerminal: rp.isTerminal,
      outcomeD: rp.outcomeD,
      pendingDeps: lp.pendingDeps.union(rp.pendingDeps),
      treeShape: ShapeBalanced,
      shapeD: 7200 + lp.shapeD + rp.shapeD,
      oneChildMode: (rc.leftN1Id == rc.rightN1Id),
      suspendedAndResumed: rc.suspendedAndResumed,
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(nId, rec) }

  // --- N3 run root (NAG-001, NAG-002, NAG-004, GPH-001, GPH-006) ---
  def canProveN3Root(ws: WorldSnap, rId: int): bool =
    val n2Id = n2NodeId(rId)
    RUN_IDS.contains(rId)
      and ws.nodeProofs.keys().contains(n2Id)
      and not(ws.nodeProofs.keys().contains(n3NodeId(rId)))
      and ws.nodeProofs.get(n2Id).isTerminal

  def applyProveN3Root(ws: WorldSnap, rId: int): WorldSnap =
    val rc = runCatalog(rId)
    val child = ws.nodeProofs.get(n2NodeId(rId))
    val pm = programCatalog(rc.progId)
    val nId = n3NodeId(rId)
    val cDigest = computeClaimDigest(pm.progD, rc.inputD, child.outcomeD)
    val base = baseNodeRec(nId, TierN3, ModeSegMerge, StmtClaim, rId, rc.progId, rc.family, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      forwardedKeyRoot: child.forwardedKeyRoot,
      chipSetsMerged: child.chipSetsMerged,
      hStart: child.hStart,
      hEnd: child.hEnd,
      preD: child.preD,
      postD: child.postD,
      regRoot: child.regRoot,
      isTerminal: child.isTerminal,
      outcomeD: child.outcomeD,
      claimD: cDigest,
      claimSet: if (child.pendingDeps.size() == 0) Set(cDigest) else Set(),
      claimsRoot: if (child.pendingDeps.size() == 0) computeSetRoot(Set(cDigest)) else 0,
      pendingDeps: child.pendingDeps,
      shapeD: 7300 + child.shapeD,
      suspendedAndResumed: rc.suspendedAndResumed,
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(nId, rec) }

  // --- Full-run node prover pipeline N1 -> N2 -> N3 (NAG-001, NAG-003, NAG-004, NAG-009, GPH-001, GPH-006) ---
  def stepRunThroughN2(ws: WorldSnap, rId: int): WorldSnap =
    val rc = runCatalog(rId)
    val ws1 =
      if (ws.nodeProofs.keys().contains(n1NodeId(rc.leftN1Id))) ws
      else applyProveN1(ws, rc.leftN1Id)
    val ws2 =
      if (ws1.nodeProofs.keys().contains(n1NodeId(rc.rightN1Id))) ws1
      else applyProveN1(ws1, rc.rightN1Id)
    if (ws2.nodeProofs.keys().contains(n2NodeId(rId))) ws2
    else applyProveN2Merge(ws2, rId)

  def canProveRunNodeTree(ws: WorldSnap, rId: int): bool =
    val rc = runCatalog(rId)
    RUN_IDS.contains(rId)
      and not(ws.nodeProofs.keys().contains(n3NodeId(rId)))
      and (
        val ws1 =
          if (ws.nodeProofs.keys().contains(n1NodeId(rc.leftN1Id))) ws
          else applyProveN1(ws, rc.leftN1Id)
        val ws2 =
          if (ws1.nodeProofs.keys().contains(n1NodeId(rc.rightN1Id))) ws1
          else applyProveN1(ws1, rc.rightN1Id)
        val n2Ok =
          ws2.nodeProofs.keys().contains(n2NodeId(rId))
            or canProveN2Merge(ws2, rId)
        n2Ok and canProveN3Root(stepRunThroughN2(ws, rId), rId)
      )

  def applyProveRunNodeTree(ws: WorldSnap, rId: int): WorldSnap =
    applyProveN3Root(stepRunThroughN2(ws, rId), rId)

  // --- L single-run check (NAG-008, GPH-001, GPH-006) ---
  def canProveLRun(ws: WorldSnap, rId: int): bool =
    val rootId = n3NodeId(rId)
    L_RUN_IDS.contains(rId)
      and ws.nodeProofs.keys().contains(rootId)
      and not(ws.lRunProofs.keys().contains(rId))
      and ws.nodeProofs.get(rootId).isTerminal
      and ws.nodeProofs.get(rootId).pendingDeps.size() == 0

  def applyProveLRun(ws: WorldSnap, rId: int): WorldSnap =
    val rc = runCatalog(rId)
    val root = ws.nodeProofs.get(n3NodeId(rId))
    val pm = programCatalog(rc.progId)
    val basePub = computeClaimPub(pm.progD, rc.inputD, root.outcomeD)
    val usedPub =
      if (MUTANT == "suspend_alters_claim_pub" and rc.suspendedAndResumed)
        basePub + 999
      else
        basePub
    val lRec: LRunProofRec = {
      runId: rId,
      n3NodeId: root.nodeId,
      progId: rc.progId,
      family: rc.family,
      verifiedLVk: ws.universalKeys.lVk,
      verifiedLFrVk: ws.universalKeys.lFrVk,
      lOutcomeD: root.outcomeD,
      claimPub: usedPub,
      suspendedAndResumed: rc.suspendedAndResumed,
      stockVerified: true,
    }
    { ...ws, lRunProofs: ws.lRunProofs.put(rId, lRec) }

  // --- ProofBundle publication (NAG-010) ---
  def canPublishProofBundle(ws: WorldSnap, rId: int): bool =
    L_RUN_IDS.contains(rId)
      and not(ws.proofBundles.keys().contains(rId))
      and ((MUTANT == "incomplete_proof_bundle") or ws.lRunProofs.keys().contains(rId))

  def applyPublishProofBundle(ws: WorldSnap, rId: int): WorldSnap =
    val rc = runCatalog(rId)
    val l1 = n1NodeId(rc.leftN1Id)
    val r1 = n1NodeId(rc.rightN1Id)
    val n2 = n2NodeId(rId)
    val n3 = n3NodeId(rId)
    if (MUTANT == "incomplete_proof_bundle" and not(ws.lRunProofs.keys().contains(rId))) {
      val badBundle: ProofBundleRec = {
        runId: rId,
        hasLeafEntries: true,
        n1NodeIds: [],
        n2NodeId: -1,
        n3NodeId: -1,
        hasLAndLFrEntries: false,
        nodeEntries: [],
        claimPub: 0,
        keyManifestSha256: ws.universalKeys.keyManifestSha256,
      }
      { ...ws, proofBundles: ws.proofBundles.put(rId, badBundle) }
    } else {
      val lp1 = ws.nodeProofs.get(l1)
      val rp1 = ws.nodeProofs.get(r1)
      val p2  = ws.nodeProofs.get(n2)
      val p3  = ws.nodeProofs.get(n3)
      val lr  = ws.lRunProofs.get(rId)
      val entries: List[BundleNodeEntry] = [
        { nodeId: l1, tier: TierN1, proofPathId: lp1.proofPathId, proofSha256: lp1.proofSha256, pubOutcomeD: lp1.outcomeD },
        { nodeId: r1, tier: TierN1, proofPathId: rp1.proofPathId, proofSha256: rp1.proofSha256, pubOutcomeD: rp1.outcomeD },
        { nodeId: n2, tier: TierN2, proofPathId: p2.proofPathId, proofSha256: p2.proofSha256, pubOutcomeD: p2.outcomeD },
        { nodeId: n3, tier: TierN3, proofPathId: p3.proofPathId, proofSha256: p3.proofSha256, pubOutcomeD: p3.outcomeD }
      ]
      val bundle: ProofBundleRec = {
        runId: rId,
        hasLeafEntries: true,
        n1NodeIds: [l1, r1],
        n2NodeId: n2,
        n3NodeId: n3,
        hasLAndLFrEntries: true,
        nodeEntries: entries,
        claimPub: lr.claimPub,
        keyManifestSha256: ws.universalKeys.keyManifestSha256,
      }
      { ...ws, proofBundles: ws.proofBundles.put(rId, bundle) }
    }

  // --- GP-2 modes: SEAL, SEAL-WITH-DEPS, JOIN, DL-STEP/DESCEND (GPH-002, GP-002) ---
  def canProveGp2Seal(ws: WorldSnap, rId: int): bool =
    val n2 = n2NodeId(rId)
    SEAL_RUN_IDS.contains(rId)
      and ws.nodeProofs.keys().contains(n2)
      and not(ws.nodeProofs.keys().contains(sealNodeId(rId)))
      and ws.nodeProofs.get(n2).hStart == 0
      and ws.nodeProofs.get(n2).isTerminal
      and ws.nodeProofs.get(n2).pendingDeps.size() == 0

  def applyProveGp2Seal(ws: WorldSnap, rId: int): WorldSnap =
    val rc = runCatalog(rId)
    val pm = programCatalog(rc.progId)
    val seg = ws.nodeProofs.get(n2NodeId(rId))
    val sId = sealNodeId(rId)
    val cDigest = computeClaimDigest(pm.progD, rc.inputD, seg.outcomeD)
    val cSet = Set(cDigest)
    val base = baseNodeRec(sId, TierN2, ModeSeal, StmtSet, rId, rc.progId, rc.family, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      hStart: seg.hStart,
      hEnd: seg.hEnd,
      preD: seg.preD,
      postD: seg.postD,
      regRoot: seg.regRoot,
      isTerminal: true,
      outcomeD: seg.outcomeD,
      claimD: cDigest,
      claimSet: cSet,
      claimsRoot: computeSetRoot(cSet),
      pendingDeps: Set(),
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(sId, rec) }

  def canProveGp2SealWithDeps(ws: WorldSnap): bool =
    val n2 = n2NodeId(5)
    ws.nodeProofs.keys().contains(n2)
      and not(ws.nodeProofs.keys().contains(45))
      and ws.nodeProofs.get(n2).hStart == 0
      and ws.nodeProofs.get(n2).isTerminal
      and ws.nodeProofs.get(n2).pendingDeps.size() > 0

  def applyProveGp2SealWithDeps(ws: WorldSnap): WorldSnap =
    val rc = runCatalog(5)
    val pm = programCatalog(rc.progId)
    val seg = ws.nodeProofs.get(n2NodeId(5))
    val cDigest = computeClaimDigest(pm.progD, rc.inputD, seg.outcomeD)
    val base = baseNodeRec(45, TierN2, ModeSealWithDeps, StmtPendingDeps, 5, rc.progId, rc.family, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      hStart: seg.hStart,
      hEnd: seg.hEnd,
      preD: seg.preD,
      postD: seg.postD,
      regRoot: seg.regRoot,
      isTerminal: true,
      outcomeD: seg.outcomeD,
      claimD: cDigest,
      claimSet: Set(),
      claimsRoot: 0,
      pendingDeps: seg.pendingDeps,
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(45, rec) }

  def canSealRunNodeTree(ws: WorldSnap, rId: int): bool =
    val rc = runCatalog(rId)
    val targetId = if (rId == 5) 45 else sealNodeId(rId)
    Set(1, 2, 3, 5).contains(rId)
      and not(ws.nodeProofs.keys().contains(targetId))
      and (
        val ws1 =
          if (ws.nodeProofs.keys().contains(n1NodeId(rc.leftN1Id))) ws
          else applyProveN1(ws, rc.leftN1Id)
        val ws2 =
          if (ws1.nodeProofs.keys().contains(n1NodeId(rc.rightN1Id))) ws1
          else applyProveN1(ws1, rc.rightN1Id)
        val n2Ok =
          ws2.nodeProofs.keys().contains(n2NodeId(rId))
            or canProveN2Merge(ws2, rId)
        val ws3 = stepRunThroughN2(ws, rId)
        n2Ok and (
          if (rId == 5) canProveGp2SealWithDeps(ws3)
          else canProveGp2Seal(ws3, rId)
        )
      )

  def applySealRunNodeTree(ws: WorldSnap, rId: int): WorldSnap =
    val ws3 = stepRunThroughN2(ws, rId)
    if (rId == 5) applyProveGp2SealWithDeps(ws3)
    else applyProveGp2Seal(ws3, rId)

  // JOIN: joinKind == 1 joins (41, 42) -> 51; joinKind == 2 joins (51, 43) -> 52 (3 claims of 3 programs!)
  def canProveGp2JoinSet(ws: WorldSnap, joinKind: int): bool =
    if (joinKind == 1)
      ws.nodeProofs.keys().contains(41)
        and ws.nodeProofs.keys().contains(42)
        and not(ws.nodeProofs.keys().contains(51))
        and ws.nodeProofs.get(41).stmtKind == StmtSet
        and ws.nodeProofs.get(42).stmtKind == StmtSet
        and ws.nodeProofs.get(41).regRoot == ws.nodeProofs.get(42).regRoot
    else if (joinKind == 2)
      ws.nodeProofs.keys().contains(51)
        and ws.nodeProofs.keys().contains(43)
        and not(ws.nodeProofs.keys().contains(52))
        and ws.nodeProofs.get(51).stmtKind == StmtSet
        and ws.nodeProofs.get(43).stmtKind == StmtSet
        and ws.nodeProofs.get(51).regRoot == ws.nodeProofs.get(43).regRoot
    else
      false

  def applyProveGp2JoinSet(ws: WorldSnap, joinKind: int): WorldSnap =
    val leftId  = if (joinKind == 1) 41 else 51
    val rightId = if (joinKind == 1) 42 else 43
    val outId   = if (joinKind == 1) 51 else 52
    val lp = ws.nodeProofs.get(leftId)
    val rp = ws.nodeProofs.get(rightId)
    val mergedSet = lp.claimSet.union(rp.claimSet)
    val base = baseNodeRec(outId, TierN3, ModeJoin, StmtSet, 0, 1, FamInterpreted, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      regRoot: lp.regRoot,
      regRootsEqual: (lp.regRoot == rp.regRoot),
      claimSet: mergedSet,
      claimsRoot: computeSetRoot(mergedSet),
      treeShape: ShapeBalanced,
      shapeD: 7500 + joinKind,
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(outId, rec) }

  def canBuildThreeClaimSetRoot(ws: WorldSnap): bool =
    not(ws.nodeProofs.keys().contains(52))
      and (ws.nodeProofs.keys().contains(41) or canSealRunNodeTree(ws, 1))
      and (
        val ws1 = if (ws.nodeProofs.keys().contains(41)) ws else applySealRunNodeTree(ws, 1)
        (ws1.nodeProofs.keys().contains(42) or canSealRunNodeTree(ws1, 2)) and (
          val ws2 = if (ws1.nodeProofs.keys().contains(42)) ws1 else applySealRunNodeTree(ws1, 2)
          (ws2.nodeProofs.keys().contains(43) or canSealRunNodeTree(ws2, 3)) and (
            val ws3 = if (ws2.nodeProofs.keys().contains(43)) ws2 else applySealRunNodeTree(ws2, 3)
            (ws3.nodeProofs.keys().contains(51) or canProveGp2JoinSet(ws3, 1)) and (
              val ws4 = if (ws3.nodeProofs.keys().contains(51)) ws3 else applyProveGp2JoinSet(ws3, 1)
              canProveGp2JoinSet(ws4, 2)
            )
          )
        )
      )

  def applyBuildThreeClaimSetRoot(ws: WorldSnap): WorldSnap =
    val ws1 = if (ws.nodeProofs.keys().contains(41)) ws else applySealRunNodeTree(ws, 1)
    val ws2 = if (ws1.nodeProofs.keys().contains(42)) ws1 else applySealRunNodeTree(ws1, 2)
    val ws3 = if (ws2.nodeProofs.keys().contains(43)) ws2 else applySealRunNodeTree(ws2, 3)
    val ws4 = if (ws3.nodeProofs.keys().contains(51)) ws3 else applyProveGp2JoinSet(ws3, 1)
    applyProveGp2JoinSet(ws4, 2)

  // DL-STEP + DESCEND: discharges pendingDeps of node 45 against proven StmtSet setNodeId (41, 51, or 52)
  def canProveGp2DischargeDeps(ws: WorldSnap, setNodeId: int): bool =
    Set(41, 51, 52).contains(setNodeId)
      and ws.nodeProofs.keys().contains(45)
      and ws.nodeProofs.keys().contains(setNodeId)
      and not(ws.nodeProofs.keys().contains(55))
      and ws.nodeProofs.get(setNodeId).stmtKind == StmtSet
      and ws.nodeProofs.get(45).regRoot == ws.nodeProofs.get(setNodeId).regRoot
      and ws.nodeProofs.get(45).pendingDeps.forall(d => ws.nodeProofs.get(setNodeId).claimSet.contains(d))

  def applyProveGp2DischargeDeps(ws: WorldSnap, setNodeId: int): WorldSnap =
    val pdNode = ws.nodeProofs.get(45)
    val sNode  = ws.nodeProofs.get(setNodeId)
    val outSet = sNode.claimSet.union(Set(pdNode.claimD))
    val base = baseNodeRec(55, TierN3, ModeDlStepDescend, StmtSet, 5, 4, FamInterpreted, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      regRoot: sNode.regRoot,
      regRootsEqual: (pdNode.regRoot == sNode.regRoot),
      claimD: pdNode.claimD,
      claimSet: outSet,
      claimsRoot: computeSetRoot(outSet),
      pendingDeps: Set(),
      depsVerifiedInSet: true,
      depsAllSealedTerminal: true,
      shapeD: 7555,
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(55, rec) }

  // --- L collection form over SET root (GPH-008, LPC-003) ---
  def canProveLCollectionOverSet(ws: WorldSnap, setNodeId: int): bool =
    HAS_L_COLLECTION_KEY
      and Set(41, 51, 52, 55, 65).contains(setNodeId)
      and ws.nodeProofs.keys().contains(setNodeId)
      and not(ws.lCollectionProofs.keys().contains(setNodeId))
      and ws.nodeProofs.get(setNodeId).stmtKind == StmtSet
      and ws.nodeProofs.get(setNodeId).pendingDeps.size() == 0

  def applyProveLCollectionOverSet(ws: WorldSnap, setNodeId: int): WorldSnap =
    val sNode = ws.nodeProofs.get(setNodeId)
    val lColl: LCollectionProofRec = {
      collId: setNodeId,
      setNodeId: setNodeId,
      verifiedLVk: ws.universalKeys.lVk,
      regPub: sNode.regRoot,
      exposedClaimsRoot: sNode.claimsRoot,
      memberClaims: sNode.claimSet,
      allMembershipsVerified: true,
      treeDepth: CLAIMS_TREE_DEPTH,
      stockVerified: true,
    }
    { ...ws, lCollectionProofs: ws.lCollectionProofs.put(setNodeId, lColl) }

  // --- GP-3 Foreign-verifier unit at N3 & X8 Compact adapter (GPH-003, GPH-004, GP-003) ---
  def canProveN3ForeignUnit(ws: WorldSnap, fvRow: int): bool =
    ws.universalKeys.foreignVkRows.contains(fvRow)
      and not(ws.nodeProofs.keys().contains(60 + fvRow))

  def applyProveN3ForeignUnit(ws: WorldSnap, fvRow: int): WorldSnap =
    val nId = 60 + fvRow
    val base = baseNodeRec(nId, TierN3, ModeFvAccept, StmtFv, 0, 1, FamInterpreted, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      fvVkRow: fvRow,
      shapeD: 7600 + fvRow,
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(nId, rec) }

  def canAdaptCompactForeignProof(ws: WorldSnap): bool =
    HAS_X8_COMPACT_VERIFIER
      and ws.universalKeys.foreignVkRows.contains(1)
      and not(ws.nodeProofs.keys().contains(65))

  def applyAdaptCompactForeignProof(ws: WorldSnap): WorldSnap =
    val cSet = Set(9601)
    val base = baseNodeRec(65, TierN2, ModeFvAccept, StmtSet, 0, 2, FamRsl, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      regRoot: VALID_REG_ROOT,
      claimD: 9601,
      claimSet: cSet,
      claimsRoot: computeSetRoot(cSet),
      fvVkRow: 1,
      compactVkId: REGISTERED_COMPACT_VK,
      adapterClass: AdapterCompactZkir3,
      adapterBoundInCircuit: true,
      shapeD: 7665,
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(65, rec) }

  // --- GP-4 MERGE-WORK 1-child (NAG-011 AFK-F9(b)) and multi-shape PCD trees (GPH-005, GP-004) ---
  def canProveOneChildMergeWork(ws: WorldSnap): bool =
    ws.mwKeyRegistry.keys().size() > 0
      and ws.nodeProofs.keys().contains(11)
      and not(ws.nodeProofs.keys().contains(70))

  def applyProveOneChildMergeWork(ws: WorldSnap): WorldSnap =
    val child = ws.nodeProofs.get(11)
    val base = baseNodeRec(70, TierMergeWork, ModePcdRoot, StmtSeg, 1, 1, FamInterpreted, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      hStart: child.hStart,
      hEnd: child.hEnd + 1,
      preD: child.preD,
      postD: child.postD + 1,
      intervalsContiguous: true,
      stateDigestsLinked: true,
      regRoot: child.regRoot,
      isTerminal: false,
      treeShape: ShapeSingle,
      shapeD: 8100,
      shapeUsesPhysState: false,
      oneChildMode: true,
      oneChildLinked: true,
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(70, rec) }

  // shapeChoice == 1 -> ShapeBalanced (nodeId 71); shapeChoice == 2 -> ShapeChain (nodeId 72)
  // Both prove the same 4 work units under universal keys and give equal claimD = 9501 (GPH-005, GP-004)
  def canProvePcdWorkTree(ws: WorldSnap, shapeChoice: int): bool =
    val nId = 70 + shapeChoice
    Set(1, 2).contains(shapeChoice)
      and ws.mwKeyRegistry.keys().size() > 0
      and not(ws.nodeProofs.keys().contains(nId))

  def applyProvePcdWorkTree(ws: WorldSnap, shapeChoice: int): WorldSnap =
    val nId = 70 + shapeChoice
    val sh = if (shapeChoice == 1) ShapeBalanced else ShapeChain
    val computedClaim =
      if (MUTANT == "pcd_shape_changes_claim" and shapeChoice == 2) 9599 else 9501
    val cSet = Set(computedClaim)
    val base = baseNodeRec(nId, TierN3, ModePcdRoot, StmtSet, 0, 1, FamInterpreted, ws.universalKeys.internalRecVk)
    val rec: NodeProofRec = {
      ...base,
      hStart: 0,
      hEnd: 4,
      claimD: computedClaim,
      claimSet: cSet,
      claimsRoot: computeSetRoot(cSet),
      treeShape: sh,
      // Distinct address-free shape commitments (8101 vs 8102), zero physical state input (GP-004)
      shapeD: 8100 + shapeChoice,
      shapeUsesPhysState: false,
    }
    { ...ws, nodeProofs: ws.nodeProofs.put(nId, rec) }

  // --- GP Gate Runner (GPH-007, DG-010, CPR-005) ---
  def canRunGpGateHonest(ws: WorldSnap): bool =
    ws.gateStatus.verdict == GateNotRun

  def applyRunGpGateHonest(ws: WorldSnap): WorldSnap =
    val gs: GateStatusRec = {
      verdict: GatePassed,
      gp2Count: GP2_CASE_FLOOR,
      x7Count: X7_CASE_FLOOR,
      caseMapMatchesBaseline: true,
      gpTargetsNarrowed: false,
    }
    { ...ws, gateStatus: gs }

  // GPH-007 Scenario: forged-dependency refusal case replaced by copy of accept case (count stays 48)
  def canRunGpGateTamperedCaseMap(ws: WorldSnap): bool =
    ws.gateStatus.verdict == GateNotRun

  def applyRunGpGateTamperedCaseMap(ws: WorldSnap): WorldSnap =
    val v = if (MUTANT == "count_only_gp_gate") GatePassed else GateFailedCaseMap
    val gs: GateStatusRec = {
      verdict: v,
      gp2Count: GP2_CASE_FLOOR,
      x7Count: X7_CASE_FLOOR,
      caseMapMatchesBaseline: false,
      gpTargetsNarrowed: false,
    }
    { ...ws, gateStatus: gs }

  // DG-010 & CPR-005 Scenario: derived gate fails -> GP target stays open, never narrowed
  def canRunGpGateFailedDerived(ws: WorldSnap): bool =
    ws.gateStatus.verdict == GateNotRun

  def applyRunGpGateFailedDerived(ws: WorldSnap): WorldSnap =
    val narrowed = (MUTANT == "narrow_gp_on_gate_fail")
    val gs: GateStatusRec = {
      verdict: GateFailedDerived,
      gp2Count: GP2_CASE_FLOOR,
      x7Count: X7_CASE_FLOOR,
      caseMapMatchesBaseline: true,
      gpTargetsNarrowed: narrowed,
    }
    { ...ws, gateStatus: gs }

  // --- Honest Refusal Guards (disabled when the corresponding mutant accepts the forgery) ---
  def canRefuseMissingChildAir(ws: WorldSnap): bool =
    MUTANT != "accept_omitted_air"
      and not(ws.refusalLog.contains({ code: RefMissingRequiredAir, targetId: 91 }))

  def canRefuseN1WrongKeyRoot(ws: WorldSnap): bool =
    MUTANT != "unopened_key_root"
      and not(ws.refusalLog.contains({ code: RefWrongProgDOpening, targetId: 92 }))

  pure def refusalOfT10q(fg: ForgeryKind): RefusalCode =
    match fg {
      | ForgeryMwF01FalseCarry       => RefT10qMwF01
      | ForgeryAfkF7ShapeCollision   => RefT10qAfkF7
      | ForgeryGp2F42UnboundType     => RefT10qGp2F42
      | ForgeryAfkF9bUnlinkedOneChild => RefT10qAfkF9b
      | _                            => RefNone
    }

  def canRefuseT10qForgery(ws: WorldSnap, fg: ForgeryKind): bool =
    val code = refusalOfT10q(fg)
    T10Q_FORGERIES.contains(fg)
      and code != RefNone
      and MUTANT != "accept_t10q_forgery"
      and not(ws.refusalLog.contains({ code: code, targetId: 93 }))

  def canRefuseGp2C21Forgery(ws: WorldSnap): bool =
    MUTANT != "gp2_c21_unfixed"
      and not(ws.refusalLog.contains({ code: RefGp2C21ConstRow, targetId: 94 }))

  def canRefuseLWrongOutcome(ws: WorldSnap): bool =
    ws.nodeProofs.keys().contains(n3NodeId(1))
      and MUTANT != "l_unbound_outcome"
      and not(ws.refusalLog.contains({ code: RefWrongOutcomeAtL, targetId: 1 }))

  def canRefuseDisjointOrSwappedChildren(ws: WorldSnap): bool =
    ws.nodeProofs.keys().contains(11)
      and ws.nodeProofs.keys().contains(12)
      and MUTANT != "skip_child_link_check"
      and not(ws.refusalLog.contains({ code: RefDisjointOrSwappedChildren, targetId: 21 }))

  def canRefuseGp2ForgedDep(ws: WorldSnap): bool =
    MUTANT != "accept_forged_dep"
      and not(ws.refusalLog.contains({ code: RefUnprovedOrUnsealedDep, targetId: 55 }))

  def canRefuseMixedRegRootJoin(ws: WorldSnap): bool =
    not(ws.refusalLog.contains({ code: RefMixedRegRootJoin, targetId: 59 }))

  def canRefuseUnlistedFvUnit(ws: WorldSnap): bool =
    MUTANT != "accept_unlisted_fv_key"
      and not(ws.refusalLog.contains({ code: RefUnlistedForeignKey, targetId: 99 }))

  def canRefuseUnboundCompactAdapter(ws: WorldSnap): bool =
    MUTANT != "unbound_foreign_adapter"
      and not(ws.refusalLog.contains({ code: RefUnboundCompactAdapter, targetId: 799 }))

  def canRefuseLCollectionForgedSetRoot(ws: WorldSnap): bool =
    MUTANT != "l_accepts_forged_set_root"
      and not(ws.refusalLog.contains({ code: RefForgedSetRootAtL, targetId: 88 }))

  // --- Mutant-Only Unguarded Transitions (disabled in all sound instances where MUTANT == "none") ---
  def canAcceptMutantForgery(ws: WorldSnap): bool =
    not(ws.nodeProofs.keys().contains(99))
      and not(ws.lRunProofs.keys().contains(99))
      and not(ws.lCollectionProofs.keys().contains(99))
      and Set(
        "wrong_node_params",
        "accept_omitted_air",
        "unopened_key_root",
        "mixed_chipset_wrong_keyroot",
        "accept_t10q_forgery",
        "gp2_c21_unfixed",
        "l_unbound_outcome",
        "skip_child_link_check",
        "accept_forged_dep",
        "accept_unlisted_fv_key",
        "unbound_foreign_adapter",
        "l_accepts_forged_set_root"
      ).contains(MUTANT)

  def applyAcceptMutantForgery(ws: WorldSnap): WorldSnap =
    val base = baseNodeRec(99, TierN2, ModeSegMerge, StmtSeg, 1, 1, FamInterpreted, ws.universalKeys.internalRecVk)
    if (MUTANT == "wrong_node_params") {
      val bad = { ...base, tier: TierN1, starkField: FieldWrong, paramSet: ParamWrong, acceptedForgery: ForgeryWrongNodeParams }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, bad) }
    } else if (MUTANT == "accept_omitted_air") {
      val bad = { ...base, tier: TierN3, allChildrenHaveRequiredAirs: false, acceptedForgery: ForgeryOmitRequiredAir }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, bad) }
    } else if (MUTANT == "unopened_key_root") {
      val bad = { ...base, tier: TierN1, openedKeyRoot: 9999, forwardedKeyRoot: 9999, acceptedForgery: ForgeryWrongKeyRoot }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, bad) }
    } else if (MUTANT == "mixed_chipset_wrong_keyroot") {
      val bad = { ...base, tier: TierN2, progId: 3, chipSetsMerged: Set(0, 3), forwardedKeyRoot: 9999, acceptedForgery: ForgeryMixedChipWrongKeyRoot }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, bad) }
    } else if (MUTANT == "accept_t10q_forgery") {
      val bad = { ...base, tier: TierMergeWork, whiLimbs16Bit: false, acceptedForgery: ForgeryMwF01FalseCarry }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, bad) }
    } else if (MUTANT == "gp2_c21_unfixed") {
      val bad = { ...base, tier: TierN1, pubvConstRowChecked: false, acceptedForgery: ForgeryGp2C21PubvConstRow }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, bad) }
    } else if (MUTANT == "skip_child_link_check") {
      val bad = { ...base, tier: TierN1, intervalsContiguous: false, stateDigestsLinked: false, acceptedForgery: ForgeryDisjointOrSwappedLink }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, bad) }
    } else if (MUTANT == "accept_forged_dep") {
      val bad = {
        ...base,
        tier: TierN3,
        gp2Mode: ModeDlStepDescend,
        stmtKind: StmtSet,
        claimSet: Set(9005),
        claimsRoot: computeSetRoot(Set(9005)),
        depsVerifiedInSet: false,
        depsAllSealedTerminal: false,
        acceptedForgery: ForgeryUnprovedDep,
      }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, bad) }
    } else if (MUTANT == "accept_unlisted_fv_key") {
      val bad = { ...base, tier: TierN3, gp2Mode: ModeFvAccept, stmtKind: StmtFv, fvVkRow: 99, acceptedForgery: ForgeryUnlistedFvKey }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, bad) }
    } else if (MUTANT == "unbound_foreign_adapter") {
      val bad = {
        ...base,
        tier: TierN2,
        gp2Mode: ModeFvAccept,
        stmtKind: StmtSet,
        fvVkRow: 1,
        compactVkId: UNREGISTERED_COMPACT_VK,
        adapterClass: AdapterUnboundRelabel,
        adapterBoundInCircuit: false,
        acceptedForgery: ForgeryUnboundForeignAdapter,
      }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, bad) }
    } else if (MUTANT == "l_unbound_outcome") {
      val rootNode = { ...base, nodeId: 99, tier: TierN3, stmtKind: StmtClaim, isTerminal: true, outcomeD: 501, claimD: 9001 }
      val badL: LRunProofRec = {
        runId: 99,
        n3NodeId: 99,
        progId: 1,
        family: FamInterpreted,
        verifiedLVk: ws.universalKeys.lVk,
        verifiedLFrVk: ws.universalKeys.lFrVk,
        lOutcomeD: 502,
        claimPub: 700502,
        suspendedAndResumed: false,
        stockVerified: true,
      }
      { ...ws, nodeProofs: ws.nodeProofs.put(99, rootNode), lRunProofs: ws.lRunProofs.put(99, badL) }
    } else {
      // MUTANT == "l_accepts_forged_set_root"
      val badColl: LCollectionProofRec = {
        collId: 99,
        setNodeId: 999,
        verifiedLVk: ws.universalKeys.lVk,
        regPub: VALID_REG_ROOT,
        exposedClaimsRoot: 88888,
        memberClaims: Set(7777),
        allMembershipsVerified: false,
        treeDepth: CLAIMS_TREE_DEPTH,
        stockVerified: true,
      }
      { ...ws, lCollectionProofs: ws.lCollectionProofs.put(99, badColl) }
    }

  // =========================================================================
  // 6. Guarded Actions (every action sets lastOp and cites its EARS IDs)
  // =========================================================================

  // Initializes observable state of Aggregation at genesis
  action init: bool =
    val emptyMw: int -> MergeWorkKeyRec = Map()
    val emptyNodes: int -> NodeProofRec = Map()
    val emptyLRun: int -> LRunProofRec = Map()
    val emptyLColl: int -> LCollectionProofRec = Map()
    val emptyBundles: int -> ProofBundleRec = Map()
    val initGate: GateStatusRec = {
      verdict: GateNotRun,
      gp2Count: 0,
      x7Count: 0,
      caseMapMatchesBaseline: true,
      gpTargetsNarrowed: false,
    }
    val initSnap: WorldSnap = {
      universalKeys: genesisUniversalKeys,
      mwKeyRegistry: emptyMw,
      nodeProofs: emptyNodes,
      lRunProofs: emptyLRun,
      lCollectionProofs: emptyLColl,
      proofBundles: emptyBundles,
      gateStatus: initGate,
      refusalLog: Set(),
    }
    commitSnap(initSnap, "init", { primaryId: 0, secondaryId: 0, forgery: ForgeryNone, refusal: RefNone })

  // Implements NAG-005, LPC-002
  action admitMergeWorkKey(kId: int): bool = all {
    canAdmitMergeWorkKey(currentSnap, kId),
    commitSnap(
      applyAdmitMergeWorkKey(currentSnap, kId),
      "admitMergeWorkKey",
      { primaryId: kId, secondaryId: 0, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements NAG-005, LPC-002 (refusal of MERGE-WORK key > 5,400 columns)
  action refuseMergeWorkKeyOverCols(kId: int): bool = all {
    canRefuseMergeWorkKeyOverCols(currentSnap, kId),
    val ev = { code: RefMergeWorkCols, targetId: kId }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseMergeWorkKeyOverCols",
      { primaryId: kId, secondaryId: 0, forgery: ForgeryNone, refusal: RefMergeWorkCols }
    ),
  }

  // Implements NAG-001, NAG-002, NAG-003, NAG-009, NAG-011, GPH-001
  action proveN1(n1Id: int): bool = all {
    canProveN1(currentSnap, n1Id),
    commitSnap(
      applyProveN1(currentSnap, n1Id),
      "proveN1",
      { primaryId: n1Id, secondaryId: n1NodeId(n1Id), forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements NAG-001, NAG-002, NAG-004, NAG-009, GPH-001, GPH-006
  action proveN2Merge(rId: int): bool = all {
    canProveN2Merge(currentSnap, rId),
    commitSnap(
      applyProveN2Merge(currentSnap, rId),
      "proveN2Merge",
      { primaryId: rId, secondaryId: n2NodeId(rId), forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements NAG-001, NAG-002, NAG-004, GPH-001, GPH-006
  action proveN3Root(rId: int): bool = all {
    canProveN3Root(currentSnap, rId),
    commitSnap(
      applyProveN3Root(currentSnap, rId),
      "proveN3Root",
      { primaryId: rId, secondaryId: n3NodeId(rId), forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements NAG-001, NAG-002, NAG-003, NAG-004, NAG-009, NAG-011, GPH-001, GPH-006 (run N1->N2->N3 pipeline)
  action proveRunNodeTree(rId: int): bool = all {
    canProveRunNodeTree(currentSnap, rId),
    commitSnap(
      applyProveRunNodeTree(currentSnap, rId),
      "proveRunNodeTree",
      { primaryId: rId, secondaryId: n3NodeId(rId), forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements NAG-008, GPH-001, GPH-006
  action proveLRun(rId: int): bool = all {
    canProveLRun(currentSnap, rId),
    commitSnap(
      applyProveLRun(currentSnap, rId),
      "proveLRun",
      { primaryId: rId, secondaryId: n3NodeId(rId), forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements NAG-010
  action publishProofBundle(rId: int): bool = all {
    canPublishProofBundle(currentSnap, rId),
    commitSnap(
      applyPublishProofBundle(currentSnap, rId),
      "publishProofBundle",
      { primaryId: rId, secondaryId: n3NodeId(rId), forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-002, GP-002 (SEAL mode)
  action proveGp2Seal(rId: int): bool = all {
    canProveGp2Seal(currentSnap, rId),
    commitSnap(
      applyProveGp2Seal(currentSnap, rId),
      "proveGp2Seal",
      { primaryId: rId, secondaryId: sealNodeId(rId), forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-002, GP-002 (SEAL-WITH-DEPS mode)
  action proveGp2SealWithDeps: bool = all {
    canProveGp2SealWithDeps(currentSnap),
    commitSnap(
      applyProveGp2SealWithDeps(currentSnap),
      "proveGp2SealWithDeps",
      { primaryId: 5, secondaryId: 45, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements NAG-001, NAG-003, NAG-009, GPH-002, GP-002 (run N1->N2->SEAL/SEAL-WITH-DEPS pipeline)
  action sealRunNodeTree(rId: int): bool = all {
    canSealRunNodeTree(currentSnap, rId),
    val outId = if (rId == 5) 45 else sealNodeId(rId)
    commitSnap(
      applySealRunNodeTree(currentSnap, rId),
      "sealRunNodeTree",
      { primaryId: rId, secondaryId: outId, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-002, GP-002, GPH-008 (JOIN mode over SET claims)
  action proveGp2JoinSet(joinKind: int): bool = all {
    canProveGp2JoinSet(currentSnap, joinKind),
    val outId = if (joinKind == 1) 51 else 52
    commitSnap(
      applyProveGp2JoinSet(currentSnap, joinKind),
      "proveGp2JoinSet",
      { primaryId: joinKind, secondaryId: outId, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-002, GP-002, GPH-008 (GP-2 SEAL + JOIN pipeline producing 3-claim SET root of 3 programs)
  action buildThreeClaimSetRoot: bool = all {
    canBuildThreeClaimSetRoot(currentSnap),
    commitSnap(
      applyBuildThreeClaimSetRoot(currentSnap),
      "buildThreeClaimSetRoot",
      { primaryId: 3, secondaryId: 52, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-002, GP-002 (DL-STEP and DESCEND dependency verification)
  action proveGp2DischargeDeps(setNodeId: int): bool = all {
    canProveGp2DischargeDeps(currentSnap, setNodeId),
    commitSnap(
      applyProveGp2DischargeDeps(currentSnap, setNodeId),
      "proveGp2DischargeDeps",
      { primaryId: setNodeId, secondaryId: 55, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-008, LPC-003 (L collection form over SET root)
  action proveLCollectionOverSet(setNodeId: int): bool = all {
    canProveLCollectionOverSet(currentSnap, setNodeId),
    commitSnap(
      applyProveLCollectionOverSet(currentSnap, setNodeId),
      "proveLCollectionOverSet",
      { primaryId: setNodeId, secondaryId: setNodeId, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-003 (N3 accepts foreign-verifier unit under foreign rows of keys_d)
  action proveN3ForeignUnit(fvRow: int): bool = all {
    canProveN3ForeignUnit(currentSnap, fvRow),
    commitSnap(
      applyProveN3ForeignUnit(currentSnap, fvRow),
      "proveN3ForeignUnit",
      { primaryId: fvRow, secondaryId: 60 + fvRow, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-004, GP-003, LPC-003 (X8 in-circuit Compact verifier + adapter class)
  action adaptCompactForeignProof: bool = all {
    canAdaptCompactForeignProof(currentSnap),
    commitSnap(
      applyAdaptCompactForeignProof(currentSnap),
      "adaptCompactForeignProof",
      { primaryId: REGISTERED_COMPACT_VK, secondaryId: 65, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements NAG-011 (honest one-child linked MERGE-WORK node AFK-F9(b))
  action proveOneChildMergeWork: bool = all {
    canProveOneChildMergeWork(currentSnap),
    commitSnap(
      applyProveOneChildMergeWork(currentSnap),
      "proveOneChildMergeWork",
      { primaryId: 11, secondaryId: 70, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-005, GP-004 (balanced tree vs chain over MERGE-WORK units)
  action provePcdWorkTree(shapeChoice: int): bool = all {
    canProvePcdWorkTree(currentSnap, shapeChoice),
    commitSnap(
      applyProvePcdWorkTree(currentSnap, shapeChoice),
      "provePcdWorkTree",
      { primaryId: shapeChoice, secondaryId: 70 + shapeChoice, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-007 (honest GP gate run)
  action runGpGateHonest: bool = all {
    canRunGpGateHonest(currentSnap),
    commitSnap(
      applyRunGpGateHonest(currentSnap),
      "runGpGateHonest",
      { primaryId: 1, secondaryId: 0, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements GPH-007 Scenario (refusal case replaced by accept copy -> count 48, fails on case map)
  action runGpGateTamperedCaseMap: bool = all {
    canRunGpGateTamperedCaseMap(currentSnap),
    commitSnap(
      applyRunGpGateTamperedCaseMap(currentSnap),
      "runGpGateTamperedCaseMap",
      { primaryId: 2, secondaryId: 0, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements DG-010, CPR-005 (failed derived gate leaves GP targets open, never narrowed)
  action runGpGateFailedDerived: bool = all {
    canRunGpGateFailedDerived(currentSnap),
    commitSnap(
      applyRunGpGateFailedDerived(currentSnap),
      "runGpGateFailedDerived",
      { primaryId: 3, secondaryId: 0, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // Implements NAG-002 (refuses child proof omitting StmtAir, ProofShapeAir, or PublicValuesAir)
  action refuseMissingChildAir: bool = all {
    canRefuseMissingChildAir(currentSnap),
    val ev = { code: RefMissingRequiredAir, targetId: 91 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseMissingChildAir",
      { primaryId: 91, secondaryId: 0, forgery: ForgeryOmitRequiredAir, refusal: RefMissingRequiredAir }
    ),
  }

  // Implements NAG-003 (refuses N1 witness with wrong key_root in prog_d opening)
  action refuseN1WrongKeyRoot: bool = all {
    canRefuseN1WrongKeyRoot(currentSnap),
    val ev = { code: RefWrongProgDOpening, targetId: 92 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseN1WrongKeyRoot",
      { primaryId: 92, secondaryId: 0, forgery: ForgeryWrongKeyRoot, refusal: RefWrongProgDOpening }
    ),
  }

  // Implements NAG-006 (refuses T10q forged inputs MW-F01, AFK-F7, GP2-F4.2, unlinked AFK-F9(b))
  action refuseT10qForgery(fg: ForgeryKind): bool = all {
    canRefuseT10qForgery(currentSnap, fg),
    val code = refusalOfT10q(fg)
    val ev = { code: code, targetId: 93 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseT10qForgery",
      { primaryId: 93, secondaryId: 0, forgery: fg, refusal: code }
    ),
  }

  // Implements NAG-007 (refuses GP2-C2.1 witness)
  action refuseGp2C21Forgery: bool = all {
    canRefuseGp2C21Forgery(currentSnap),
    val ev = { code: RefGp2C21ConstRow, targetId: 94 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseGp2C21Forgery",
      { primaryId: 94, secondaryId: 0, forgery: ForgeryGp2C21PubvConstRow, refusal: RefGp2C21ConstRow }
    ),
  }

  // Implements NAG-008 (L refuses node root when outcome word differs from leaf-proved outcome)
  action refuseLWrongOutcome: bool = all {
    canRefuseLWrongOutcome(currentSnap),
    val ev = { code: RefWrongOutcomeAtL, targetId: 1 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseLWrongOutcome",
      { primaryId: 1, secondaryId: n3NodeId(1), forgery: ForgeryWrongLOutcome, refusal: RefWrongOutcomeAtL }
    ),
  }

  // Implements NAG-009 (N1/N2 refuses out-of-order or state-mismatched children)
  action refuseDisjointOrSwappedChildren: bool = all {
    canRefuseDisjointOrSwappedChildren(currentSnap),
    val ev = { code: RefDisjointOrSwappedChildren, targetId: 21 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseDisjointOrSwappedChildren",
      { primaryId: 12, secondaryId: 11, forgery: ForgeryDisjointOrSwappedLink, refusal: RefDisjointOrSwappedChildren }
    ),
  }

  // Implements GPH-002, GP-002 (refuses forged/unproved dependency or unsealed segment claim)
  action refuseGp2ForgedDep: bool = all {
    canRefuseGp2ForgedDep(currentSnap),
    val ev = { code: RefUnprovedOrUnsealedDep, targetId: 55 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseGp2ForgedDep",
      { primaryId: 55, secondaryId: 0, forgery: ForgeryUnprovedDep, refusal: RefUnprovedOrUnsealedDep }
    ),
  }

  // Implements GPH-002, NAG-011 AFK-F3 (JOIN refuses SET children with mixed reg_root)
  action refuseMixedRegRootJoin: bool = all {
    canRefuseMixedRegRootJoin(currentSnap),
    val ev = { code: RefMixedRegRootJoin, targetId: 59 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseMixedRegRootJoin",
      { primaryId: 59, secondaryId: 0, forgery: ForgeryNone, refusal: RefMixedRegRootJoin }
    ),
  }

  // Implements GPH-003 (N3 refuses FV unit whose key is not in foreign rows of keys_d)
  action refuseUnlistedFvUnit: bool = all {
    canRefuseUnlistedFvUnit(currentSnap),
    val ev = { code: RefUnlistedForeignKey, targetId: 99 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseUnlistedFvUnit",
      { primaryId: 99, secondaryId: 0, forgery: ForgeryUnlistedFvKey, refusal: RefUnlistedForeignKey }
    ),
  }

  // Implements GPH-004, GP-003 (refuses unregistered foreign circuit or unbound adapter relabeling)
  action refuseUnboundCompactAdapter: bool = all {
    canRefuseUnboundCompactAdapter(currentSnap),
    val ev = { code: RefUnboundCompactAdapter, targetId: 799 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseUnboundCompactAdapter",
      { primaryId: 799, secondaryId: 0, forgery: ForgeryUnboundForeignAdapter, refusal: RefUnboundCompactAdapter }
    ),
  }

  // Implements LPC-003, GPH-008 (L refuses forged SET root)
  action refuseLCollectionForgedSetRoot: bool = all {
    canRefuseLCollectionForgedSetRoot(currentSnap),
    val ev = { code: RefForgedSetRootAtL, targetId: 88 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseLCollectionForgedSetRoot",
      { primaryId: 88, secondaryId: 0, forgery: ForgeryForgedSetRoot, refusal: RefForgedSetRootAtL }
    ),
  }

  // Mutant-only unguarded acceptance action (disabled in all sound instances)
  action acceptMutantForgery: bool = all {
    canAcceptMutantForgery(currentSnap),
    commitSnap(
      applyAcceptMutantForgery(currentSnap),
      "acceptMutantForgery",
      { primaryId: 99, secondaryId: 0, forgery: ForgeryNone, refusal: RefNone }
    ),
  }

  // =========================================================================
  // 7. System Transition Step (balanced pre-filtered branches for fast reach)
  // =========================================================================

  action step: bool = any {
    {
      val cands = MW_KEY_IDS.filter(k => canAdmitMergeWorkKey(currentSnap, k))
      all {
        cands.size() > 0,
        nondet k = cands.oneOf()
        admitMergeWorkKey(k),
      }
    },
    {
      val cands = N1_SPEC_IDS.filter(id => canProveN1(currentSnap, id))
      all {
        cands.size() > 0,
        nondet id = cands.oneOf()
        proveN1(id),
      }
    },
    {
      val cands = RUN_IDS.filter(r => canProveN2Merge(currentSnap, r))
      all {
        cands.size() > 0,
        nondet r = cands.oneOf()
        proveN2Merge(r),
      }
    },
    {
      val cands = RUN_IDS.filter(r => canProveN3Root(currentSnap, r))
      all {
        cands.size() > 0,
        nondet r = cands.oneOf()
        proveN3Root(r),
      }
    },
    {
      val cands = RUN_IDS.filter(r => canProveRunNodeTree(currentSnap, r))
      all {
        cands.size() > 0,
        nondet r = cands.oneOf()
        proveRunNodeTree(r),
      }
    },
    {
      val cands = L_RUN_IDS.filter(r => canProveLRun(currentSnap, r))
      all {
        cands.size() > 0,
        nondet r = cands.oneOf()
        proveLRun(r),
      }
    },
    {
      val cands = L_RUN_IDS.filter(r => canPublishProofBundle(currentSnap, r))
      all {
        cands.size() > 0,
        nondet r = cands.oneOf()
        publishProofBundle(r),
      }
    },
    {
      val cands = Set(1, 2, 3, 5).filter(r => canSealRunNodeTree(currentSnap, r))
      all {
        cands.size() > 0,
        nondet r = cands.oneOf()
        sealRunNodeTree(r),
      }
    },
    any {
      {
        val cands = SEAL_RUN_IDS.filter(r => canProveGp2Seal(currentSnap, r))
        all {
          cands.size() > 0,
          nondet r = cands.oneOf()
          proveGp2Seal(r),
        }
      },
      proveGp2SealWithDeps,
    },
    {
      val cands = Set(1, 2).filter(jk => canProveGp2JoinSet(currentSnap, jk))
      all {
        cands.size() > 0,
        nondet jk = cands.oneOf()
        proveGp2JoinSet(jk),
      }
    },
    buildThreeClaimSetRoot,
    {
      val cands = Set(41, 51, 52).filter(sid => canProveGp2DischargeDeps(currentSnap, sid))
      all {
        cands.size() > 0,
        nondet sid = cands.oneOf()
        proveGp2DischargeDeps(sid),
      }
    },
    {
      val cands = Set(41, 51, 52, 55, 65).filter(sid => canProveLCollectionOverSet(currentSnap, sid))
      all {
        cands.size() > 0,
        nondet sid = cands.oneOf()
        proveLCollectionOverSet(sid),
      }
    },
    any {
      {
        val cands = universalKeys.foreignVkRows.filter(fr => canProveN3ForeignUnit(currentSnap, fr))
        all {
          cands.size() > 0,
          nondet fr = cands.oneOf()
          proveN3ForeignUnit(fr),
        }
      },
      adaptCompactForeignProof,
      proveOneChildMergeWork,
      {
        val cands = Set(1, 2).filter(sc => canProvePcdWorkTree(currentSnap, sc))
        all {
          cands.size() > 0,
          nondet sc = cands.oneOf()
          provePcdWorkTree(sc),
        }
      },
    },
    any {
      runGpGateHonest,
      runGpGateTamperedCaseMap,
      runGpGateFailedDerived,
    },
    any {
      {
        val cands = MW_KEY_IDS.filter(k => canRefuseMergeWorkKeyOverCols(currentSnap, k))
        all {
          cands.size() > 0,
          nondet k = cands.oneOf()
          refuseMergeWorkKeyOverCols(k),
        }
      },
      refuseMissingChildAir,
      refuseN1WrongKeyRoot,
      {
        val cands = T10Q_FORGERIES.filter(fg => canRefuseT10qForgery(currentSnap, fg))
        all {
          cands.size() > 0,
          nondet fg = cands.oneOf()
          refuseT10qForgery(fg),
        }
      },
      refuseGp2C21Forgery,
      refuseLWrongOutcome,
      refuseDisjointOrSwappedChildren,
      refuseGp2ForgedDep,
      refuseMixedRegRootJoin,
      refuseUnlistedFvUnit,
      refuseUnboundCompactAdapter,
      refuseLCollectionForgedSetRoot,
    },
    acceptMutantForgery,
  }

  // =========================================================================
  // 8. Requirement Invariants (inv_<ID>) and Consistency Checks
  // =========================================================================

  // NAG-001: N1, N2 and N3 are proved as STARKs over BabyBear at INTERNAL parameter set
  // and accepted by the stock verifier under the internal-recursive key.
  val inv_NAG_001: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      Set(TierN1, TierN2, TierN3).contains(np.tier) implies (
        np.starkField == FieldBabyBear
          and np.paramSet == ParamInternal
          and np.stockVerified
          and np.verifiedVk == universalKeys.internalRecVk
      )
    )

  // NAG-002: Parent node refuses any child omitting StmtAir, ProofShapeAir or PublicValuesAir.
  val inv_NAG_002: bool =
    nodeProofs.keys().forall(nid => nodeProofs.get(nid).allChildrenHaveRequiredAirs)

  // NAG-003: N1 statement AIR opens prog_d = P2(DOM_PROG, image_d, plan_d, key_root, machine_version)
  // and forwards key_root to its parent.
  val inv_NAG_003: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      val pm = programCatalog(np.progId)
      (np.tier == TierN1) implies (
        np.openedProgD == pm.progD
          and np.openedImageD == pm.imageD
          and np.openedPlanD == pm.planD
          and np.openedKeyRoot == pm.keyRoot
          and np.openedMachineVer == pm.machineVersion
          and np.forwardedKeyRoot == pm.keyRoot
      )
    )

  // NAG-004: Multi-chip-set runs merge under N2 and N3 iff all merged children belong to one key_root.
  val inv_NAG_004: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      (np.chipSetsMerged.size() > 1) implies (
        Set(TierN2, TierN3).contains(np.tier)
          and np.forwardedKeyRoot == programCatalog(np.progId).keyRoot
          and np.stockVerified
      )
    )

  // NAG-005: Key generator refuses any MERGE-WORK key with more than MERGE_WORK_MAX_COLS (5,400) columns.
  val inv_NAG_005: bool =
    mwKeyRegistry.keys().forall(kId =>
      mwKeyRegistry.get(kId).colCount <= MERGE_WORK_MAX_COLS
    )

  // NAG-006: Node AIRs refuse T10q forgeries MW-F01, AFK-F7, GP2-F4.2 and unlinked 1-child AFK-F9(b).
  val inv_NAG_006: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      not(T10Q_FORGERIES.contains(np.acceptedForgery))
        and np.whiLimbs16Bit
        and np.shapeBindsCut
        and np.nominalTypeBound
        and (np.oneChildMode implies np.oneChildLinked)
    )

  // NAG-007: Node AIRs refuse the forged input of counterexample GP2-C2.1.
  val inv_NAG_007: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      np.acceptedForgery != ForgeryGp2C21PubvConstRow and np.pubvConstRowChecked
    )

  // NAG-008: L circuit refuses any single-run node root whose outcome word differs from the proved outcome.
  val inv_NAG_008: bool =
    lRunProofs.keys().forall(rId =>
      val lr = lRunProofs.get(rId)
      nodeProofs.keys().contains(lr.n3NodeId)
        and nodeProofs.get(lr.n3NodeId).tier == TierN3
        and nodeProofs.get(lr.n3NodeId).isTerminal
        and lr.lOutcomeD == nodeProofs.get(lr.n3NodeId).outcomeD
    )

  // NAG-009: N1 and N2 merge constraints enforce contiguous step intervals and post_d == pre_d linking.
  val inv_NAG_009: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      (Set(TierN1, TierN2).contains(np.tier) and np.gp2Mode == ModeSegMerge) implies (
        np.intervalsContiguous and np.stateDigestsLinked and np.hStart < np.hEnd
      )
    )

  // NAG-010: Published ProofBundle holds leaf, N1, N2, N3, L/L_Fr entries, sha256s, claim_pub, and key manifest hash.
  val inv_NAG_010: bool =
    proofBundles.keys().forall(rId =>
      val pb = proofBundles.get(rId)
      lRunProofs.keys().contains(rId)
        and pb.hasLeafEntries
        and pb.hasLAndLFrEntries
        and pb.n1NodeIds.length() >= 1
        and pb.n1NodeIds.indices().forall(i => nodeProofs.keys().contains(pb.n1NodeIds[i]))
        and nodeProofs.keys().contains(pb.n2NodeId)
        and nodeProofs.keys().contains(pb.n3NodeId)
        and pb.nodeEntries.length() == 4
        and pb.nodeEntries.indices().forall(i =>
          val ent = pb.nodeEntries[i]
          nodeProofs.keys().contains(ent.nodeId)
            and ent.proofPathId == nodeProofs.get(ent.nodeId).proofPathId
            and ent.proofSha256 == nodeProofs.get(ent.nodeId).proofSha256
        )
        and pb.claimPub == lRunProofs.get(rId).claimPub
        and pb.keyManifestSha256 == universalKeys.keyManifestSha256
    )

  // NAG-011: Honest nonzero reg_root N1 witnesses preserve reg_root across merges, and 1-child MERGE-WORK is linked.
  val inv_NAG_011: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      np.regRootsEqual and (np.oneChildMode implies np.oneChildLinked)
    )

  // GPH-001: Interpreted and R_SL_* programs prove N1, N2, N3, L, and L_Fr under the same universal key set.
  val inv_GPH_001: bool =
    universalKeys == genesisUniversalKeys
      and lRunProofs.keys().forall(rId =>
        val lr = lRunProofs.get(rId)
        lr.verifiedLVk == universalKeys.lVk and lr.verifiedLFrVk == universalKeys.lFrVk
      )

  // GPH-002: Consumed claims verified through GP-2 modes must be proved in-circuit (no forged dependency).
  val inv_GPH_002: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      (np.gp2Mode == ModeDlStepDescend) implies (
        np.depsVerifiedInSet
          and np.depsAllSealedTerminal
          and np.pendingDeps.size() == 0
          and np.regRootsEqual
      )
    )

  // GPH-003: N3 accepts a foreign-verifier unit child iff its key is in the foreign rows of keys_d.
  val inv_GPH_003: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      (np.stmtKind == StmtFv) implies (
        np.tier == TierN3 and universalKeys.foreignVkRows.contains(np.fvVkRow)
      )
    )

  // GPH-004: Adapted Compact foreign proofs require HAS_X8_COMPACT_VERIFIER and name class compact-zkir3-kzg-bls12381-blake2b/1.
  val inv_GPH_004: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      (np.gp2Mode == ModeFvAccept and np.stmtKind == StmtSet) implies (
        HAS_X8_COMPACT_VERIFIER
          and np.adapterClass == AdapterCompactZkir3
          and np.compactVkId == REGISTERED_COMPACT_VK
          and np.adapterBoundInCircuit
      )
    )

  // GPH-005: Balanced tree and chain over the same MERGE-WORK units verify under universal keys and give equal claimD.
  val inv_GPH_005: bool =
    ((nodeProofs.keys().contains(71) and nodeProofs.keys().contains(72)) implies (
      nodeProofs.get(71).verifiedVk == universalKeys.internalRecVk
        and nodeProofs.get(72).verifiedVk == universalKeys.internalRecVk
        and nodeProofs.get(71).claimD == nodeProofs.get(72).claimD
    ))

  // GPH-006: Suspended-and-resumed run (Run 4) produces the exact same claim_pub as uninterrupted Run 1.
  val inv_GPH_006: bool =
    (lRunProofs.keys().contains(4) implies (
      val rc4 = runCatalog(4)
      val pm4 = programCatalog(rc4.progId)
      lRunProofs.get(4).claimPub == computeClaimPub(pm4.progD, rc4.inputD, lRunProofs.get(4).lOutcomeD)
    ))
      and ((lRunProofs.keys().contains(1) and lRunProofs.keys().contains(4)) implies (
        lRunProofs.get(1).claimPub == lRunProofs.get(4).claimPub
      ))

  // GPH-007: GP gate passes only if every case in the GP case map matches its expected outcome AND floors hold.
  val inv_GPH_007: bool =
    (gateStatus.verdict == GatePassed) implies (
      gateStatus.caseMapMatchesBaseline
        and gateStatus.gp2Count >= GP2_CASE_FLOOR
        and gateStatus.x7Count >= X7_CASE_FLOOR
    )

  // GPH-008: L collection form over a SET root exposes claims_root as a public word with verified memberships.
  val inv_GPH_008: bool =
    lCollectionProofs.keys().forall(cid =>
      val lc = lCollectionProofs.get(cid)
      HAS_L_COLLECTION_KEY
        and nodeProofs.keys().contains(lc.setNodeId)
        and nodeProofs.get(lc.setNodeId).stmtKind == StmtSet
        and lc.exposedClaimsRoot == nodeProofs.get(lc.setNodeId).claimsRoot
        and lc.memberClaims == nodeProofs.get(lc.setNodeId).claimSet
        and lc.allMembershipsVerified
        and lc.treeDepth == CLAIMS_TREE_DEPTH
    )

  // LPC-002: When STACKS AIR and VALUE DIGEST land in inline kernel, MERGE-WORK key has <= 5,400 columns.
  val inv_LPC_002: bool =
    mwKeyRegistry.keys().forall(kId =>
      val mk = mwKeyRegistry.get(kId)
      (mk.hasStacksAir and mk.hasValueDigest) implies (mk.colCount <= MERGE_WORK_MAX_COLS)
    )

  // LPC-003: L over a SET root refuses forged SET roots and only accepts verified real SET roots.
  val inv_LPC_003: bool =
    lCollectionProofs.keys().forall(cid =>
      val lc = lCollectionProofs.get(cid)
      nodeProofs.keys().contains(lc.setNodeId)
        and nodeProofs.get(lc.setNodeId).stockVerified
        and lc.allMembershipsVerified
    )

  // Adopted GP-002: Consumed claims are terminal sealed claims (never raw segment statements).
  val inv_GP_002: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      (np.stmtKind == StmtSet) implies (np.isTerminal and np.depsAllSealedTerminal)
    )

  // Adopted GP-003: Foreign proofs contributing claims verify registry membership and authenticated adapter binding.
  val inv_GP_003: bool =
    nodeProofs.keys().forall(nid =>
      val np = nodeProofs.get(nid)
      (np.gp2Mode == ModeFvAccept and np.stmtKind == StmtSet) implies (
        np.compactVkId == REGISTERED_COMPACT_VK
          and np.regRoot == VALID_REG_ROOT
          and np.adapterClass != AdapterUnboundRelabel
          and np.adapterBoundInCircuit
      )
    )

  // Adopted GP-004: Shape commitment uses address-free descriptors (never physical state) and may differ across equal claims.
  val inv_GP_004: bool =
    nodeProofs.keys().forall(nid => not(nodeProofs.get(nid).shapeUsesPhysState))
      and ((nodeProofs.keys().contains(71) and nodeProofs.keys().contains(72)) implies (
        nodeProofs.get(71).shapeD != nodeProofs.get(72).shapeD
          and nodeProofs.get(71).claimD == nodeProofs.get(72).claimD
      ))

  // Adopted DG-010 & CPR-005: A failed derived gate never narrows GP-001..GP-004 or GP-ZK.
  val inv_DG_010: bool =
    not(gateStatus.gpTargetsNarrowed)

  val inv_CPR_005: bool =
    not(gateStatus.gpTargetsNarrowed)

  // Master invariant conjunction
  val inv: bool = and {
    inv_NAG_001,
    inv_NAG_002,
    inv_NAG_003,
    inv_NAG_004,
    inv_NAG_005,
    inv_NAG_006,
    inv_NAG_007,
    inv_NAG_008,
    inv_NAG_009,
    inv_NAG_010,
    inv_NAG_011,
    inv_GPH_001,
    inv_GPH_002,
    inv_GPH_003,
    inv_GPH_004,
    inv_GPH_005,
    inv_GPH_006,
    inv_GPH_007,
    inv_GPH_008,
    inv_LPC_002,
    inv_LPC_003,
    inv_GP_002,
    inv_GP_003,
    inv_GP_004,
    inv_DG_010,
    inv_CPR_005,
  }

  // Pure / constant consistency checks for SPEC-CHECKED LPC-001 and cross-requirement findings:
  // LPC-001 (SPEC-CHECKED candidate): unbuilt hooks map to HookFreezePending, built hooks to HookFrozen.
  def hookRowStatus(built: bool): HookFreezeState =
    if (built) HookFrozen else HookFreezePending

  val consistency_LPC_001: bool =
    (hookRowStatus(true) == HookFrozen)
      and (hookRowStatus(false) == HookFreezePending)
      and ((not(HAS_X8_COMPACT_VERIFIER)) implies (hookRowStatus(HAS_X8_COMPACT_VERIFIER) == HookFreezePending))
      and ((not(HAS_L_COLLECTION_KEY)) implies (hookRowStatus(HAS_L_COLLECTION_KEY) == HookFreezePending))

  // Finding Q-aggregation-1: LPC-002 Scenario ("WHEN the key has 5,417 columns THEN ... raises the bound")
  // contradicts LPC-002 ("SHALL have at most 5,400 columns") and NAG-005 ("more than 5,400 -> refuse").
  val consistency_LPC_002_scenario_vs_NAG_005: bool =
    5417 <= MERGE_WORK_MAX_COLS

  // Finding Q-aggregation-2: GPH-003 places FV units only as children of N3 (StmtFv), which cannot directly
  // feed GP-2 inner-node dependency discharge (GPH-002 / GPH-004 Scenario) without a StmtSet adapter node.
  pure val consistency_GPH_003_vs_GPH_004_consumption: bool =
    StmtFv == StmtSet

  // =========================================================================
  // 9. Negated Reachability Witnesses (MUST be violated in aggregation_main)
  // =========================================================================

  val reach_n1_n2_n3_chain: bool = not(
    nodeProofs.keys().contains(n3NodeId(1))
  )

  val reach_mixed_chipset_merged: bool = not(
    nodeProofs.keys().contains(n3NodeId(3))
      and nodeProofs.get(n3NodeId(3)).chipSetsMerged == Set(0, 3)
  )

  val reach_both_families_proved_same_keys: bool = not(
    lRunProofs.keys().contains(1) and lRunProofs.keys().contains(2)
  )

  val reach_suspended_run_equal_claim_pub: bool = not(
    lRunProofs.keys().contains(1)
      and lRunProofs.keys().contains(4)
      and lRunProofs.get(1).claimPub == lRunProofs.get(4).claimPub
  )

  val reach_proof_bundle_published: bool = not(
    proofBundles.keys().size() > 0
  )

  val reach_honest_t10q_accepted: bool = not(
    nodeProofs.keys().contains(11)
      and nodeProofs.get(11).regRoot == VALID_REG_ROOT
      and nodeProofs.keys().contains(70)
      and nodeProofs.get(70).oneChildMode
  )

  val reach_gp2_dep_discharged: bool = not(
    nodeProofs.keys().contains(55)
  )

  val reach_l_collection_3claims: bool = not(
    lCollectionProofs.keys().exists(cid =>
      lCollectionProofs.get(cid).memberClaims.size() == 3
    )
  )

  val reach_fv_unit_and_compact_adapter: bool = not(
    nodeProofs.keys().contains(61) and nodeProofs.keys().contains(65)
  )

  val reach_pcd_two_shapes_same_claim: bool = not(
    nodeProofs.keys().contains(71)
      and nodeProofs.keys().contains(72)
      and nodeProofs.get(71).claimD == nodeProofs.get(72).claimD
  )

  val reach_mw_key_admitted_and_refused: bool = not(
    mwKeyRegistry.keys().contains(1)
      and refusalLog.exists(ev => ev.code == RefMergeWorkCols)
  )

  val reach_forgeries_refused: bool = not(
    refusalLog.exists(ev =>
      Set(
        RefT10qMwF01,
        RefGp2C21ConstRow,
        RefMissingRequiredAir,
        RefUnprovedOrUnsealedDep,
        RefForgedSetRootAtL
      ).contains(ev.code)
    )
  )

  val reach_gp_gate_case_map_failed: bool = not(
    gateStatus.verdict == GateFailedCaseMap
  )
}

// ===========================================================================
// 10. Concrete Instances (main, 2 edge instances, 18 safety mutants)
// ===========================================================================

module aggregation_main {
  import aggregation(
    MERGE_WORK_MAX_COLS     = 5400,
    MW_BASE_COLS            = 4988,
    MW_VALUE_DIGEST_DELTA   = 62,
    MW_STACKS_DELTA         = 71,
    KEYS_D_NODE_ROWS        = 7,
    KEYS_D_FOREIGN_ROWS     = 2,
    GP2_CASE_FLOOR          = 48,
    X7_CASE_FLOOR           = 22,
    CLAIMS_TREE_DEPTH       = 20,
    HAS_X8_COMPACT_VERIFIER = true,
    HAS_L_COLLECTION_KEY    = true,
    MUTANT                  = "none"
  ).*
}

// Edge instance 1: T25 pre-T8c configuration (smallest legal bounds, tightest 5,121 MW cap, X8 & L-SET unbuilt)
module aggregation_edge_t25_pre_t8c {
  import aggregation(
    MERGE_WORK_MAX_COLS     = 5121,
    MW_BASE_COLS            = 4988,
    MW_VALUE_DIGEST_DELTA   = 62,
    MW_STACKS_DELTA         = 71,
    KEYS_D_NODE_ROWS        = 7,
    KEYS_D_FOREIGN_ROWS     = 1,
    GP2_CASE_FLOOR          = 4,
    X7_CASE_FLOOR           = 2,
    CLAIMS_TREE_DEPTH       = 2,
    HAS_X8_COMPACT_VERIFIER = false,
    HAS_L_COLLECTION_KEY    = false,
    MUTANT                  = "none"
  ).*
}

// Edge instance 2: MERGE-WORK inline kernel width right at the 5,400-column limit (5267 + 62 + 71 = 5400)
module aggregation_edge_exact_5400 {
  import aggregation(
    MERGE_WORK_MAX_COLS     = 5400,
    MW_BASE_COLS            = 5267,
    MW_VALUE_DIGEST_DELTA   = 62,
    MW_STACKS_DELTA         = 71,
    KEYS_D_NODE_ROWS        = 7,
    KEYS_D_FOREIGN_ROWS     = 2,
    GP2_CASE_FLOOR          = 48,
    X7_CASE_FLOOR           = 22,
    CLAIMS_TREE_DEPTH       = 20,
    HAS_X8_COMPACT_VERIFIER = true,
    HAS_L_COLLECTION_KEY    = true,
    MUTANT                  = "none"
  ).*
}

// Mutant 1: breaks inv_NAG_001
module aggregation_m_wrong_node_params {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "wrong_node_params"
  ).*
}

// Mutant 2: breaks inv_NAG_002
module aggregation_m_accept_omitted_air {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "accept_omitted_air"
  ).*
}

// Mutant 3: breaks inv_NAG_003
module aggregation_m_unopened_key_root {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "unopened_key_root"
  ).*
}

// Mutant 4: breaks inv_NAG_004
module aggregation_m_mixed_chipset_wrong_keyroot {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "mixed_chipset_wrong_keyroot"
  ).*
}

// Mutant 5: breaks inv_NAG_005 and inv_LPC_002
module aggregation_m_allow_mw_over_5400 {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "allow_mw_over_5400"
  ).*
}

// Mutant 6: breaks inv_NAG_006
module aggregation_m_accept_t10q_forgery {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "accept_t10q_forgery"
  ).*
}

// Mutant 7: breaks inv_NAG_007 (represents open unfixed GP2-C2.1 defect)
module aggregation_m_gp2_c21_unfixed {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "gp2_c21_unfixed"
  ).*
}

// Mutant 8: breaks inv_NAG_008
module aggregation_m_l_unbound_outcome {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "l_unbound_outcome"
  ).*
}

// Mutant 9: breaks inv_NAG_009
module aggregation_m_skip_child_link_check {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "skip_child_link_check"
  ).*
}

// Mutant 10: breaks inv_NAG_010
module aggregation_m_incomplete_proof_bundle {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "incomplete_proof_bundle"
  ).*
}

// Mutant 11: breaks inv_GPH_002 and inv_GP_002
module aggregation_m_accept_forged_dep {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "accept_forged_dep"
  ).*
}

// Mutant 12: breaks inv_GPH_003
module aggregation_m_accept_unlisted_fv_key {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "accept_unlisted_fv_key"
  ).*
}

// Mutant 13: breaks inv_GPH_004 and inv_GP_003
module aggregation_m_unbound_foreign_adapter {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "unbound_foreign_adapter"
  ).*
}

// Mutant 14: breaks inv_GPH_005 and inv_GP_004
module aggregation_m_pcd_shape_changes_claim {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "pcd_shape_changes_claim"
  ).*
}

// Mutant 15: breaks inv_GPH_006
module aggregation_m_suspend_alters_claim_pub {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "suspend_alters_claim_pub"
  ).*
}

// Mutant 16: breaks inv_GPH_007
module aggregation_m_count_only_gp_gate {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "count_only_gp_gate"
  ).*
}

// Mutant 17: breaks inv_DG_010 and inv_CPR_005
module aggregation_m_narrow_gp_on_gate_fail {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "narrow_gp_on_gate_fail"
  ).*
}

// Mutant 18: breaks inv_GPH_008 and inv_LPC_003
module aggregation_m_l_accepts_forged_set_root {
  import aggregation(
    MERGE_WORK_MAX_COLS = 5400, MW_BASE_COLS = 4988, MW_VALUE_DIGEST_DELTA = 62, MW_STACKS_DELTA = 71,
    KEYS_D_NODE_ROWS = 7, KEYS_D_FOREIGN_ROWS = 2, GP2_CASE_FLOOR = 48, X7_CASE_FLOOR = 22,
    CLAIMS_TREE_DEPTH = 20, HAS_X8_COMPACT_VERIFIER = true, HAS_L_COLLECTION_KEY = true,
    MUTANT = "l_accepts_forged_set_root"
  ).*
}
