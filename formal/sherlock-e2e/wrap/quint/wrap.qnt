// -*- mode: Bluespec; -*-
// Independent Quint specification for bounded context Wrap (lane e2e-wrap).
// Sources:
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-wrap/spec.md
//     (WRP-001..004, WRP-006..012; note WRP-005 is absent in spec.md)
//   - openspec/changes/sherlock-completion-20261115/specs/sherlock-rekey/spec.md
//     (RKY-003, RKY-004)
//   - openspec/changes/sherlock-processor-final-20261111/specs/*
//     (adopted ANC-001, DG-002, DG-003, GPW-001, GPW-003, GPW-007,
//      LW-002..006, LW-012..014, LW-016, MID-001..003, ZK-003)

module wrap {
  // =========================================================================
  // Constants (numeric bounds from the EARS requirements)
  // =========================================================================

  // DG-002: smallest candidate circuit parameter k in {19, 20, 21}.
  const MIN_WRAP_K: int
  // WRP-002, DG-002, LW-013, RKY-003: maximum circuit k ("never k = 22" => k <= 21).
  const MAX_WRAP_K: int
  // WRP-003, RKY-003, DG-002: floor(0.85 * (2^21 - 7)) = 1,782,573 rows at k = 21.
  const MAX_ROWS_AT_K21: int
  // WRP-004, RKY-004, DG-003, MID-003: serialized wrap VK byte ceiling (50,000 bytes).
  const MAX_VK_BYTES: int
  // LW-003, MID-003: call vector public input count = 2 + L_TX = 2 + 1024 = 1,026.
  const CALL_VECTOR_PUBLIC_INPUTS: int
  // WRP-007, ANC-001: sealed anchor slot constant (15).
  const ANCHOR_SLOT_CONST: int
  // WRP-007, ANC-001: anchor read key lane index (6).
  const ANCHOR_READ_LANE: int
  // WRP-007, ANC-001: chunk write key lane index (15).
  const CHUNK_WRITE_LANE: int
  // WRP-009: WrapOutput file count (wrap.vk, wrap.proof, wrap.public.json => 3).
  const WRAP_OUTPUT_FILE_COUNT: int
  // WRP-012: number of required LW pass checks on T25 (LW-003, LW-004, LW-005, LW-006, LW-014 => 5).
  const LW_QUAL_CHECK_COUNT: int
  // LW-002: L_Fr transcript arity/parameter u = 7.
  const LFR_TRANSCRIPT_U: int
  // LW-014: minimum log capacity needed to hold largest reservation R.
  const MIN_LOG_RESERVATION_R: int
  // Mutant selector: 0 = honest specification; 1..19 = individual guard mutants.
  const MUTANT_ID: int

  // =========================================================================
  // Mutant identifiers
  // =========================================================================

  pure val MUT_NONE: int = 0
  pure val MUT_WRP001_WRONG_SCHEME: int = 1
  pure val MUT_WRP002_ALLOW_K22: int = 2
  pure val MUT_WRP003_IGNORE_ROW_GATE: int = 3
  pure val MUT_WRP004_WRITE_OVERSIZE_VK: int = 4
  pure val MUT_WRP006_REUSE_BLINDER: int = 5
  pure val MUT_WRP007_SKIP_SLOT15_PIN: int = 6
  pure val MUT_WRP008_IGNORE_EXPECT_VK: int = 7
  pure val MUT_WRP009_PARTIAL_OUTPUT: int = 8
  pure val MUT_WRP010_PER_PROGRAM_VK: int = 9
  pure val MUT_WRP011_DIVERGENT_GPU_VK: int = 10
  pure val MUT_WRP012_MISSING_LW004: int = 11
  pure val MUT_RKY003_ROW_OVERFLOW: int = 12
  pure val MUT_RKY004_VK_OVERFLOW: int = 13
  pure val MUT_DG002_K20_ROW_OVERFLOW: int = 14
  pure val MUT_ZK003_SEEDED_IN_PROD: int = 15
  pure val MUT_GPW003_DET_MISMATCH: int = 16
  pure val MUT_LW002_UNVERIFIED_LFR: int = 17
  pure val MUT_LW003_PUBLIC_LEAK: int = 18
  pure val MUT_LW004_BAD_FIELD_LOG: int = 19
  pure val MUT_LW005_SWAPPED_TAG: int = 20
  pure val MUT_LW006_X_MISMATCH: int = 21
  pure val MUT_LW014_SHORT_LOG_CAP: int = 22
  pure val MUT_LW016_ALL_NOP_QUAL: int = 23
  pure val MUT_ANC001_PLAIN_PROGD: int = 24

  // =========================================================================
  // Domain types
  // =========================================================================

  type CallMode = OrdMode | ChunkMode
  type DeviceKind = CpuDevice | GpuDevice
  type BuildMode = ProdBuild | TestSeedBuild
  type BlinderSource = OsCsprng | SeededDet | ConstantBad
  type AnchorTag = TagProgc | TagLock
  type ProofScheme = PlonkKzgBls12381 | NonPlonkScheme
  type TranscriptKind = Blake2bTranscript | NonBlake2bTranscript
  type CircuitArch = ZkStdLibArch | ForeignOrG4Arch

  type WrapKeyRecord = {
    vkId: int,
    circuitK: int,
    measuredRows: int,
    vkBytes: int,
    arch: CircuitArch,
    hasForeignOrG4Chip: bool,
    logCap: int,
    lAndLfrKeysMatchFrozen: bool,
    hasSecondFrFallback: bool
  }

  type KeygenAttemptRecord = {
    attemptId: int,
    requestedK: int,
    measuredRows: int,
    vkBytes: int,
    logCap: int,
    lAndLfrKeysMatchFrozen: bool,
    hasSecondFrFallback: bool,
    stoppedBeforeKeygen: bool,
    stoppedWithoutWrapVk: bool,
    stopMessage: str,
    keyWritten: bool
  }

  type LfrCallEvent = {
    callId: int,
    progId: int,
    callMode: CallMode,
    chunkIdx: int,
    chunkCount: int,
    lfrValidInFr: bool,
    lfrVkMatchesFrozen: bool,
    lfrTranscriptU: int,
    publicInputCount: int,
    lane0EqBind: bool,
    lane1EqComm: bool,
    unusedLanesZero: bool,
    leaksPrivateWord: bool,
    fieldLogValid: bool,
    popeqEqAIn: bool,
    anchorsFromDigestLog: bool,
    aInTag: AnchorTag,
    aOutTag: AnchorTag,
    layoutDEqKeyConst: bool,
    dagXCanonAndEqCached: bool,
    lane6ReadKey: int,
    hasChunkWritePrologue: bool,
    lane15WriteKey: int,
    anchorHoldsPlainProgD: bool,
    userAddressesSlot15: bool
  }

  type WrapProofRunRecord = {
    runId: int,
    callId: int,
    progId: int,
    callMode: CallMode,
    chunkIdx: int,
    chunkCount: int,
    device: DeviceKind,
    buildMode: BuildMode,
    gwDet: bool,
    detSeed: int,
    vkId: int,
    vkBytes: int,
    scheme: ProofScheme,
    transcript: TranscriptKind,
    blinderSource: BlinderSource,
    blinderId: int,
    proofBytesTag: int,
    proofCountInRun: int,
    libraryVerified: bool,
    expectVkSet: bool,
    expectedVkId: int,
    stoppedOnVkMismatch: bool,
    stoppedOnVkSize: bool,
    writtenFiles: Set[str],
    lfrValidInFr: bool,
    lfrVkMatchesFrozen: bool,
    lfrTranscriptU: int,
    publicInputCount: int,
    lane0EqBind: bool,
    lane1EqComm: bool,
    unusedLanesZero: bool,
    leaksPrivateWord: bool,
    fieldLogValid: bool,
    popeqEqAIn: bool,
    anchorsFromDigestLog: bool,
    aInTag: AnchorTag,
    aOutTag: AnchorTag,
    layoutDEqKeyConst: bool,
    dagXCanonAndEqCached: bool,
    lane6ReadKey: int,
    hasChunkWritePrologue: bool,
    lane15WriteKey: int,
    anchorHoldsPlainProgD: bool,
    userAddressesSlot15: bool
  }

  type ProgramDeployRecord = {
    progId: int,
    installedWriteVkId: int,
    readEntriesKeepNativeVk: bool,
    usesT25Key: bool
  }

  type WrapState = {
    eraKeyInitialized: bool,
    eraWrapVkSet: Set[int],
    cpuWrapKey: WrapKeyRecord,
    gpuWrapVkId: int,
    gpuConfigured: bool,
    usableGpuPresent: bool,
    expectVkSet: bool,
    expectedVkId: int,
    nextAttemptId: int,
    keygenAttempts: Set[KeygenAttemptRecord],
    lfrEvents: int -> LfrCallEvent,
    knownCallIds: Set[int],
    nextRunId: int,
    freshBlinderSeq: int,
    proofRuns: Set[WrapProofRunRecord],
    lastRefusalReason: str,
    deployments: Set[ProgramDeployRecord],
    rekeyCompleted: bool,
    rekeyMeasuredK: int,
    rekeyMeasuredRows: int,
    rekeyMeasuredVkBytes: int,
    rekeyLeverListWritten: bool,
    t25PassedChecks: Set[str],
    t25UsedAllNopBench: bool,
    t25Qualified: bool
  }

