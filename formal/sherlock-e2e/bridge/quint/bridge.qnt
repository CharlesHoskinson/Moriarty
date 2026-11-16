// Quint specification for Sherlock E2E Prototype — Bounded Context: Bridge
// Lane: e2e-bridge
// Sources: LTR-001..008, CLB-001..008, ANC-013..019, LW-020..022, and architecture/DDD.md.

module bridge {
  // =========================================================================
  // 1. Spec Constants (numeric bounds and cross-component configuration)
  // =========================================================================

  const L_MAX_COLS: int                // LTR-003: max columns in L key (2,200)
  const L_MAX_INTERACTIONS: int        // LTR-003: max interactions per AIR in L key (100)
  const L_FP3_COLS: int                // LTR-003: post-FP3 L width (2,071 derived; 2,200 at limit)
  const IFACE_PUBLIC_WORDS: int        // CLB-001: L interface public word count (12,166)
  const IFACE_LANE_PAIRS: int          // CLB-001: L interface lane-map pair count (5)
  const ANCHOR_SLOT: int               // CLB-004, ANC-013..015: ledger slot for C_P (15)
  const K_MAX: int                     // CLB-007: chunk planner maximum chunks per call (2)
  const LFR_CONFIG_SLOT: int           // CLB-008: L_Fr interface configured anchor slot (15)
  const WRAP_DECODER_CONFIG_SLOT: int  // CLB-008: wrap decoder configured anchor slot (15)
  const SDK_LEDGER_CONFIG_SLOT: int    // ANC-014, CLB-008: SherlockLedger/SDK configured slot (15)
  const MUTANT: str                    // "none" in honest instances; mutant tag in bridge_m_*

  // =========================================================================
  // 2. Domain Types (enums and cohesive state records)
  // =========================================================================

  type StarkField =
    | FieldBabyBear
    | FieldBls12381Fr
    | FieldWrong

  type HashConfig =
    | HashMidnightPoseidonFr
    | HashWrong

  type TreeForm =
    | TreePathC
    | TreeFull

  type CallMode =
    | ModeOrdinary
    | ModeChunked

  type MemBusElemType =
    | TyBb
    | TyFr
    | TyUntyped

  type RefusalCode =
    | RefNone
    | RefRekeySlotMismatch
    | RefLKeyBounds
    | RefWhirShapeLogRsLtK
    | RefCompilerSlot15Collision
    | RefExceedsKMax
    | RefShiftedEffectLane
    | RefWrongChunkCount
    | RefMode0AnchorTemplateIface
    | RefMissingTapeExpose
    | RefIllTypedTapeOperand
    | RefTranscriptChallengeMismatch
    | RefMemBusTypeImbalance
    | RefChunkLockMismatch
    | RefStockLfrForgedAnchor
    | RefCorruptedProofTransfer
    | RefWrapWrongSlotPin
    | RefMode0AnchorTemplateWrap

  type RekeyStateRec = {
    started: bool,
    lfrSlotCfg: int,
    wrapDecoderSlotCfg: int,
    sdkLedgerSlotCfg: int,
    admittedLKeyId: int,
    lKeyCols: int,
    lKeyMaxInteractions: int,
    lVkId: int,
    lVkSha256: int,
    lfrVkId: int,
    lfrVkPrehash: int,
    lfrVkSha256: int,
    whirShapeChecked: bool,
    whirLogRs: int,
    whirFoldingK: int,
  }

  type LKeyCandidate = {
    keyId: int,
    colCount: int,
    maxInteractions: int,
  }

  type WhirShapeCandidate = {
    shapeId: int,
    whirLogRs: int,
    whirFoldingK: int,
  }

  type ContractRec = {
    contractId: int,
    topLevelFieldCount: int,
    usesSlot15: bool,
    compiled: bool,
    deployed: bool,
    ledgerAnchorSlot: int,
    slot15ValueCp: int,
    slot0IsUserField: bool,
  }

  type CallSpec = {
    callId: int,
    contractId: int,
    callMode: CallMode,
    treeForm: TreeForm,
    leafCount: int,
    sliceCount: int,
    cP: int,
    runTag: int,
    dIn: int,
    dOut: int,
    claimPub: int,
  }

  type ChunkPlanRec = {
    callId: int,
    callMode: CallMode,
    treeForm: TreeForm,
    sliceCount: int,
    chunkCount: int,
    cP: int,
    runTag: int,
    dIn: int,
    dOut: int,
    claimPub: int,
  }

  type ProofSlotSpec = {
    pSpecId: int,
    callId: int,
    chunkIdx: int,
  }

  type LProofRec = {
    lProofId: int,
    callId: int,
    chunkIdx: int,
    callMode: CallMode,
    treeForm: TreeForm,
    leafCount: int,
    statedK: int,
    trueSliceCount: int,
    baseField: StarkField,
    hashConfig: HashConfig,
    verifiedLVk: int,
    lKeySha256: int,
    publicWordsCount: int,
    laneMapPairs: int,
    effectLanesMatchGuest: bool,
    forwardChainValid: bool,
    inChainLink: int,
    outChainLink: int,
    carriesMode0AnchorWriteTemplate: bool,
    decodedInterfaceOk: bool,
    stockChallengesD: int,
    stockVerified: bool,
  }

  type LfrMachineRec = {
    tapeId: int,
    lProofId: int,
    usedTapeExpose: bool,
    operandsWellTyped: bool,
    replayedChallengesD: int,
    machineAccepted: bool,
  }

  type LfrProofRec = {
    lfrProofId: int,
    tapeId: int,
    lProofId: int,
    callId: int,
    chunkIdx: int,
    statedK: int,
    callMode: CallMode,
    treeForm: TreeForm,
    baseField: StarkField,
    extDegree: int,
    verifiedLfrVk: int,
    lfrKeyPrehash: int,
    lfrKeySha256: int,
    memBusSixthElem: MemBusElemType,
    memBusBalanced: bool,
    usedTapeExpose: bool,
    transcriptReplayedOutD: int,
    stockTranscriptOutD: int,
    anchorReadSlot: int,
    writesAnchor: bool,
    anchorWriteSlot: int,
    slot0PreservedAsUserWord: bool,
    readsPriorChunkLock: bool,
    readLockOp: int,
    expectedReadLockOp: int,
    writesNextChunkLock: bool,
    writeLockOp: int,
    expectedWriteLockOp: int,
    stockVerified: bool,
    proofPathId: int,
    proofSha256: int,
    publicWordsCount: int,
    claimPub: int,
  }

  type LfrProofReadyRec = {
    lfrProofId: int,
    proofPathId: int,
    proofSha256: int,
    lfrVkPrehash: int,
    publicWordsCount: int,
    claimPub: int,
    movedBytesSha256: int,
    muninnCheckPassed: bool,
  }

  type WrapAcceptanceRec = {
    wrapId: int,
    lfrProofId: int,
    callMode: CallMode,
    whirShapeChecked: bool,
    lane6ReadKeyConst: int,
    lane15ChunkWriteKeyConst: int,
    carriesMode0AnchorWriteTemplate: bool,
    transferSha256Matched: bool,
    wrapAccepted: bool,
  }

  type RefusalEvent = {
    code: RefusalCode,
    targetId: int,
  }

  type StepArgs = {
    primaryId: int,
    secondaryId: int,
    refusal: RefusalCode,
  }

  type WorldSnap = {
    rekeyState: RekeyStateRec,
    compiledContracts: int -> ContractRec,
    chunkPlans: int -> ChunkPlanRec,
    lProofs: int -> LProofRec,
    lfrMachines: int -> LfrMachineRec,
    lfrProofs: int -> LfrProofRec,
    readyRecords: int -> LfrProofReadyRec,
    wrapAcceptances: int -> WrapAcceptanceRec,
    refusalLog: Set[RefusalEvent],
  }

  // =========================================================================
  // 3. Fixed Finite Catalogs and Deterministic Hash / Lock Encodings
  // =========================================================================

  pure val UNIV_L_VK_ID: int        = 8101
  pure val UNIV_L_VK_SHA256: int    = 8201
  pure val UNIV_LFR_VK_ID: int      = 8102
  pure val UNIV_LFR_VK_PREHASH: int = 8302
  pure val UNIV_LFR_VK_SHA256: int  = 8202
  pure val U_LOCK_DOMAIN: int       = 8

  pure val L_KEY_CAND_IDS: Set[int] = 1.to(4)
  pure val CONTRACT_IDS: Set[int]   = Set(1, 2)
  pure val CALL_IDS: Set[int]       = 1.to(5)
  pure val PROOF_SLOT_IDS: Set[int] = 1.to(6)

  // Forward claim chain C_j = vd(Cons(rec_j, C_(j+1))) with C_k = 0 (Nil)
  pure def chainLinkAt(callId: int, sliceCount: int, idx: int): int =
    if (idx >= sliceCount) 0
    else callId * 1000 + (sliceCount - idx) * 111

  // 7-operand lock sponge: sponge(U_LOCK; c, pack(run_tag), k, d_in, d_out, pack(C_after), chunk_idx)
  pure def computeLockOperand(
    cP: int,
    runTag: int,
    kChunks: int,
    dIn: int,
    dOut: int,
    chainAfter: int,
    chunkIdx: int
  ): int =
    U_LOCK_DOMAIN * 10000000
      + (cP % 100) * 100000
      + (runTag % 1000) * 100
      + kChunks * 10000
      + (dIn % 100) * 37
      + (dOut % 100) * 13
      + chainAfter * 3
      + chunkIdx + 1

  pure def computeTranscriptChallenges(callId: int, chunkIdx: int): int =
    60000 + callId * 100 + chunkIdx

  pure def computeProofSha256(lfrProofId: int, claimPub: int): int =
    900000 + lfrProofId * 1000 + (claimPub % 1000)

  def lKeyCandidate(kId: int): LKeyCandidate =
    if (kId == 1) {
      keyId: 1,
      colCount: L_FP3_COLS,
      maxInteractions: if (L_FP3_COLS == L_MAX_COLS) L_MAX_INTERACTIONS else 99,
    } else if (kId == 2) {
      keyId: 2,
      colCount: L_MAX_COLS,
      maxInteractions: L_MAX_INTERACTIONS,
    } else if (kId == 3) {
      keyId: 3,
      colCount: L_MAX_COLS + 1,
      maxInteractions: 99,
    } else {
      keyId: 4,
      colCount: 1995,
      maxInteractions: L_MAX_INTERACTIONS + 1,
    }

