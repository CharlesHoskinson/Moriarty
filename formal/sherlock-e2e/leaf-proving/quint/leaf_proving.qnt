// Quint specification for Sherlock E2E Prototype — Bounded Context: Leaf Proving
// Lane: e2e-leaf-proving
// Sources: LFP-001..012, RSL-001..008, LPC-018, and adopted SPL-004, INIT-001..008, GP-001.

module leaf_proving {
  // =========================================================================
  // 1. Spec Constants (numeric bounds and optional kit flags)
  // =========================================================================

  const CHIP_SET_RSL: int         // LFP-002: R_SL chip set id (3)
  const LFP_MAX_APP_AIRS: int     // LFP-003: max app AIRs before refusal (8 in LFP-003 text)
  const RSL_BUCKET_LOW: int       // RSL-002: 6-AIR bucket with FP-1 (no block hash)
  const RSL_BUCKET_HIGH: int      // RSL-002: 9-AIR bucket with FP-1 (one block hash)
  const MAX_APP_VK_COLS: int      // LFP-004, INIT-008: max app VK columns (30,000)
  const SHARD_STEP_CAP: int       // LFP-008: step capacity of one leaf shape
  const MAX_SHARDS_PER_RUN: int   // LFP-008, LFP-009: max shards modeled per run
  const SUITE_CALL_TOTAL: int     // RSL-003: total suite calls (142)
  const JUBJUB_COFACTOR: int      // RSL-006: subgroup cofactor check 8Q = P (8)
  const HAS_KECCAK_KIT: bool      // RSL-004: machine version includes keccak256 kit
  const HAS_SECP256K1_KIT: bool   // RSL-005: machine version includes secp256k1 kit
  const MUTANT: str               // "none" in sound instances, mutant name in leaf_proving_m_*

  // =========================================================================
  // 2. Domain Types (enums and records — no string manipulation)
  // =========================================================================

  type RuleFamily =
    | FamRsl
    | FamInterpreted

  type HashKit =
    | KitNone
    | KitSha256
    | KitKeccak256
    | KitBothShaAndKeccak

  type StarkCfg =
    | CfgBabyBearPoseidon2
    | CfgWrongFieldOrHash

  type JubjubState =
    | JubjubNotUsed
    | JubjubValidSubgroup
    | JubjubOffCurve
    | JubjubOutSubgroup

  type SuiteStatus =
    | SuitePending
    | SuiteProved
    | SuiteBlockedVerifierErr

  type RefusalCode =
    | RefNone
    | RefOverMaxAirs
    | RefOverVkCols
    | RefKitDisabled
    | RefBadShardHeights
    | RefWrongProgDSlot
    | RefN1WrongProgD
    | RefBadFp1Outcome
    | RefWrongTapeInputD
    | RefBadJubjubRow
    | RefMisalignedSha
    | RefNonCanonicalWord
    | RefBadGenesisInit

  type TamperKind =
    | TamperNone
    | TamperBadFieldCfg
    | TamperPrehashDrift
    | TamperSwappedHeights
    | TamperWrongProgDSlot
    | TamperRekeyLongRun
    | TamperBadFp1Outcome
    | TamperZeroRslProgD
    | TamperWrongTapeInput
    | TamperJubjubOutSubgroup
    | TamperMisalignedSha
    | TamperNonCanonicalWord
    | TamperInitWitnessGate

  type ProgramSpec = {
    progId: int,
    family: RuleFamily,
    hashKit: HashKit,
    usesSecp256k1: bool,
    usesJubjub: bool,
    nonFp1AppAirs: int,
    totalAppAirs: int,
    publicAirCount: int,
    progD: int,
    ers006ProgD: int,
    keyRoot: int,
    appVkCols: int,
    admittedHeights: List[int],
  }

  type ProgramKeyRec = {
    progId: int,
    keyRoot: int,
    chipSetId: int,
    airCount: int,
    appVkId: int,
    leafVkId: int,
    internalForLeafVkId: int,
    appVkCols: int,
    leafN1PreHash: int,
  }

  type UniversalKeyState = {
    universalN1PreHash: int,
    internalRecVk: int,
    lVk: int,
    wrapVk: int,
    manifestSha256: int,
  }

  type RunSpec = {
    runId: int,
    progId: int,
    totalSteps: int,
    shardCount: int,
    progD: int,
    inputD: int,
    traceComm: int,
    traceOrderedW: List[int],
  }

  type AppProofRec = {
    slotId: int,
    runId: int,
    shardIdx: int,
    progId: int,
    chipSetId: int,
    starkCfg: StarkCfg,
    hasBls12381: bool,
    stockVerified: bool,
    hasFp1: bool,
    totalAppAirs: int,
    nonFp1AppAirs: int,
    shaInPresent: bool,
    shaInEmpty: bool,
    shaInActive: bool,
    shaReqActive: bool,
    shaAligned: bool,
    usesSecp256k1: bool,
    outcomeMatchesNative: bool,
    usesJubjub: bool,
    addedJubjubAirs: int,
    jubjubCheck: JubjubState,
    jubjubCofactorChecked: int,
    wordsCanonical: bool,
    airHeights: List[int],
    traceComm: int,
    traceOrderedW: List[int],
    fp1OutcomeD: int,
  }

  type LeafProofRec = {
    slotId: int,
    runId: int,
    shardIdx: int,
    progId: int,
    family: RuleFamily,
    leafVkId: int,
    starkCfg: StarkCfg,
    hasBls12381: bool,
    stockVerified: bool,
    n1PreHash: int,
    airHeights: List[int],
    proofFileId: int,
    proofSha256: int,
    pubProgD: int,
    pubInputD: int,
    consumedTapeD: int,
    pubFp1OutcomeD: int,
    hPre: int,
    witnessInitGate: bool,
    preStateMatchesInit: bool,
    onlyInitDigestMatched: bool,
    prevPostMatchesPre: bool,
    openedProgRootMatches: bool,
    openedInputSalted: bool,
    fuelLimbsBelow2Pow64: bool,
    genesisMemCanonical: bool,
    genesisRecordKindsValid: bool,
    coprocessorFormsOpened: bool,
    requiredInitAirsPresent: bool,
    appVkColsUsed: int,
  }

  type LeafKeyRefRec = {
    keyRoot: int,
    chipSetId: int,
    airCount: int,
  }

  type ReadyLeafEntry = {
    shardIdx: int,
    proofFileId: int,
    proofSha256: int,
    pubProgD: int,
    pubInputD: int,
    pubFp1OutcomeD: int,
  }

  type LeafProofsReadyRec = {
    runId: int,
    progId: int,
    entries: List[ReadyLeafEntry],
    leafKeyRef: LeafKeyRefRec,
  }

  type SuiteEntry = {
    progId: int,
    disposition: SuiteStatus,
    appProofSlot: int,
    verifierErrorRecorded: bool,
  }

  type RefusalEvent = {
    code: RefusalCode,
    targetId: int,
  }

  type StepArgs = {
    progId: int,
    runId: int,
    shardIdx: int,
    slotId: int,
    refusal: RefusalCode,
  }

  type WorldSnap = {
    universalKeys: UniversalKeyState,
    admittedProgs: Set[int],
    programKeys: int -> ProgramKeyRec,
    admittedKeySnap: int -> ProgramKeyRec,
    appProofs: int -> AppProofRec,
    leafProofs: int -> LeafProofRec,
    readyEvents: int -> LeafProofsReadyRec,
    n1Accepted: int -> Set[int],
    suiteTable: int -> SuiteEntry,
    refusalLog: Set[RefusalEvent],
  }

  // =========================================================================
  // 3. Fixed Finite Catalogs and Pure Helpers
  // =========================================================================

  pure val UNIVERSAL_N1_PREHASH: int    = 9001
  pure val UNIVERSAL_INTERNAL_VK: int   = 9101
  pure val UNIVERSAL_L_VK: int          = 9102
  pure val UNIVERSAL_WRAP_VK: int       = 9103
  pure val UNIVERSAL_MANIFEST_HASH: int = 9200
  pure val CHIP_SET_INTERP: int         = 1

  pure val PROG_IDS: Set[int]       = 1.to(8)
  pure val SUITE_PROG_IDS: Set[int] = 1.to(5)
  pure val RUN_IDS: Set[int]        = 1.to(6)

  pure val RSL_TAMPER_POOL: Set[TamperKind] = Set(
    TamperBadFieldCfg,
    TamperPrehashDrift,
    TamperSwappedHeights,
    TamperWrongProgDSlot,
    TamperRekeyLongRun,
    TamperBadFp1Outcome,
    TamperZeroRslProgD,
    TamperWrongTapeInput,
    TamperJubjubOutSubgroup,
    TamperMisalignedSha,
    TamperNonCanonicalWord
  )

  pure val INTERP_TAMPER_POOL: Set[TamperKind] = Set(
    TamperBadFieldCfg,
    TamperSwappedHeights,
    TamperWrongProgDSlot,
    TamperRekeyLongRun,
    TamperInitWitnessGate
  )

  pure def minInt(a: int, b: int): int = if (a < b) a else b

  pure def ceilDiv(num: int, den: int): int =
    if (den <= 0) 0 else (num + den - 1) / den

  pure def shardSlotId(runId: int, shardIdx: int): int =
    runId * 10 + shardIdx

  pure def computeOutcomeDigest(commVal: int, words: List[int]): int =
    words.foldl(commVal * 100, (acc, w) => acc * 31 + w)

  pure def swapFirstTwoHeights(hs: List[int]): List[int] =
    if (hs.length() >= 2)
      [hs[1], hs[0]].concat(hs.tail().tail())
    else
      [999]