  // =========================================================================
  // Observable state variables
  // =========================================================================

  var wrp: WrapState
  var lastOp: str

  // =========================================================================
  // Pure constants & helper predicates
  // =========================================================================

  pure val WRAP_OUTPUT_FILES: Set[str] =
    Set("wrap.vk", "wrap.proof", "wrap.public.json")

  pure val REQUIRED_LW_CHECKS: Set[str] =
    Set("LW-003", "LW-004", "LW-005", "LW-006", "LW-014")

  pure val FOUR_LW_CHECKS_MISSING_LW004: Set[str] =
    Set("LW-003", "LW-005", "LW-006", "LW-014")

  pure val EMPTY_WRAP_KEY: WrapKeyRecord = {
    vkId: 0,
    circuitK: 0,
    measuredRows: 0,
    vkBytes: 0,
    arch: ZkStdLibArch,
    hasForeignOrG4Chip: false,
    logCap: 0,
    lAndLfrKeysMatchFrozen: false,
    hasSecondFrFallback: false
  }

  // DG-002: floor(0.85 * (2^k - 7)) usable row gate for k in {19, 20, 21}.
  // At k == MAX_WRAP_K, binds directly to MAX_ROWS_AT_K21 so edge instances scale cleanly.
  pure def usableRows85Pct(k: int): int = {
    if (k == MAX_WRAP_K) MAX_ROWS_AT_K21
    else if (k == 19) 445638
    else if (k == 20) 891283
    else if (k == 21) 1782573
    else 0
  }

  // Deterministic abstract proof-bytes hash over (callId, vkId, blinderId).
  pure def computeProofBytesTag(callId: int, vkId: int, blinderId: int): int =
    callId * 100000 + vkId * 1000 + blinderId

  // Expected LW-005 input/output anchor tags for (callMode, chunkIdx, chunkCount).
  pure def expectedAInTag(cm: CallMode, j: int): AnchorTag =
    if (cm == OrdMode or j == 1) TagProgc else TagLock

  pure def expectedAOutTag(cm: CallMode, j: int, kChunks: int): AnchorTag =
    if (cm == OrdMode or j == kChunks) TagProgc else TagLock

  // WRP-007 & ANC-016: anchor read key in lane 6 == 15 on every call;
  // chunk write prologue present iff ChunkMode, and when present lane 15 == 15.
  pure def validAnchorPrologue(ev: LfrCallEvent): bool =
    ev.lane6ReadKey == ANCHOR_SLOT_CONST and
    ((ev.callMode == ChunkMode) == ev.hasChunkWritePrologue) and
    (ev.hasChunkWritePrologue implies (ev.lane15WriteKey == ANCHOR_SLOT_CONST))

  // ANC-001: anchor cell holds salted C_P or lock A_j (never plain prog_d); user never addresses slot 15.
  pure def validAnc001(ev: LfrCallEvent): bool =
    not(ev.anchorHoldsPlainProgD) and not(ev.userAddressesSlot15)

  // LW-002: wrap verifies L_Fr natively in Fr under frozen L_Fr key and u = 7 transcript.
  pure def validLw002(ev: LfrCallEvent): bool =
    ev.lfrValidInFr and ev.lfrVkMatchesFrozen and ev.lfrTranscriptU == LFR_TRANSCRIPT_U

  // LW-003: call vector has 1,026 public inputs, lane 0 = binding_input, lane 1 = comm,
  // unused lanes constrained zero, no forbidden private word exposed.
  pure def validLw003(ev: LfrCallEvent): bool =
    ev.publicInputCount == CALL_VECTOR_PUBLIC_INPUTS and
    ev.lane0EqBind and
    ev.lane1EqComm and
    ev.unusedLanesZero and
    not(ev.leaksPrivateWord)

  // LW-004: native replay of field log passes Canon, quotients, squeezes, pv eq, padding, and R entries.
  pure def validLw004(ev: LfrCallEvent): bool =
    ev.fieldLogValid

  // LW-005: popeq == a_in, DIGEST entries, mode/chunk tag rules, lane 0 == bind, layout_d == key constant.
  pure def validLw005(ev: LfrCallEvent): bool =
    ev.popeqEqAIn and
    ev.anchorsFromDigestLog and
    ev.aInTag == expectedAInTag(ev.callMode, ev.chunkIdx) and
    ev.aOutTag == expectedAOutTag(ev.callMode, ev.chunkIdx, ev.chunkCount) and
    ev.lane0EqBind and
    ev.layoutDEqKeyConst

  // LW-006: X limbs canonical and X == cached-trace commitment of L.
  pure def validLw006(ev: LfrCallEvent): bool =
    ev.dagXCanonAndEqCached

  pure def validWrapWitness(ev: LfrCallEvent): bool =
    validAnchorPrologue(ev) and
    validAnc001(ev) and
    validLw002(ev) and
    validLw003(ev) and
    validLw004(ev) and
    validLw005(ev) and
    validLw006(ev)

  // Witness checks relaxed only for the targeted mutant.
  pure def witnessAcceptedUnderMutant(ev: LfrCallEvent): bool = {
    val okSlot15 = (MUTANT_ID == MUT_WRP007_SKIP_SLOT15_PIN) or validAnchorPrologue(ev)
    val okAnc001 = (MUTANT_ID == MUT_ANC001_PLAIN_PROGD) or validAnc001(ev)
    val okLw002 = (MUTANT_ID == MUT_LW002_UNVERIFIED_LFR) or validLw002(ev)
    val okLw003 = (MUTANT_ID == MUT_LW003_PUBLIC_LEAK) or validLw003(ev)
    val okLw004 = (MUTANT_ID == MUT_LW004_BAD_FIELD_LOG) or validLw004(ev)
    val okLw005 = (MUTANT_ID == MUT_LW005_SWAPPED_TAG) or validLw005(ev)
    val okLw006 = (MUTANT_ID == MUT_LW006_X_MISMATCH) or validLw006(ev)
    okSlot15 and okAnc001 and okLw002 and okLw003 and okLw004 and okLw005 and okLw006
  }

  // Guard predicates (exposed for scenario tests in wrap_test.qnt without .fail()).
  pure def canKeygenWrap(
    s: WrapState,
    reqK: int,
    rows: int,
    vkByteLen: int,
    logCap: int,
    frozenMatch: bool,
    secondFrFallback: bool
  ): bool = {
    val kOk =
      if (MUTANT_ID == MUT_WRP002_ALLOW_K22) (reqK >= MIN_WRAP_K and reqK <= MAX_WRAP_K + 1)
      else (reqK >= MIN_WRAP_K and reqK <= MAX_WRAP_K)
    val rowOk =
      if (MUTANT_ID == MUT_WRP002_ALLOW_K22 and reqK == MAX_WRAP_K + 1)
        (rows > 0 and rows <= 3565152)
      else if (MUTANT_ID == MUT_WRP003_IGNORE_ROW_GATE)
        (reqK == MAX_WRAP_K or rows <= usableRows85Pct(reqK))
      else if (MUTANT_ID == MUT_DG002_K20_ROW_OVERFLOW)
        (reqK == MAX_WRAP_K implies rows <= MAX_ROWS_AT_K21)
      else
        (rows > 0 and rows <= usableRows85Pct(reqK))
    val vkOk =
      if (MUTANT_ID == MUT_WRP004_WRITE_OVERSIZE_VK) (vkByteLen > 0)
      else (vkByteLen > 0 and vkByteLen <= MAX_VK_BYTES)
    val lw014Ok =
      if (MUTANT_ID == MUT_LW014_SHORT_LOG_CAP) true
      else (logCap >= MIN_LOG_RESERVATION_R and frozenMatch and not(secondFrFallback))
    not(s.eraKeyInitialized) and s.nextAttemptId <= 6 and kOk and rowOk and vkOk and lw014Ok
  }

  pure def canProveHonestKeyGates(s: WrapState, vkByteLen: int, runVkId: int): bool = {
    val vkSizeOk = (MUTANT_ID == MUT_WRP004_WRITE_OVERSIZE_VK) or (vkByteLen <= MAX_VK_BYTES)
    val expectVkOk =
      (MUTANT_ID == MUT_WRP008_IGNORE_EXPECT_VK) or
      not(s.expectVkSet) or
      (runVkId == s.expectedVkId)
    s.eraKeyInitialized and s.nextRunId <= 8 and vkSizeOk and expectVkOk
  }

  pure def canProveCall(s: WrapState, callId: int): bool =
    s.knownCallIds.contains(callId) and
    canProveHonestKeyGates(s, s.cpuWrapKey.vkBytes, s.cpuWrapKey.vkId) and
    witnessAcceptedUnderMutant(s.lfrEvents.get(callId))

  pure def canProveSeeded(s: WrapState, callId: int, bMode: BuildMode, gwDetFlag: bool): bool = {
    val buildOk =
      if (MUTANT_ID == MUT_ZK003_SEEDED_IN_PROD) gwDetFlag
      else (bMode == TestSeedBuild and gwDetFlag)
    canProveCall(s, callId) and buildOk
  }