  pure def whirCandidate(shapeId: int): WhirShapeCandidate =
    if (shapeId == 1) {
      shapeId: 1, whirLogRs: 5, whirFoldingK: 4,
    } else {
      // LW-022 Scenario: log_rs = 3 and k = 4 -> refused
      shapeId: 2, whirLogRs: 3, whirFoldingK: 4,
    }

  pure def contractSpec(cId: int): { contractId: int, topLevelFieldCount: int, cP: int } =
    if (cId == 1) {
      // Valid 4-field Compact contract whose slot 0 holds user data (CLB-004, ANC-014)
      contractId: 1, topLevelFieldCount: 4, cP: 3001,
    } else {
      // Colliding 16-field contract using top-level index 15 (ANC-015 Scenario)
      contractId: 2, topLevelFieldCount: 16, cP: 3002,
    }

  // Calls:
  // 1: Ordinary 1-leaf call on Path C (LTR-007, CLB-004)
  // 2: Ordinary 3-leaf run on Full Tree (LTR-001, LTR-007)
  // 3: Chunked 2-slice call with runTag = 444, dIn = 503 (CLB-003, CLB-005, ANC-017, ANC-019)
  // 4: Chunked 2-slice call sharing runTag = 444 with Call 3, differing in dIn = 599 (CLB-005, ANC-017 Scenario)
  // 5: Oversized 3-slice call exceeding K_MAX = 2 (CLB-007 Scenario)
  pure def callCatalog(callId: int): CallSpec =
    if (callId == 1) {
      callId: 1, contractId: 1, callMode: ModeOrdinary, treeForm: TreePathC,
      leafCount: 1, sliceCount: 1, cP: 3001, runTag: 401, dIn: 501, dOut: 601, claimPub: 7001,
    } else if (callId == 2) {
      callId: 2, contractId: 1, callMode: ModeOrdinary, treeForm: TreeFull,
      leafCount: 3, sliceCount: 1, cP: 3001, runTag: 402, dIn: 502, dOut: 602, claimPub: 7002,
    } else if (callId == 3) {
      callId: 3, contractId: 1, callMode: ModeChunked, treeForm: TreeFull,
      leafCount: 3, sliceCount: 2, cP: 3001, runTag: 444, dIn: 503, dOut: 603, claimPub: 7003,
    } else if (callId == 4) {
      callId: 4, contractId: 1, callMode: ModeChunked, treeForm: TreeFull,
      leafCount: 3, sliceCount: 2, cP: 3001, runTag: 444, dIn: 599, dOut: 604, claimPub: 7004,
    } else {
      callId: 5, contractId: 1, callMode: ModeChunked, treeForm: TreeFull,
      leafCount: 3, sliceCount: 3, cP: 3001, runTag: 405, dIn: 505, dOut: 605, claimPub: 7005,
    }

  pure def proofSlotCatalog(pSpecId: int): ProofSlotSpec =
    if (pSpecId == 1)      { pSpecId: 1, callId: 1, chunkIdx: 0 }
    else if (pSpecId == 2) { pSpecId: 2, callId: 2, chunkIdx: 0 }
    else if (pSpecId == 3) { pSpecId: 3, callId: 3, chunkIdx: 0 }
    else if (pSpecId == 4) { pSpecId: 4, callId: 3, chunkIdx: 1 }
    else if (pSpecId == 5) { pSpecId: 5, callId: 4, chunkIdx: 0 }
    else                   { pSpecId: 6, callId: 4, chunkIdx: 1 }

  // =========================================================================
  // 4. State Variables (observable state + replay driver metadata)
  // =========================================================================

  var lastOp: str
  var lastArgs: StepArgs
  var rekeyState: RekeyStateRec
  var compiledContracts: int -> ContractRec
  var chunkPlans: int -> ChunkPlanRec
  var lProofs: int -> LProofRec
  var lfrMachines: int -> LfrMachineRec
  var lfrProofs: int -> LfrProofRec
  var readyRecords: int -> LfrProofReadyRec
  var wrapAcceptances: int -> WrapAcceptanceRec
  var refusalLog: Set[RefusalEvent]

  val currentSnap: WorldSnap = {
    rekeyState: rekeyState,
    compiledContracts: compiledContracts,
    chunkPlans: chunkPlans,
    lProofs: lProofs,
    lfrMachines: lfrMachines,
    lfrProofs: lfrProofs,
    readyRecords: readyRecords,
    wrapAcceptances: wrapAcceptances,
    refusalLog: refusalLog,
  }

  action commitSnap(ws: WorldSnap, opName: str, args: StepArgs): bool = all {
    lastOp'            = opName,
    lastArgs'          = args,
    rekeyState'        = ws.rekeyState,
    compiledContracts' = ws.compiledContracts,
    chunkPlans'        = ws.chunkPlans,
    lProofs'           = ws.lProofs,
    lfrMachines'       = ws.lfrMachines,
    lfrProofs'         = ws.lfrProofs,
    readyRecords'      = ws.readyRecords,
    wrapAcceptances'   = ws.wrapAcceptances,
    refusalLog'        = ws.refusalLog,
  }

  // =========================================================================
  // 5. Functional Guards and State Transformers
  // =========================================================================

  // --- Rekey runner, L key bounds, and WHIR shape check (CLB-008, LTR-003, LW-022) ---
  def canStartRekeyAndAdmitKeys(ws: WorldSnap, kId: int): bool =
    val kc = lKeyCandidate(kId)
    val wc = whirCandidate(1)
    L_KEY_CAND_IDS.contains(kId)
      and not(ws.rekeyState.started)
      and ((MUTANT == "rekey_starts_on_slot_mismatch") or (LFR_CONFIG_SLOT == WRAP_DECODER_CONFIG_SLOT))
      and (
        (MUTANT == "allow_l_key_over_bounds")
          or (kc.colCount <= L_MAX_COLS and kc.maxInteractions <= L_MAX_INTERACTIONS)
      )
      and (wc.whirLogRs >= wc.whirFoldingK)

  def applyStartRekeyAndAdmitKeys(ws: WorldSnap, kId: int): WorldSnap =
    val kc = lKeyCandidate(kId)
    val wc = whirCandidate(1)
    val badWhir = (MUTANT == "wrap_skips_whir_shape_check")
    val usedWhir = if (badWhir) whirCandidate(2) else wc
    val nextRekey: RekeyStateRec = {
      started: true,
      lfrSlotCfg: LFR_CONFIG_SLOT,
      wrapDecoderSlotCfg: WRAP_DECODER_CONFIG_SLOT,
      sdkLedgerSlotCfg: SDK_LEDGER_CONFIG_SLOT,
      admittedLKeyId: kId,
      lKeyCols: kc.colCount,
      lKeyMaxInteractions: kc.maxInteractions,
      lVkId: UNIV_L_VK_ID,
      lVkSha256: UNIV_L_VK_SHA256,
      lfrVkId: UNIV_LFR_VK_ID,
      lfrVkPrehash: UNIV_LFR_VK_PREHASH,
      lfrVkSha256: UNIV_LFR_VK_SHA256,
      whirShapeChecked: not(badWhir),
      whirLogRs: usedWhir.whirLogRs,
      whirFoldingK: usedWhir.whirFoldingK,
    }
    { ...ws, rekeyState: nextRekey }

  // --- Mycroft contract compilation & SherlockLedger deployment (ANC-014, ANC-015) ---
  def canCompileAndDeployContract(ws: WorldSnap, cId: int): bool =
    val cs = contractSpec(cId)
    val collidesSlot15 = (cs.topLevelFieldCount > ANCHOR_SLOT)
    CONTRACT_IDS.contains(cId)
      and not(ws.compiledContracts.keys().contains(cId))
      and ((MUTANT == "compiler_allows_slot15_field") or not(collidesSlot15))

  def applyCompileAndDeployContract(ws: WorldSnap, cId: int): WorldSnap =
    val cs = contractSpec(cId)
    val collidesSlot15 = (cs.topLevelFieldCount > ANCHOR_SLOT)
    val deployedSlot =
      if (MUTANT == "ledger_stores_cp_in_slot0") 0 else ANCHOR_SLOT
    val rec: ContractRec = {
      contractId: cId,
      topLevelFieldCount: cs.topLevelFieldCount,
      usesSlot15: collidesSlot15,
      compiled: true,
      deployed: true,
      ledgerAnchorSlot: deployedSlot,
      slot15ValueCp: if (deployedSlot == ANCHOR_SLOT) cs.cP else 0,
      slot0IsUserField: (deployedSlot != 0),
    }
    { ...ws, compiledContracts: ws.compiledContracts.put(cId, rec) }

  // --- Chunk planner with K_MAX cap (CLB-007) ---
  def canPlanCallChunks(ws: WorldSnap, callId: int): bool =
    val cs = callCatalog(callId)
    CALL_IDS.contains(callId)
      and ws.compiledContracts.keys().contains(cs.contractId)
      and not(ws.chunkPlans.keys().contains(callId))
      and ((MUTANT == "planner_allows_over_kmax") or (cs.sliceCount <= K_MAX))

  def applyPlanCallChunks(ws: WorldSnap, callId: int): WorldSnap =
    val cs = callCatalog(callId)
    val plan: ChunkPlanRec = {
      callId: callId,
      callMode: cs.callMode,
      treeForm: cs.treeForm,
      sliceCount: cs.sliceCount,
      chunkCount: cs.sliceCount,
      cP: cs.cP,
      runTag: cs.runTag,
      dIn: cs.dIn,
      dOut: cs.dOut,
      claimPub: cs.claimPub,
    }
    { ...ws, chunkPlans: ws.chunkPlans.put(callId, plan) }

  // --- L proving with EffectAir, forward chain, and interface layout (LTR-001, LTR-007, CLB-001..003, CLB-006, ANC-018..019) ---
  def canProveLCallOrChunk(ws: WorldSnap, pSpecId: int): bool =
    val ps = proofSlotCatalog(pSpecId)
    PROOF_SLOT_IDS.contains(pSpecId)
      and ws.rekeyState.started
      and ws.chunkPlans.keys().contains(ps.callId)
      and not(ws.lProofs.keys().contains(pSpecId))