  pure val GENESIS_UNIVERSAL_KEYS: UniversalKeyState = {
    universalN1PreHash: UNIVERSAL_N1_PREHASH,
    internalRecVk: UNIVERSAL_INTERNAL_VK,
    lVk: UNIVERSAL_L_VK,
    wrapVk: UNIVERSAL_WRAP_VK,
    manifestSha256: UNIVERSAL_MANIFEST_HASH,
  }

  // Static catalog of candidate programs covering every EARS scenario:
  // 1: a05 (R_SL, no hash, 4 PUBLIC AIRs, 5 non-FP-1 / 6 total AIRs)
  // 2: t02 increment (R_SL, no hash, 5 non-FP-1 / 6 total AIRs)
  // 3: s01 (R_SL, Jubjub micro-ops, 5 non-FP-1 / 6 total AIRs)
  // 4: d04 withdraw (R_SL, SHA-256 persistentHash, 8 non-FP-1 / 9 total AIRs, 16,346 cols)
  // 5: s02 foreign (R_SL, keccak256 + secp256k1, 8 non-FP-1 / 9 total AIRs, 30,000 cols)
  // 6: interp0 (Interpreted family with INIT admission kit)
  // 7: badDualHash (R_SL with BOTH SHA-256 and keccak256: 11 non-FP-1 / 12 total AIRs)
  // 8: badOverCols (R_SL with 30,001 app VK columns)
  pure def programCatalog(pId: int): ProgramSpec =
    if (pId == 1) {
      progId: 1, family: FamRsl, hashKit: KitNone, usesSecp256k1: false, usesJubjub: false,
      nonFp1AppAirs: 5, totalAppAirs: 6, publicAirCount: 4, progD: 101, ers006ProgD: 101,
      keyRoot: 1001, appVkCols: 12000, admittedHeights: [16, 32, 8],
    } else if (pId == 2) {
      progId: 2, family: FamRsl, hashKit: KitNone, usesSecp256k1: false, usesJubjub: false,
      nonFp1AppAirs: 5, totalAppAirs: 6, publicAirCount: 1, progD: 102, ers006ProgD: 102,
      keyRoot: 1002, appVkCols: 8000, admittedHeights: [16, 32, 8],
    } else if (pId == 3) {
      progId: 3, family: FamRsl, hashKit: KitNone, usesSecp256k1: false, usesJubjub: true,
      nonFp1AppAirs: 5, totalAppAirs: 6, publicAirCount: 1, progD: 103, ers006ProgD: 103,
      keyRoot: 1003, appVkCols: 14000, admittedHeights: [16, 32, 8],
    } else if (pId == 4) {
      progId: 4, family: FamRsl, hashKit: KitSha256, usesSecp256k1: false, usesJubjub: false,
      nonFp1AppAirs: 8, totalAppAirs: 9, publicAirCount: 1, progD: 104, ers006ProgD: 104,
      keyRoot: 1004, appVkCols: 16346, admittedHeights: [16, 32, 8, 4],
    } else if (pId == 5) {
      progId: 5, family: FamRsl, hashKit: KitKeccak256, usesSecp256k1: true, usesJubjub: false,
      nonFp1AppAirs: 8, totalAppAirs: 9, publicAirCount: 1, progD: 105, ers006ProgD: 105,
      keyRoot: 1005, appVkCols: 30000, admittedHeights: [16, 32, 8, 4],
    } else if (pId == 6) {
      progId: 6, family: FamInterpreted, hashKit: KitNone, usesSecp256k1: false, usesJubjub: false,
      nonFp1AppAirs: 7, totalAppAirs: 7, publicAirCount: 1, progD: 106, ers006ProgD: 0,
      keyRoot: 1006, appVkCols: 15000, admittedHeights: [32, 16, 8],
    } else if (pId == 7) {
      progId: 7, family: FamRsl, hashKit: KitBothShaAndKeccak, usesSecp256k1: false, usesJubjub: false,
      nonFp1AppAirs: 11, totalAppAirs: 12, publicAirCount: 1, progD: 107, ers006ProgD: 107,
      keyRoot: 1007, appVkCols: 22000, admittedHeights: [16, 32, 8, 4, 4],
    } else {
      progId: 8, family: FamRsl, hashKit: KitNone, usesSecp256k1: false, usesJubjub: false,
      nonFp1AppAirs: 5, totalAppAirs: 6, publicAirCount: 1, progD: 108, ers006ProgD: 108,
      keyRoot: 1008, appVkCols: 30001, admittedHeights: [16, 32, 8],
    }

  def runCatalog(rId: int): RunSpec =
    val s3 = minInt(3, MAX_SHARDS_PER_RUN)
    val s2 = minInt(2, MAX_SHARDS_PER_RUN)
    if (rId == 1) {
      runId: 1, progId: 1, totalSteps: SHARD_STEP_CAP, shardCount: 1,
      progD: 101, inputD: 501, traceComm: 701, traceOrderedW: [11, 12],
    } else if (rId == 2) {
      runId: 2, progId: 2, totalSteps: s3 * SHARD_STEP_CAP, shardCount: s3,
      progD: 102, inputD: 502, traceComm: 702, traceOrderedW: [21, 22],
    } else if (rId == 3) {
      runId: 3, progId: 4, totalSteps: s2 * SHARD_STEP_CAP, shardCount: s2,
      progD: 104, inputD: 503, traceComm: 703, traceOrderedW: [31, 32],
    } else if (rId == 4) {
      runId: 4, progId: 5, totalSteps: SHARD_STEP_CAP, shardCount: 1,
      progD: 105, inputD: 504, traceComm: 704, traceOrderedW: [41, 42],
    } else if (rId == 5) {
      runId: 5, progId: 6, totalSteps: s2 * SHARD_STEP_CAP, shardCount: s2,
      progD: 106, inputD: 505, traceComm: 705, traceOrderedW: [51, 52],
    } else {
      runId: 6, progId: 3, totalSteps: SHARD_STEP_CAP, shardCount: 1,
      progD: 103, inputD: 506, traceComm: 706, traceOrderedW: [61, 62],
    }

  // =========================================================================
  // 4. State Variables (observable state + replay driver metadata)
  // =========================================================================

  var lastOp: str
  var lastArgs: StepArgs
  var universalKeys: UniversalKeyState
  var admittedProgs: Set[int]
  var programKeys: int -> ProgramKeyRec
  var admittedKeySnap: int -> ProgramKeyRec
  var appProofs: int -> AppProofRec
  var leafProofs: int -> LeafProofRec
  var readyEvents: int -> LeafProofsReadyRec
  var n1Accepted: int -> Set[int]
  var suiteTable: int -> SuiteEntry
  var refusalLog: Set[RefusalEvent]

  val currentSnap: WorldSnap = {
    universalKeys: universalKeys,
    admittedProgs: admittedProgs,
    programKeys: programKeys,
    admittedKeySnap: admittedKeySnap,
    appProofs: appProofs,
    leafProofs: leafProofs,
    readyEvents: readyEvents,
    n1Accepted: n1Accepted,
    suiteTable: suiteTable,
    refusalLog: refusalLog,
  }

  action commitSnap(ws: WorldSnap, opName: str, args: StepArgs): bool = all {
    lastOp'          = opName,
    lastArgs'        = args,
    universalKeys'   = ws.universalKeys,
    admittedProgs'   = ws.admittedProgs,
    programKeys'     = ws.programKeys,
    admittedKeySnap' = ws.admittedKeySnap,
    appProofs'       = ws.appProofs,
    leafProofs'      = ws.leafProofs,
    readyEvents'     = ws.readyEvents,
    n1Accepted'      = ws.n1Accepted,
    suiteTable'      = ws.suiteTable,
    refusalLog'      = ws.refusalLog,
  }

  // =========================================================================
  // 5. Pure Functional Guards and Transitions
  // =========================================================================

  pure def kitEnabledForProg(p: ProgramSpec, keccakOn: bool, secpOn: bool): bool =
    val keccakOk = (p.hashKit == KitKeccak256 or p.hashKit == KitBothShaAndKeccak) implies keccakOn
    val secpOk   = p.usesSecp256k1 implies secpOn
    keccakOk and secpOk

  def airCountAllowed(p: ProgramSpec): bool =
    if (p.family == FamInterpreted)
      true
    else if (MUTANT == "allow_over_8_airs")
      true
    else if (MUTANT == "lfp003_literal_8_vs_9")
      // Literal reading of LFP-003 ("more than 8 app AIRs") against totalAppAirs (6 or 9 in RSL-002)
      p.totalAppAirs <= LFP_MAX_APP_AIRS
        and Set(RSL_BUCKET_LOW, RSL_BUCKET_HIGH).contains(p.totalAppAirs)
    else
      p.nonFp1AppAirs <= LFP_MAX_APP_AIRS
        and Set(RSL_BUCKET_LOW, RSL_BUCKET_HIGH).contains(p.totalAppAirs)

  def vkColsAllowed(p: ProgramSpec): bool =
    (MUTANT == "allow_over_30000_cols") or (p.appVkCols <= MAX_APP_VK_COLS)

  def canAdmitProgram(ws: WorldSnap, pId: int): bool =
    val p = programCatalog(pId)
    not(ws.admittedProgs.contains(pId))
      and kitEnabledForProg(p, HAS_KECCAK_KIT, HAS_SECP256K1_KIT)
      and airCountAllowed(p)
      and vkColsAllowed(p)