  pure def canCloseT25Qualification(s: WrapState): bool = {
    val checksOk =
      if (MUTANT_ID == MUT_WRP012_MISSING_LW004)
        FOUR_LW_CHECKS_MISSING_LW004.subseteq(s.t25PassedChecks)
      else
        (REQUIRED_LW_CHECKS.subseteq(s.t25PassedChecks) and
          s.t25PassedChecks.size() >= LW_QUAL_CHECK_COUNT)
    val benchOk =
      (MUTANT_ID == MUT_LW016_ALL_NOP_QUAL) or not(s.t25UsedAllNopBench)
    s.eraKeyInitialized and not(s.t25Qualified) and checksOk and benchOk
  }

  pure def canCompleteRekey(s: WrapState, rkK: int, rkRows: int, rkVkBytes: int): bool = {
    val rowsOk =
      (MUTANT_ID == MUT_RKY003_ROW_OVERFLOW) or (rkRows > 0 and rkRows <= MAX_ROWS_AT_K21)
    val vkOk =
      (MUTANT_ID == MUT_RKY004_VK_OVERFLOW) or (rkVkBytes > 0 and rkVkBytes <= MAX_VK_BYTES)
    not(s.rekeyCompleted) and rkK == MAX_WRAP_K and rowsOk and vkOk
  }

  // Build an honest LfrProofReady event for OrdMode or ChunkMode.
  pure def mkHonestLfrEvent(cId: int, pId: int, cm: CallMode, jIdx: int, kCnt: int): LfrCallEvent = {
    callId: cId,
    progId: pId,
    callMode: cm,
    chunkIdx: jIdx,
    chunkCount: kCnt,
    lfrValidInFr: true,
    lfrVkMatchesFrozen: true,
    lfrTranscriptU: LFR_TRANSCRIPT_U,
    publicInputCount: CALL_VECTOR_PUBLIC_INPUTS,
    lane0EqBind: true,
    lane1EqComm: true,
    unusedLanesZero: true,
    leaksPrivateWord: false,
    fieldLogValid: true,
    popeqEqAIn: true,
    anchorsFromDigestLog: true,
    aInTag: expectedAInTag(cm, jIdx),
    aOutTag: expectedAOutTag(cm, jIdx, kCnt),
    layoutDEqKeyConst: true,
    dagXCanonAndEqCached: true,
    lane6ReadKey: ANCHOR_SLOT_CONST,
    hasChunkWritePrologue: (cm == ChunkMode),
    lane15WriteKey: if (cm == ChunkMode) ANCHOR_SLOT_CONST else 0,
    anchorHoldsPlainProgD: false,
    userAddressesSlot15: false
  }

  // Build a WrapProofRunRecord from an event and execution parameters.
  pure def mkProofRunRecord(
    rId: int,
    ev: LfrCallEvent,
    dev: DeviceKind,
    bMode: BuildMode,
    gwDetFlag: bool,
    dSeed: int,
    vkIdent: int,
    vkByteLen: int,
    sch: ProofScheme,
    trKind: TranscriptKind,
    bSrc: BlinderSource,
    bId: int,
    pBytesTag: int,
    pCount: int,
    libVer: bool,
    expSet: bool,
    expVk: int,
    stopMismatch: bool,
    stopSize: bool,
    outFiles: Set[str]
  ): WrapProofRunRecord = {
    runId: rId,
    callId: ev.callId,
    progId: ev.progId,
    callMode: ev.callMode,
    chunkIdx: ev.chunkIdx,
    chunkCount: ev.chunkCount,
    device: dev,
    buildMode: bMode,
    gwDet: gwDetFlag,
    detSeed: dSeed,
    vkId: vkIdent,
    vkBytes: vkByteLen,
    scheme: sch,
    transcript: trKind,
    blinderSource: bSrc,
    blinderId: bId,
    proofBytesTag: pBytesTag,
    proofCountInRun: pCount,
    libraryVerified: libVer,
    expectVkSet: expSet,
    expectedVkId: expVk,
    stoppedOnVkMismatch: stopMismatch,
    stoppedOnVkSize: stopSize,
    writtenFiles: outFiles,
    lfrValidInFr: ev.lfrValidInFr,
    lfrVkMatchesFrozen: ev.lfrVkMatchesFrozen,
    lfrTranscriptU: ev.lfrTranscriptU,
    publicInputCount: ev.publicInputCount,
    lane0EqBind: ev.lane0EqBind,
    lane1EqComm: ev.lane1EqComm,
    unusedLanesZero: ev.unusedLanesZero,
    leaksPrivateWord: ev.leaksPrivateWord,
    fieldLogValid: ev.fieldLogValid,
    popeqEqAIn: ev.popeqEqAIn,
    anchorsFromDigestLog: ev.anchorsFromDigestLog,
    aInTag: ev.aInTag,
    aOutTag: ev.aOutTag,
    layoutDEqKeyConst: ev.layoutDEqKeyConst,
    dagXCanonAndEqCached: ev.dagXCanonAndEqCached,
    lane6ReadKey: ev.lane6ReadKey,
    hasChunkWritePrologue: ev.hasChunkWritePrologue,
    lane15WriteKey: ev.lane15WriteKey,
    anchorHoldsPlainProgD: ev.anchorHoldsPlainProgD,
    userAddressesSlot15: ev.userAddressesSlot15
  }

  // =========================================================================
  // Consistency checks & explicit spec-gap witness predicates
  // =========================================================================

  // Pure check that instance bounds are mutually consistent with MID-003, DG-002, DG-003, WRP-002..012.
  val consistency_constants: bool =
    MIN_WRAP_K >= 19 and
    MIN_WRAP_K <= MAX_WRAP_K and
    MAX_WRAP_K <= 21 and
    MAX_ROWS_AT_K21 > 0 and
    MAX_ROWS_AT_K21 <= 1782573 and
    MAX_VK_BYTES >= 1325 and
    MAX_VK_BYTES <= 50000 and
    CALL_VECTOR_PUBLIC_INPUTS == 1026 and
    CALL_VECTOR_PUBLIC_INPUTS <= 16384 and
    CALL_VECTOR_PUBLIC_INPUTS <= 65536 and
    ANCHOR_SLOT_CONST == 15 and
    ANCHOR_READ_LANE == 6 and
    CHUNK_WRITE_LANE == 15 and
    WRAP_OUTPUT_FILE_COUNT == 3 and
    WRAP_OUTPUT_FILES.size() == WRAP_OUTPUT_FILE_COUNT and
    LW_QUAL_CHECK_COUNT == 5 and
    REQUIRED_LW_CHECKS.size() == LW_QUAL_CHECK_COUNT and
    LFR_TRANSCRIPT_U == 7 and
    MIN_LOG_RESERVATION_R >= 1

  // Finding Q-wrap-1 witness: k = 20 with T10m measured rows = 1,685,845 satisfies literal WRP-002
  // (20 <= 21) and skips literal WRP-003 (k != 21), yet overflows DG-002 (891,283) and 2^20 - 7 (1,048,569).
  pure val gap_q_wrap_1_k20_row_overflow: bool = {
    val sampleK = 20
    val t10mRows = 1685845
    val passesWrp002 = (sampleK <= 21)
    val passesLiteralWrp003 = not(sampleK == 21 and t10mRows > 1782573)
    val violatesDg002 = (t10mRows > 891283)
    passesWrp002 and passesLiteralWrp003 and violatesDg002
  }

  // Finding Q-wrap-3 witness: an honest OrdMode call has lane6ReadKey == 15 and unused lane15WriteKey == 0
  // (LW-003, ANC-016), which violates literal unconditioned WRP-007 (lane6 == 15 and lane15 == 15).
  val gap_q_wrap_3_ord_lane15_zero: bool = {
    val ordEv = mkHonestLfrEvent(1, 1, OrdMode, 1, 1)
    validAnchorPrologue(ordEv) and
      validLw003(ordEv) and
      not(ordEv.lane6ReadKey == ANCHOR_SLOT_CONST and ordEv.lane15WriteKey == ANCHOR_SLOT_CONST)
  }

  // =========================================================================
  // Initialization
  // =========================================================================