  def applyProveLCallOrChunk(ws: WorldSnap, pSpecId: int): WorldSnap =
    val ps = proofSlotCatalog(pSpecId)
    val cp = ws.chunkPlans.get(ps.callId)
    val cs = callCatalog(ps.callId)
    val usedField =
      if (MUTANT == "l_wrong_field_or_hash") FieldWrong else FieldBabyBear
    val usedHash =
      if (MUTANT == "l_wrong_field_or_hash") HashWrong else HashMidnightPoseidonFr
    val usedLKeySha =
      if (MUTANT == "split_pathc_and_full_keys" and cp.treeForm == TreePathC)
        ws.rekeyState.lVkSha256 + 77
      else
        ws.rekeyState.lVkSha256
    val usedWords =
      if (MUTANT == "wrong_iface_layout") IFACE_PUBLIC_WORDS + 14 else IFACE_PUBLIC_WORDS
    val usedPairs =
      if (MUTANT == "wrong_iface_layout") IFACE_LANE_PAIRS - 1 else IFACE_LANE_PAIRS
    val statedK =
      if (MUTANT == "accept_wrong_chunk_count" and cp.callMode == ModeChunked)
        cp.sliceCount + 1
      else
        cp.chunkCount
    val lanesOk = not(MUTANT == "accept_shifted_effect_lane")
    val chainOk = (statedK == cp.sliceCount)
    val mode0BadTemplate =
      (MUTANT == "iface_allows_mode0_anchor_write" and cp.callMode == ModeOrdinary)
    val rec: LProofRec = {
      lProofId: pSpecId,
      callId: ps.callId,
      chunkIdx: ps.chunkIdx,
      callMode: cp.callMode,
      treeForm: cp.treeForm,
      leafCount: cs.leafCount,
      statedK: statedK,
      trueSliceCount: cp.sliceCount,
      baseField: usedField,
      hashConfig: usedHash,
      verifiedLVk: ws.rekeyState.lVkId,
      lKeySha256: usedLKeySha,
      publicWordsCount: usedWords,
      laneMapPairs: usedPairs,
      effectLanesMatchGuest: lanesOk,
      forwardChainValid: chainOk,
      inChainLink: chainLinkAt(ps.callId, cp.sliceCount, ps.chunkIdx),
      outChainLink: chainLinkAt(ps.callId, cp.sliceCount, ps.chunkIdx + 1),
      carriesMode0AnchorWriteTemplate: mode0BadTemplate,
      decodedInterfaceOk: not(mode0BadTemplate),
      stockChallengesD: computeTranscriptChallenges(ps.callId, ps.chunkIdx),
      stockVerified: true,
    }
    { ...ws, lProofs: ws.lProofs.put(pSpecId, rec) }

  // --- L_Fr Machine::new check (LTR-004, LW-021) ---
  def canBuildLfrMachine(ws: WorldSnap, pSpecId: int): bool =
    PROOF_SLOT_IDS.contains(pSpecId)
      and ws.lProofs.keys().contains(pSpecId)
      and not(ws.lfrMachines.keys().contains(pSpecId))

  def applyBuildLfrMachine(ws: WorldSnap, pSpecId: int): WorldSnap =
    val lp = ws.lProofs.get(pSpecId)
    val usedExpose = not(MUTANT == "accept_plain_write_tape")
    val wellTyped = not(MUTANT == "machine_accepts_ill_typed_tape")
    val replayedChal =
      if (MUTANT == "accept_wrong_transcript") lp.stockChallengesD + 1 else lp.stockChallengesD
    val mRec: LfrMachineRec = {
      tapeId: pSpecId,
      lProofId: pSpecId,
      usedTapeExpose: usedExpose,
      operandsWellTyped: wellTyped,
      replayedChallengesD: replayedChal,
      machineAccepted: true,
    }
    { ...ws, lfrMachines: ws.lfrMachines.put(pSpecId, mRec) }

  // --- L_Fr proving & stock verification (LTR-002, LTR-005, LTR-007, CLB-004, CLB-005, ANC-017, LW-020) ---
  def canProveAndVerifyLfr(ws: WorldSnap, pSpecId: int): bool =
    val ps = proofSlotCatalog(pSpecId)
    PROOF_SLOT_IDS.contains(pSpecId)
      and ws.lfrMachines.keys().contains(pSpecId)
      and not(ws.lfrProofs.keys().contains(pSpecId))
      and (
        // Chunk j > 0 requires prior chunk j - 1 of the same call to be proved so its lock exists
        ps.chunkIdx == 0 or ws.lfrProofs.keys().contains(pSpecId - 1)
      )

  def applyProveAndVerifyLfr(ws: WorldSnap, pSpecId: int): WorldSnap =
    val ps = proofSlotCatalog(pSpecId)
    val cp = ws.chunkPlans.get(ps.callId)
    val lp = ws.lProofs.get(pSpecId)
    val mc = ws.lfrMachines.get(pSpecId)
    val usedBaseField =
      if (MUTANT == "lfr_uses_extension_field") FieldBabyBear else FieldBls12381Fr
    val usedExtDeg =
      if (MUTANT == "lfr_uses_extension_field") 4 else 1
    val usedLfrKeySha =
      if (MUTANT == "split_pathc_and_full_keys" and cp.treeForm == TreePathC)
        ws.rekeyState.lfrVkSha256 + 88
      else
        ws.rekeyState.lfrVkSha256
    val usedMemBusType =
      if (MUTANT == "membus_untyped_balance") TyUntyped else TyFr
    val usedReadSlot =
      if (MUTANT == "lfr_uses_slot0_or_writes_mode0_anchor") 0 else ANCHOR_SLOT
    val isChunkCall = (cp.callMode == ModeChunked)
    val writesAnc =
      if (MUTANT == "lfr_uses_slot0_or_writes_mode0_anchor" and not(isChunkCall))
        true
      else
        isChunkCall
    val usedWriteSlot = if (writesAnc) usedReadSlot else -1
    val readsPrior = (isChunkCall and ps.chunkIdx > 0)
    val writesNextLock = (isChunkCall and (ps.chunkIdx + 1 < cp.chunkCount))
    val expReadOp =
      if (readsPrior)
        computeLockOperand(cp.cP, cp.runTag, cp.chunkCount, cp.dIn, cp.dOut, lp.inChainLink, ps.chunkIdx - 1)
      else
        0
    val expWriteOp =
      if (writesNextLock)
        computeLockOperand(cp.cP, cp.runTag, cp.chunkCount, cp.dIn, cp.dOut, lp.outChainLink, ps.chunkIdx)
      else
        0
    val actualReadOp =
      if (MUTANT == "accept_spliced_lock" and readsPrior)
        // Splices lock of another run sharing runTag = 444 with different dIn = 999
        computeLockOperand(cp.cP, cp.runTag, cp.chunkCount, 999, cp.dOut, lp.inChainLink, ps.chunkIdx - 1)
      else
        expReadOp
    val pSha = computeProofSha256(pSpecId, cp.claimPub)
    val rec: LfrProofRec = {
      lfrProofId: pSpecId,
      tapeId: pSpecId,
      lProofId: pSpecId,
      callId: ps.callId,
      chunkIdx: ps.chunkIdx,
      statedK: lp.statedK,
      callMode: cp.callMode,
      treeForm: cp.treeForm,
      baseField: usedBaseField,
      extDegree: usedExtDeg,
      verifiedLfrVk: ws.rekeyState.lfrVkId,
      lfrKeyPrehash: ws.rekeyState.lfrVkPrehash,
      lfrKeySha256: usedLfrKeySha,
      memBusSixthElem: usedMemBusType,
      memBusBalanced: (usedMemBusType != TyUntyped),
      usedTapeExpose: mc.usedTapeExpose,
      transcriptReplayedOutD: mc.replayedChallengesD,
      stockTranscriptOutD: lp.stockChallengesD,
      anchorReadSlot: usedReadSlot,
      writesAnchor: writesAnc,
      anchorWriteSlot: usedWriteSlot,
      slot0PreservedAsUserWord: (usedReadSlot != 0 and usedWriteSlot != 0),
      readsPriorChunkLock: readsPrior,
      readLockOp: actualReadOp,
      expectedReadLockOp: expReadOp,
      writesNextChunkLock: writesNextLock,
      writeLockOp: expWriteOp,
      expectedWriteLockOp: expWriteOp,
      stockVerified: true,
      proofPathId: 6000 + pSpecId,
      proofSha256: pSha,
      publicWordsCount: lp.publicWordsCount,
      claimPub: cp.claimPub,
    }
    { ...ws, lfrProofs: ws.lfrProofs.put(pSpecId, rec) }

  // --- Publish LfrProofReady after stock L_Fr verification (LTR-006, LTR-008) ---
  def canPublishLfrReady(ws: WorldSnap, pSpecId: int): bool =
    PROOF_SLOT_IDS.contains(pSpecId)
      and ws.lfrProofs.keys().contains(pSpecId)
      and not(ws.readyRecords.keys().contains(pSpecId))
      and ws.lfrProofs.get(pSpecId).stockVerified

  def applyPublishLfrReady(ws: WorldSnap, pSpecId: int): WorldSnap =
    val lfr = ws.lfrProofs.get(pSpecId)
    val movedSha =
      if (MUTANT == "wrap_starts_on_corrupted_transfer")
        lfr.proofSha256 + 1
      else
        lfr.proofSha256
    val rdy: LfrProofReadyRec = {
      lfrProofId: pSpecId,
      proofPathId: lfr.proofPathId,
      proofSha256: lfr.proofSha256,
      lfrVkPrehash: lfr.lfrKeyPrehash,
      publicWordsCount: lfr.publicWordsCount,
      claimPub: lfr.claimPub,
      movedBytesSha256: movedSha,
      muninnCheckPassed: (movedSha == lfr.proofSha256),
    }
    { ...ws, readyRecords: ws.readyRecords.put(pSpecId, rdy) }

  // --- Muninn transfer digest check & Wrap acceptance (LTR-006, ANC-013, ANC-016, LW-022) ---
  def canVerifyTransferAndWrap(ws: WorldSnap, pSpecId: int): bool =
    PROOF_SLOT_IDS.contains(pSpecId)
      and ws.readyRecords.keys().contains(pSpecId)
      and not(ws.wrapAcceptances.keys().contains(pSpecId))
      and (
        (MUTANT == "wrap_starts_on_corrupted_transfer")
          or ws.readyRecords.get(pSpecId).muninnCheckPassed
      )