  def applyAdmitProgram(ws: WorldSnap, pId: int): WorldSnap =
    val p = programCatalog(pId)
    val csId = if (p.family == FamRsl) CHIP_SET_RSL else CHIP_SET_INTERP
    val pk: ProgramKeyRec = {
      progId: pId,
      keyRoot: p.keyRoot,
      chipSetId: csId,
      airCount: p.totalAppAirs,
      appVkId: 200 + pId,
      leafVkId: 300 + pId,
      internalForLeafVkId: 400 + pId,
      appVkCols: p.appVkCols,
      leafN1PreHash: UNIVERSAL_N1_PREHASH,
    }
    {
      ...ws,
      admittedProgs: ws.admittedProgs.union(Set(pId)),
      programKeys: ws.programKeys.put(pId, pk),
      admittedKeySnap: ws.admittedKeySnap.put(pId, pk),
    }

  def canRefuseProgramOverAirs(ws: WorldSnap, pId: int): bool =
    val p = programCatalog(pId)
    val ev = { code: RefOverMaxAirs, targetId: pId }
    not(ws.admittedProgs.contains(pId))
      and p.family == FamRsl
      and not(airCountAllowed(p))
      and not(ws.refusalLog.contains(ev))

  def canRefuseProgramOverCols(ws: WorldSnap, pId: int): bool =
    val p = programCatalog(pId)
    val ev = { code: RefOverVkCols, targetId: pId }
    not(ws.admittedProgs.contains(pId))
      and not(vkColsAllowed(p))
      and not(ws.refusalLog.contains(ev))

  def rslTamperAllowedByMutant(p: ProgramSpec, r: RunSpec, sIdx: int, t: TamperKind): bool =
    match t {
      | TamperNone              => true
      | TamperBadFieldCfg       => MUTANT == "bad_field_config"
      | TamperPrehashDrift      => MUTANT == "non_universal_prehash"
      | TamperSwappedHeights    => MUTANT == "swapped_air_heights"
      | TamperWrongProgDSlot    => MUTANT == "unconstrained_progd"
      | TamperRekeyLongRun      => MUTANT == "rekey_on_long_run" and r.shardCount > 1 and sIdx > 0
      | TamperBadFp1Outcome     => MUTANT == "unbound_fp1_outcome"
      | TamperZeroRslProgD      => MUTANT == "zero_rsl_progd"
      | TamperWrongTapeInput    => MUTANT == "unbound_tape_input"
      | TamperJubjubOutSubgroup => MUTANT == "skip_jubjub_subgroup" and p.usesJubjub
      | TamperMisalignedSha     => MUTANT == "allow_misaligned_sha" and p.hashKit == KitSha256
      | TamperNonCanonicalWord  => MUTANT == "skip_canonical_range"
      | TamperInitWitnessGate   => false
    }

  pure def rslTamperApplicable(p: ProgramSpec, r: RunSpec, sIdx: int, t: TamperKind): bool =
    match t {
      | TamperNone              => true
      | TamperJubjubOutSubgroup => p.usesJubjub
      | TamperMisalignedSha     => p.hashKit == KitSha256
      | TamperRekeyLongRun      => r.shardCount > 1 and sIdx > 0
      | TamperInitWitnessGate   => false
      | _                       => true
    }

  pure def refusalCodeOfTamper(t: TamperKind): RefusalCode =
    match t {
      | TamperNone              => RefNone
      | TamperBadFieldCfg       => RefNone
      | TamperPrehashDrift      => RefNone
      | TamperSwappedHeights    => RefBadShardHeights
      | TamperWrongProgDSlot    => RefWrongProgDSlot
      | TamperRekeyLongRun      => RefNone
      | TamperBadFp1Outcome     => RefBadFp1Outcome
      | TamperZeroRslProgD      => RefWrongProgDSlot
      | TamperWrongTapeInput    => RefWrongTapeInputD
      | TamperJubjubOutSubgroup => RefBadJubjubRow
      | TamperMisalignedSha     => RefMisalignedSha
      | TamperNonCanonicalWord  => RefNonCanonicalWord
      | TamperInitWitnessGate   => RefBadGenesisInit
    }

  def canProveRslShard(ws: WorldSnap, rId: int, sIdx: int, t: TamperKind): bool =
    val r = runCatalog(rId)
    val p = programCatalog(r.progId)
    val slot = shardSlotId(rId, sIdx)
    p.family == FamRsl
      and ws.admittedProgs.contains(r.progId)
      and 0 <= sIdx and sIdx < r.shardCount
      and not(ws.leafProofs.keys().contains(slot))
      and rslTamperApplicable(p, r, sIdx, t)
      and rslTamperAllowedByMutant(p, r, sIdx, t)

  def applyProveRslShard(ws: WorldSnap, rId: int, sIdx: int, t: TamperKind): WorldSnap =
    val r = runCatalog(rId)
    val p = programCatalog(r.progId)
    val pk = ws.programKeys.get(r.progId)
    val slot = shardSlotId(rId, sIdx)
    val expOutD = computeOutcomeDigest(r.traceComm, r.traceOrderedW)
    val usedCfg = if (t == TamperBadFieldCfg) CfgWrongFieldOrHash else CfgBabyBearPoseidon2
    val usedBls = (t == TamperBadFieldCfg)
    val usedHeights = if (t == TamperSwappedHeights) swapFirstTwoHeights(p.admittedHeights) else p.admittedHeights
    val usedFp1Out = if (t == TamperBadFp1Outcome) 0 else expOutD
    val usedPreHash = if (t == TamperPrehashDrift) 9999 else pk.leafN1PreHash
    val usedPubProgD =
      if (t == TamperWrongProgDSlot) 999
      else if (t == TamperZeroRslProgD) 0
      else p.ers006ProgD
    val usedConsumedTape = if (t == TamperWrongTapeInput) r.inputD + 77 else r.inputD
    val usedJubjub =
      if (not(p.usesJubjub)) JubjubNotUsed
      else if (t == TamperJubjubOutSubgroup) JubjubOutSubgroup
      else JubjubValidSubgroup
    val usedShaAligned = if (t == TamperMisalignedSha) false else true
    val usedCanonical = if (t == TamperNonCanonicalWord) false else true
    val shaActive = (p.hashKit == KitSha256 or p.hashKit == KitBothShaAndKeccak)
    val appRec: AppProofRec = {
      slotId: slot,
      runId: rId,
      shardIdx: sIdx,
      progId: r.progId,
      chipSetId: CHIP_SET_RSL,
      starkCfg: usedCfg,
      hasBls12381: usedBls,
      stockVerified: true,
      hasFp1: true,
      totalAppAirs: p.totalAppAirs,
      nonFp1AppAirs: p.nonFp1AppAirs,
      shaInPresent: true,
      shaInEmpty: (p.hashKit == KitNone),
      shaInActive: shaActive,
      shaReqActive: shaActive,
      shaAligned: usedShaAligned,
      usesSecp256k1: p.usesSecp256k1,
      outcomeMatchesNative: true,
      usesJubjub: p.usesJubjub,
      addedJubjubAirs: 0,
      jubjubCheck: usedJubjub,
      jubjubCofactorChecked: if (p.usesJubjub and t != TamperJubjubOutSubgroup) JUBJUB_COFACTOR else 0,
      wordsCanonical: usedCanonical,
      airHeights: usedHeights,
      traceComm: r.traceComm,
      traceOrderedW: r.traceOrderedW,
      fp1OutcomeD: usedFp1Out,
    }
    val leafRec: LeafProofRec = {
      slotId: slot,
      runId: rId,
      shardIdx: sIdx,
      progId: r.progId,
      family: FamRsl,
      leafVkId: pk.leafVkId,
      starkCfg: usedCfg,
      hasBls12381: usedBls,
      stockVerified: true,
      n1PreHash: usedPreHash,
      airHeights: usedHeights,
      proofFileId: 5000 + slot,
      proofSha256: 8000 + slot,
      pubProgD: usedPubProgD,
      pubInputD: r.inputD,
      consumedTapeD: usedConsumedTape,
      pubFp1OutcomeD: usedFp1Out,
      hPre: sIdx * SHARD_STEP_CAP,
      witnessInitGate: (sIdx == 0),
      preStateMatchesInit: (sIdx == 0),
      onlyInitDigestMatched: false,
      prevPostMatchesPre: true,
      openedProgRootMatches: true,
      openedInputSalted: true,
      fuelLimbsBelow2Pow64: true,
      genesisMemCanonical: true,
      genesisRecordKindsValid: true,
      coprocessorFormsOpened: true,
      requiredInitAirsPresent: true,
      appVkColsUsed: pk.appVkCols,
    }
    val nextProgKeys =
      if (t == TamperRekeyLongRun)
        ws.programKeys.put(r.progId, { ...pk, keyRoot: pk.keyRoot + 500 })
      else
        ws.programKeys
    val nextUnivKeys =
      if (t == TamperRekeyLongRun)
        { ...ws.universalKeys, manifestSha256: ws.universalKeys.manifestSha256 + 1 }
      else
        ws.universalKeys
    {
      ...ws,
      universalKeys: nextUnivKeys,
      programKeys: nextProgKeys,
      appProofs: ws.appProofs.put(slot, appRec),
      leafProofs: ws.leafProofs.put(slot, leafRec),
    }

  def canRefuseRslShardTamper(ws: WorldSnap, rId: int, sIdx: int, t: TamperKind): bool =
    val r = runCatalog(rId)
    val p = programCatalog(r.progId)
    val slot = shardSlotId(rId, sIdx)
    val code = refusalCodeOfTamper(t)
    val ev = { code: code, targetId: slot }
    p.family == FamRsl
      and ws.admittedProgs.contains(r.progId)
      and 0 <= sIdx and sIdx < r.shardCount
      and code != RefNone
      and rslTamperApplicable(p, r, sIdx, t)
      and not(rslTamperAllowedByMutant(p, r, sIdx, t))
      and not(ws.refusalLog.contains(ev))