  action init = all {
    wrp' = {
      eraKeyInitialized: false,
      eraWrapVkSet: Set(),
      cpuWrapKey: EMPTY_WRAP_KEY,
      gpuWrapVkId: 0,
      gpuConfigured: false,
      usableGpuPresent: true,
      expectVkSet: false,
      expectedVkId: 0,
      nextAttemptId: 1,
      keygenAttempts: Set(),
      lfrEvents: Map(),
      knownCallIds: Set(),
      nextRunId: 1,
      freshBlinderSeq: 1,
      proofRuns: Set(),
      lastRefusalReason: "none",
      deployments: Set(),
      rekeyCompleted: false,
      rekeyMeasuredK: 0,
      rekeyMeasuredRows: 0,
      rekeyMeasuredVkBytes: 0,
      rekeyLeverListWritten: false,
      t25PassedChecks: Set(),
      t25UsedAllNopBench: false,
      t25Qualified: false
    },
    lastOp' = "init"
  }

  // =========================================================================
  // Parameterized actions (used by step and deterministic tests)
  // =========================================================================

  // Implements WRP-002, WRP-003, WRP-004, WRP-010, DG-002, DG-003, LW-013, LW-014.
  action doKeygenWrap(
    reqK: int,
    rows: int,
    vkByteLen: int,
    logCap: int,
    frozenMatch: bool,
    secondFrFallback: bool
  ): bool = all {
    canKeygenWrap(wrp, reqK, rows, vkByteLen, logCap, frozenMatch, secondFrFallback),
    val keyRec: WrapKeyRecord = {
      vkId: 1,
      circuitK: reqK,
      measuredRows: rows,
      vkBytes: vkByteLen,
      arch: ZkStdLibArch,
      hasForeignOrG4Chip: false,
      logCap: logCap,
      lAndLfrKeysMatchFrozen: frozenMatch,
      hasSecondFrFallback: secondFrFallback
    }
    val attemptRec: KeygenAttemptRecord = {
      attemptId: wrp.nextAttemptId,
      requestedK: reqK,
      measuredRows: rows,
      vkBytes: vkByteLen,
      logCap: logCap,
      lAndLfrKeysMatchFrozen: frozenMatch,
      hasSecondFrFallback: secondFrFallback,
      stoppedBeforeKeygen: false,
      stoppedWithoutWrapVk: false,
      stopMessage: "ok",
      keyWritten: true
    }
    wrp' = {
      ...wrp,
      eraKeyInitialized: true,
      eraWrapVkSet: Set(1),
      cpuWrapKey: keyRec,
      nextAttemptId: wrp.nextAttemptId + 1,
      keygenAttempts: wrp.keygenAttempts.union(Set(attemptRec))
    },
    lastOp' = "keygenWrap"
  }

  // Implements WRP-002: stop before keygen with message "never k = 22" when requestedK > 21.
  action doRefuseKeygenAboveK21(reqK: int): bool = all {
    reqK > MAX_WRAP_K,
    wrp.nextAttemptId <= 6,
    val attemptRec: KeygenAttemptRecord = {
      attemptId: wrp.nextAttemptId,
      requestedK: reqK,
      measuredRows: 0,
      vkBytes: 0,
      logCap: MIN_LOG_RESERVATION_R,
      lAndLfrKeysMatchFrozen: true,
      hasSecondFrFallback: false,
      stoppedBeforeKeygen: true,
      stoppedWithoutWrapVk: true,
      stopMessage: "never k = 22",
      keyWritten: false
    }
    wrp' = {
      ...wrp,
      nextAttemptId: wrp.nextAttemptId + 1,
      keygenAttempts: wrp.keygenAttempts.union(Set(attemptRec)),
      lastRefusalReason: "never k = 22"
    },
    lastOp' = "refuseKeygenAboveK21"
  }

  // Implements WRP-003, DG-002: stop before keygen when cost model exceeds usable row gate.
  action doRefuseKeygenRowGate(reqK: int, rows: int): bool = all {
    reqK >= MIN_WRAP_K,
    reqK <= MAX_WRAP_K,
    rows > usableRows85Pct(reqK),
    wrp.nextAttemptId <= 6,
    val attemptRec: KeygenAttemptRecord = {
      attemptId: wrp.nextAttemptId,
      requestedK: reqK,
      measuredRows: rows,
      vkBytes: 0,
      logCap: MIN_LOG_RESERVATION_R,
      lAndLfrKeysMatchFrozen: true,
      hasSecondFrFallback: false,
      stoppedBeforeKeygen: true,
      stoppedWithoutWrapVk: true,
      stopMessage: "row_gate_exceeded",
      keyWritten: false
    }
    wrp' = {
      ...wrp,
      nextAttemptId: wrp.nextAttemptId + 1,
      keygenAttempts: wrp.keygenAttempts.union(Set(attemptRec)),
      lastRefusalReason: "row_gate_exceeded"
    },
    lastOp' = "refuseKeygenRowGate"
  }

  // Implements WRP-004, DG-003, MID-003: stop without writing wrap.vk when VK > 50,000 bytes.
  action doRefuseVkTooLarge(vkByteLen: int): bool = all {
    vkByteLen > MAX_VK_BYTES,
    wrp.nextAttemptId <= 6,
    val attemptRec: KeygenAttemptRecord = {
      attemptId: wrp.nextAttemptId,
      requestedK: MAX_WRAP_K,
      measuredRows: usableRows85Pct(MAX_WRAP_K),
      vkBytes: vkByteLen,
      logCap: MIN_LOG_RESERVATION_R,
      lAndLfrKeysMatchFrozen: true,
      hasSecondFrFallback: false,
      stoppedBeforeKeygen: false,
      stoppedWithoutWrapVk: true,
      stopMessage: "vk_size_exceeded",
      keyWritten: false
    }
    wrp' = {
      ...wrp,
      nextAttemptId: wrp.nextAttemptId + 1,
      keygenAttempts: wrp.keygenAttempts.union(Set(attemptRec)),
      lastRefusalReason: "vk_size_exceeded"
    },
    lastOp' = "refuseVkTooLarge"
  }

  // Implements LW-014: keygen refuses insufficient log capacity, frozen key drift, or second Fr fallback.
  action doRefuseKeygenLw014(logCap: int, frozenMatch: bool, secondFrFallback: bool): bool = all {
    logCap < MIN_LOG_RESERVATION_R or not(frozenMatch) or secondFrFallback,
    wrp.nextAttemptId <= 6,
    val attemptRec: KeygenAttemptRecord = {
      attemptId: wrp.nextAttemptId,
      requestedK: MAX_WRAP_K,
      measuredRows: usableRows85Pct(MAX_WRAP_K),
      vkBytes: 1325,
      logCap: logCap,
      lAndLfrKeysMatchFrozen: frozenMatch,
      hasSecondFrFallback: secondFrFallback,
      stoppedBeforeKeygen: true,
      stoppedWithoutWrapVk: true,
      stopMessage: "lw014_keygen_refused",
      keyWritten: false
    }
    wrp' = {
      ...wrp,
      nextAttemptId: wrp.nextAttemptId + 1,
      keygenAttempts: wrp.keygenAttempts.union(Set(attemptRec)),
      lastRefusalReason: "lw014_keygen_refused"
    },
    lastOp' = "refuseKeygenLw014"
  }

  // Implements WRP-011, GPW-001: configure GPU prover with VK byte-equal to CPU wrap VK (or toggle GPU availability).
  action doConfigureGpuProver(gpuAvail: bool, mutUseDivergentVk: bool): bool = all {
    wrp.eraKeyInitialized,
    val assignedGpuVk =
      if (MUTANT_ID == MUT_WRP011_DIVERGENT_GPU_VK and mutUseDivergentVk)
        wrp.cpuWrapKey.vkId + 10
      else
        wrp.cpuWrapKey.vkId
    wrp' = {
      ...wrp,
      usableGpuPresent: gpuAvail,
      gpuConfigured: gpuAvail,
      gpuWrapVkId: if (gpuAvail) assignedGpuVk else 0
    },
    lastOp' = "configureGpuProver"
  }

  // Implements WRP-008: set or update E6_EXPECT_VK key pin.
  action doConfigureExpectVk(pinActive: bool, pinnedVkId: int): bool = all {
    wrp' = {
      ...wrp,
      expectVkSet: pinActive,
      expectedVkId: if (pinActive) pinnedVkId else 0
    },
    lastOp' = "configureExpectVk"
  }

  // Implements WRP-001, ANC-001, LW-002..006: receive an honest LfrProofReady event.
  action doReceiveHonestLfrCall(cId: int, pId: int, cm: CallMode, jIdx: int, kCnt: int): bool = all {
    not(wrp.knownCallIds.contains(cId)),
    val ev = mkHonestLfrEvent(cId, pId, cm, jIdx, kCnt)
    wrp' = {
      ...wrp,
      lfrEvents: wrp.lfrEvents.put(cId, ev),
      knownCallIds: wrp.knownCallIds.union(Set(cId))
    },
    lastOp' = "receiveHonestLfrCall"
  }

  // Implements WRP-007, ANC-001, LW-002..006: receive a forged/malformed LfrProofReady event.
  action doReceiveForgedLfrCall(cId: int, forgeryKind: int): bool = all {
    not(wrp.knownCallIds.contains(cId)),
    val base = mkHonestLfrEvent(cId, 1, ChunkMode, 1, 2)
    val forgedEv: LfrCallEvent =
      if (forgeryKind == 1) { ...base, lane6ReadKey: 3, lane15WriteKey: 1 }
      else if (forgeryKind == 2) {
        ...mkHonestLfrEvent(cId, 1, OrdMode, 1, 1),
        hasChunkWritePrologue: true,
        lane15WriteKey: ANCHOR_SLOT_CONST
      }
      else if (forgeryKind == 3) { ...base, lfrValidInFr: false, lfrTranscriptU: 5 }
      else if (forgeryKind == 4) { ...base, leaksPrivateWord: true, unusedLanesZero: false }
      else if (forgeryKind == 5) { ...base, fieldLogValid: false }
      else if (forgeryKind == 6) { ...mkHonestLfrEvent(cId, 1, OrdMode, 1, 1), aInTag: TagLock }
      else if (forgeryKind == 7) { ...base, dagXCanonAndEqCached: false }
      else { ...base, anchorHoldsPlainProgD: true, userAddressesSlot15: true }
    wrp' = {
      ...wrp,
      lfrEvents: wrp.lfrEvents.put(cId, forgedEv),
      knownCallIds: wrp.knownCallIds.union(Set(cId))
    },
    lastOp' = "receiveForgedLfrCall"
  }

  // Implements WRP-001, WRP-004, WRP-006, WRP-007, WRP-008, WRP-009, GPW-001, LW-012, ZK-003:
  // CPU wrap proving in a production build (also serves as fallback when no GPU is present).
  action doProveWrapCpu(cId: int, mutTrigger: bool): bool = all {
    canProveCall(wrp, cId),
    val ev = wrp.lfrEvents.get(cId)
    val usedScheme =
      if (MUTANT_ID == MUT_WRP001_WRONG_SCHEME and mutTrigger) NonPlonkScheme
      else PlonkKzgBls12381
    val usedProofCount =
      if (MUTANT_ID == MUT_WRP001_WRONG_SCHEME and mutTrigger) 2 else 1
    val usedBlinderSrc =
      if (MUTANT_ID == MUT_WRP006_REUSE_BLINDER and mutTrigger) ConstantBad else OsCsprng
    val usedBlinderId =
      if (MUTANT_ID == MUT_WRP006_REUSE_BLINDER and mutTrigger) 0 else wrp.freshBlinderSeq
    val nextSeq =
      if (MUTANT_ID == MUT_WRP006_REUSE_BLINDER and mutTrigger) wrp.freshBlinderSeq
      else wrp.freshBlinderSeq + 1
    val outFiles =
      if (MUTANT_ID == MUT_WRP009_PARTIAL_OUTPUT and mutTrigger)
        Set("wrap.vk", "wrap.proof")
      else
        WRAP_OUTPUT_FILES
    val pTag = computeProofBytesTag(cId, wrp.cpuWrapKey.vkId, usedBlinderId)
    val rec = mkProofRunRecord(
      wrp.nextRunId,
      ev,
      CpuDevice,
      ProdBuild,
      false,
      0,
      wrp.cpuWrapKey.vkId,
      wrp.cpuWrapKey.vkBytes,
      usedScheme,
      Blake2bTranscript,
      usedBlinderSrc,
      usedBlinderId,
      pTag,
      usedProofCount,
      true,
      wrp.expectVkSet,
      wrp.expectedVkId,
      false,
      false,
      outFiles
    )
    wrp' = {
      ...wrp,
      nextRunId: wrp.nextRunId + 1,
      freshBlinderSeq: nextSeq,
      proofRuns: wrp.proofRuns.union(Set(rec))
    },
    lastOp' = "proveWrapCpu"
  }

  // Implements WRP-001, WRP-006, WRP-009, WRP-011, GPW-001:
  // GPU wrap proving in a production build using gpuWrapVkId == cpuWrapKey.vkId.
  action doProveWrapGpu(cId: int): bool = all {
    wrp.gpuConfigured,
    wrp.usableGpuPresent,
    wrp.knownCallIds.contains(cId),
    canProveHonestKeyGates(wrp, wrp.cpuWrapKey.vkBytes, wrp.gpuWrapVkId),
    witnessAcceptedUnderMutant(wrp.lfrEvents.get(cId)),
    val ev = wrp.lfrEvents.get(cId)
    val bId = wrp.freshBlinderSeq
    val pTag = computeProofBytesTag(cId, wrp.gpuWrapVkId, bId)
    val rec = mkProofRunRecord(
      wrp.nextRunId,
      ev,
      GpuDevice,
      ProdBuild,
      false,
      0,
      wrp.gpuWrapVkId,
      wrp.cpuWrapKey.vkBytes,
      PlonkKzgBls12381,
      Blake2bTranscript,
      OsCsprng,
      bId,
      pTag,
      1,
      true,
      wrp.expectVkSet,
      wrp.expectedVkId,
      false,
      false,
      WRAP_OUTPUT_FILES
    )
    wrp' = {
      ...wrp,
      nextRunId: wrp.nextRunId + 1,
      freshBlinderSeq: wrp.freshBlinderSeq + 1,
      proofRuns: wrp.proofRuns.union(Set(rec))
    },
    lastOp' = "proveWrapGpu"
  }

  // Implements GPW-001, GPW-003, LW-012, ZK-003:
  // Deterministic seeded wrap proving under GW_DET=1 in a test-seed build.
  action doProveSeededDetWrap(cId: int, dev: DeviceKind, bMode: BuildMode, dSeed: int): bool = all {
    dev == GpuDevice implies (wrp.gpuConfigured and wrp.usableGpuPresent),
    canProveSeeded(wrp, cId, bMode, true),
    val ev = wrp.lfrEvents.get(cId)
    val usedVkId = if (dev == GpuDevice) wrp.gpuWrapVkId else wrp.cpuWrapKey.vkId
    val seededBlinderId = 500 + dSeed
    val baseTag = computeProofBytesTag(cId, usedVkId, seededBlinderId)
    val finalTag =
      if (MUTANT_ID == MUT_GPW003_DET_MISMATCH and dev == GpuDevice) baseTag + 777
      else baseTag
    val rec = mkProofRunRecord(
      wrp.nextRunId,
      ev,
      dev,
      bMode,
      true,
      dSeed,
      usedVkId,
      wrp.cpuWrapKey.vkBytes,
      PlonkKzgBls12381,
      Blake2bTranscript,
      SeededDet,
      seededBlinderId,
      finalTag,
      1,
      true,
      wrp.expectVkSet,
      wrp.expectedVkId,
      false,
      false,
      WRAP_OUTPUT_FILES
    )
    wrp' = {
      ...wrp,
      nextRunId: wrp.nextRunId + 1,
      proofRuns: wrp.proofRuns.union(Set(rec))
    },
    lastOp' = "proveSeededDetWrap"
  }

  // Implements ZK-003, LW-012, WRP-006: seeded blinder refused in production build.
  action doRefuseSeededBlinderInProd(cId: int): bool = all {
    wrp.eraKeyInitialized,
    wrp.knownCallIds.contains(cId),
    wrp' = {
      ...wrp,
      lastRefusalReason: "seeded_blinder_refused_in_prod"
    },
    lastOp' = "refuseSeededBlinderInProd"
  }

  // Implements WRP-007, ANC-001, LW-002..006: refuse malformed/forged wrap witness.
  action doRefuseInvalidWrapWitness(cId: int): bool = all {
    wrp.eraKeyInitialized,
    wrp.knownCallIds.contains(cId),
    not(validWrapWitness(wrp.lfrEvents.get(cId))),
    wrp' = {
      ...wrp,
      lastRefusalReason: "witness_refused"
    },
    lastOp' = "refuseInvalidWrapWitness"
  }

  // Implements WRP-008: stop with "the wrap key differs" before writing any output file.
  action doRefuseExpectVkMismatch(cId: int): bool = all {
    wrp.eraKeyInitialized,
    wrp.knownCallIds.contains(cId),
    wrp.expectVkSet,
    wrp.cpuWrapKey.vkId != wrp.expectedVkId,
    wrp.nextRunId <= 8,
    val ev = wrp.lfrEvents.get(cId)
    val rec = mkProofRunRecord(
      wrp.nextRunId,
      ev,
      CpuDevice,
      ProdBuild,
      false,
      0,
      wrp.cpuWrapKey.vkId,
      wrp.cpuWrapKey.vkBytes,
      PlonkKzgBls12381,
      Blake2bTranscript,
      OsCsprng,
      0,
      0,
      0,
      false,
      wrp.expectVkSet,
      wrp.expectedVkId,
      true,
      false,
      Set()
    )
    wrp' = {
      ...wrp,
      nextRunId: wrp.nextRunId + 1,
      proofRuns: wrp.proofRuns.union(Set(rec)),
      lastRefusalReason: "the wrap key differs"
    },
    lastOp' = "refuseExpectVkMismatch"
  }

  // Implements WRP-010, WRP-012, MID-002: deploy a program's SherlockLedger with the era's universal wrap VK.
  action doDeployProgramLedger(pId: int, forT25: bool, mutUseOtherVk: bool): bool = all {
    wrp.eraKeyInitialized,
    forT25 implies wrp.t25Qualified,
    not(wrp.deployments.exists(d => d.progId == pId)),
    val chosenVkId =
      if (MUTANT_ID == MUT_WRP010_PER_PROGRAM_VK and mutUseOtherVk) wrp.cpuWrapKey.vkId + 1
      else wrp.cpuWrapKey.vkId
    val depRec: ProgramDeployRecord = {
      progId: pId,
      installedWriteVkId: chosenVkId,
      readEntriesKeepNativeVk: true,
      usesT25Key: forT25
    }
    wrp' = {
      ...wrp,
      eraWrapVkSet: wrp.eraWrapVkSet.union(Set(chosenVkId)),
      deployments: wrp.deployments.union(Set(depRec))
    },
    lastOp' = "deployProgramLedger"
  }

  // Implements RKY-003, RKY-004: measure wrap rows at k = 21 and wrap VK byte length upon rekey completion.
  action doCompleteRekeyWrapMeasurement(rkRows: int, rkVkBytes: int): bool = all {
    canCompleteRekey(wrp, MAX_WRAP_K, rkRows, rkVkBytes),
    wrp' = {
      ...wrp,
      rekeyCompleted: true,
      rekeyMeasuredK: MAX_WRAP_K,
      rekeyMeasuredRows: rkRows,
      rekeyMeasuredVkBytes: rkVkBytes
    },
    lastOp' = "completeRekeyWrapMeasurement"
  }

  // Implements RKY-003, RKY-004: stop rekey and write lever list when rows or VK bytes exceed bounds.
  action doRefuseRekeyBoundsOverflow(rkRows: int, rkVkBytes: int): bool = all {
    not(wrp.rekeyCompleted),
    rkRows > MAX_ROWS_AT_K21 or rkVkBytes > MAX_VK_BYTES,
    wrp' = {
      ...wrp,
      rekeyLeverListWritten: true,
      lastRefusalReason: "rekey_bounds_exceeded"
    },
    lastOp' = "refuseRekeyBoundsOverflow"
  }

  // Implements WRP-012, LW-016: record T25 qualification passes (all 5 checks, 4 checks missing LW-004, or all-NOP bench).
  action doRecordT25LwPasses(includeLw004: bool, fromAllNopBench: bool): bool = all {
    wrp.eraKeyInitialized,
    not(wrp.t25Qualified),
    val allowAllNop = (MUTANT_ID == MUT_LW016_ALL_NOP_QUAL) or not(fromAllNopBench)
    all {
      allowAllNop,
      val newChecks =
        if (includeLw004) REQUIRED_LW_CHECKS else FOUR_LW_CHECKS_MISSING_LW004
      wrp' = {
        ...wrp,
        t25PassedChecks: wrp.t25PassedChecks.union(newChecks),
        t25UsedAllNopBench: wrp.t25UsedAllNopBench or fromAllNopBench
      },
      lastOp' = "recordT25LwQualificationPasses"
    }
  }

  // Implements WRP-012, LW-016: close the T25 qualification record once all 5 LW checks hold real passes.
  action doCloseT25Qualification: bool = all {
    canCloseT25Qualification(wrp),
    wrp' = {
      ...wrp,
      t25Qualified: true
    },
    lastOp' = "closeT25Qualification"
  }

  // Implements WRP-012, LW-016: refuse incomplete T25 qualification or all-NOP benchmark offer.
  action doRefuseIncompleteT25Qualification(offeredAllNopBench: bool): bool = all {
    wrp.eraKeyInitialized,
    not(wrp.t25Qualified),
    offeredAllNopBench or not(REQUIRED_LW_CHECKS.subseteq(wrp.t25PassedChecks)),
    wrp' = {
      ...wrp,
      lastRefusalReason:
        if (offeredAllNopBench) "all_nop_bench_qualifies_no_gate"
        else "t25_qualification_incomplete"
    },
    lastOp' = "refuseIncompleteT25Qualification"
  }

  // =========================================================================
  // Nondeterministic step relation
  // =========================================================================

  action step = any {
    // Keygen & keygen refusals (WRP-002, WRP-003, WRP-004, DG-002, LW-014)
    {
      nondet reqK = Set(19, 20, 21, 22).oneOf()
      nondet rows = Set(400000, 1685845, 1782573, 1782574).oneOf()
      nondet vkB = Set(1325, 50000, 50001).oneOf()
      val logCandidates =
        if (MUTANT_ID == MUT_LW014_SHORT_LOG_CAP) Set(0, MIN_LOG_RESERVATION_R)
        else Set(MIN_LOG_RESERVATION_R)
      nondet logC = logCandidates.oneOf()
      doKeygenWrap(reqK, rows, vkB, logC, true, false)
    },
    doRefuseKeygenAboveK21(MAX_WRAP_K + 1),
    doRefuseKeygenRowGate(MAX_WRAP_K, MAX_ROWS_AT_K21 + 1),
    doRefuseVkTooLarge(MAX_VK_BYTES + 1),
    doRefuseKeygenLw014(MIN_LOG_RESERVATION_R - 1, false, false),

    // GPU & E6_EXPECT_VK configuration (WRP-008, WRP-011, GPW-001)
    {
      nondet gpuAvail = Set(true, false).oneOf()
      nondet mutDiv = Set(true, false).oneOf()
      doConfigureGpuProver(gpuAvail, mutDiv)
    },
    {
      nondet pinOn = Set(true, false).oneOf()
      nondet pinVk = Set(1, 99).oneOf()
      doConfigureExpectVk(pinOn, pinVk)
    },

    // Incoming LfrProofReady events (honest OrdMode & ChunkMode, plus forged calls)
    {
      nondet cId = Set(1, 2).oneOf()
      nondet pId = Set(1, 2).oneOf()
      nondet cm = Set(OrdMode, ChunkMode).oneOf()
      val kCnt = if (cm == ChunkMode) 2 else 1
      nondet jIdx = Set(1, 2).oneOf()
      val normJ = if (cm == OrdMode) 1 else jIdx
      doReceiveHonestLfrCall(cId, pId, cm, normJ, kCnt)
    },
    {
      val preferredForgery =
        if (MUTANT_ID == MUT_WRP007_SKIP_SLOT15_PIN) Set(1, 2)
        else if (MUTANT_ID == MUT_LW002_UNVERIFIED_LFR) Set(3)
        else if (MUTANT_ID == MUT_LW003_PUBLIC_LEAK) Set(4)
        else if (MUTANT_ID == MUT_LW004_BAD_FIELD_LOG) Set(5)
        else if (MUTANT_ID == MUT_LW005_SWAPPED_TAG) Set(6)
        else if (MUTANT_ID == MUT_LW006_X_MISMATCH) Set(7)
        else if (MUTANT_ID == MUT_ANC001_PLAIN_PROGD) Set(8)
        else Set(1, 2, 3, 4, 5, 6, 7, 8)
      nondet fKind = preferredForgery.oneOf()
      doReceiveForgedLfrCall(3, fKind)
    },

    // Proving & proving refusals (WRP-001, WRP-006, WRP-007, WRP-008, WRP-009, WRP-011, GPW-003, ZK-003)
    {
      nondet cId = Set(1, 2, 3).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doProveWrapCpu(cId, mutTrig)
    },
    {
      nondet cId = Set(1, 2, 3).oneOf()
      doProveWrapGpu(cId)
    },
    {
      nondet cId = Set(1, 2).oneOf()
      nondet dev = Set(CpuDevice, GpuDevice).oneOf()
      val buildModes =
        if (MUTANT_ID == MUT_ZK003_SEEDED_IN_PROD) Set(ProdBuild, TestSeedBuild)
        else Set(TestSeedBuild)
      nondet bMode = buildModes.oneOf()
      doProveSeededDetWrap(cId, dev, bMode, 42)
    },
    {
      nondet cId = Set(1, 2).oneOf()
      doRefuseSeededBlinderInProd(cId)
    },
    doRefuseInvalidWrapWitness(3),
    {
      nondet cId = Set(1, 2).oneOf()
      doRefuseExpectVkMismatch(cId)
    },

    // Deployment (WRP-010, WRP-012, MID-002)
    {
      nondet pId = Set(1, 2).oneOf()
      nondet forT25 = Set(true, false).oneOf()
      nondet mutVk = Set(true, false).oneOf()
      doDeployProgramLedger(pId, forT25, mutVk)
    },

    // Rekey completion & refusal (RKY-003, RKY-004)
    {
      nondet rkRows = Set(400000, 1685845, 1782573, 1782574).oneOf()
      nondet rkVk = Set(1325, 50000, 50001).oneOf()
      doCompleteRekeyWrapMeasurement(rkRows, rkVk)
    },
    doRefuseRekeyBoundsOverflow(MAX_ROWS_AT_K21 + 1, MAX_VK_BYTES + 1),

    // T25 qualification (WRP-012, LW-016)
    {
      nondet incLw004 = Set(true, false).oneOf()
      val allNopSet =
        if (MUTANT_ID == MUT_LW016_ALL_NOP_QUAL) Set(true, false)
        else Set(false)
      nondet allNop = allNopSet.oneOf()
      doRecordT25LwPasses(incLw004, allNop)
    },
    doCloseT25Qualification,
    {
      nondet allNopOffer = Set(true, false).oneOf()
      doRefuseIncompleteT25Qualification(allNopOffer)
    }
  }

  // =========================================================================
  // Safety invariants (one inv_<ID> per expressible requirement)
  // =========================================================================

  // WRP-001: Every completed wrap proof run comes from a known LfrProofReady call and produces
  // exactly 1 midnight-proofs PLONK proof with KZG over BLS12-381 and the Blake2b transcript.
  val inv_WRP_001: bool =
    wrp.proofRuns.forall(r =>
      r.libraryVerified implies (
        wrp.knownCallIds.contains(r.callId) and
        r.proofCountInRun == 1 and
        r.scheme == PlonkKzgBls12381 and
        r.transcript == Blake2bTranscript
      )
    )

  // WRP-002: Any request above k = 21 stops before keygen with "never k = 22" and writes no key;
  // no initialized wrap key has circuitK > MAX_WRAP_K.
  val inv_WRP_002: bool =
    (wrp.eraKeyInitialized implies wrp.cpuWrapKey.circuitK <= MAX_WRAP_K) and
    wrp.keygenAttempts.forall(a =>
      (a.requestedK > MAX_WRAP_K) implies (
        a.stoppedBeforeKeygen and
        a.stopMessage == "never k = 22" and
        not(a.keyWritten)
      )
    )

  // WRP-003: At k = 21 (MAX_WRAP_K), any cost model above MAX_ROWS_AT_K21 stops before keygen;
  // no initialized wrap key at k = MAX_WRAP_K exceeds MAX_ROWS_AT_K21 rows.
  val inv_WRP_003: bool =
    ((wrp.eraKeyInitialized and wrp.cpuWrapKey.circuitK == MAX_WRAP_K) implies
      wrp.cpuWrapKey.measuredRows <= MAX_ROWS_AT_K21) and
    wrp.keygenAttempts.forall(a =>
      (a.requestedK == MAX_WRAP_K and a.measuredRows > MAX_ROWS_AT_K21) implies (
        a.stoppedBeforeKeygen and not(a.keyWritten)
      )
    )

  // WRP-004 (and DG-003, MID-003): Serialized wrap VK > 50,000 bytes never writes wrap.vk.
  val inv_WRP_004: bool =
    (wrp.eraKeyInitialized implies wrp.cpuWrapKey.vkBytes <= MAX_VK_BYTES) and
    wrp.keygenAttempts.forall(a =>
      (a.vkBytes > MAX_VK_BYTES) implies (a.stoppedWithoutWrapVk and not(a.keyWritten))
    ) and
    wrp.proofRuns.forall(r =>
      r.writtenFiles.contains("wrap.vk") implies (r.vkBytes <= MAX_VK_BYTES)
    )

  // WRP-006: In production builds, every wrap proof draws fresh blinders from the OS CSPRNG,
  // and any two distinct production proofs have distinct blinders and distinct proof bytes.
  val inv_WRP_006: bool =
    wrp.proofRuns.forall(r =>
      (r.libraryVerified and r.buildMode == ProdBuild) implies (r.blinderSource == OsCsprng)
    ) and
    wrp.proofRuns.forall(r1 =>
      wrp.proofRuns.forall(r2 =>
        (r1.libraryVerified and r2.libraryVerified and
          r1.buildMode == ProdBuild and r2.buildMode == ProdBuild and
          r1.runId != r2.runId) implies (
          r1.blinderId != r2.blinderId and r1.proofBytesTag != r2.proofBytesTag
        )
      )
    )

  // WRP-007: Every accepted wrap proof pins anchor read key (lane 6) == 15, enables the chunk write
  // prologue iff ChunkMode (ANC-016), and when active pins chunk write key (lane 15) == 15.
  val inv_WRP_007: bool =
    wrp.proofRuns.forall(r =>
      r.libraryVerified implies (
        r.lane6ReadKey == ANCHOR_SLOT_CONST and
        ((r.callMode == ChunkMode) == r.hasChunkWritePrologue) and
        (r.hasChunkWritePrologue implies (r.lane15WriteKey == ANCHOR_SLOT_CONST))
      )
    )

  // WRP-008: When E6_EXPECT_VK is set and differs from the wrap key, the wrap tool stops before writing any output file.
  val inv_WRP_008: bool =
    wrp.proofRuns.forall(r =>
      (r.expectVkSet and r.vkId != r.expectedVkId) implies (
        r.stoppedOnVkMismatch and r.writtenFiles == Set()
      )
    )

  // WRP-009: When the library verifier accepts a new wrap proof, the wrap tool writes all 3 files
  // wrap.vk, wrap.proof and wrap.public.json; unverified runs write none.
  val inv_WRP_009: bool =
    wrp.proofRuns.forall(r =>
      if (r.libraryVerified)
        (r.writtenFiles == WRAP_OUTPUT_FILES and r.writtenFiles.size() == WRAP_OUTPUT_FILE_COUNT)
      else
        (r.writtenFiles == Set())
    )

  // WRP-010 (and MID-002): One universal wrap key for every program, both modes, and every ledger
  // deployment of the era (with Sherlock write entries holding that VK and read entries keeping native keys).
  val inv_WRP_010: bool =
    (wrp.eraKeyInitialized implies (wrp.eraWrapVkSet == Set(wrp.cpuWrapKey.vkId))) and
    wrp.deployments.forall(d =>
      wrp.eraKeyInitialized and
      d.installedWriteVkId == wrp.cpuWrapKey.vkId and
      d.readEntriesKeepNativeVk
    ) and
    wrp.proofRuns.forall(r =>
      r.libraryVerified implies (wrp.eraKeyInitialized and r.vkId == wrp.cpuWrapKey.vkId)
    )

  // WRP-011: Where the wrap prover runs on a GPU, its verifying key is byte-equal to the CPU wrap key.
  val inv_WRP_011: bool =
    (wrp.gpuConfigured implies (wrp.eraKeyInitialized and wrp.gpuWrapVkId == wrp.cpuWrapKey.vkId)) and
    wrp.proofRuns.forall(r =>
      (r.libraryVerified and r.device == GpuDevice) implies (r.vkId == wrp.cpuWrapKey.vkId)
    )

  // WRP-012: When the wrap is qualified on the T25 key set, the qualification record holds a pass
  // for each of the 5 requirements LW-003, LW-004, LW-005, LW-006, and LW-014; T25 deploys require qualification.
  val inv_WRP_012: bool =
    (wrp.t25Qualified implies (
      REQUIRED_LW_CHECKS.subseteq(wrp.t25PassedChecks) and
      wrp.t25PassedChecks.size() >= LW_QUAL_CHECK_COUNT
    )) and
    wrp.deployments.forall(d => d.usesT25Key implies wrp.t25Qualified)

  // RKY-003: When the rekey completes, the wrap is measured at k = 21 with at most 1,782,573 rows.
  val inv_RKY_003: bool =
    wrp.rekeyCompleted implies (
      wrp.rekeyMeasuredK == MAX_WRAP_K and
      wrp.rekeyMeasuredRows > 0 and
      wrp.rekeyMeasuredRows <= MAX_ROWS_AT_K21
    )

  // RKY-004: When the rekey completes, the measured wrap verifying key is at most 50,000 bytes.
  val inv_RKY_004: bool =
    wrp.rekeyCompleted implies (
      wrp.rekeyMeasuredVkBytes > 0 and
      wrp.rekeyMeasuredVkBytes <= MAX_VK_BYTES
    )

  // Adopted invariants checkable in Wrap's state:

  // DG-002 & LW-013: Wrap fits 85% of (2^k - 7) usable rows for k in {19, 20, 21} using ZkStdLib chips only.
  val inv_DG_002: bool =
    wrp.eraKeyInitialized implies (
      wrp.cpuWrapKey.circuitK >= MIN_WRAP_K and
      wrp.cpuWrapKey.circuitK <= MAX_WRAP_K and
      wrp.cpuWrapKey.measuredRows > 0 and
      wrp.cpuWrapKey.measuredRows <= usableRows85Pct(wrp.cpuWrapKey.circuitK) and
      wrp.cpuWrapKey.arch == ZkStdLibArch and
      not(wrp.cpuWrapKey.hasForeignOrG4Chip)
    )

  // ZK-003 & LW-012: Seeded blinders are accepted ONLY in TestSeedBuild under GW_DET=1; never ConstantBad.
  val inv_ZK_003: bool =
    wrp.proofRuns.forall(r =>
      r.libraryVerified implies (
        r.blinderSource != ConstantBad and
        (r.blinderSource == SeededDet implies (r.buildMode == TestSeedBuild and r.gwDet))
      )
    )

  // GPW-001 & GPW-003: Under GW_DET=1 in TestSeedBuild, CPU and GPU wrap proofs of the same call and seed
  // produce one byte-equal proof SHA-256 (proofBytesTag) and both pass verification.
  val inv_GPW_003: bool =
    wrp.proofRuns.forall(r1 =>
      wrp.proofRuns.forall(r2 =>
        (r1.libraryVerified and r2.libraryVerified and
          r1.buildMode == TestSeedBuild and r2.buildMode == TestSeedBuild and
          r1.gwDet and r2.gwDet and
          r1.callId == r2.callId and r1.detSeed == r2.detSeed and r1.vkId == r2.vkId) implies (
          r1.proofBytesTag == r2.proofBytesTag
        )
      )
    )

  // ANC-001: Anchor cell never holds plain prog_d and user code never addresses slot 15 in accepted wraps.
  val inv_ANC_001: bool =
    wrp.proofRuns.forall(r =>
      r.libraryVerified implies (not(r.anchorHoldsPlainProgD) and not(r.userAddressesSlot15))
    )

  // LW-002: Wrap verifies L_Fr natively in Fr under frozen L_Fr key and u = 7 transcript.
  val inv_LW_002: bool =
    wrp.proofRuns.forall(r =>
      r.libraryVerified implies (
        r.lfrValidInFr and r.lfrVkMatchesFrozen and r.lfrTranscriptU == LFR_TRANSCRIPT_U
      )
    )

  // LW-003: Public inputs are the 1,026-word call vector [binding_input, comm, ...], zero-padded, no private leak.
  val inv_LW_003: bool =
    wrp.proofRuns.forall(r =>
      r.libraryVerified implies (
        r.publicInputCount == CALL_VECTOR_PUBLIC_INPUTS and
        r.lane0EqBind and
        r.lane1EqComm and
        r.unusedLanesZero and
        not(r.leaksPrivateWord)
      )
    )

  // LW-004: Native replay of the field log succeeds on every accepted wrap proof.
  val inv_LW_004: bool =
    wrp.proofRuns.forall(r => r.libraryVerified implies r.fieldLogValid)

  // LW-005: Anchor tags (SHK_PROGC vs SHK_LOCK), popeq == a_in, DIGEST entries, lane 0 == bind, layout_d == key constant.
  val inv_LW_005: bool =
    wrp.proofRuns.forall(r =>
      r.libraryVerified implies (
        r.popeqEqAIn and
        r.anchorsFromDigestLog and
        r.aInTag == expectedAInTag(r.callMode, r.chunkIdx) and
        r.aOutTag == expectedAOutTag(r.callMode, r.chunkIdx, r.chunkCount) and
        r.lane0EqBind and
        r.layoutDEqKeyConst
      )
    )

  // LW-006: X limbs canonical and X == cached-trace commitment of L.
  val inv_LW_006: bool =
    wrp.proofRuns.forall(r => r.libraryVerified implies r.dagXCanonAndEqCached)

  // LW-014: Wrap keygen refuses log capacity < reservation R, frozen key drift, or a second call-time Fr engine.
  val inv_LW_014: bool =
    wrp.eraKeyInitialized implies (
      wrp.cpuWrapKey.logCap >= MIN_LOG_RESERVATION_R and
      wrp.cpuWrapKey.lAndLfrKeysMatchFrozen and
      not(wrp.cpuWrapKey.hasSecondFrFallback)
    )

  // LW-016: A benchmark on L_cached with an all-NOP log qualifies no gate.
  val inv_LW_016: bool =
    not(wrp.t25UsedAllNopBench)

  // Master safety invariant.
  val inv: bool =
    consistency_constants and
    inv_WRP_001 and
    inv_WRP_002 and
    inv_WRP_003 and
    inv_WRP_004 and
    inv_WRP_006 and
    inv_WRP_007 and
    inv_WRP_008 and
    inv_WRP_009 and
    inv_WRP_010 and
    inv_WRP_011 and
    inv_WRP_012 and
    inv_RKY_003 and
    inv_RKY_004 and
    inv_DG_002 and
    inv_ZK_003 and
    inv_GPW_003 and
    inv_ANC_001 and
    inv_LW_002 and
    inv_LW_003 and
    inv_LW_004 and
    inv_LW_005 and
    inv_LW_006 and
    inv_LW_014 and
    inv_LW_016

  // =========================================================================
  // Reachability witnesses (negated goals — MUST be violated under wrap_main)
  // =========================================================================

  // Witness 1: Both CPU and GPU production wrap proofs succeed and write the 3 WrapOutput files under one VK.
  val reach_cpu_and_gpu_wrap_output: bool = not(
    wrp.eraKeyInitialized and
    wrp.proofRuns.exists(r =>
      r.libraryVerified and r.device == CpuDevice and r.writtenFiles == WRAP_OUTPUT_FILES
    ) and
    wrp.proofRuns.exists(r =>
      r.libraryVerified and r.device == GpuDevice and r.writtenFiles == WRAP_OUTPUT_FILES and
      r.vkId == wrp.cpuWrapKey.vkId
    )
  )

  // Witness 2: Two production wrap proofs of the same call verify and have distinct proof bytes (WRP-006).
  val reach_two_prod_proofs_differ: bool = not(
    wrp.proofRuns.exists(r1 =>
      wrp.proofRuns.exists(r2 =>
        r1.runId != r2.runId and
        r1.callId == r2.callId and
        r1.libraryVerified and r2.libraryVerified and
        r1.buildMode == ProdBuild and r2.buildMode == ProdBuild and
        r1.proofBytesTag != r2.proofBytesTag
      )
    )
  )

  // Witness 3: Under TestSeedBuild + GW_DET=1, CPU and GPU wrap proofs of one call match byte-for-byte (GPW-003, ZK-003).
  val reach_det_seed_gpu_cpu_byte_equal: bool = not(
    wrp.proofRuns.exists(r1 =>
      wrp.proofRuns.exists(r2 =>
        r1.device == CpuDevice and r2.device == GpuDevice and
        r1.buildMode == TestSeedBuild and r2.buildMode == TestSeedBuild and
        r1.gwDet and r2.gwDet and
        r1.callId == r2.callId and
        r1.libraryVerified and r2.libraryVerified and
        r1.proofBytesTag == r2.proofBytesTag
      )
    )
  )

  // Witness 4: Keygen k=22 ("never k = 22") and row-gate refusals both fire.
  val reach_keygen_k22_and_row_refusals: bool = not(
    wrp.keygenAttempts.exists(a => a.requestedK > MAX_WRAP_K and a.stopMessage == "never k = 22") and
    wrp.keygenAttempts.exists(a => a.measuredRows > MAX_ROWS_AT_K21 and a.stoppedBeforeKeygen)
  )

  // Witness 5: Witness/slot-15 refusal and E6_EXPECT_VK mismatch refusal both fire.
  val reach_witness_and_expect_vk_refusals: bool = not(
    wrp.proofRuns.exists(r => r.stoppedOnVkMismatch and r.writtenFiles == Set()) and
    wrp.lastRefusalReason == "witness_refused"
  )

  // Witness 6: Both OrdMode and ChunkMode calls wrap under one universal VK deployed across 2 programs.
  val reach_ord_and_chunk_modes_two_programs: bool = not(
    wrp.deployments.exists(d1 => wrp.deployments.exists(d2 =>
      d1.progId != d2.progId and d1.installedWriteVkId == d2.installedWriteVkId
    )) and
    wrp.proofRuns.exists(r => r.libraryVerified and r.callMode == OrdMode) and
    wrp.proofRuns.exists(r => r.libraryVerified and r.callMode == ChunkMode)
  )

  // Witness 7: T25 5-check qualification closes and rekey wrap measurement completes within bounds.
  val reach_t25_qualification_and_rekey: bool = not(
    wrp.t25Qualified and wrp.rekeyCompleted
  )
}