  def applyVerifyTransferAndWrap(ws: WorldSnap, pSpecId: int): WorldSnap =
    val rdy = ws.readyRecords.get(pSpecId)
    val lfr = ws.lfrProofs.get(pSpecId)
    val lp  = ws.lProofs.get(pSpecId)
    val pinSlot = if (MUTANT == "wrap_allows_slot0_pin") 0 else ANCHOR_SLOT
    val badMode0Template =
      (MUTANT == "wrap_allows_mode0_anchor_write" and lfr.callMode == ModeOrdinary)
    val wRec: WrapAcceptanceRec = {
      wrapId: pSpecId,
      lfrProofId: pSpecId,
      callMode: lfr.callMode,
      whirShapeChecked: ws.rekeyState.whirShapeChecked,
      lane6ReadKeyConst: pinSlot,
      lane15ChunkWriteKeyConst: pinSlot,
      carriesMode0AnchorWriteTemplate: (lp.carriesMode0AnchorWriteTemplate or badMode0Template),
      transferSha256Matched: (rdy.movedBytesSha256 == rdy.proofSha256),
      wrapAccepted: true,
    }
    { ...ws, wrapAcceptances: ws.wrapAcceptances.put(pSpecId, wRec) }

  // --- Full Call/Chunk Bridge Pipeline (composes the exact stage guards & transformers) ---
  def stepSlotPrereqs(ws: WorldSnap, pSpecId: int): WorldSnap =
    val ps = proofSlotCatalog(pSpecId)
    val cs = callCatalog(ps.callId)
    val ws0 =
      if (ws.rekeyState.started) ws
      else applyStartRekeyAndAdmitKeys(ws, 1)
    val ws1 =
      if (ws0.compiledContracts.keys().contains(cs.contractId)) ws0
      else applyCompileAndDeployContract(ws0, cs.contractId)
    val ws2 =
      if (ws1.chunkPlans.keys().contains(ps.callId)) ws1
      else applyPlanCallChunks(ws1, ps.callId)
    // If chunkIdx == 1 and chunk 0 is not yet proved in lfrProofs, complete chunk 0 through L_Fr first
    val ws3 =
      if (ps.chunkIdx == 0 or ws2.lfrProofs.keys().contains(pSpecId - 1)) ws2
      else (
        val prevId = pSpecId - 1
        val wA = if (ws2.lProofs.keys().contains(prevId)) ws2 else applyProveLCallOrChunk(ws2, prevId)
        val wB = if (wA.lfrMachines.keys().contains(prevId)) wA else applyBuildLfrMachine(wA, prevId)
        applyProveAndVerifyLfr(wB, prevId)
      )
    val ws4 = if (ws3.lProofs.keys().contains(pSpecId)) ws3 else applyProveLCallOrChunk(ws3, pSpecId)
    val ws5 = if (ws4.lfrMachines.keys().contains(pSpecId)) ws4 else applyBuildLfrMachine(ws4, pSpecId)
    val ws6 = if (ws5.lfrProofs.keys().contains(pSpecId)) ws5 else applyProveAndVerifyLfr(ws5, pSpecId)
    if (ws6.readyRecords.keys().contains(pSpecId)) ws6 else applyPublishLfrReady(ws6, pSpecId)

  def canBridgeCallEndToEnd(ws: WorldSnap, pSpecId: int): bool =
    val ps = proofSlotCatalog(pSpecId)
    val cs = callCatalog(ps.callId)
    PROOF_SLOT_IDS.contains(pSpecId)
      and not(ws.wrapAcceptances.keys().contains(pSpecId))
      and (ws.rekeyState.started or canStartRekeyAndAdmitKeys(ws, 1))
      and (
        val ws0 = if (ws.rekeyState.started) ws else applyStartRekeyAndAdmitKeys(ws, 1)
        (ws0.compiledContracts.keys().contains(cs.contractId) or canCompileAndDeployContract(ws0, cs.contractId)) and (
          val ws1 = if (ws0.compiledContracts.keys().contains(cs.contractId)) ws0 else applyCompileAndDeployContract(ws0, cs.contractId)
          (ws1.chunkPlans.keys().contains(ps.callId) or canPlanCallChunks(ws1, ps.callId)) and (
            val pre = stepSlotPrereqs(ws, pSpecId)
            canVerifyTransferAndWrap(pre, pSpecId)
          )
        )
      )

  def applyBridgeCallEndToEnd(ws: WorldSnap, pSpecId: int): WorldSnap =
    applyVerifyTransferAndWrap(stepSlotPrereqs(ws, pSpecId), pSpecId)

  // --- Mutant-Only Refused-L_Fr Publication (breaks inv_LTR_008 when MUTANT == "publish_ready_on_refused_lfr") ---
  def canMutantPublishOnRefusedLfr(ws: WorldSnap): bool =
    MUTANT == "publish_ready_on_refused_lfr"
      and ws.rekeyState.started
      and not(ws.lfrProofs.keys().contains(99))
      and not(ws.readyRecords.keys().contains(99))

  def applyMutantPublishOnRefusedLfr(ws: WorldSnap): WorldSnap =
    val badLfr: LfrProofRec = {
      lfrProofId: 99,
      tapeId: 99,
      lProofId: 99,
      callId: 1,
      chunkIdx: 0,
      statedK: 1,
      callMode: ModeOrdinary,
      treeForm: TreePathC,
      baseField: FieldBls12381Fr,
      extDegree: 1,
      verifiedLfrVk: ws.rekeyState.lfrVkId,
      lfrKeyPrehash: ws.rekeyState.lfrVkPrehash,
      lfrKeySha256: ws.rekeyState.lfrVkSha256,
      memBusSixthElem: TyFr,
      memBusBalanced: true,
      usedTapeExpose: true,
      transcriptReplayedOutD: 60100,
      stockTranscriptOutD: 60100,
      anchorReadSlot: ANCHOR_SLOT,
      writesAnchor: false,
      anchorWriteSlot: -1,
      slot0PreservedAsUserWord: true,
      readsPriorChunkLock: false,
      readLockOp: 0,
      expectedReadLockOp: 0,
      writesNextChunkLock: false,
      writeLockOp: 0,
      expectedWriteLockOp: 0,
      // Stock verifier REFUSED this forged-anchor L_Fr proof!
      stockVerified: false,
      proofPathId: 6099,
      proofSha256: 999001,
      publicWordsCount: IFACE_PUBLIC_WORDS,
      claimPub: 7001,
    }
    val badReady: LfrProofReadyRec = {
      lfrProofId: 99,
      proofPathId: 6099,
      proofSha256: 999001,
      lfrVkPrehash: ws.rekeyState.lfrVkPrehash,
      publicWordsCount: IFACE_PUBLIC_WORDS,
      claimPub: 7001,
      movedBytesSha256: 999001,
      muninnCheckPassed: true,
    }
    {
      ...ws,
      lfrProofs: ws.lfrProofs.put(99, badLfr),
      readyRecords: ws.readyRecords.put(99, badReady),
    }

  // --- Honest Refusal Guards (disabled when the corresponding mutant accepts the invalid input) ---
  def canRefuseRekeySlotMismatch(ws: WorldSnap): bool =
    MUTANT != "rekey_starts_on_slot_mismatch"
      and not(ws.refusalLog.contains({ code: RefRekeySlotMismatch, targetId: 80 }))

  def canRefuseLKeyOverBounds(ws: WorldSnap, kId: int): bool =
    val kc = lKeyCandidate(kId)
    val ev = { code: RefLKeyBounds, targetId: kId }
    L_KEY_CAND_IDS.contains(kId)
      and (kc.colCount > L_MAX_COLS or kc.maxInteractions > L_MAX_INTERACTIONS)
      and MUTANT != "allow_l_key_over_bounds"
      and not(ws.refusalLog.contains(ev))

  def canRefuseInvalidWhirShape(ws: WorldSnap): bool =
    val wc = whirCandidate(2)
    wc.whirLogRs < wc.whirFoldingK
      and MUTANT != "wrap_skips_whir_shape_check"
      and not(ws.refusalLog.contains({ code: RefWhirShapeLogRsLtK, targetId: 2 }))

  def canRefuseCollidingSlot15Contract(ws: WorldSnap): bool =
    val cs = contractSpec(2)
    cs.topLevelFieldCount > ANCHOR_SLOT
      and MUTANT != "compiler_allows_slot15_field"
      and not(ws.refusalLog.contains({ code: RefCompilerSlot15Collision, targetId: 2 }))

  def canRefuseCallOverKMax(ws: WorldSnap, callId: int): bool =
    val cs = callCatalog(callId)
    val ev = { code: RefExceedsKMax, targetId: callId }
    CALL_IDS.contains(callId)
      and cs.sliceCount > K_MAX
      and MUTANT != "planner_allows_over_kmax"
      and not(ws.refusalLog.contains(ev))

  def canRefuseShiftedEffectLaneWrite(ws: WorldSnap): bool =
    MUTANT != "accept_shifted_effect_lane"
      and not(ws.refusalLog.contains({ code: RefShiftedEffectLane, targetId: 81 }))

  def canRefuseWrongChunkCountInL(ws: WorldSnap): bool =
    MUTANT != "accept_wrong_chunk_count"
      and not(ws.refusalLog.contains({ code: RefWrongChunkCount, targetId: 82 }))

  def canRefuseMode0AnchorTemplateOnIface(ws: WorldSnap): bool =
    MUTANT != "iface_allows_mode0_anchor_write"
      and not(ws.refusalLog.contains({ code: RefMode0AnchorTemplateIface, targetId: 83 }))

  def canRefuseTapeWithoutExpose(ws: WorldSnap): bool =
    MUTANT != "accept_plain_write_tape"
      and not(ws.refusalLog.contains({ code: RefMissingTapeExpose, targetId: 84 }))

  def canRefuseIllTypedTapeInMachineNew(ws: WorldSnap): bool =
    MUTANT != "machine_accepts_ill_typed_tape"
      and not(ws.refusalLog.contains({ code: RefIllTypedTapeOperand, targetId: 85 }))

  def canRefuseTamperedTranscriptChallenge(ws: WorldSnap): bool =
    MUTANT != "accept_wrong_transcript"
      and not(ws.refusalLog.contains({ code: RefTranscriptChallengeMismatch, targetId: 86 }))

  def canRefuseMemBusTypeMismatchInCircuit(ws: WorldSnap): bool =
    MUTANT != "membus_untyped_balance"
      and not(ws.refusalLog.contains({ code: RefMemBusTypeImbalance, targetId: 87 }))

  def canRefuseSplicedOrAlteredChunkLock(ws: WorldSnap): bool =
    MUTANT != "accept_spliced_lock"
      and not(ws.refusalLog.contains({ code: RefChunkLockMismatch, targetId: 88 }))