  def interpTamperAllowedByMutant(r: RunSpec, sIdx: int, t: TamperKind): bool =
    match t {
      | TamperNone            => true
      | TamperBadFieldCfg     => MUTANT == "bad_field_config"
      | TamperSwappedHeights  => MUTANT == "swapped_air_heights"
      | TamperWrongProgDSlot  => MUTANT == "unconstrained_progd"
      | TamperRekeyLongRun    => MUTANT == "rekey_on_long_run" and r.shardCount > 1 and sIdx > 0
      | TamperInitWitnessGate => MUTANT == "init_witness_flag" and sIdx == 0
      | _                     => false
    }

  pure def interpTamperApplicable(r: RunSpec, sIdx: int, t: TamperKind): bool =
    match t {
      | TamperNone            => true
      | TamperBadFieldCfg     => true
      | TamperSwappedHeights  => true
      | TamperWrongProgDSlot  => true
      | TamperRekeyLongRun    => r.shardCount > 1 and sIdx > 0
      | TamperInitWitnessGate => sIdx == 0
      | _                     => false
    }

  def canProveInterpretedShard(ws: WorldSnap, rId: int, sIdx: int, t: TamperKind): bool =
    val r = runCatalog(rId)
    val p = programCatalog(r.progId)
    val slot = shardSlotId(rId, sIdx)
    p.family == FamInterpreted
      and ws.admittedProgs.contains(r.progId)
      and 0 <= sIdx and sIdx < r.shardCount
      and not(ws.leafProofs.keys().contains(slot))
      and interpTamperApplicable(r, sIdx, t)
      and interpTamperAllowedByMutant(r, sIdx, t)

  def applyProveInterpretedShard(ws: WorldSnap, rId: int, sIdx: int, t: TamperKind): WorldSnap =
    val r = runCatalog(rId)
    val p = programCatalog(r.progId)
    val pk = ws.programKeys.get(r.progId)
    val slot = shardSlotId(rId, sIdx)
    val hVal = sIdx * SHARD_STEP_CAP
    val isGenesis = (hVal == 0)
    val initCorrupted = (t == TamperInitWitnessGate and isGenesis)
    val usedCfg = if (t == TamperBadFieldCfg) CfgWrongFieldOrHash else CfgBabyBearPoseidon2
    val usedHeights = if (t == TamperSwappedHeights) swapFirstTwoHeights(p.admittedHeights) else p.admittedHeights
    val usedPubProgD = if (t == TamperWrongProgDSlot) 999 else p.progD
    val expOutD = computeOutcomeDigest(r.traceComm, r.traceOrderedW)
    val leafRec: LeafProofRec = {
      slotId: slot,
      runId: rId,
      shardIdx: sIdx,
      progId: r.progId,
      family: FamInterpreted,
      leafVkId: pk.leafVkId,
      starkCfg: usedCfg,
      hasBls12381: (t == TamperBadFieldCfg),
      stockVerified: true,
      n1PreHash: pk.leafN1PreHash,
      airHeights: usedHeights,
      proofFileId: 5000 + slot,
      proofSha256: 8000 + slot,
      pubProgD: usedPubProgD,
      pubInputD: r.inputD,
      consumedTapeD: r.inputD,
      pubFp1OutcomeD: expOutD,
      hPre: hVal,
      witnessInitGate: if (initCorrupted) false else isGenesis,
      // INIT-002: non-genesis shards (hPre != 0) have arbitrary pre-state bound by prev post_d, not Init!
      preStateMatchesInit: if (isGenesis) not(initCorrupted) else false,
      onlyInitDigestMatched: initCorrupted,
      prevPostMatchesPre: true,
      openedProgRootMatches: not(initCorrupted),
      openedInputSalted: not(initCorrupted),
      fuelLimbsBelow2Pow64: not(initCorrupted),
      genesisMemCanonical: not(initCorrupted),
      genesisRecordKindsValid: not(initCorrupted),
      coprocessorFormsOpened: not(initCorrupted),
      requiredInitAirsPresent: true,
      appVkColsUsed: pk.appVkCols,
    }
    val nextProgKeys =
      if (t == TamperRekeyLongRun)
        ws.programKeys.put(r.progId, { ...pk, keyRoot: pk.keyRoot + 500 })
      else
        ws.programKeys
    {
      ...ws,
      programKeys: nextProgKeys,
      leafProofs: ws.leafProofs.put(slot, leafRec),
    }

  def canRefuseInterpretedShardTamper(ws: WorldSnap, rId: int, sIdx: int, t: TamperKind): bool =
    val r = runCatalog(rId)
    val p = programCatalog(r.progId)
    val slot = shardSlotId(rId, sIdx)
    val code = refusalCodeOfTamper(t)
    val ev = { code: code, targetId: slot }
    p.family == FamInterpreted
      and ws.admittedProgs.contains(r.progId)
      and 0 <= sIdx and sIdx < r.shardCount
      and code != RefNone
      and interpTamperApplicable(r, sIdx, t)
      and not(interpTamperAllowedByMutant(r, sIdx, t))
      and not(ws.refusalLog.contains(ev))

  def allRunShardsProved(ws: WorldSnap, rId: int): bool =
    val r = runCatalog(rId)
    0.to(r.shardCount - 1).forall(sIdx => ws.leafProofs.keys().contains(shardSlotId(rId, sIdx)))

  def canPublishLeafProofsReady(ws: WorldSnap, rId: int): bool =
    val r = runCatalog(rId)
    ws.admittedProgs.contains(r.progId)
      and not(ws.readyEvents.keys().contains(rId))
      and ((MUTANT == "bad_ready_event" and r.shardCount > 1) or allRunShardsProved(ws, rId))

  def applyPublishLeafProofsReady(ws: WorldSnap, rId: int): WorldSnap =
    val r = runCatalog(rId)
    val pk = ws.programKeys.get(r.progId)
    val builtEntries = range(0, r.shardCount).foldl([], (acc, sIdx) =>
      val slot = shardSlotId(rId, sIdx)
      if (ws.leafProofs.keys().contains(slot)) {
        val lp = ws.leafProofs.get(slot)
        acc.append({
          shardIdx: sIdx,
          proofFileId: lp.proofFileId,
          proofSha256: lp.proofSha256,
          pubProgD: lp.pubProgD,
          pubInputD: lp.pubInputD,
          pubFp1OutcomeD: lp.pubFp1OutcomeD,
        })
      } else {
        acc
      }
    )
    val evRec: LeafProofsReadyRec = {
      runId: rId,
      progId: r.progId,
      entries: builtEntries,
      leafKeyRef: {
        keyRoot: pk.keyRoot,
        chipSetId: pk.chipSetId,
        airCount: pk.airCount,
      },
    }
    { ...ws, readyEvents: ws.readyEvents.put(rId, evRec) }

  def canN1AcceptLeaf(ws: WorldSnap, rId: int, slot: int): bool =
    val r = runCatalog(rId)
    // Lead fix (replay R-leaf-1): N1 (Aggregation) consumes the run's leaves through the
    // LeafProofsReady event (LFP-009; DDD.md section 2 row Leaf Proving -> Aggregation and
    // section 4 row LeafProofsReady), so N1 accepts a leaf only after the run's record exists.
    ws.leafProofs.keys().contains(slot)
      and ws.readyEvents.keys().contains(rId)
      and not(ws.n1Accepted.get(rId).contains(slot))
      and (
        MUTANT == "n1_accepts_wrong_progd"
        or ws.leafProofs.get(slot).pubProgD == r.progD
      )

  def applyN1AcceptLeaf(ws: WorldSnap, rId: int, slot: int): WorldSnap =
    { ...ws, n1Accepted: ws.n1Accepted.put(rId, ws.n1Accepted.get(rId).union(Set(slot))) }

  def canN1RefuseWrongProgD(ws: WorldSnap, rId: int, slot: int): bool =
    val r = runCatalog(rId)
    val ev = { code: RefN1WrongProgD, targetId: rId * 100 + slot }
    ws.leafProofs.keys().contains(slot)
      and ws.leafProofs.get(slot).pubProgD != r.progD
      and MUTANT != "n1_accepts_wrong_progd"
      and not(ws.refusalLog.contains(ev))

  def suiteAppSlotForProg(ws: WorldSnap, pId: int): int =
    // Maps suite program pId in 1..5 to its canonical shard-0 slot
    val rId =
      if (pId == 1) 1
      else if (pId == 2) 2
      else if (pId == 3) 6
      else if (pId == 4) 3
      else 4
    shardSlotId(rId, 0)

  def canRunSuiteCallProve(ws: WorldSnap, pId: int, verifierOk: bool): bool =
    val slot = suiteAppSlotForProg(ws, pId)
    SUITE_PROG_IDS.contains(pId)
      and ws.suiteTable.get(pId).disposition == SuitePending
      and (
        (verifierOk and ws.appProofs.keys().contains(slot) and ws.appProofs.get(slot).stockVerified)
        or (not(verifierOk) and MUTANT == "accept_unverified_suite" and ws.admittedProgs.contains(pId))
      )

  def applyRunSuiteCallProve(ws: WorldSnap, pId: int, verifierOk: bool): WorldSnap =
    val slot = suiteAppSlotForProg(ws, pId)
    val entry: SuiteEntry = {
      progId: pId,
      disposition: SuiteProved,
      appProofSlot: if (verifierOk) slot else -1,
      verifierErrorRecorded: not(verifierOk),
    }
    { ...ws, suiteTable: ws.suiteTable.put(pId, entry) }