// ===========================================================================
// Concrete instances: main, edge instances, and guard mutants
// ===========================================================================

module wrap_main {
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
  ).*
}

// Smallest legal circuit parameters (k = 19, 85% usable rows = 445,638, measured T10m VK = 1,325 bytes).
module wrap_edge_min {
  import wrap(
    MIN_WRAP_K = 19,
    MAX_WRAP_K = 19,
    MAX_ROWS_AT_K21 = 445638,
    MAX_VK_BYTES = 1325,
    CALL_VECTOR_PUBLIC_INPUTS = 1026,
    ANCHOR_SLOT_CONST = 15,
    ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15,
    WRAP_OUTPUT_FILE_COUNT = 3,
    LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7,
    MIN_LOG_RESERVATION_R = 1,
    MUTANT_ID = 0
  ).*
}

// Pinned right at k = 21 with exact limit bounds (rows = 1,782,573, VK = 50,000 bytes).
module wrap_edge_at_limit {
  import wrap(
    MIN_WRAP_K = 21,
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
  ).*
}

// Mutants (each removes or weakens one refusal/binding guard and must break its named invariant)

module wrap_m_wrp001_wrong_scheme {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 1
  ).*
}

module wrap_m_wrp002_allow_k22 {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 2
  ).*
}

module wrap_m_wrp003_ignore_row_gate {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 3
  ).*
}