  def canRefuseForgedAnchorLfrNoRecord(ws: WorldSnap): bool =
    MUTANT != "publish_ready_on_refused_lfr"
      and not(ws.refusalLog.contains({ code: RefStockLfrForgedAnchor, targetId: 89 }))

  def canRefuseCorruptedProofTransfer(ws: WorldSnap): bool =
    MUTANT != "wrap_starts_on_corrupted_transfer"
      and not(ws.refusalLog.contains({ code: RefCorruptedProofTransfer, targetId: 90 }))

  def canRefuseWrapWrongSlotPin(ws: WorldSnap): bool =
    MUTANT != "wrap_allows_slot0_pin"
      and not(ws.refusalLog.contains({ code: RefWrapWrongSlotPin, targetId: 91 }))

  def canRefuseMode0AnchorTemplateInWrap(ws: WorldSnap): bool =
    MUTANT != "wrap_allows_mode0_anchor_write"
      and not(ws.refusalLog.contains({ code: RefMode0AnchorTemplateWrap, targetId: 92 }))

  // =========================================================================
  // 6. Guarded Actions (every action sets lastOp and cites its EARS IDs)
  // =========================================================================

  action init: bool =
    val initRekey: RekeyStateRec = {
      started: false,
      lfrSlotCfg: LFR_CONFIG_SLOT,
      wrapDecoderSlotCfg: WRAP_DECODER_CONFIG_SLOT,
      sdkLedgerSlotCfg: SDK_LEDGER_CONFIG_SLOT,
      admittedLKeyId: 0,
      lKeyCols: 0,
      lKeyMaxInteractions: 0,
      lVkId: 0,
      lVkSha256: 0,
      lfrVkId: 0,
      lfrVkPrehash: 0,
      lfrVkSha256: 0,
      whirShapeChecked: false,
      whirLogRs: 0,
      whirFoldingK: 0,
    }
    val emptyContracts: int -> ContractRec = Map()
    val emptyPlans: int -> ChunkPlanRec = Map()
    val emptyLP: int -> LProofRec = Map()
    val emptyMach: int -> LfrMachineRec = Map()
    val emptyLfr: int -> LfrProofRec = Map()
    val emptyRdy: int -> LfrProofReadyRec = Map()
    val emptyWrap: int -> WrapAcceptanceRec = Map()
    val initSnap: WorldSnap = {
      rekeyState: initRekey,
      compiledContracts: emptyContracts,
      chunkPlans: emptyPlans,
      lProofs: emptyLP,
      lfrMachines: emptyMach,
      lfrProofs: emptyLfr,
      readyRecords: emptyRdy,
      wrapAcceptances: emptyWrap,
      refusalLog: Set(),
    }
    commitSnap(initSnap, "init", { primaryId: 0, secondaryId: 0, refusal: RefNone })

  // Implements CLB-008, LTR-003, LW-022
  action startRekeyAndAdmitKeys(kId: int): bool = all {
    canStartRekeyAndAdmitKeys(currentSnap, kId),
    commitSnap(
      applyStartRekeyAndAdmitKeys(currentSnap, kId),
      "startRekeyAndAdmitKeys",
      { primaryId: kId, secondaryId: 1, refusal: RefNone }
    ),
  }

  // Implements ANC-014, ANC-015
  action compileAndDeployContract(cId: int): bool = all {
    canCompileAndDeployContract(currentSnap, cId),
    commitSnap(
      applyCompileAndDeployContract(currentSnap, cId),
      "compileAndDeployContract",
      { primaryId: cId, secondaryId: ANCHOR_SLOT, refusal: RefNone }
    ),
  }

  // Implements CLB-007
  action planCallChunks(callId: int): bool = all {
    canPlanCallChunks(currentSnap, callId),
    commitSnap(
      applyPlanCallChunks(currentSnap, callId),
      "planCallChunks",
      { primaryId: callId, secondaryId: callCatalog(callId).sliceCount, refusal: RefNone }
    ),
  }

  // Implements LTR-001, LTR-007, CLB-001, CLB-002, CLB-003, CLB-006, ANC-018, ANC-019
  action proveLCallOrChunk(pSpecId: int): bool = all {
    canProveLCallOrChunk(currentSnap, pSpecId),
    commitSnap(
      applyProveLCallOrChunk(currentSnap, pSpecId),
      "proveLCallOrChunk",
      { primaryId: pSpecId, secondaryId: proofSlotCatalog(pSpecId).callId, refusal: RefNone }
    ),
  }

  // Implements LTR-004, LW-021
  action buildLfrMachine(pSpecId: int): bool = all {
    canBuildLfrMachine(currentSnap, pSpecId),
    commitSnap(
      applyBuildLfrMachine(currentSnap, pSpecId),
      "buildLfrMachine",
      { primaryId: pSpecId, secondaryId: pSpecId, refusal: RefNone }
    ),
  }

  // Implements LTR-002, LTR-005, LTR-007, CLB-004, CLB-005, ANC-017, LW-020
  action proveAndVerifyLfr(pSpecId: int): bool = all {
    canProveAndVerifyLfr(currentSnap, pSpecId),
    commitSnap(
      applyProveAndVerifyLfr(currentSnap, pSpecId),
      "proveAndVerifyLfr",
      { primaryId: pSpecId, secondaryId: proofSlotCatalog(pSpecId).callId, refusal: RefNone }
    ),
  }

  // Implements LTR-006, LTR-008
  action publishLfrReady(pSpecId: int): bool = all {
    canPublishLfrReady(currentSnap, pSpecId),
    commitSnap(
      applyPublishLfrReady(currentSnap, pSpecId),
      "publishLfrReady",
      { primaryId: pSpecId, secondaryId: 6000 + pSpecId, refusal: RefNone }
    ),
  }

  // Implements LTR-006, ANC-013, ANC-016, LW-022
  action verifyTransferAndWrap(pSpecId: int): bool = all {
    canVerifyTransferAndWrap(currentSnap, pSpecId),
    commitSnap(
      applyVerifyTransferAndWrap(currentSnap, pSpecId),
      "verifyTransferAndWrap",
      { primaryId: pSpecId, secondaryId: pSpecId, refusal: RefNone }
    ),
  }

  // Implements LTR-001..008, CLB-001..008, ANC-013..019, LW-020..022 (end-to-end call/chunk bridge pipeline)
  action bridgeCallEndToEnd(pSpecId: int): bool = all {
    canBridgeCallEndToEnd(currentSnap, pSpecId),
    commitSnap(
      applyBridgeCallEndToEnd(currentSnap, pSpecId),
      "bridgeCallEndToEnd",
      { primaryId: pSpecId, secondaryId: proofSlotCatalog(pSpecId).callId, refusal: RefNone }
    ),
  }

  // Implements CLB-008 (refusal when L_Fr and wrap decoder anchor slot constants differ)
  action refuseRekeySlotMismatch: bool = all {
    canRefuseRekeySlotMismatch(currentSnap),
    val ev = { code: RefRekeySlotMismatch, targetId: 80 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseRekeySlotMismatch",
      { primaryId: 15, secondaryId: 0, refusal: RefRekeySlotMismatch }
    ),
  }