  def canRecordSuiteCallBlocked(ws: WorldSnap, pId: int): bool =
    SUITE_PROG_IDS.contains(pId)
      and ws.suiteTable.get(pId).disposition == SuitePending
      and ws.admittedProgs.contains(pId)

  def applyRecordSuiteCallBlocked(ws: WorldSnap, pId: int): WorldSnap =
    val entry: SuiteEntry = {
      progId: pId,
      disposition: SuiteBlockedVerifierErr,
      appProofSlot: -1,
      verifierErrorRecorded: true,
    }
    { ...ws, suiteTable: ws.suiteTable.put(pId, entry) }

  // =========================================================================
  // 6. Guarded Actions (each sets lastOp and cites its EARS IDs)
  // =========================================================================

  // Initializes observable state at genesis (no proofs produced yet)
  action init: bool =
    val emptyProgKeys: int -> ProgramKeyRec = Map()
    val emptyApps: int -> AppProofRec = Map()
    val emptyLeaves: int -> LeafProofRec = Map()
    val emptyReady: int -> LeafProofsReadyRec = Map()
    val initN1: int -> Set[int] = RUN_IDS.mapBy(_ => Set())
    val initSuite: int -> SuiteEntry = SUITE_PROG_IDS.mapBy(pId => {
      progId: pId,
      disposition: SuitePending,
      appProofSlot: -1,
      verifierErrorRecorded: false,
    })
    val initRefusals: Set[RefusalEvent] = Set()
    val initSnap: WorldSnap = {
      universalKeys: GENESIS_UNIVERSAL_KEYS,
      admittedProgs: Set(),
      programKeys: emptyProgKeys,
      admittedKeySnap: emptyProgKeys,
      appProofs: emptyApps,
      leafProofs: emptyLeaves,
      readyEvents: emptyReady,
      n1Accepted: initN1,
      suiteTable: initSuite,
      refusalLog: initRefusals,
    }
    commitSnap(initSnap, "init", { progId: 0, runId: 0, shardIdx: 0, slotId: 0, refusal: RefNone })

  // Implements SPL-004, RSL-002, LFP-003, LFP-004, RSL-004, RSL-005
  action admitProgram(pId: int): bool = all {
    canAdmitProgram(currentSnap, pId),
    commitSnap(
      applyAdmitProgram(currentSnap, pId),
      "admitProgram",
      { progId: pId, runId: 0, shardIdx: 0, slotId: 0, refusal: RefNone }
    ),
  }