module wrap_m_wrp004_write_oversize_vk {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 4
  ).*
}

module wrap_m_wrp006_reuse_blinder {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 5
  ).*
}

module wrap_m_wrp007_skip_slot15_pin {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 6
  ).*
}

module wrap_m_wrp008_ignore_expect_vk {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 7
  ).*
}

module wrap_m_wrp009_partial_output {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 8
  ).*
}

module wrap_m_wrp010_per_program_vk {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 9
  ).*
}

module wrap_m_wrp011_divergent_gpu_vk {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 10
  ).*
}

module wrap_m_wrp012_missing_lw004 {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 11
  ).*
}

module wrap_m_rky003_row_overflow {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 12
  ).*
}

module wrap_m_rky004_vk_overflow {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 13
  ).*
}

module wrap_m_dg002_k20_row_overflow {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 14
  ).*
}

module wrap_m_zk003_seeded_in_prod {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 15
  ).*
}

module wrap_m_gpw003_det_mismatch {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 16
  ).*
}

module wrap_m_lw002_unverified_lfr {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 17
  ).*
}

module wrap_m_lw003_public_leak {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 18
  ).*
}

module wrap_m_lw004_bad_field_log {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 19
  ).*
}

module wrap_m_lw005_swapped_tag {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 20
  ).*
}

module wrap_m_lw006_x_mismatch {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 21
  ).*
}

module wrap_m_lw014_short_log_cap {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 22
  ).*
}

module wrap_m_lw016_all_nop_qual {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 23
  ).*
}

module wrap_m_anc001_plain_progd {
  import wrap(
    MIN_WRAP_K = 19, MAX_WRAP_K = 21, MAX_ROWS_AT_K21 = 1782573, MAX_VK_BYTES = 50000,
    CALL_VECTOR_PUBLIC_INPUTS = 1026, ANCHOR_SLOT_CONST = 15, ANCHOR_READ_LANE = 6,
    CHUNK_WRITE_LANE = 15, WRAP_OUTPUT_FILE_COUNT = 3, LW_QUAL_CHECK_COUNT = 5,
    LFR_TRANSCRIPT_U = 7, MIN_LOG_RESERVATION_R = 8, MUTANT_ID = 24
  ).*
}