  // Implements LTR-003 (refusal of L key exceeding 2,200 columns or 100 interactions)
  action refuseLKeyOverBounds(kId: int): bool = all {
    canRefuseLKeyOverBounds(currentSnap, kId),
    val ev = { code: RefLKeyBounds, targetId: kId }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseLKeyOverBounds",
      { primaryId: kId, secondaryId: 0, refusal: RefLKeyBounds }
    ),
  }

  // Implements LW-022 (WhirShape::check refuses log_rs < k before synthesizing cells)
  action refuseInvalidWhirShape: bool = all {
    canRefuseInvalidWhirShape(currentSnap),
    val ev = { code: RefWhirShapeLogRsLtK, targetId: 2 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseInvalidWhirShape",
      { primaryId: 3, secondaryId: 4, refusal: RefWhirShapeLogRsLtK }
    ),
  }

  // Implements ANC-015 (Mycroft compiler refuses 16-field contract colliding with slot 15)
  action refuseCollidingSlot15Contract: bool = all {
    canRefuseCollidingSlot15Contract(currentSnap),
    val ev = { code: RefCompilerSlot15Collision, targetId: 2 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseCollidingSlot15Contract",
      { primaryId: 2, secondaryId: 15, refusal: RefCompilerSlot15Collision }
    ),
  }

  // Implements CLB-007 (chunk planner refuses call exceeding K_MAX before any proof starts)
  action refuseCallOverKMax(callId: int): bool = all {
    canRefuseCallOverKMax(currentSnap, callId),
    val ev = { code: RefExceedsKMax, targetId: callId }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseCallOverKMax",
      { primaryId: callId, secondaryId: K_MAX, refusal: RefExceedsKMax }
    ),
  }

  // Implements CLB-002, ANC-018 (L EffectAir refuses shifted/unwritten effect lane word)
  action refuseShiftedEffectLaneWrite: bool = all {
    canRefuseShiftedEffectLaneWrite(currentSnap),
    val ev = { code: RefShiftedEffectLane, targetId: 81 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseShiftedEffectLaneWrite",
      { primaryId: 81, secondaryId: 12, refusal: RefShiftedEffectLane }
    ),
  }

  // Implements CLB-003, ANC-019 (L EffectAir refuses stated chunk count k != slice count)
  action refuseWrongChunkCountInL: bool = all {
    canRefuseWrongChunkCountInL(currentSnap),
    val ev = { code: RefWrongChunkCount, targetId: 82 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseWrongChunkCountInL",
      { primaryId: 82, secondaryId: 3, refusal: RefWrongChunkCount }
    ),
  }

  // Implements CLB-006 (iface::decode_with refuses mode 0 call carrying anchor-write template)
  action refuseMode0AnchorTemplateOnIface: bool = all {
    canRefuseMode0AnchorTemplateOnIface(currentSnap),
    val ev = { code: RefMode0AnchorTemplateIface, targetId: 83 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseMode0AnchorTemplateOnIface",
      { primaryId: 83, secondaryId: 0, refusal: RefMode0AnchorTemplateIface }
    ),
  }

  // Implements LTR-004 (L_Fr Machine::new refuses tape that omits Tape::expose)
  action refuseTapeWithoutExpose: bool = all {
    canRefuseTapeWithoutExpose(currentSnap),
    val ev = { code: RefMissingTapeExpose, targetId: 84 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseTapeWithoutExpose",
      { primaryId: 84, secondaryId: 0, refusal: RefMissingTapeExpose }
    ),
  }

  // Implements LW-021 (L_Fr Machine::new refuses tape passing Fr cell to BbAlu)
  action refuseIllTypedTapeInMachineNew: bool = all {
    canRefuseIllTypedTapeInMachineNew(currentSnap),
    val ev = { code: RefIllTypedTapeOperand, targetId: 85 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseIllTypedTapeInMachineNew",
      { primaryId: 85, secondaryId: 1, refusal: RefIllTypedTapeOperand }
    ),
  }

  // Implements LTR-005 (L_Fr refuses tape with altered Fiat-Shamir challenge word)
  action refuseTamperedTranscriptChallenge: bool = all {
    canRefuseTamperedTranscriptChallenge(currentSnap),
    val ev = { code: RefTranscriptChallengeMismatch, targetId: 86 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseTamperedTranscriptChallenge",
      { primaryId: 86, secondaryId: 1, refusal: RefTranscriptChallengeMismatch }
    ),
  }

  // Implements LW-020 (MEM_BUS argument fails balance when Fr value is read as BabyBear operand)
  action refuseMemBusTypeMismatchInCircuit: bool = all {
    canRefuseMemBusTypeMismatchInCircuit(currentSnap),
    val ev = { code: RefMemBusTypeImbalance, targetId: 87 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseMemBusTypeMismatchInCircuit",
      { primaryId: 87, secondaryId: 0, refusal: RefMemBusTypeImbalance }
    ),
  }

  // Implements CLB-005, ANC-017 (L_Fr refuses chunk with spliced equal-run_tag lock or altered k/link)
  action refuseSplicedOrAlteredChunkLock: bool = all {
    canRefuseSplicedOrAlteredChunkLock(currentSnap),
    val ev = { code: RefChunkLockMismatch, targetId: 88 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseSplicedOrAlteredChunkLock",
      { primaryId: 3, secondaryId: 4, refusal: RefChunkLockMismatch }
    ),
  }

  // Implements LTR-008 (stock verifier refuses forged-anchor L_Fr proof; no LfrProofReady published)
  action refuseForgedAnchorLfrNoRecord: bool = all {
    canRefuseForgedAnchorLfrNoRecord(currentSnap),
    val ev = { code: RefStockLfrForgedAnchor, targetId: 89 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseForgedAnchorLfrNoRecord",
      { primaryId: 89, secondaryId: 0, refusal: RefStockLfrForgedAnchor }
    ),
  }

  // Implements LTR-006 (muninn refuses corrupted L_Fr proof transfer and does not start wrap)
  action refuseCorruptedProofTransfer: bool = all {
    canRefuseCorruptedProofTransfer(currentSnap),
    val ev = { code: RefCorruptedProofTransfer, targetId: 90 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseCorruptedProofTransfer",
      { primaryId: 90, secondaryId: 0, refusal: RefCorruptedProofTransfer }
    ),
  }

  // Implements ANC-013 (wrap refuses witness writing slot 0 in lane 6 / lane 15 template pin)
  action refuseWrapWrongSlotPin: bool = all {
    canRefuseWrapWrongSlotPin(currentSnap),
    val ev = { code: RefWrapWrongSlotPin, targetId: 91 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseWrapWrongSlotPin",
      { primaryId: 91, secondaryId: 0, refusal: RefWrapWrongSlotPin }
    ),
  }

  // Implements ANC-016 (wrap MockProver refuses ordinary mode 0 call carrying anchor-write template)
  action refuseMode0AnchorTemplateInWrap: bool = all {
    canRefuseMode0AnchorTemplateInWrap(currentSnap),
    val ev = { code: RefMode0AnchorTemplateWrap, targetId: 92 }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseMode0AnchorTemplateWrap",
      { primaryId: 92, secondaryId: 11, refusal: RefMode0AnchorTemplateWrap }
    ),
  }

  // Mutant-only unguarded publication on refused L_Fr proof (disabled in all honest instances)
  action mutantPublishOnRefusedLfr: bool = all {
    canMutantPublishOnRefusedLfr(currentSnap),
    commitSnap(
      applyMutantPublishOnRefusedLfr(currentSnap),
      "mutantPublishOnRefusedLfr",
      { primaryId: 99, secondaryId: 99, refusal: RefNone }
    ),
  }

  // =========================================================================
  // 7. System Transition Step (balanced top-level branches)
  // =========================================================================

  action step: bool = any {
    {
      val cands = L_KEY_CAND_IDS.filter(k => canStartRekeyAndAdmitKeys(currentSnap, k))
      all {
        cands.size() > 0,
        nondet k = cands.oneOf()
        startRekeyAndAdmitKeys(k),
      }
    },
    {
      val cands = CONTRACT_IDS.filter(c => canCompileAndDeployContract(currentSnap, c))
      all {
        cands.size() > 0,
        nondet c = cands.oneOf()
        compileAndDeployContract(c),
      }
    },
    {
      val cands = CALL_IDS.filter(cid => canPlanCallChunks(currentSnap, cid))
      all {
        cands.size() > 0,
        nondet cid = cands.oneOf()
        planCallChunks(cid),
      }
    },
    {
      val cands = PROOF_SLOT_IDS.filter(ps => canProveLCallOrChunk(currentSnap, ps))
      all {
        cands.size() > 0,
        nondet ps = cands.oneOf()
        proveLCallOrChunk(ps),
      }
    },
    {
      val cands = PROOF_SLOT_IDS.filter(ps => canBuildLfrMachine(currentSnap, ps))
      all {
        cands.size() > 0,
        nondet ps = cands.oneOf()
        buildLfrMachine(ps),
      }
    },
    {
      val cands = PROOF_SLOT_IDS.filter(ps => canProveAndVerifyLfr(currentSnap, ps))
      all {
        cands.size() > 0,
        nondet ps = cands.oneOf()
        proveAndVerifyLfr(ps),
      }
    },
    {
      val cands = PROOF_SLOT_IDS.filter(ps => canPublishLfrReady(currentSnap, ps))
      all {
        cands.size() > 0,
        nondet ps = cands.oneOf()
        publishLfrReady(ps),
      }
    },
    {
      val cands = PROOF_SLOT_IDS.filter(ps => canVerifyTransferAndWrap(currentSnap, ps))
      all {
        cands.size() > 0,
        nondet ps = cands.oneOf()
        verifyTransferAndWrap(ps),
      }
    },
    {
      val cands = PROOF_SLOT_IDS.filter(ps => canBridgeCallEndToEnd(currentSnap, ps))
      all {
        cands.size() > 0,
        nondet ps = cands.oneOf()
        bridgeCallEndToEnd(ps),
      }
    },
    any {
      refuseRekeySlotMismatch,
      {
        val cands = L_KEY_CAND_IDS.filter(k => canRefuseLKeyOverBounds(currentSnap, k))
        all {
          cands.size() > 0,
          nondet k = cands.oneOf()
          refuseLKeyOverBounds(k),
        }
      },
      refuseInvalidWhirShape,
      refuseCollidingSlot15Contract,
      {
        val cands = CALL_IDS.filter(cid => canRefuseCallOverKMax(currentSnap, cid))
        all {
          cands.size() > 0,
          nondet cid = cands.oneOf()
          refuseCallOverKMax(cid),
        }
      },
      refuseShiftedEffectLaneWrite,
      refuseWrongChunkCountInL,
      refuseMode0AnchorTemplateOnIface,
      refuseTapeWithoutExpose,
      refuseIllTypedTapeInMachineNew,
      refuseTamperedTranscriptChallenge,
      refuseMemBusTypeMismatchInCircuit,
      refuseSplicedOrAlteredChunkLock,
      refuseForgedAnchorLfrNoRecord,
      refuseCorruptedProofTransfer,
      refuseWrapWrongSlotPin,
      refuseMode0AnchorTemplateInWrap,
    },
    mutantPublishOnRefusedLfr,
  }

  // =========================================================================
  // 8. Requirement Invariants (inv_<ID> for all 26 EARS IDs)
  // =========================================================================

  // LTR-001: L is proved as a STARK over BabyBear with Midnight Poseidon over BLS12-381 Fr.
  val inv_LTR_001: bool =
    lProofs.keys().forall(pid =>
      val lp = lProofs.get(pid)
      lp.baseField == FieldBabyBear
        and lp.hashConfig == HashMidnightPoseidonFr
        and lp.stockVerified
        and lp.verifiedLVk == rekeyState.lVkId
    )

  // LTR-002: L_Fr is proved as a STARK over BLS12-381 Fr with no extension field (extDegree == 1).
  val inv_LTR_002: bool =
    lfrProofs.keys().forall(pid =>
      val p = lfrProofs.get(pid)
      p.baseField == FieldBls12381Fr
        and p.extDegree == 1
        and p.verifiedLfrVk == rekeyState.lfrVkId
    )

  // LTR-003: Admitted L key has at most 2,200 columns and at most 100 interactions per AIR.
  val inv_LTR_003: bool =
    rekeyState.started implies (
      rekeyState.lKeyCols <= L_MAX_COLS
        and rekeyState.lKeyMaxInteractions <= L_MAX_INTERACTIONS
    )

  // LTR-004: Accepted L_Fr tapes/machines expose public words through Tape::expose.
  val inv_LTR_004: bool =
    lfrMachines.keys().forall(tid =>
      lfrMachines.get(tid).machineAccepted implies lfrMachines.get(tid).usedTapeExpose
    )

  // LTR-005: L_Fr tape replays the Fiat-Shamir transcript of L with outputs equal to stock transcript.
  val inv_LTR_005: bool =
    lfrMachines.keys().forall(tid =>
      val mc = lfrMachines.get(tid)
      lProofs.keys().contains(mc.lProofId)
        and mc.replayedChallengesD == lProofs.get(mc.lProofId).stockChallengesD
    )
      and lfrProofs.keys().forall(pid =>
        lfrProofs.get(pid).transcriptReplayedOutD == lfrProofs.get(pid).stockTranscriptOutD
      )

  // LTR-006: Published LfrProofReady matches L_Fr proof sha256, VK pre-hash, public words, and claim_pub;
  // wrap starts only when moved proof bytes match proof_sha256.
  val inv_LTR_006: bool =
    readyRecords.keys().forall(pid =>
      val rdy = readyRecords.get(pid)
      lfrProofs.keys().contains(pid)
        and rdy.proofPathId == lfrProofs.get(pid).proofPathId
        and rdy.proofSha256 == lfrProofs.get(pid).proofSha256
        and rdy.lfrVkPrehash == lfrProofs.get(pid).lfrKeyPrehash
        and rdy.publicWordsCount == lfrProofs.get(pid).publicWordsCount
        and rdy.claimPub == lfrProofs.get(pid).claimPub
    )
      and wrapAcceptances.keys().forall(wid =>
        val wr = wrapAcceptances.get(wid)
        readyRecords.keys().contains(wr.lfrProofId)
          and wr.transferSha256Matched
          and readyRecords.get(wr.lfrProofId).movedBytesSha256 == readyRecords.get(wr.lfrProofId).proofSha256
      )

  // LTR-007: Path C (1-leaf) and Full Tree (3-leaf) runs use the exact same L key and L_Fr key.
  val inv_LTR_007: bool =
    lProofs.keys().forall(pid => lProofs.get(pid).lKeySha256 == rekeyState.lVkSha256)
      and lfrProofs.keys().forall(pid => lfrProofs.get(pid).lfrKeySha256 == rekeyState.lfrVkSha256)
      and ((lfrProofs.keys().contains(1) and lfrProofs.keys().contains(2)) implies (
        lProofs.get(1).lKeySha256 == lProofs.get(2).lKeySha256
          and lfrProofs.get(1).lfrKeySha256 == lfrProofs.get(2).lfrKeySha256
      ))

  // LTR-008: No LfrProofReady record is published for an L_Fr proof refused by the stock verifier.
  val inv_LTR_008: bool =
    readyRecords.keys().forall(pid =>
      lfrProofs.keys().contains(pid) and lfrProofs.get(pid).stockVerified
    )

  // CLB-001: L interface exposes IFACE_PUBLIC_WORDS (12,166) public words with IFACE_LANE_PAIRS (5) pairs.
  val inv_CLB_001: bool =
    lProofs.keys().forall(pid =>
      val lp = lProofs.get(pid)
      lp.publicWordsCount == IFACE_PUBLIC_WORDS and lp.laneMapPairs == IFACE_LANE_PAIRS
    )

  // CLB-002: L EffectAir refuses any witness whose call-vector effect lane differs from guest write.
  val inv_CLB_002: bool =
    lProofs.keys().forall(pid => lProofs.get(pid).effectLanesMatchGuest)

  // CLB-003: L EffectAir refuses any chunked run stating k != slice count.
  val inv_CLB_003: bool =
    lProofs.keys().forall(pid =>
      val lp = lProofs.get(pid)
      lp.statedK == lp.trueSliceCount and lp.forwardChainValid
    )

  // CLB-004: L_Fr interface uses ANCHOR_SLOT = 15 for every anchor read and chunk-only anchor write, leaving slot 0 user data.
  val inv_CLB_004: bool =
    lfrProofs.keys().forall(pid =>
      val p = lfrProofs.get(pid)
      p.anchorReadSlot == ANCHOR_SLOT
        and p.slot0PreservedAsUserWord
        and ((p.callMode == ModeOrdinary) implies (not(p.writesAnchor) and p.anchorWriteSlot == -1))
        and ((p.callMode == ModeChunked) implies (p.writesAnchor and p.anchorWriteSlot == ANCHOR_SLOT))
    )

  // CLB-005: L_Fr refuses any chunk whose lock operand differs from the 7-input sponge of its run.
  val inv_CLB_005: bool =
    lfrProofs.keys().forall(pid =>
      val p = lfrProofs.get(pid)
      (p.readsPriorChunkLock implies (
        p.readLockOp == p.expectedReadLockOp
          and lfrProofs.keys().contains(pid - 1)
          and p.readLockOp == lfrProofs.get(pid - 1).writeLockOp
      ))
        and (p.writesNextChunkLock implies (p.writeLockOp == p.expectedWriteLockOp))
    )

  // CLB-006: iface::decode_with refuses any ordinary call (mode 0) carrying the anchor-write template.
  val inv_CLB_006: bool =
    lProofs.keys().forall(pid =>
      val lp = lProofs.get(pid)
      lp.decodedInterfaceOk
        and ((lp.callMode == ModeOrdinary) implies not(lp.carriesMode0AnchorWriteTemplate))
    )

  // CLB-007: Chunk planner refuses any call cut into more than K_MAX chunks before proving starts.
  val inv_CLB_007: bool =
    chunkPlans.keys().forall(cid =>
      chunkPlans.get(cid).chunkCount <= K_MAX and chunkPlans.get(cid).sliceCount <= K_MAX
    )

  // CLB-008: Rekey runner refuses to start if L_Fr interface anchor slot != wrap decoder anchor slot.
  val inv_CLB_008: bool =
    rekeyState.started implies (
      rekeyState.lfrSlotCfg == rekeyState.wrapDecoderSlotCfg
    )

  // ANC-013: Wrap circuit pins anchor read key (lane 6) and chunk write key (lane 15) to key constant 15.
  val inv_ANC_013: bool =
    wrapAcceptances.keys().forall(wid =>
      val wr = wrapAcceptances.get(wid)
      wr.lane6ReadKeyConst == ANCHOR_SLOT and wr.lane15ChunkWriteKeyConst == ANCHOR_SLOT
    )

  // ANC-014: Deployed SherlockLedger contract stores C_P in top-level slot 15 and keeps slot 0 as user field.
  val inv_ANC_014: bool =
    compiledContracts.keys().forall(cid =>
      val cr = compiledContracts.get(cid)
      cr.deployed implies (
        cr.ledgerAnchorSlot == ANCHOR_SLOT
          and cr.slot15ValueCp == contractSpec(cid).cP
          and cr.slot0IsUserField
      )
    )

  // ANC-015: Mycroft compiler refuses any contract whose ledger layout uses top-level index 15.
  val inv_ANC_015: bool =
    compiledContracts.keys().forall(cid =>
      val cr = compiledContracts.get(cid)
      cr.compiled implies (not(cr.usesSlot15) and cr.topLevelFieldCount <= ANCHOR_SLOT)
    )

  // ANC-016: Wrap circuit refuses any ordinary call (mode 0) carrying the anchor-write template.
  val inv_ANC_016: bool =
    wrapAcceptances.keys().forall(wid =>
      val wr = wrapAcceptances.get(wid)
      (wr.callMode == ModeOrdinary) implies not(wr.carriesMode0AnchorWriteTemplate)
    )

  // ANC-017: L_Fr lock operand binds (c, run_tag, k, d_in, d_out, pack(C_link), j) so equal-run_tag runs with different d_in never collide.
  val inv_ANC_017: bool =
    lfrProofs.keys().forall(pid =>
      val p = lfrProofs.get(pid)
      (p.readsPriorChunkLock implies (p.readLockOp == p.expectedReadLockOp))
        and (p.writesNextChunkLock implies (p.writeLockOp == p.expectedWriteLockOp))
    )
      and (computeLockOperand(3001, 444, 2, 503, 603, chainLinkAt(3, 2, 1), 0)
        != computeLockOperand(3001, 444, 2, 599, 604, chainLinkAt(4, 2, 1), 0))

  // ANC-018: L circuit binds every call-vector effect lane to the guest effect word written at that lane.
  val inv_ANC_018: bool =
    lProofs.keys().forall(pid => lProofs.get(pid).effectLanesMatchGuest)

  // ANC-019: L circuit binds k to the true slice count through the forward claim chain.
  val inv_ANC_019: bool =
    lProofs.keys().forall(pid =>
      val lp = lProofs.get(pid)
      lp.statedK == lp.trueSliceCount and lp.forwardChainValid
    )

  // LW-020: L_Fr MEM_BUS carries TY_BB = 0 or TY_FR = 1 as 6th element and balances by type.
  val inv_LW_020: bool =
    lfrProofs.keys().forall(pid =>
      val p = lfrProofs.get(pid)
      Set(TyBb, TyFr).contains(p.memBusSixthElem) and p.memBusBalanced
    )

  // LW-021: L_Fr host Machine::new refuses ill-typed tapes before proving.
  val inv_LW_021: bool =
    lfrMachines.keys().forall(tid =>
      lfrMachines.get(tid).machineAccepted implies lfrMachines.get(tid).operandsWellTyped
    )

  // LW-022: Wrap host WhirShape::check refuses any verifying-key WHIR shape with log_rs < k.
  val inv_LW_022: bool =
    (rekeyState.started implies (
      rekeyState.whirShapeChecked and rekeyState.whirLogRs >= rekeyState.whirFoldingK
    ))
      and wrapAcceptances.keys().forall(wid => wrapAcceptances.get(wid).whirShapeChecked)

  // Master invariant conjunction across all 26 EARS IDs
  val inv: bool = and {
    inv_LTR_001,
    inv_LTR_002,
    inv_LTR_003,
    inv_LTR_004,
    inv_LTR_005,
    inv_LTR_006,
    inv_LTR_007,
    inv_LTR_008,
    inv_CLB_001,
    inv_CLB_002,
    inv_CLB_003,
    inv_CLB_004,
    inv_CLB_005,
    inv_CLB_006,
    inv_CLB_007,
    inv_CLB_008,
    inv_ANC_013,
    inv_ANC_014,
    inv_ANC_015,
    inv_ANC_016,
    inv_ANC_017,
    inv_ANC_018,
    inv_ANC_019,
    inv_LW_020,
    inv_LW_021,
    inv_LW_022,
  }

  // =========================================================================
  // 9. Pure Consistency Checks Capturing Spec Contradictions (Q-bridge-1..4)
  // =========================================================================

  // Q-bridge-1: CLB-001 SHALL ("SHALL expose 12,166 public words") contradicts CLB-001 Scenario
  // ("or the record names the new count") when FP-3 + slot 15 change the T25 interface word count (+delta != 0).
  val consistency_CLB_001_post_rekey_nonzero_delta: bool =
    (IFACE_PUBLIC_WORDS + 14) == IFACE_PUBLIC_WORDS

  // Q-bridge-2: ANC-017 & CLB-005 state "When a chunk call reads or writes a lock, L_Fr SHALL compute x_j
  // from ... pack(C_(j+1)) and j". For chunk 1 of a 2-chunk call (Call 3), literal x_1 != x_0 written by chunk 0!
  pure val consistency_ANC_017_CLB_005_literal_lock_read_write: bool =
    val x0WrittenByChunk0 = computeLockOperand(3001, 444, 2, 503, 603, chainLinkAt(3, 2, 1), 0)
    val x1LiteralReadByChunk1 = computeLockOperand(3001, 444, 2, 503, 603, chainLinkAt(3, 2, 2), 1)
    x0WrittenByChunk0 == x1LiteralReadByChunk1

  // Q-bridge-3: CLB-006 and ANC-016 only check lanes 11..19 and 21 for the mode-0 anchor-write template;
  // a non-Mycroft guest writing slot 15 at user lane >= 22 satisfies FP-3 (guest wrote it) and bypasses both!
  pure def mode0GuardCatchesSlot15WriteAtLane(laneIdx: int): bool =
    (11 <= laneIdx and laneIdx <= 19) or (laneIdx == 21)

  pure val consistency_mode0_slot15_outside_template_blocked: bool =
    mode0GuardCatchesSlot15WriteAtLane(22)

  // Q-bridge-4: CLB-008's guard (lfrSlot == wrapSlot) neither checks == 15 (0 == 0 passes!)
  // nor checks the 3rd copy in SherlockLedger / anchor-slot.mjs (15 == 15 with sdkSlot = 0 passes!).
  pure def clb008LiteralCheck(lfrSlot: int, wrapSlot: int): bool =
    lfrSlot == wrapSlot

  val consistency_CLB_008_catches_zero_and_sdk_drift: bool =
    not(clb008LiteralCheck(0, 0))
      and not(clb008LiteralCheck(ANCHOR_SLOT, ANCHOR_SLOT) and (0 != ANCHOR_SLOT))

  // =========================================================================
  // 10. Negated Reachability Witnesses (MUST be violated in bridge_main)
  // =========================================================================

  val reach_rekey_and_contract_deployed: bool = not(
    rekeyState.started and compiledContracts.keys().contains(1)
  )

  val reach_ordinary_call_wrapped: bool = not(
    wrapAcceptances.keys().contains(1) or wrapAcceptances.keys().contains(2)
  )

  val reach_pathc_and_full_tree_same_keys: bool = not(
    lfrProofs.keys().contains(1)
      and lfrProofs.keys().contains(2)
      and lfrProofs.get(1).lfrKeySha256 == lfrProofs.get(2).lfrKeySha256
      and lProofs.get(1).lKeySha256 == lProofs.get(2).lKeySha256
  )

  val reach_two_chunk_call_linked_and_wrapped: bool = not(
    wrapAcceptances.keys().contains(3)
      and wrapAcceptances.keys().contains(4)
      and lfrProofs.get(4).readLockOp == lfrProofs.get(3).writeLockOp
  )

  val reach_equal_run_tag_distinct_runs_proved: bool = not(
    lfrProofs.keys().contains(3)
      and lfrProofs.keys().contains(5)
      and lfrProofs.get(3).writeLockOp != lfrProofs.get(5).writeLockOp
  )

  val reach_key_whir_and_kmax_refusals: bool = not(
    refusalLog.exists(ev => ev.code == RefLKeyBounds)
      and refusalLog.exists(ev => ev.code == RefExceedsKMax)
  )

  val reach_compiler_and_rekey_refusals: bool = not(
    refusalLog.exists(ev => ev.code == RefCompilerSlot15Collision)
      and refusalLog.exists(ev => ev.code == RefRekeySlotMismatch)
  )

  val reach_l_and_iface_refusals: bool = not(
    refusalLog.exists(ev =>
      Set(
        RefShiftedEffectLane,
        RefWrongChunkCount,
        RefMode0AnchorTemplateIface
      ).contains(ev.code)
    )
  )

  val reach_lfr_and_wrap_refusals: bool = not(
    refusalLog.exists(ev =>
      Set(
        RefMissingTapeExpose,
        RefIllTypedTapeOperand,
        RefTranscriptChallengeMismatch,
        RefMemBusTypeImbalance,
        RefChunkLockMismatch,
        RefStockLfrForgedAnchor,
        RefCorruptedProofTransfer,
        RefWrapWrongSlotPin,
        RefMode0AnchorTemplateWrap
      ).contains(ev.code)
    )
  )
}