  // Implements LFP-003, RSL-002 (refusal of R_SL program exceeding app AIR bucket)
  action refuseProgramOverAirs(pId: int): bool = all {
    canRefuseProgramOverAirs(currentSnap, pId),
    val ev = { code: RefOverMaxAirs, targetId: pId }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseProgramOverAirs",
      { progId: pId, runId: 0, shardIdx: 0, slotId: 0, refusal: RefOverMaxAirs }
    ),
  }

  // Implements LFP-004, INIT-008 (refusal of app VK over 30,000 columns)
  action refuseProgramOverCols(pId: int): bool = all {
    canRefuseProgramOverCols(currentSnap, pId),
    val ev = { code: RefOverVkCols, targetId: pId }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseProgramOverCols",
      { progId: pId, runId: 0, shardIdx: 0, slotId: 0, refusal: RefOverVkCols }
    ),
  }

  // Implements LFP-001, LFP-002, LFP-005, LFP-006, LFP-008, LFP-010, LFP-011, LFP-012,
  //            RSL-001, RSL-002, RSL-004, RSL-005, RSL-006, RSL-007, RSL-008
  action proveRslShard(rId: int, sIdx: int, tamper: TamperKind): bool = all {
    canProveRslShard(currentSnap, rId, sIdx, tamper),
    val r = runCatalog(rId)
    val slot = shardSlotId(rId, sIdx)
    commitSnap(
      applyProveRslShard(currentSnap, rId, sIdx, tamper),
      "proveRslShard",
      { progId: r.progId, runId: rId, shardIdx: sIdx, slotId: slot, refusal: RefNone }
    ),
  }

  // Implements LFP-001, LFP-005, LFP-006, LFP-008, INIT-001..004, INIT-007, INIT-008
  action proveInterpretedShard(rId: int, sIdx: int, tamper: TamperKind): bool = all {
    canProveInterpretedShard(currentSnap, rId, sIdx, tamper),
    val r = runCatalog(rId)
    val slot = shardSlotId(rId, sIdx)
    commitSnap(
      applyProveInterpretedShard(currentSnap, rId, sIdx, tamper),
      "proveInterpretedShard",
      { progId: r.progId, runId: rId, shardIdx: sIdx, slotId: slot, refusal: RefNone }
    ),
  }

  // Implements LFP-005, LFP-006, LFP-010, LFP-012, RSL-006, RSL-007, RSL-008 (AIR/host refusal)
  action refuseRslShardTamper(rId: int, sIdx: int, tamper: TamperKind): bool = all {
    canRefuseRslShardTamper(currentSnap, rId, sIdx, tamper),
    val r = runCatalog(rId)
    val slot = shardSlotId(rId, sIdx)
    val code = refusalCodeOfTamper(tamper)
    val ev = { code: code, targetId: slot }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseRslShardTamper",
      { progId: r.progId, runId: rId, shardIdx: sIdx, slotId: slot, refusal: code }
    ),
  }

  // Implements LFP-005, LFP-006, INIT-001..004, INIT-007, INIT-008 (interpreted/INIT refusal)
  action refuseInterpretedShardTamper(rId: int, sIdx: int, tamper: TamperKind): bool = all {
    canRefuseInterpretedShardTamper(currentSnap, rId, sIdx, tamper),
    val r = runCatalog(rId)
    val slot = shardSlotId(rId, sIdx)
    val code = refusalCodeOfTamper(tamper)
    val ev = { code: code, targetId: slot }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "refuseInterpretedShardTamper",
      { progId: r.progId, runId: rId, shardIdx: sIdx, slotId: slot, refusal: code }
    ),
  }

  // Implements LFP-009 (publishes LeafProofsReady once all shards of a run complete)
  action publishLeafProofsReady(rId: int): bool = all {
    canPublishLeafProofsReady(currentSnap, rId),
    val r = runCatalog(rId)
    commitSnap(
      applyPublishLeafProofsReady(currentSnap, rId),
      "publishLeafProofsReady",
      { progId: r.progId, runId: rId, shardIdx: 0, slotId: 0, refusal: RefNone }
    ),
  }

  // Implements LFP-002, LFP-006, LFP-007, RSL-001 (N1 accepts leaf matching run prog_d)
  action n1AcceptLeaf(rId: int, slot: int): bool = all {
    canN1AcceptLeaf(currentSnap, rId, slot),
    val r = runCatalog(rId)
    commitSnap(
      applyN1AcceptLeaf(currentSnap, rId, slot),
      "n1AcceptLeaf",
      { progId: r.progId, runId: rId, shardIdx: 0, slotId: slot, refusal: RefNone }
    ),
  }

  // Implements LFP-007 (N1 statement AIR refuses leaf carrying wrong prog_d)
  action n1RefuseWrongProgD(rId: int, slot: int): bool = all {
    canN1RefuseWrongProgD(currentSnap, rId, slot),
    val r = runCatalog(rId)
    val ev = { code: RefN1WrongProgD, targetId: rId * 100 + slot }
    commitSnap(
      { ...currentSnap, refusalLog: currentSnap.refusalLog.union(Set(ev)) },
      "n1RefuseWrongProgD",
      { progId: r.progId, runId: rId, shardIdx: 0, slotId: slot, refusal: RefN1WrongProgD }
    ),
  }

  // Implements RSL-003, LFP-001 (suite prover proves an R_SL call accepted by stock verifier)
  action runSuiteCallProve(pId: int, verifierOk: bool): bool = all {
    canRunSuiteCallProve(currentSnap, pId, verifierOk),
    val slot = suiteAppSlotForProg(currentSnap, pId)
    commitSnap(
      applyRunSuiteCallProve(currentSnap, pId, verifierOk),
      "runSuiteCallProve",
      { progId: pId, runId: 0, shardIdx: 0, slotId: slot, refusal: RefNone }
    ),
  }

  // Implements RSL-003 Scenario (records suite call as BLOCKED with verifier error on failure)
  action recordSuiteCallBlocked(pId: int): bool = all {
    canRecordSuiteCallBlocked(currentSnap, pId),
    commitSnap(
      applyRecordSuiteCallBlocked(currentSnap, pId),
      "recordSuiteCallBlocked",
      { progId: pId, runId: 0, shardIdx: 0, slotId: -1, refusal: RefNone }
    ),
  }

  // =========================================================================
  // 7. System Transition Step (pre-filtered candidate sets for fast sampling)
  // =========================================================================

  action step: bool = any {
    {
      val cands = PROG_IDS.filter(p => canAdmitProgram(currentSnap, p))
      all {
        cands.size() > 0,
        nondet p = cands.oneOf()
        admitProgram(p),
      }
    },
    {
      val cands = PROG_IDS.filter(p => canRefuseProgramOverAirs(currentSnap, p))
      all {
        cands.size() > 0,
        nondet p = cands.oneOf()
        refuseProgramOverAirs(p),
      }
    },
    {
      val cands = PROG_IDS.filter(p => canRefuseProgramOverCols(currentSnap, p))
      all {
        cands.size() > 0,
        nondet p = cands.oneOf()
        refuseProgramOverCols(p),
      }
    },
    {
      // Honest R_SL shard proving
      val pairs = RUN_IDS.fold(Set(), (acc, rId) =>
        0.to(runCatalog(rId).shardCount - 1)
          .filter(sIdx => canProveRslShard(currentSnap, rId, sIdx, TamperNone))
          .fold(acc, (a2, sIdx) => a2.union(Set((rId, sIdx))))
      )
      all {
        pairs.size() > 0,
        nondet rs = pairs.oneOf()
        proveRslShard(rs._1, rs._2, TamperNone),
      }
    },
    {
      // Mutant-only unguarded R_SL tamper proving (disabled in sound instances)
      val triples = RUN_IDS.fold(Set(), (acc, rId) =>
        0.to(runCatalog(rId).shardCount - 1).fold(acc, (a2, sIdx) =>
          RSL_TAMPER_POOL
            .filter(t => canProveRslShard(currentSnap, rId, sIdx, t))
            .fold(a2, (a3, t) => a3.union(Set((rId, sIdx, t))))
        )
      )
      all {
        triples.size() > 0,
        nondet rst = triples.oneOf()
        proveRslShard(rst._1, rst._2, rst._3),
      }
    },
    {
      // Honest Interpreted shard proving
      val pairs = RUN_IDS.fold(Set(), (acc, rId) =>
        0.to(runCatalog(rId).shardCount - 1)
          .filter(sIdx => canProveInterpretedShard(currentSnap, rId, sIdx, TamperNone))
          .fold(acc, (a2, sIdx) => a2.union(Set((rId, sIdx))))
      )
      all {
        pairs.size() > 0,
        nondet rs = pairs.oneOf()
        proveInterpretedShard(rs._1, rs._2, TamperNone),
      }
    },
    {
      // Mutant-only unguarded Interpreted tamper proving (disabled in sound instances)
      val triples = RUN_IDS.fold(Set(), (acc, rId) =>
        0.to(runCatalog(rId).shardCount - 1).fold(acc, (a2, sIdx) =>
          INTERP_TAMPER_POOL
            .filter(t => canProveInterpretedShard(currentSnap, rId, sIdx, t))
            .fold(a2, (a3, t) => a3.union(Set((rId, sIdx, t))))
        )
      )
      all {
        triples.size() > 0,
        nondet rst = triples.oneOf()
        proveInterpretedShard(rst._1, rst._2, rst._3),
      }
    },
    {
      // Refusal of tampered R_SL shard inputs
      val triples = RUN_IDS.fold(Set(), (acc, rId) =>
        0.to(runCatalog(rId).shardCount - 1).fold(acc, (a2, sIdx) =>
          RSL_TAMPER_POOL
            .filter(t => canRefuseRslShardTamper(currentSnap, rId, sIdx, t))
            .fold(a2, (a3, t) => a3.union(Set((rId, sIdx, t))))
        )
      )
      all {
        triples.size() > 0,
        nondet rst = triples.oneOf()
        refuseRslShardTamper(rst._1, rst._2, rst._3),
      }
    },
    {
      // Refusal of tampered Interpreted shard inputs
      val triples = RUN_IDS.fold(Set(), (acc, rId) =>
        0.to(runCatalog(rId).shardCount - 1).fold(acc, (a2, sIdx) =>
          INTERP_TAMPER_POOL
            .filter(t => canRefuseInterpretedShardTamper(currentSnap, rId, sIdx, t))
            .fold(a2, (a3, t) => a3.union(Set((rId, sIdx, t))))
        )
      )
      all {
        triples.size() > 0,
        nondet rst = triples.oneOf()
        refuseInterpretedShardTamper(rst._1, rst._2, rst._3),
      }
    },
    {
      val cands = RUN_IDS.filter(r => canPublishLeafProofsReady(currentSnap, r))
      all {
        cands.size() > 0,
        nondet r = cands.oneOf()
        publishLeafProofsReady(r),
      }
    },
    {
      val pairs = RUN_IDS.fold(Set(), (acc, rId) =>
        leafProofs.keys()
          .filter(slot => canN1AcceptLeaf(currentSnap, rId, slot))
          .fold(acc, (a2, slot) => a2.union(Set((rId, slot))))
      )
      all {
        pairs.size() > 0,
        nondet rp = pairs.oneOf()
        n1AcceptLeaf(rp._1, rp._2),
      }
    },
    {
      val pairs = RUN_IDS.fold(Set(), (acc, rId) =>
        leafProofs.keys()
          .filter(slot => canN1RefuseWrongProgD(currentSnap, rId, slot))
          .fold(acc, (a2, slot) => a2.union(Set((rId, slot))))
      )
      all {
        pairs.size() > 0,
        nondet rp = pairs.oneOf()
        n1RefuseWrongProgD(rp._1, rp._2),
      }
    },
    {
      val pairs = SUITE_PROG_IDS.fold(Set(), (acc, pId) =>
        Set(true, false)
          .filter(vOk => canRunSuiteCallProve(currentSnap, pId, vOk))
          .fold(acc, (a2, vOk) => a2.union(Set((pId, vOk))))
      )
      all {
        pairs.size() > 0,
        nondet pv = pairs.oneOf()
        runSuiteCallProve(pv._1, pv._2),
      }
    },
    {
      val cands = SUITE_PROG_IDS.filter(p => canRecordSuiteCallBlocked(currentSnap, p))
      all {
        cands.size() > 0,
        nondet p = cands.oneOf()
        recordSuiteCallBlocked(p),
      }
    },
  }

  // =========================================================================
  // 8. Requirement Invariants (inv_<ID>) and Consistency Checks
  // =========================================================================

  // LFP-001: Every app shard proof and every leaf-layer proof is a BabyBear Poseidon2 STARK
  // with no BLS12-381 commitment and accepted by the stock STARK verifier.
  val inv_LFP_001: bool =
    appProofs.keys().forall(slot =>
      val ap = appProofs.get(slot)
      ap.starkCfg == CfgBabyBearPoseidon2 and not(ap.hasBls12381) and ap.stockVerified
    ) and leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      lp.starkCfg == CfgBabyBearPoseidon2 and not(lp.hasBls12381) and lp.stockVerified
    )

  // LFP-002: Every proved shard of an R_SL run has one app proof under chip set 3 (CHIP_SET_RSL)
  // and one leaf-layer proof whose N1 pre-hash equals the universal N1 pre-hash.
  val inv_LFP_002: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      (lp.family == FamRsl) implies (
        appProofs.keys().contains(slot)
          and appProofs.get(slot).chipSetId == CHIP_SET_RSL
          and lp.n1PreHash == universalKeys.universalN1PreHash
      )
    ) and appProofs.keys().forall(slot => leafProofs.keys().contains(slot))

  // LFP-003: An R_SL app shape needing more than LFP_MAX_APP_AIRS (non-FP-1) AIRs is refused before proving.
  val inv_LFP_003: bool =
    admittedProgs.forall(pId =>
      val p = programCatalog(pId)
      (p.family == FamRsl) implies (p.nonFp1AppAirs <= LFP_MAX_APP_AIRS)
    ) and appProofs.keys().forall(slot =>
      appProofs.get(slot).nonFp1AppAirs <= LFP_MAX_APP_AIRS
    )

  // LFP-004: Any app verifying key over MAX_APP_VK_COLS (30,000) columns is refused.
  val inv_LFP_004: bool =
    programKeys.keys().forall(pId =>
      programKeys.get(pId).appVkCols <= MAX_APP_VK_COLS
    ) and leafProofs.keys().forall(slot =>
      leafProofs.get(slot).appVkColsUsed <= MAX_APP_VK_COLS
    )

  // LFP-005: Every proved leaf shard carries the exact ordered AIR height vector of its admitted shape,
  // and all shards of a multi-shard run share that shape's ordered height vector.
  val inv_LFP_005: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      val p = programCatalog(lp.progId)
      lp.airHeights == p.admittedHeights
    ) and appProofs.keys().forall(slot =>
      val ap = appProofs.get(slot)
      val p = programCatalog(ap.progId)
      ap.airHeights == p.admittedHeights
    )

  // LFP-006: The leaf AIR constrains public value slot PROG_D to the prog_d of the shard's program.
  val inv_LFP_006: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      lp.pubProgD == programCatalog(lp.progId).progD
    )

  // LFP-007: N1 statement AIR refuses any leaf proof whose prog_d differs from the run's prog_d.
  val inv_LFP_007: bool =
    RUN_IDS.forall(rId =>
      val r = runCatalog(rId)
      n1Accepted.get(rId).forall(slot =>
        leafProofs.keys().contains(slot)
          and leafProofs.get(slot).pubProgD == r.progD
      )
    )

  // LFP-008: Multi-shard runs add leaf shards (ceil(steps / S)) while leaving every per-program key
  // and every universal key unchanged.
  val inv_LFP_008: bool =
    universalKeys == GENESIS_UNIVERSAL_KEYS
      and programKeys.keys().forall(pId =>
        admittedKeySnap.keys().contains(pId)
          and programKeys.get(pId) == admittedKeySnap.get(pId)
      )
      and RUN_IDS.forall(rId =>
        val r = runCatalog(rId)
        r.shardCount == ceilDiv(r.totalSteps, SHARD_STEP_CAP)
      )

  // LFP-009: Published LeafProofsReady record holds every shard's file id, sha256, public values,
  // and the run's exact LeafKeyRef, only after all shards of the run are proved.
  val inv_LFP_009: bool =
    readyEvents.keys().forall(rId =>
      val ev = readyEvents.get(rId)
      val r = runCatalog(rId)
      admittedProgs.contains(r.progId)
        and programKeys.keys().contains(r.progId)
        and allRunShardsProved(currentSnap, rId)
        and ev.entries.length() == r.shardCount
        and ev.entries.indices().forall(i =>
          val ent = ev.entries[i]
          val slot = shardSlotId(rId, i)
          val lp = leafProofs.get(slot)
          ent.shardIdx == i
            and ent.proofFileId == lp.proofFileId
            and ent.proofSha256 == lp.proofSha256
            and ent.pubProgD == lp.pubProgD
            and ent.pubInputD == lp.pubInputD
            and ent.pubFp1OutcomeD == lp.pubFp1OutcomeD
        )
        and ev.leafKeyRef == {
          keyRoot: programKeys.get(r.progId).keyRoot,
          chipSetId: programKeys.get(r.progId).chipSetId,
          airCount: programKeys.get(r.progId).airCount,
        }
    )

  // LFP-010: R_SL app AIR refuses any trace whose FP-1 outcome commitment differs from digest(comm, ordered W).
  val inv_LFP_010: bool =
    appProofs.keys().forall(slot =>
      val ap = appProofs.get(slot)
      ap.fp1OutcomeD == computeOutcomeDigest(ap.traceComm, ap.traceOrderedW)
    )

  // LFP-011: Every proved R_SL leaf shard carries the ERS-006 prog_d (> 0) of its program in PROG_D.
  val inv_LFP_011: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      val p = programCatalog(lp.progId)
      (lp.family == FamRsl) implies (lp.pubProgD == p.ers006ProgD and p.ers006ProgD > 0)
    )

  // LFP-012: R_SL leaf AIR refuses any shard whose consumed tape digest differs from statement input_d.
  val inv_LFP_012: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      (lp.family == FamRsl) implies (lp.consumedTapeD == lp.pubInputD)
    )

  // RSL-001: Every R_SL leaf-layer proof (across 4-PUBLIC-AIR a05 and 1-PUBLIC-AIR t02/s01/d04/s02)
  // gives an N1 pre-hash equal to the universal N1 pre-hash.
  val inv_RSL_001: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      (lp.family == FamRsl) implies (lp.n1PreHash == UNIVERSAL_N1_PREHASH)
    )

  // RSL-002: R_SL chip set gives every admitted/proved R_SL app 6 or 9 AIRs with FP-1,
  // with SHA_IN present (and empty when the program has no hash).
  val inv_RSL_002: bool =
    admittedProgs.forall(pId =>
      val p = programCatalog(pId)
      (p.family == FamRsl) implies Set(RSL_BUCKET_LOW, RSL_BUCKET_HIGH).contains(p.totalAppAirs)
    ) and appProofs.keys().forall(slot =>
      val ap = appProofs.get(slot)
      ap.hasFp1
        and ap.shaInPresent
        and ((programCatalog(ap.progId).hashKit == KitNone) implies ap.shaInEmpty)
        and Set(RSL_BUCKET_LOW, RSL_BUCKET_HIGH).contains(ap.totalAppAirs)
    )

  // RSL-003: Suite prover marks a call SuiteProved only when one stock-verified R_SL app proof exists;
  // verifier failures are recorded as SuiteBlockedVerifierErr with verifierErrorRecorded == true.
  val inv_RSL_003: bool =
    suiteTable.keys().size() <= SUITE_CALL_TOTAL
      and SUITE_PROG_IDS.forall(pId =>
        val se = suiteTable.get(pId)
        ((se.disposition == SuiteProved) implies (
          appProofs.keys().contains(se.appProofSlot)
            and appProofs.get(se.appProofSlot).progId == pId
            and appProofs.get(se.appProofSlot).stockVerified
            and not(se.verifierErrorRecorded)
        )) and ((se.disposition == SuiteBlockedVerifierErr) implies se.verifierErrorRecorded)
      )

  // RSL-004: Where HAS_KECCAK_KIT holds, keccak256 calls prove inside the 6-or-9 AIR bucket with
  // universal leaf pre-hash; when disabled, no keccak256 call is admitted or proved.
  val inv_RSL_004: bool =
    appProofs.keys().forall(slot =>
      val ap = appProofs.get(slot)
      val p = programCatalog(ap.progId)
      (p.hashKit == KitKeccak256) implies (
        HAS_KECCAK_KIT
          and Set(RSL_BUCKET_LOW, RSL_BUCKET_HIGH).contains(ap.totalAppAirs)
          and leafProofs.keys().contains(slot)
          and leafProofs.get(slot).n1PreHash == UNIVERSAL_N1_PREHASH
      )
    )

  // RSL-005: Where HAS_SECP256K1_KIT holds, secp256k1 calls prove inside the 6-or-9 AIR bucket
  // with outcome matching native; when disabled, no secp256k1 call is admitted or proved.
  val inv_RSL_005: bool =
    appProofs.keys().forall(slot =>
      val ap = appProofs.get(slot)
      ap.usesSecp256k1 implies (
        HAS_SECP256K1_KIT
          and Set(RSL_BUCKET_LOW, RSL_BUCKET_HIGH).contains(ap.totalAppAirs)
          and ap.outcomeMatchesNative
      )
    )

  // RSL-006: Jubjub operations check on_curve and in_subgroup (8Q = P) with micro-ops and 0 new AIRs.
  val inv_RSL_006: bool =
    appProofs.keys().forall(slot =>
      val ap = appProofs.get(slot)
      ap.usesJubjub implies (
        ap.addedJubjubAirs == 0
          and ap.jubjubCheck == JubjubValidSubgroup
          and ap.jubjubCofactorChecked == JUBJUB_COFACTOR
          and Set(RSL_BUCKET_LOW, RSL_BUCKET_HIGH).contains(ap.totalAppAirs)
      )
    )

  // RSL-007: persistentHash proves on SHA_IN and SHA_REQ inside the 9-AIR bucket and refuses unaligned calls.
  val inv_RSL_007: bool =
    appProofs.keys().forall(slot =>
      val ap = appProofs.get(slot)
      val p = programCatalog(ap.progId)
      (p.hashKit == KitSha256) implies (
        ap.shaInActive and ap.shaReqActive and ap.shaAligned and ap.totalAppAirs == RSL_BUCKET_HIGH
      )
    )

  // RSL-008: Non-canonical ALU or Poseidon input words (p + x) are refused by range checks.
  val inv_RSL_008: bool =
    appProofs.keys().forall(slot => appProofs.get(slot).wordsCanonical)

  // Adopted SPL-004: Per-program app, leaf-layer, and internal-for-leaf keys are committed under key_root,
  // while internal-recursive, L, and wrap keys are identical for all programs.
  val inv_SPL_004: bool =
    programKeys.keys().forall(pId =>
      val pk = programKeys.get(pId)
      pk.keyRoot == programCatalog(pId).keyRoot
        and programKeys.keys().forall(p2 =>
          (pId != p2) implies (
            pk.appVkId != programKeys.get(p2).appVkId
              and pk.leafVkId != programKeys.get(p2).leafVkId
              and pk.internalForLeafVkId != programKeys.get(p2).internalForLeafVkId
          )
        )
    ) and universalKeys.internalRecVk == UNIVERSAL_INTERNAL_VK
      and universalKeys.lVk == UNIVERSAL_L_VK
      and universalKeys.wrapVk == UNIVERSAL_WRAP_VK

  // Adopted INIT-001: At h_pre = 0, pre-state is constrained field-by-field to Init(prog_d, input_d).
  val inv_INIT_001: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      (lp.family == FamInterpreted and lp.hPre == 0) implies (
        lp.preStateMatchesInit and not(lp.onlyInitDigestMatched)
      )
    )

  // Adopted INIT-002: Genesis selector is is_zero(h_pre), not a witness flag; at h_pre != 0 INIT constrains nothing.
  val inv_INIT_002: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      (lp.family == FamInterpreted) implies (
        ((lp.hPre == 0) implies lp.preStateMatchesInit)
          and ((lp.hPre != 0) implies lp.prevPostMatchesPre)
      )
    )

  // Adopted INIT-003: At h_pre = 0, prog_d and salted input_d are opened with limbs < 2^64.
  val inv_INIT_003: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      (lp.family == FamInterpreted and lp.hPre == 0) implies (
        lp.openedProgRootMatches and lp.openedInputSalted and lp.fuelLimbsBelow2Pow64
      )
    )

  // Adopted INIT-004: At h_pre = 0, genesis memory is canonical and record kinds are in 1..=4.
  val inv_INIT_004: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      (lp.family == FamInterpreted and lp.hPre == 0) implies (
        lp.genesisMemCanonical and lp.genesisRecordKindsValid
      )
    )

  // Adopted INIT-007: At h_pre = 0, co-processor forms of prog_d and salted input_d are opened.
  val inv_INIT_007: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      (lp.family == FamInterpreted and lp.hPre == 0) implies lp.coprocessorFormsOpened
    )

  // Adopted INIT-008: Required INIT AIRs are present with fixed menu heights and <= MAX_APP_VK_COLS columns.
  val inv_INIT_008: bool =
    leafProofs.keys().forall(slot =>
      val lp = leafProofs.get(slot)
      (lp.family == FamInterpreted) implies (
        lp.requiredInitAirsPresent
          and lp.appVkColsUsed <= MAX_APP_VK_COLS
          and lp.airHeights == programCatalog(lp.progId).admittedHeights
      )
    )

  // Master safety conjunction across all checkable Leaf Proving & adopted invariants
  val inv: bool = and {
    inv_LFP_001,
    inv_LFP_002,
    inv_LFP_003,
    inv_LFP_004,
    inv_LFP_005,
    inv_LFP_006,
    inv_LFP_007,
    inv_LFP_008,
    inv_LFP_009,
    inv_LFP_010,
    inv_LFP_011,
    inv_LFP_012,
    inv_RSL_001,
    inv_RSL_002,
    inv_RSL_003,
    inv_RSL_004,
    inv_RSL_005,
    inv_RSL_006,
    inv_RSL_007,
    inv_RSL_008,
    inv_SPL_004,
    inv_INIT_001,
    inv_INIT_002,
    inv_INIT_003,
    inv_INIT_004,
    inv_INIT_007,
    inv_INIT_008,
  }

  // Pure consistency checks for SPEC-CHECKED candidates and cross-requirement findings:
  // Finding Q-leaf-proving-1: literal LFP-003 max 8 AIRs vs RSL-002 9-AIR bucket with FP-1
  val consistency_LFP_003_vs_RSL_002: bool =
    RSL_BUCKET_HIGH <= LFP_MAX_APP_AIRS

  // Finding Q-leaf-proving-2: RSL-004 (keccak256) + RSL-007 (persistentHash) in one program vs LFP-003 / GP-001
  val consistency_RSL_004_007_vs_LFP_003: bool =
    not(HAS_KECCAK_KIT) or (programCatalog(7).nonFp1AppAirs <= LFP_MAX_APP_AIRS)

  // LPC-018 (SPEC-CHECKED candidate): every modeled refusal tamper maps to a non-RefNone RefusalCode
  // and leaves TamperNone as RefNone, so negative test coverage never contradicts honest acceptance.
  pure val consistency_LPC_018: bool =
    refusalCodeOfTamper(TamperNone) == RefNone
      and Set(
        TamperSwappedHeights,
        TamperWrongProgDSlot,
        TamperBadFp1Outcome,
        TamperWrongTapeInput,
        TamperJubjubOutSubgroup,
        TamperMisalignedSha,
        TamperNonCanonicalWord,
        TamperInitWitnessGate
      ).forall(t => refusalCodeOfTamper(t) != RefNone)

  // =========================================================================
  // 9. Negated Reachability Witnesses (MUST be violated in leaf_proving_main)
  // =========================================================================

  val reach_rsl_6air_proved: bool = not(
    leafProofs.keys().exists(slot =>
      val lp = leafProofs.get(slot)
      lp.family == FamRsl and programCatalog(lp.progId).totalAppAirs == RSL_BUCKET_LOW
    )
  )

  val reach_rsl_9air_sha_proved: bool = not(
    leafProofs.keys().exists(slot =>
      val lp = leafProofs.get(slot)
      lp.progId == 4 and lp.n1PreHash == UNIVERSAL_N1_PREHASH
    )
  )

  val reach_rsl_9air_keccak_secp_proved: bool = not(
    leafProofs.keys().exists(slot =>
      val lp = leafProofs.get(slot)
      lp.progId == 5 and lp.n1PreHash == UNIVERSAL_N1_PREHASH
    )
  )

  val reach_rsl_jubjub_proved: bool = not(
    appProofs.keys().exists(slot =>
      val ap = appProofs.get(slot)
      ap.usesJubjub and ap.jubjubCheck == JubjubValidSubgroup
    )
  )

  val reach_interpreted_multishard_proved: bool = not(
    leafProofs.keys().contains(shardSlotId(5, 0))
      and leafProofs.keys().contains(shardSlotId(5, 1))
  )

  val reach_multi_shard_ready_published: bool = not(
    readyEvents.keys().exists(rId => runCatalog(rId).shardCount > 1)
  )

  val reach_n1_accepted: bool = not(
    RUN_IDS.exists(rId => n1Accepted.get(rId).size() > 0)
  )

  val reach_n1_wrong_progd_refused: bool = not(
    refusalLog.exists(ev => ev.code == RefN1WrongProgD)
  )

  val reach_over_airs_refused: bool = not(
    refusalLog.exists(ev => ev.code == RefOverMaxAirs)
  )

  val reach_over_cols_refused: bool = not(
    refusalLog.exists(ev => ev.code == RefOverVkCols)
  )

  val reach_shard_tamper_refused: bool = not(
    refusalLog.exists(ev =>
      Set(
        RefBadShardHeights,
        RefBadFp1Outcome,
        RefWrongTapeInputD,
        RefBadJubjubRow,
        RefMisalignedSha,
        RefNonCanonicalWord,
        RefBadGenesisInit
      ).contains(ev.code)
    )
  )

  val reach_suite_proved: bool = not(
    SUITE_PROG_IDS.exists(pId => suiteTable.get(pId).disposition == SuiteProved)
  )

  val reach_suite_blocked: bool = not(
    SUITE_PROG_IDS.exists(pId => suiteTable.get(pId).disposition == SuiteBlockedVerifierErr)
  )
}

// ===========================================================================
// 10. Concrete Instances (main, 2 edge instances, 17 mutants, 1 conflict demo)
// ===========================================================================

module leaf_proving_main {
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
  ).*
}

// Edge instance 1: smallest legal capacities, optional kits disabled, tightest measured R_SL VK bound (16,346)
module leaf_proving_edge_minimal {
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
  ).*
}

// Edge instance 2: bounds right at their spec limits (30,000 cols admitted; 3 full shards; 5 suite calls = 5)
module leaf_proving_edge_exact_bounds {
  import leaf_proving(
    CHIP_SET_RSL       = 3,
    LFP_MAX_APP_AIRS   = 8,
    RSL_BUCKET_LOW     = 6,
    RSL_BUCKET_HIGH    = 9,
    MAX_APP_VK_COLS    = 30000,
    SHARD_STEP_CAP     = 2,
    MAX_SHARDS_PER_RUN = 3,
    SUITE_CALL_TOTAL   = 5,
    JUBJUB_COFACTOR    = 8,
    HAS_KECCAK_KIT     = true,
    HAS_SECP256K1_KIT  = true,
    MUTANT             = "none"
  ).*
}

// Mutant 1: breaks inv_LFP_001
module leaf_proving_m_bad_field_config {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "bad_field_config"
  ).*
}