// ===========================================================================
// 11. Concrete Instances (main, 2 edge instances, 23 safety mutants)
// ===========================================================================

module bridge_main {
  import bridge(
    L_MAX_COLS               = 2200,
    L_MAX_INTERACTIONS       = 100,
    L_FP3_COLS               = 2071,
    IFACE_PUBLIC_WORDS       = 12166,
    IFACE_LANE_PAIRS         = 5,
    ANCHOR_SLOT              = 15,
    K_MAX                    = 2,
    LFR_CONFIG_SLOT          = 15,
    WRAP_DECODER_CONFIG_SLOT = 15,
    SDK_LEDGER_CONFIG_SLOT   = 15,
    MUTANT                   = "none"
  ).*
}

// Edge instance 1: Smallest legal chunk cap K_MAX = 1 (1-slice calls wrap; 2-slice calls refused at planner)
module bridge_edge_min_k1 {
  import bridge(
    L_MAX_COLS               = 2200,
    L_MAX_INTERACTIONS       = 100,
    L_FP3_COLS               = 1995,
    IFACE_PUBLIC_WORDS       = 12166,
    IFACE_LANE_PAIRS         = 5,
    ANCHOR_SLOT              = 15,
    K_MAX                    = 1,
    LFR_CONFIG_SLOT          = 15,
    WRAP_DECODER_CONFIG_SLOT = 15,
    SDK_LEDGER_CONFIG_SLOT   = 15,
    MUTANT                   = "none"
  ).*
}

// Edge instance 2: L key columns & interactions right at exact spec limits (2,200 columns, 100 interactions)
module bridge_edge_exact_bounds {
  import bridge(
    L_MAX_COLS               = 2200,
    L_MAX_INTERACTIONS       = 100,
    L_FP3_COLS               = 2200,
    IFACE_PUBLIC_WORDS       = 12166,
    IFACE_LANE_PAIRS         = 5,
    ANCHOR_SLOT              = 15,
    K_MAX                    = 2,
    LFR_CONFIG_SLOT          = 15,
    WRAP_DECODER_CONFIG_SLOT = 15,
    SDK_LEDGER_CONFIG_SLOT   = 15,
    MUTANT                   = "none"
  ).*
}

// Mutant 1: breaks inv_LTR_001
module bridge_m_l_wrong_field_or_hash {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "l_wrong_field_or_hash"
  ).*
}

// Mutant 2: breaks inv_LTR_002
module bridge_m_lfr_uses_extension_field {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "lfr_uses_extension_field"
  ).*
}

// Mutant 3: breaks inv_LTR_003
module bridge_m_allow_l_key_over_bounds {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "allow_l_key_over_bounds"
  ).*
}

// Mutant 4: breaks inv_LTR_004
module bridge_m_accept_plain_write_tape {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "accept_plain_write_tape"
  ).*
}

// Mutant 5: breaks inv_LTR_005
module bridge_m_accept_wrong_transcript {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "accept_wrong_transcript"
  ).*
}

// Mutant 6: breaks inv_LTR_006
module bridge_m_wrap_starts_on_corrupted_transfer {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "wrap_starts_on_corrupted_transfer"
  ).*
}

// Mutant 7: breaks inv_LTR_007
module bridge_m_split_pathc_and_full_keys {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "split_pathc_and_full_keys"
  ).*
}

// Mutant 8: breaks inv_LTR_008
module bridge_m_publish_ready_on_refused_lfr {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "publish_ready_on_refused_lfr"
  ).*
}

// Mutant 9: breaks inv_CLB_001
module bridge_m_wrong_iface_layout {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "wrong_iface_layout"
  ).*
}

// Mutant 10: breaks inv_CLB_002 and inv_ANC_018
module bridge_m_accept_shifted_effect_lane {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "accept_shifted_effect_lane"
  ).*
}

// Mutant 11: breaks inv_CLB_003 and inv_ANC_019
module bridge_m_accept_wrong_chunk_count {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "accept_wrong_chunk_count"
  ).*
}

// Mutant 12: breaks inv_CLB_004
module bridge_m_lfr_uses_slot0_or_writes_mode0_anchor {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "lfr_uses_slot0_or_writes_mode0_anchor"
  ).*
}

// Mutant 13: breaks inv_CLB_005 and inv_ANC_017
module bridge_m_accept_spliced_lock {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "accept_spliced_lock"
  ).*
}

// Mutant 14: breaks inv_CLB_006
module bridge_m_iface_allows_mode0_anchor_write {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "iface_allows_mode0_anchor_write"
  ).*
}

// Mutant 15: breaks inv_CLB_007
module bridge_m_planner_allows_over_kmax {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "planner_allows_over_kmax"
  ).*
}

// Mutant 16: breaks inv_CLB_008 (LFR_CONFIG_SLOT = 15 != WRAP_DECODER_CONFIG_SLOT = 0)
module bridge_m_rekey_starts_on_slot_mismatch {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 0, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "rekey_starts_on_slot_mismatch"
  ).*
}

// Mutant 17: breaks inv_ANC_013
module bridge_m_wrap_allows_slot0_pin {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "wrap_allows_slot0_pin"
  ).*
}

// Mutant 18: breaks inv_ANC_014
module bridge_m_ledger_stores_cp_in_slot0 {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "ledger_stores_cp_in_slot0"
  ).*
}

// Mutant 19: breaks inv_ANC_015
module bridge_m_compiler_allows_slot15_field {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "compiler_allows_slot15_field"
  ).*
}

// Mutant 20: breaks inv_ANC_016
module bridge_m_wrap_allows_mode0_anchor_write {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "wrap_allows_mode0_anchor_write"
  ).*
}

// Mutant 21: breaks inv_LW_020
module bridge_m_membus_untyped_balance {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "membus_untyped_balance"
  ).*
}

// Mutant 22: breaks inv_LW_021
module bridge_m_machine_accepts_ill_typed_tape {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "machine_accepts_ill_typed_tape"
  ).*
}

// Mutant 23: breaks inv_LW_022
module bridge_m_wrap_skips_whir_shape_check {
  import bridge(
    L_MAX_COLS = 2200, L_MAX_INTERACTIONS = 100, L_FP3_COLS = 2071, IFACE_PUBLIC_WORDS = 12166,
    IFACE_LANE_PAIRS = 5, ANCHOR_SLOT = 15, K_MAX = 2, LFR_CONFIG_SLOT = 15,
    WRAP_DECODER_CONFIG_SLOT = 15, SDK_LEDGER_CONFIG_SLOT = 15, MUTANT = "wrap_skips_whir_shape_check"
  ).*
}