// Mutant 2: breaks inv_LFP_002 and inv_RSL_001
module leaf_proving_m_non_universal_prehash {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "non_universal_prehash"
  ).*
}

// Mutant 3: breaks inv_LFP_003 and inv_RSL_002
module leaf_proving_m_allow_over_8_airs {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "allow_over_8_airs"
  ).*
}

// Mutant 4: breaks inv_LFP_004
module leaf_proving_m_allow_over_30000_cols {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "allow_over_30000_cols"
  ).*
}

// Mutant 5: breaks inv_LFP_005
module leaf_proving_m_swapped_air_heights {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "swapped_air_heights"
  ).*
}

// Mutant 6: breaks inv_LFP_006
module leaf_proving_m_unconstrained_progd {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "unconstrained_progd"
  ).*
}

// Mutant 7: breaks inv_LFP_007
module leaf_proving_m_n1_accepts_wrong_progd {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "n1_accepts_wrong_progd"
  ).*
}

// Mutant 8: breaks inv_LFP_008
module leaf_proving_m_rekey_on_long_run {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "rekey_on_long_run"
  ).*
}

// Mutant 9: breaks inv_LFP_009
module leaf_proving_m_bad_ready_event {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "bad_ready_event"
  ).*
}

// Mutant 10: breaks inv_LFP_010
module leaf_proving_m_unbound_fp1_outcome {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "unbound_fp1_outcome"
  ).*
}

// Mutant 11: breaks inv_LFP_011 (and inv_LFP_006)
module leaf_proving_m_zero_rsl_progd {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "zero_rsl_progd"
  ).*
}

// Mutant 12: breaks inv_LFP_012
module leaf_proving_m_unbound_tape_input {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "unbound_tape_input"
  ).*
}

// Mutant 13: breaks inv_RSL_003
module leaf_proving_m_accept_unverified_suite {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "accept_unverified_suite"
  ).*
}

// Mutant 14: breaks inv_RSL_006
module leaf_proving_m_skip_jubjub_subgroup {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "skip_jubjub_subgroup"
  ).*
}

// Mutant 15: breaks inv_RSL_007
module leaf_proving_m_allow_misaligned_sha {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "allow_misaligned_sha"
  ).*
}

// Mutant 16: breaks inv_RSL_008
module leaf_proving_m_skip_canonical_range {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "skip_canonical_range"
  ).*
}

// Mutant 17: breaks inv_INIT_001 and inv_INIT_002
module leaf_proving_m_init_witness_flag {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "init_witness_flag"
  ).*
}

// Contradiction witness instance (Finding Q-leaf-proving-1): literal LFP-003 (totalAppAirs <= 8)
// refuses 9-AIR R_SL programs d04 and s02 required by RSL-002 / LFP-002 / RSL-004 / RSL-007.
module leaf_proving_m_lfp003_literal_8_vs_9 {
  import leaf_proving(
    CHIP_SET_RSL = 3, LFP_MAX_APP_AIRS = 8, RSL_BUCKET_LOW = 6, RSL_BUCKET_HIGH = 9,
    MAX_APP_VK_COLS = 30000, SHARD_STEP_CAP = 4, MAX_SHARDS_PER_RUN = 3, SUITE_CALL_TOTAL = 142,
    JUBJUB_COFACTOR = 8, HAS_KECCAK_KIT = true, HAS_SECP256K1_KIT = true,
    MUTANT = "lfp003_literal_8_vs_9"
  ).*
}
