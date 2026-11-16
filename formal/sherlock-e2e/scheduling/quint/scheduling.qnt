// Quint formal specification for Sherlock E2E Bounded Context: Scheduling
// Lane: e2e-scheduling
// Own IDs: STM-001 .. STM-019 (sherlock-e2e-setup-mode), ELA-001 .. ELA-007 (sherlock-e2e-elastic-proving)
// Completion IDs: LPC-006 (sherlock-loop-completion)
// Adopted IDs modelled here: SPL-011 (sherlock-shard-plan), ST-001..008, PF-001..008, SC-001..008, SC-010..014 (SETUP-SPEC.md)

module scheduling {
  // =========================================================================
  // 1. Module Constants (Spec Numeric Bounds & Mutant Selector)
  // =========================================================================

  const PROFILE_FIELD_COUNT: int         // STM-006, PF-001: 11 JSON fields in sherlock-profile/1
  const GPU_ENTRY_FIELDS_REQUIRED: int   // STM-001, PF-002: 7 fields per `gpus` entry
  const WARMUP_REPS: int                 // STM-004, ST-004: 1 warm-up proof per shape/device
  const DEFAULT_CAL_REPS: int            // STM-004, ST-004: default T16_CAL_REPS = 3 timed proofs
  const CPU_CORES_PER_LEAF: int          // STM-011, SC-004: at most max(1, cores / 4) CPU leaves
  const MAX_LEAVES_BOUND: int            // STM-009, SC-003: MAX_LEAVES (2^24 = 16777216 in spec)
  const PIPE_HEADROOM_DIV: int           // STM-015, PF-007: 1/16 GPU memory headroom divisor (16)
  const FULL_OVERLAP_MIN_MB: int         // STM-015, PF-007: MiB threshold for full overlap without eviction
  const EVICT_OVERLAP_MIN_MB: int        // STM-015, PF-007: MiB threshold for coset eviction + overlap
  const STARK_EXTRA_MB: int              // STM-019, PF-008: STARK extra device MiB
  const WRAP_PEAK_MB: int                // STM-019, PF-008: wrap peak device MiB
  const RESIDENT_COSETS_MB: int          // STM-019, PF-008: resident cosets MiB kept under no-evict
  const WRAP_MARGIN_MB: int              // STM-019, PF-008: safety margin MiB for early wrap start
  const MUTANT_ID: int                   // 0 = sound; 1..22 = single-guard/binding mutants

  // Mutant ID tags
  pure val MUT_NONE: int                             = 0
  pure val MUT_STM002_UNIFIED_NOT_MARKED: int        = 1
  pure val MUT_STM003_IGNORE_CGROUP_CAP: int         = 2
  pure val MUT_STM005_WRITE_ON_BAD_CAL_PROOF: int    = 3
  pure val MUT_STM007_ACCEPT_ZERO_LEAF_S: int        = 4
  pure val MUT_STM008_IGNORE_KEYSET_MISMATCH: int    = 5
  pure val MUT_STM010_LIMIT_RAISES_BUDGET: int       = 6
  pure val MUT_STM011_IGNORE_HOST_CAP_ON_GPU: int    = 7
  pure val MUT_STM012_SCHEDULE_ZERO_FIT: int         = 8
  pure val MUT_STM013_BYPASS_CHECK_FREE: int         = 9
  pure val MUT_STM014_LEGACY_MENU_CHECK: int         = 10
  pure val MUT_STM015_WRONG_WRAP_POLICY: int         = 11
  pure val MUT_STM016_WRONG_DEFAULT_PATH: int        = 12
  pure val MUT_STM017_WRONG_TIE_ORDER: int           = 13
  pure val MUT_STM018_LOWER_BUDGET_FASTER: int       = 14
  pure val MUT_STM019_ADMIT_EARLY_WRAP_SHORT: int    = 15
  pure val MUT_ELA001_SCHEDULE_CHANGES_CLAIM: int    = 16
  pure val MUT_ELA002_DEVICE_SPECIFIC_L_KEY: int     = 17
  pure val MUT_ELA004_GPU_USED_WHEN_HIDDEN: int      = 18
  pure val MUT_ELA006_SKIP_LEAF_VERIFY_ON_JOIN: int  = 19
  pure val MUT_ELA007_INCOMPLETE_RUN_RECORD: int     = 20
  pure val MUT_LPC006_LONGER_RUN_MUTATES_PROGD: int  = 21
  pure val MUT_SPL011_UNLISTED_SHAPE_OR_UNOWNED: int = 22

  // =========================================================================
  // 2. Domain Enums & Cohesive Record Types
  // =========================================================================

  type BoxId =
    | BoxHuginn
    | BoxMuninn
    | BoxOdin

  type DeviceKind =
    | DevDiscreteGpu
    | DevUnifiedGpu
    | DevCpu

  type ProfileFormatTag =
    | FormatNone
    | FormatSherlockProfileV1
    | FormatBadVersion

  type ProfilePathKind =
    | ProfilePathDefaultBox
    | ProfilePathEnvOverride
    | ProfilePathWrongFallback

  type WrapPipelinePolicy =
    | PipeFullOverlap
    | PipeCosetEvict
    | PipeNoOverlap

  type PreparedRunId =
    | PrepRunA_Short
    | PrepRunB_Long
    | PrepRunC_FreshSalt

  type LimitPreset =
    | LimitNone
    | LimitRaiseMem80Gb
    | LimitLowMem4Gb
    | LimitGpuMemOneLeaf
    | LimitNoGpu
    | LimitTooSmall1Gb

  type MalformedProfileKind =
    | BadFormatTag
    | BadMissingField
    | BadZeroLeafTime

  type RefusalReason =
    | SchedRefusalNone
    | SchedRefusalBadCalibrationProof(BoxId)
    | SchedRefusalMalformedProfile((BoxId, MalformedProfileKind))
    | SchedRefusalUncalibratedKeySet((BoxId, int, int))
    | SchedRefusalNoShapeFitsBudget((BoxId, int))
    | SchedRefusalCheckFreeFailed(int)
    | SchedRefusalEarlyWrapShort((BoxId, int, int))
    | SchedRefusalJoinCorruptedLeaf((BoxId, BoxId, int))
    | SchedRefusalUnownedBox(BoxId)

  type BoxHwSpec = {
    boxOwned: bool,
    gpuCount: int,
    gpuHasOwnVram: bool,
    gpuVramTotalMb: int,       // -1 models JSON `null` on unified GPU (STM-002)
    gpuVramFreeMb: int,        // -1 models JSON `null` on unified GPU
    gpuComputeCap: int,
    gpuSmClockMhz: int,
    cpuModelCode: int,
    cores: int,
    ramTotalGb: int,
    memAvailableGb: int,
    hasCgroupLimit: bool,
    cgroupHeadroomGb: int,
    diskFreeGb: int,
  }

  type GpuProfileEntry = {
    gpuIndex: int,
    modelCode: int,
    computeCap: int,
    vramTotalMb: int,          // -1 models `null` when unified == true
    vramFreeMb: int,
    unified: bool,
    smClockMhz: int,
    entryFieldCount: int,
  }

  type CalEntry = {
    shapeIdx: int,
    devKind: DeviceKind,
    warmupCount: int,
    timedReps: int,
    leafMs: int,
    appMs: int,
    hostGb: int,
    deviceGb: int,
  }

  type BoxProfileRec = {
    written: bool,
    atDefaultBoxPath: bool,
    formatTag: ProfileFormatTag,
    fieldCount: int,
    hostBox: BoxId,
    keyRoot: int,
    keysD: int,
    gpuEntries: List[GpuProfileEntry],
    cpuModelCode: int,
    cores: int,
    ramTotalGb: int,
    memBudgetGb: int,
    diskFreeGb: int,
    calEntries: List[CalEntry],
    setupSec: int,
    allCalProofsVerified: bool,
  }

  type PreparedRunSpec = {
    prepId: PreparedRunId,
    progId: int,
    planD: int,
    progD: int,
    keyRoot: int,
    admittedShapes: Set[int],
    inputId: int,
    runSteps: int,
    runSalt: int,
    expectedClaimPub: int,
  }

  type ScheduledRunRec = {
    prepId: PreparedRunId,
    boxId: BoxId,
    boxOwned: bool,
    loadedPathKind: ProfilePathKind,
    hasEnvProfileOverride: bool,
    profileKeyRoot: int,
    profileKeysD: int,
    proverKeyRoot: int,
    proverKeysD: int,
    calibratedForKeySet: bool,
    progId: int,
    inputId: int,
    planD: int,
    progD: int,
    runSteps: int,
    runSalt: int,
    hasMemLimitOverride: bool,
    memLimitOverrideGb: int,
    hasGpuCountOverride: bool,
    gpuCountOverride: int,
    hasGpuMemOverride: bool,
    gpuMemOverrideGb: int,
    effectiveMemBudgetGb: int,
    effectiveGpuCount: int,
    effectiveGpuMemGb: int,
    shapeIdx: int,
    shapeStepCap: int,
    shapeInKeyRoot: bool,
    devKind: DeviceKind,
    hostGbPerLeaf: int,
    deviceGbPerLeaf: int,
    concurrency: int,
    cutLeafCount: int,
    cutTotalSteps: int,
    minLeafSteps: int,
    maxLeafSteps: int,
    forcedOverride: bool,
    overrideNamed: bool,
    usedLegacyMenuCheck: bool,
    estWallMs: int,
    wrapPolicy: WrapPipelinePolicy,
    scheduleStaysLocal: bool,
    usedKeySetTuple: (int, int, int, int, int, int),
    recordWritten: bool,
    shardCount: int,
    leafStageTimeMs: int,
    nodeStageTimeMs: int,
    peakHostMemGb: int,
    lStockVerified: bool,
    nodeAccepted: bool,
    claimPub: int,
  }

  type EarlyWrapCheckRec = {
    boxId: BoxId,
    wrapPolicy: WrapPipelinePolicy,
    freeDeviceMb: int,
    requiredMb: int,
    earlyWrapAdmitted: bool,
    startedAfterStarkStage: bool,
  }

  type CrossBoxJoinRec = {
    boxA: BoxId,
    boxB: BoxId,
    bothBoxesOwned: bool,
    manifestsEqualCombiner: bool,
    allLeavesStockVerified: bool,
    intervalsContiguous: bool,
    nodeProofStarted: bool,
    claimPub: int,
  }

  type SchedulingState = {
    proverKeyEpoch: int,
    proverKeysD: int,
    proverKeyRoot: int,
    keyRootHasScheduleMenu: bool,
    profiles: BoxId -> BoxProfileRec,
    provedRuns: Set[ScheduledRunRec],
    uncalibratedRecords: Set[(BoxId, PreparedRunId, bool)],
    earlyWrapChecks: Set[EarlyWrapCheckRec],
    completedJoins: Set[CrossBoxJoinRec],
    lastRefusal: RefusalReason,
  }

  // =========================================================================
  // 3. Domain Constants & Pure Functional Layer (Step 3)
  // =========================================================================

  pure val ALL_BOXES: Set[BoxId] = Set(BoxHuginn, BoxMuninn, BoxOdin)
  pure val ALL_PREP_RUNS: Set[PreparedRunId] = Set(PrepRunA_Short, PrepRunB_Long, PrepRunC_FreshSalt)
  pure val ALL_SHAPES: Set[int] = Set(0, 1, 2)
  pure val ADMITTED_KEYROOT_SHAPES: Set[int] = Set(0, 1)

  pure val T10M_KEYS_D: int = 100
  pure val T25_KEYS_D: int = 250
  pure val PROG1_KEY_ROOT: int = 701
  pure val PROG1_PLAN_D: int = 501
  pure val PROG1_PROG_D: int = 601

  pure def minInt(a: int, b: int): int = if (a < b) a else b
  pure def maxInt(a: int, b: int): int = if (a > b) a else b

  pure def ceilDiv(num: int, den: int): int =
    if (den <= 0) 0 else (num + den - 1) / den

  pure def shapeStepCapOf(sh: int): int =
    if (sh == 0) 4
    else if (sh == 1) 8
    else 16

  pure def deviceTieRank(d: DeviceKind): int =
    match d {
      | DevDiscreteGpu => 0
      | DevUnifiedGpu  => 0
      | DevCpu         => 1
    }

  pure def boxHardware(b: BoxId): BoxHwSpec =
    match b {
      | BoxHuginn => {
          boxOwned: true,
          gpuCount: 1,
          gpuHasOwnVram: true,
          gpuVramTotalMb: 16384,
          gpuVramFreeMb: 16384,
          gpuComputeCap: 120,
          gpuSmClockMhz: 2617,
          cpuModelCode: 101,
          cores: 16,
          ramTotalGb: 64,
          memAvailableGb: 64,
          hasCgroupLimit: true,
          cgroupHeadroomGb: 60,
          diskFreeGb: 500,
        }
      | BoxMuninn => {
          boxOwned: true,
          gpuCount: 1,
          gpuHasOwnVram: false,
          gpuVramTotalMb: -1,
          gpuVramFreeMb: -1,
          gpuComputeCap: 121,
          gpuSmClockMhz: 2400,
          cpuModelCode: 102,
          cores: 20,
          ramTotalGb: 128,
          memAvailableGb: 100,
          hasCgroupLimit: false,
          cgroupHeadroomGb: 0,
          diskFreeGb: 800,
        }
      | BoxOdin => {
          boxOwned: true,
          gpuCount: 1,
          gpuHasOwnVram: true,
          gpuVramTotalMb: 32768,
          gpuVramFreeMb: 32768,
          gpuComputeCap: 120,
          gpuSmClockMhz: 2700,
          cpuModelCode: 103,
          cores: 24,
          ramTotalGb: 128,
          memAvailableGb: 128,
          hasCgroupLimit: false,
          cgroupHeadroomGb: 0,
          diskFreeGb: 1000,
        }
    }

  pure def prepRunSpec(pr: PreparedRunId): PreparedRunSpec =
    match pr {
      | PrepRunA_Short => {
          prepId: PrepRunA_Short,
          progId: 1,
          planD: PROG1_PLAN_D,
          progD: PROG1_PROG_D,
          keyRoot: PROG1_KEY_ROOT,
          admittedShapes: ADMITTED_KEYROOT_SHAPES,
          inputId: 10,
          runSteps: 5,
          runSalt: 9001,
          expectedClaimPub: 41001,
        }
      | PrepRunB_Long => {
          prepId: PrepRunB_Long,
          progId: 1,
          planD: PROG1_PLAN_D,
          progD: PROG1_PROG_D,
          keyRoot: PROG1_KEY_ROOT,
          admittedShapes: ADMITTED_KEYROOT_SHAPES,
          inputId: 20,
          runSteps: 12,
          runSalt: 9002,
          expectedClaimPub: 42002,
        }
      | PrepRunC_FreshSalt => {
          prepId: PrepRunC_FreshSalt,
          progId: 1,
          planD: PROG1_PLAN_D,
          progD: PROG1_PROG_D,
          keyRoot: PROG1_KEY_ROOT,
          admittedShapes: ADMITTED_KEYROOT_SHAPES,
          inputId: 10,
          runSteps: 5,
          runSalt: 9003,
          expectedClaimPub: 41003,
        }
    }

  // STM-003: Effective setup memory budget is min(MemAvailable, cgroup headroom) when cgroup is set
  pure def computeSetupMemBudget(memAvailGb: int, hasCg: bool, cgGb: int, mutId: int): int =
    if (mutId == MUT_STM003_IGNORE_CGROUP_CAP) memAvailGb
    else if (hasCg) minInt(memAvailGb, cgGb)
    else memAvailGb

  // Build calibration entries for one box (STM-004, ST-004, ST-006)
  // Note: On BoxOdin GPU, Shape 0 (cap 4) and Shape 1 (cap 8) both take 2000ms per wave, creating an
  // exact tie on `estWallMs` for a 5-step run (2 leaves vs 1 leaf) that tests STM-017's tie-breaker!
  pure def buildCalEntriesForBox(b: BoxId, repsCount: int): List[CalEntry] =
    val gpuDev = match b {
      | BoxMuninn => DevUnifiedGpu
      | _         => DevDiscreteGpu
    }
    val gpuLeaf0 = match b {
      | BoxHuginn => 2200
      | BoxMuninn => 23000
      | BoxOdin   => 2000
    }
    val gpuLeaf1 = match b {
      | BoxHuginn => 3000
      | BoxMuninn => 28000
      | BoxOdin   => 2000
    }
    val cpuLeaf0 = match b {
      | BoxHuginn => 8800
      | BoxMuninn => 12000
      | BoxOdin   => 10000
    }
    val cpuLeaf1 = match b {
      | BoxHuginn => 12000
      | BoxMuninn => 16000
      | BoxOdin   => 13000
    }
    List(
      { shapeIdx: 0, devKind: gpuDev, warmupCount: WARMUP_REPS, timedReps: repsCount, leafMs: gpuLeaf0, appMs: 500, hostGb: 4, deviceGb: 2 },
      { shapeIdx: 1, devKind: gpuDev, warmupCount: WARMUP_REPS, timedReps: repsCount, leafMs: gpuLeaf1, appMs: 800, hostGb: 8, deviceGb: 4 },
      { shapeIdx: 0, devKind: DevCpu, warmupCount: WARMUP_REPS, timedReps: repsCount, leafMs: cpuLeaf0, appMs: 1500, hostGb: 4, deviceGb: 0 },
      { shapeIdx: 1, devKind: DevCpu, warmupCount: WARMUP_REPS, timedReps: repsCount, leafMs: cpuLeaf1, appMs: 2400, hostGb: 8, deviceGb: 0 }
    )

  // STM-001..006: Build a valid `sherlock-profile/1` record from hardware probe & calibration
  pure def buildProfileForBox(b: BoxId, keyRt: int, kD: int, repsCount: int, calValid: bool, mutId: int): BoxProfileRec =
    val hw = boxHardware(b)
    val isUnif =
      if (mutId == MUT_STM002_UNIFIED_NOT_MARKED) false
      else not(hw.gpuHasOwnVram)
    val gpuEnt: GpuProfileEntry = {
      gpuIndex: 0,
      modelCode: if (b == BoxMuninn) 910 else if (b == BoxOdin) 5090 else 5080,
      computeCap: hw.gpuComputeCap,
      vramTotalMb: hw.gpuVramTotalMb,
      vramFreeMb: hw.gpuVramFreeMb,
      unified: isUnif,
      smClockMhz: hw.gpuSmClockMhz,
      entryFieldCount: GPU_ENTRY_FIELDS_REQUIRED,
    }
    val budgetGb = computeSetupMemBudget(hw.memAvailableGb, hw.hasCgroupLimit, hw.cgroupHeadroomGb, mutId)
    {
      written: true,
      atDefaultBoxPath: true,
      formatTag: FormatSherlockProfileV1,
      fieldCount: PROFILE_FIELD_COUNT,
      hostBox: b,
      keyRoot: keyRt,
      keysD: kD,
      gpuEntries: List(gpuEnt),
      cpuModelCode: hw.cpuModelCode,
      cores: hw.cores,
      ramTotalGb: hw.ramTotalGb,
      memBudgetGb: budgetGb,
      diskFreeGb: hw.diskFreeGb,
      calEntries: buildCalEntriesForBox(b, repsCount),
      setupSec: 120,
      allCalProofsVerified: calValid,
    }

  // STM-007: Validate `sherlock-profile/1` format, field count, and finite positive `leaf_s`
  pure def isProfileValid(prof: BoxProfileRec): bool =
    prof.written
      and prof.formatTag == FormatSherlockProfileV1
      and prof.fieldCount == PROFILE_FIELD_COUNT
      and prof.calEntries.length() > 0
      and prof.calEntries.indices().forall(i => prof.calEntries.nth(i).leafMs > 0)

  // Profile GPU memory in GB (discrete VRAM in GB, or shared host budget on unified GPU)
  pure def profileGpuMemGb(prof: BoxProfileRec): int =
    if (prof.gpuEntries.length() == 0) 0
    else {
      val g0 = prof.gpuEntries.nth(0)
      if (g0.unified) prof.memBudgetGb else (g0.vramFreeMb / 1024)
    }

  // Profile largest GPU memory in MiB (for STM-015 / PF-007 wrap pipeline policy)
  pure def profileLargestGpuMb(prof: BoxProfileRec): int =
    if (prof.gpuEntries.length() == 0) 0
    else {
      val g0 = prof.gpuEntries.nth(0)
      if (g0.unified) (prof.memBudgetGb * 1024) else g0.vramTotalMb
    }

  // STM-015, PF-007: Derive wrap pipeline policy from largest GPU memory less 1/16 headroom
  pure def deriveWrapPolicy(largestGpuMb: int, mutId: int): WrapPipelinePolicy =
    if (mutId == MUT_STM015_WRONG_WRAP_POLICY) PipeFullOverlap
    else {
      val usableMb = largestGpuMb - (largestGpuMb / PIPE_HEADROOM_DIV)
      if (usableMb >= FULL_OVERLAP_MIN_MB) PipeFullOverlap
      else if (usableMb >= EVICT_OVERLAP_MIN_MB) PipeCosetEvict
      else PipeNoOverlap
    }

  // STM-019, PF-008: Required free device MiB for early wrap start (`starkExtra + wrapTail + margin`)
  pure def earlyWrapRequiredMb(pol: WrapPipelinePolicy): int =
    val wrapTail = match pol {
      | PipeFullOverlap => WRAP_PEAK_MB - RESIDENT_COSETS_MB
      | _               => WRAP_PEAK_MB
    }
    STARK_EXTRA_MB + wrapTail + WRAP_MARGIN_MB

  // STM-010: Apply operator limits (`SHERLOCK_MEM_BUDGET`, `SHERLOCK_GPUS`, `SHERLOCK_GPU_MEM`)
  pure def effectiveLimitsForPreset(
    prof: BoxProfileRec,
    preset: LimitPreset,
    mutId: int
  ): {
    hasMemOv: bool, memOvGb: int,
    hasGpuCntOv: bool, gpuCntOv: int,
    hasGpuMemOv: bool, gpuMemOvGb: int,
    effMemGb: int, effGpuCnt: int, effGpuMemGb: int
  } =
    val profGpuCnt = prof.gpuEntries.length()
    val profGpuGb = profileGpuMemGb(prof)
    match preset {
      | LimitNone => {
          hasMemOv: false, memOvGb: 0,
          hasGpuCntOv: false, gpuCntOv: 0,
          hasGpuMemOv: false, gpuMemOvGb: 0,
          effMemGb: prof.memBudgetGb, effGpuCnt: profGpuCnt, effGpuMemGb: profGpuGb,
        }
      | LimitRaiseMem80Gb => {
          val effM = if (mutId == MUT_STM010_LIMIT_RAISES_BUDGET) 80 else minInt(80, prof.memBudgetGb)
          {
            hasMemOv: true, memOvGb: 80,
            hasGpuCntOv: false, gpuCntOv: 0,
            hasGpuMemOv: false, gpuMemOvGb: 0,
            effMemGb: effM, effGpuCnt: profGpuCnt, effGpuMemGb: if (prof.gpuEntries.nth(0).unified) effM else profGpuGb,
          }
        }
      | LimitLowMem4Gb => {
          val effM = minInt(4, prof.memBudgetGb)
          val effGM = if (prof.gpuEntries.nth(0).unified) minInt(effM, profGpuGb) else profGpuGb
          {
            hasMemOv: true, memOvGb: 4,
            hasGpuCntOv: false, gpuCntOv: 0,
            hasGpuMemOv: false, gpuMemOvGb: 0,
            effMemGb: effM, effGpuCnt: profGpuCnt, effGpuMemGb: effGM,
          }
        }
      | LimitGpuMemOneLeaf => {
          val effGM = minInt(2, profGpuGb)
          val effM = if (prof.gpuEntries.nth(0).unified) minInt(6, prof.memBudgetGb) else prof.memBudgetGb
          {
            hasMemOv: prof.gpuEntries.nth(0).unified, memOvGb: 6,
            hasGpuCntOv: false, gpuCntOv: 0,
            hasGpuMemOv: true, gpuMemOvGb: 2,
            effMemGb: effM, effGpuCnt: profGpuCnt, effGpuMemGb: if (prof.gpuEntries.nth(0).unified) effM else effGM,
          }
        }
      | LimitNoGpu => {
          hasMemOv: false, memOvGb: 0,
          hasGpuCntOv: true, gpuCntOv: 0,
          hasGpuMemOv: false, gpuMemOvGb: 0,
          effMemGb: prof.memBudgetGb, effGpuCnt: 0, effGpuMemGb: 0,
        }
      | LimitTooSmall1Gb => {
          val effM = minInt(1, prof.memBudgetGb)
          {
            hasMemOv: true, memOvGb: 1,
            hasGpuCntOv: false, gpuCntOv: 0,
            hasGpuMemOv: false, gpuMemOvGb: 0,
            effMemGb: effM, effGpuCnt: profGpuCnt, effGpuMemGb: if (prof.gpuEntries.nth(0).unified) effM else profGpuGb,
          }
        }
    }

  // STM-011, SC-004: Leaf concurrency for one calibration entry under effective limits
  pure def computeLeafConcurrency(
    ce: CalEntry,
    cores: int,
    effMemGb: int,
    effGpuCnt: int,
    effGpuMemGb: int,
    mutId: int
  ): int =
    match ce.devKind {
      | DevCpu => {
          val byHost = if (ce.hostGb > 0) (effMemGb / ce.hostGb) else 0
          val cpuCap = maxInt(1, cores / CPU_CORES_PER_LEAF)
          val raw = minInt(byHost, cpuCap)
          if (mutId == MUT_STM012_SCHEDULE_ZERO_FIT and raw == 0) 1 else raw
        }
      | DevDiscreteGpu => {
          val gpuAllowed = (effGpuCnt > 0) or (mutId == MUT_ELA004_GPU_USED_WHEN_HIDDEN)
          if (not(gpuAllowed)) 0
          else {
            val gpuMemForCalc = if (mutId == MUT_ELA004_GPU_USED_WHEN_HIDDEN and effGpuCnt == 0) 16 else effGpuMemGb
            val byVram = if (ce.deviceGb > 0) (gpuMemForCalc / ce.deviceGb) else 0
            val byHost = if (ce.hostGb > 0) (effMemGb / ce.hostGb) else 0
            if (mutId == MUT_STM011_IGNORE_HOST_CAP_ON_GPU) byVram
            else minInt(byVram, byHost)
          }
        }
      | DevUnifiedGpu => {
          val gpuAllowed = (effGpuCnt > 0) or (mutId == MUT_ELA004_GPU_USED_WHEN_HIDDEN)
          if (not(gpuAllowed)) 0
          else {
            val poolGb = minInt(effMemGb, effGpuMemGb)
            val combined = ce.deviceGb + ce.hostGb
            if (combined > 0) (poolGb / combined) else 0
          }
        }
    }

  // Estimated wall time (`waves * leafMs`) for a candidate calibration entry
  pure def estimateWallMs(runSteps: int, ce: CalEntry, conc: int, isLoweredBudget: bool, mutId: int): int =
    val cap = shapeStepCapOf(ce.shapeIdx)
    val leafCnt = ceilDiv(runSteps, cap)
    val waves = ceilDiv(leafCnt, conc)
    if (mutId == MUT_STM018_LOWER_BUDGET_FASTER and isLoweredBudget)
      maxInt(100, (waves * ce.leafMs) / 4)
    else
      waves * ce.leafMs

  // STM-017, SC-005: Lexicographic comparison `(estWallMs, leafCnt, shapeIdx, deviceTieRank)`
  pure def isCandidateBetter(
    cand: { ok: bool, ce: CalEntry, conc: int, leafCnt: int, estMs: int },
    best: { ok: bool, ce: CalEntry, conc: int, leafCnt: int, estMs: int },
    mutId: int
  ): bool =
    if (not(cand.ok)) false
    else if (not(best.ok)) true
    else if (cand.estMs != best.estMs) (cand.estMs < best.estMs)
    else if (mutId == MUT_STM017_WRONG_TIE_ORDER) {
      // Wrong tie order: picks smaller shapeIdx (or CPU) BEFORE fewer leaves!
      if (cand.ce.shapeIdx != best.ce.shapeIdx) (cand.ce.shapeIdx < best.ce.shapeIdx)
      else (deviceTieRank(cand.ce.devKind) > deviceTieRank(best.ce.devKind))
    } else {
      if (cand.leafCnt != best.leafCnt) (cand.leafCnt < best.leafCnt)
      else if (cand.ce.shapeIdx != best.ce.shapeIdx) (cand.ce.shapeIdx < best.ce.shapeIdx)
      else (deviceTieRank(cand.ce.devKind) < deviceTieRank(best.ce.devKind))
    }

  // Evaluate and pick the best feasible `(shape, device)` over any list of calibration entries
  pure def pickFromCalList(
    entries: List[CalEntry],
    admittedShapes: Set[int],
    cores: int,
    runSteps: int,
    effMemGb: int,
    effGpuCnt: int,
    effGpuMemGb: int,
    isLoweredBudget: bool,
    mutId: int
  ): { ok: bool, ce: CalEntry, conc: int, leafCnt: int, estMs: int } =
    val dummyCe: CalEntry = {
      shapeIdx: 99, devKind: DevCpu, warmupCount: 0, timedReps: 0,
      leafMs: 0, appMs: 0, hostGb: 0, deviceGb: 0
    }
    val initBest = { ok: false, ce: dummyCe, conc: 0, leafCnt: 0, estMs: 0 }
    range(0, entries.length()).foldl(initBest, (acc, idx) => {
      val ce = entries.nth(idx)
      val cap = shapeStepCapOf(ce.shapeIdx)
      val leafCnt = ceilDiv(runSteps, cap)
      val conc = computeLeafConcurrency(ce, cores, effMemGb, effGpuCnt, effGpuMemGb, mutId)
      val shapeOk = admittedShapes.contains(ce.shapeIdx) and leafCnt >= 1 and leafCnt <= MAX_LEAVES_BOUND
      val cand = {
        ok: shapeOk and conc >= 1,
        ce: ce,
        conc: conc,
        leafCnt: leafCnt,
        estMs: if (conc >= 1) estimateWallMs(runSteps, ce, conc, isLoweredBudget, mutId) else 0,
      }
      if (isCandidateBetter(cand, acc, mutId)) cand else acc
    })

  // STM-013, SC-010: Pure `check_free` predicate (no schedule menu or shard-count list!)
  pure def checkFreeSchedule(
    admittedShapes: Set[int],
    shapeIdx: int,
    runSteps: int,
    cutLeafCount: int,
    cutTotalSteps: int,
    minLeafSteps: int,
    maxLeafSteps: int
  ): bool =
    val cap = shapeStepCapOf(shapeIdx)
    admittedShapes.contains(shapeIdx)
      and cutLeafCount >= 1
      and cutLeafCount <= MAX_LEAVES_BOUND
      and cutTotalSteps == runSteps
      and minLeafSteps >= 1
      and maxLeafSteps >= minLeafSteps
      and maxLeafSteps <= cap

  // Universal + per-program key tuple `(app, leaf, N1, N2, N3, L)` (ELA-002)
  pure def keyTupleForRun(progId: int, keysD: int, dev: DeviceKind, mutId: int): (int, int, int, int, int, int) =
    val lKey =
      if (mutId == MUT_ELA002_DEVICE_SPECIFIC_L_KEY and dev == DevCpu) (keysD + 99)
      else (keysD + 4)
    (1000 + progId, 2000 + progId, keysD + 1, keysD + 2, keysD + 3, lKey)

  // =========================================================================
  // 4. State Variables & Initialization (Step 4)
  // =========================================================================

  var sched: SchedulingState
  var lastOp: str

  pure val EMPTY_PROFILE: BoxProfileRec = {
    written: false,
    atDefaultBoxPath: false,
    formatTag: FormatNone,
    fieldCount: 0,
    hostBox: BoxHuginn,
    keyRoot: 0,
    keysD: 0,
    gpuEntries: List(),
    cpuModelCode: 0,
    cores: 0,
    ramTotalGb: 0,
    memBudgetGb: 0,
    diskFreeGb: 0,
    calEntries: List(),
    setupSec: 0,
    allCalProofsVerified: false,
  }

  action init: bool = all {
    sched' = {
      proverKeyEpoch: 1,
      proverKeysD: T25_KEYS_D,
      proverKeyRoot: PROG1_KEY_ROOT,
      keyRootHasScheduleMenu: (MUTANT_ID == MUT_STM014_LEGACY_MENU_CHECK),
      profiles: ALL_BOXES.mapBy(_ => EMPTY_PROFILE),
      provedRuns: Set(),
      uncalibratedRecords: Set(),
      earlyWrapChecks: Set(),
      completedJoins: Set(),
      lastRefusal: SchedRefusalNone,
    },
    lastOp' = "init",
  }

  // =========================================================================
  // 5. Guard Predicates & Thin Actions (Tagged with EARS IDs)
  // =========================================================================

  pure def canRunSetupOnBox(s: SchedulingState, b: BoxId): bool =
    not(s.profiles.get(b).written) or s.profiles.get(b).keysD != s.proverKeysD or not( isProfileValid(s.profiles.get(b)) )

  // Implements STM-001, STM-002, STM-003, STM-004, STM-006 (ST-001..004, ST-006, ST-008, PF-001..004):
  // Probe GPUs & host (with cgroup min), calibrate shapes on CPU + GPU with warmup + T16_CAL_REPS, write sherlock-profile/1.
  action runSetupOnBox(b: BoxId): bool = all {
    canRunSetupOnBox(sched, b),
    val prof = buildProfileForBox(b, sched.proverKeyRoot, sched.proverKeysD, DEFAULT_CAL_REPS, true, MUTANT_ID)
    sched' = {
      ...sched,
      profiles: sched.profiles.put(b, prof),
      lastRefusal: SchedRefusalNone,
    },
    lastOp' = "runSetupOnBox",
  }

  // Implements STM-005 (ST-005): Stock verifier refuses a corrupted calibration proof -> stop setup and write no profile.
  action abortSetupBadCalProof(b: BoxId): bool = all {
    not(sched.profiles.get(b).written),
    val postProf =
      if (MUTANT_ID == MUT_STM005_WRITE_ON_BAD_CAL_PROOF)
        buildProfileForBox(b, sched.proverKeyRoot, sched.proverKeysD, DEFAULT_CAL_REPS, false, MUTANT_ID)
      else
        EMPTY_PROFILE
    sched' = {
      ...sched,
      profiles: sched.profiles.put(b, postProf),
      lastRefusal: SchedRefusalBadCalibrationProof(b),
    },
    lastOp' = "abortSetupBadCalProof",
  }

  // Inject a malformed profile (`format != sherlock-profile/1`, missing field, or `leaf_s == 0`) to test STM-007
  action injectMalformedProfile(b: BoxId, badKind: MalformedProfileKind): bool = all {
    not(sched.profiles.get(b).written),
    val base = buildProfileForBox(b, sched.proverKeyRoot, sched.proverKeysD, DEFAULT_CAL_REPS, true, MUTANT_ID)
    val zeroCal = range(0, base.calEntries.length()).foldl(List(), (acc, i) => {
      val e = base.calEntries.nth(i)
      acc.append(if (i == 0) { ...e, leafMs: 0 } else e)
    })
    val badProf = match badKind {
      | BadFormatTag    => { ...base, formatTag: FormatBadVersion }
      | BadMissingField => { ...base, fieldCount: PROFILE_FIELD_COUNT - 1 }
      | BadZeroLeafTime => { ...base, calEntries: zeroCal }
    }
    sched' = {
      ...sched,
      profiles: sched.profiles.put(b, badProf),
    },
    lastOp' = "injectMalformedProfile",
  }

  // Implements STM-007 (PF-005): Prover refuses malformed profile and names the bad field/reason.
  action refuseMalformedProfile(b: BoxId): bool =
    val prof = sched.profiles.get(b)
    all {
      prof.written,
      not(isProfileValid(prof)),
      val kind =
        if (prof.formatTag != FormatSherlockProfileV1) BadFormatTag
        else if (prof.fieldCount != PROFILE_FIELD_COUNT) BadMissingField
        else BadZeroLeafTime
      sched' = {
        ...sched,
        lastRefusal: SchedRefusalMalformedProfile((b, kind)),
      },
      lastOp' = "refuseMalformedProfile",
    }

  // Install a pre-rekey T10m profile on box `b` to exercise STM-008 (`PF-006`) after the T25 rekey
  action installStaleT10mProfile(b: BoxId): bool = all {
    not(sched.profiles.get(b).written),
    val staleProf = buildProfileForBox(b, sched.proverKeyRoot, T10M_KEYS_D, DEFAULT_CAL_REPS, true, MUTANT_ID)
    sched' = {
      ...sched,
      profiles: sched.profiles.put(b, staleProf),
    },
    lastOp' = "installStaleT10mProfile",
  }

  // Implements STM-008 (PF-006): Profile `key_root` or `keys_d` differs from prover's universal key set ->
  // record run as not calibrated for this key set.
  action recordUncalibratedProfileKeyMismatch(b: BoxId, pr: PreparedRunId): bool =
    val prof = sched.profiles.get(b)
    val pSpec = prepRunSpec(pr)
    all {
      isProfileValid(prof),
      prof.keysD != sched.proverKeysD or prof.keyRoot != pSpec.keyRoot,
      val calFlag = (MUTANT_ID == MUT_STM008_IGNORE_KEYSET_MISMATCH)
      sched' = {
        ...sched,
        uncalibratedRecords: sched.uncalibratedRecords.union(Set((b, pr, calFlag))),
        lastRefusal: SchedRefusalUncalibratedKeySet((b, prof.keysD, sched.proverKeysD)),
      },
      lastOp' = "recordUncalibratedProfileKeyMismatch",
    }

  // Pure guard for `scheduleAndProveFromProfile`
  pure def canScheduleFromProfile(
    s: SchedulingState,
    b: BoxId,
    pr: PreparedRunId,
    preset: LimitPreset,
    useEnvPathOverride: bool,
    mutId: int
  ): bool =
    val prof = s.profiles.get(b)
    val pSpec = prepRunSpec(pr)
    val profOk =
      if (mutId == MUT_STM007_ACCEPT_ZERO_LEAF_S) prof.written
      else isProfileValid(prof)
    if (not(profOk)) false
    else if (prof.keysD != s.proverKeysD or prof.keyRoot != pSpec.keyRoot) false
    else {
      val lim = effectiveLimitsForPreset(prof, preset, mutId)
      val isLow = lim.effMemGb < prof.memBudgetGb
      val pick = pickFromCalList(
        prof.calEntries,
        pSpec.admittedShapes,
        prof.cores,
        pSpec.runSteps,
        lim.effMemGb,
        lim.effGpuCnt,
        lim.effGpuMemGb,
        isLow,
        mutId
      )
      pick.ok and (useEnvPathOverride or prof.atDefaultBoxPath)
    }

  // Implements STM-009, STM-010, STM-011, STM-013, STM-014, STM-015, STM-016, STM-017, STM-018,
  //            ELA-001, ELA-002, ELA-003, ELA-004, ELA-005, ELA-007, LPC-006, SPL-011:
  // Derive optimal schedule from box profile & runtime limits, check_free, prove, and write run record.
  action scheduleAndProveFromProfile(
    b: BoxId,
    pr: PreparedRunId,
    preset: LimitPreset,
    useEnvPathOverride: bool
  ): bool =
    val prof = sched.profiles.get(b)
    val hw = boxHardware(b)
    val pSpec = prepRunSpec(pr)
    val lim = effectiveLimitsForPreset(prof, preset, MUTANT_ID)
    val isLow = lim.effMemGb < prof.memBudgetGb
    val pick = pickFromCalList(
      prof.calEntries,
      pSpec.admittedShapes,
      prof.cores,
      pSpec.runSteps,
      lim.effMemGb,
      lim.effGpuCnt,
      lim.effGpuMemGb,
      isLow,
      MUTANT_ID
    )
    all {
      canScheduleFromProfile(sched, b, pr, preset, useEnvPathOverride, MUTANT_ID),
      val cap = shapeStepCapOf(pick.ce.shapeIdx)
      val rem = pSpec.runSteps - (pick.leafCnt - 1) * cap
      val minSteps = if (rem < cap) rem else cap
      val maxSteps = if (pick.leafCnt == 1) pSpec.runSteps else cap
      val pathUsed =
        if (useEnvPathOverride) ProfilePathEnvOverride
        else if (MUTANT_ID == MUT_STM016_WRONG_DEFAULT_PATH) ProfilePathWrongFallback
        else ProfilePathDefaultBox
      val wrapPol = deriveWrapPolicy(profileLargestGpuMb(prof), MUTANT_ID)
      val claim =
        if (MUTANT_ID == MUT_ELA001_SCHEDULE_CHANGES_CLAIM) (pSpec.expectedClaimPub + pick.leafCnt)
        else pSpec.expectedClaimPub
      val usedProgD =
        if (MUTANT_ID == MUT_LPC006_LONGER_RUN_MUTATES_PROGD and pSpec.runSteps > 5) (pSpec.progD + 77)
        else pSpec.progD
      val recWritten = not(MUTANT_ID == MUT_ELA007_INCOMPLETE_RUN_RECORD)
      val rec: ScheduledRunRec = {
        prepId: pr,
        boxId: b,
        boxOwned: hw.boxOwned,
        loadedPathKind: pathUsed,
        hasEnvProfileOverride: useEnvPathOverride,
        profileKeyRoot: prof.keyRoot,
        profileKeysD: prof.keysD,
        proverKeyRoot: pSpec.keyRoot,
        proverKeysD: sched.proverKeysD,
        calibratedForKeySet: (prof.keyRoot == pSpec.keyRoot and prof.keysD == sched.proverKeysD),
        progId: pSpec.progId,
        inputId: pSpec.inputId,
        planD: pSpec.planD,
        progD: usedProgD,
        runSteps: pSpec.runSteps,
        runSalt: pSpec.runSalt,
        hasMemLimitOverride: lim.hasMemOv,
        memLimitOverrideGb: lim.memOvGb,
        hasGpuCountOverride: lim.hasGpuCntOv,
        gpuCountOverride: lim.gpuCntOv,
        hasGpuMemOverride: lim.hasGpuMemOv,
        gpuMemOverrideGb: lim.gpuMemOvGb,
        effectiveMemBudgetGb: lim.effMemGb,
        effectiveGpuCount: lim.effGpuCnt,
        effectiveGpuMemGb: lim.effGpuMemGb,
        shapeIdx: pick.ce.shapeIdx,
        shapeStepCap: cap,
        shapeInKeyRoot: pSpec.admittedShapes.contains(pick.ce.shapeIdx),
        devKind: pick.ce.devKind,
        hostGbPerLeaf: pick.ce.hostGb,
        deviceGbPerLeaf: pick.ce.deviceGb,
        concurrency: pick.conc,
        cutLeafCount: pick.leafCnt,
        cutTotalSteps: pSpec.runSteps,
        minLeafSteps: minSteps,
        maxLeafSteps: maxSteps,
        forcedOverride: false,
        overrideNamed: false,
        usedLegacyMenuCheck: sched.keyRootHasScheduleMenu,
        estWallMs: pick.estMs,
        wrapPolicy: wrapPol,
        scheduleStaysLocal: true,
        usedKeySetTuple: keyTupleForRun(pSpec.progId, sched.proverKeysD, pick.ce.devKind, MUTANT_ID),
        recordWritten: recWritten,
        shardCount: if (recWritten) pick.leafCnt else 0,
        leafStageTimeMs: if (recWritten) pick.estMs else 0,
        nodeStageTimeMs: if (recWritten) 5000 else 0,
        peakHostMemGb: if (recWritten) maxInt(1, pick.conc * pick.ce.hostGb) else 0,
        lStockVerified: true,
        nodeAccepted: true,
        claimPub: claim,
      }
      sched' = {
        ...sched,
        provedRuns: sched.provedRuns.union(Set(rec)),
        lastRefusal: SchedRefusalNone,
      },
      lastOp' = "scheduleAndProveFromProfile",
    }

  // Implements STM-012, ELA-003 (Scenario: A 1 GB budget fits no leaf on any device -> refuse before any proof)
  action refuseUnfittableBudget(b: BoxId, pr: PreparedRunId): bool =
    val prof = sched.profiles.get(b)
    val pSpec = prepRunSpec(pr)
    val lim = effectiveLimitsForPreset(prof, LimitTooSmall1Gb, MUTANT_ID)
    val pick = pickFromCalList(
      prof.calEntries,
      pSpec.admittedShapes,
      prof.cores,
      pSpec.runSteps,
      lim.effMemGb,
      lim.effGpuCnt,
      lim.effGpuMemGb,
      true,
      MUT_NONE
    )
    all {
      isProfileValid(prof),
      not(pick.ok),
      sched' = {
        ...sched,
        lastRefusal: SchedRefusalNoShapeFitsBudget((b, lim.effMemGb)),
      },
      lastOp' = "refuseUnfittableBudget",
    }

  // Pure guard for forced-cut override (`T16_CUT` / `T16_SHAPE` in STM-013, SC-008, ELA-001, SPL-011)
  pure def canScheduleForcedCut(
    s: SchedulingState,
    b: BoxId,
    pr: PreparedRunId,
    shapeIdx: int,
    cutLeafCount: int,
    cutTotalSteps: int,
    minLeafSteps: int,
    maxLeafSteps: int,
    onOwnedBox: bool,
    mutId: int
  ): bool =
    val prof = s.profiles.get(b)
    val pSpec = prepRunSpec(pr)
    if (not(isProfileValid(prof)) or prof.keysD != s.proverKeysD) false
    else if (mutId == MUT_STM013_BYPASS_CHECK_FREE) true
    else if (mutId == MUT_SPL011_UNLISTED_SHAPE_OR_UNOWNED)
      (cutLeafCount >= 1 and cutTotalSteps == pSpec.runSteps and minLeafSteps >= 1)
    else
      onOwnedBox and checkFreeSchedule(
        pSpec.admittedShapes,
        shapeIdx,
        pSpec.runSteps,
        cutLeafCount,
        cutTotalSteps,
        minLeafSteps,
        maxLeafSteps
      )

  // Implements STM-013, STM-014, ELA-001, ELA-007, SPL-011:
  // Prove a run under an operator-forced cut (e.g. `T16_CUT = 1,1,1,1,1`, 5 leaves of 1 step each)
  action scheduleAndProveForcedCut(
    b: BoxId,
    pr: PreparedRunId,
    shapeIdx: int,
    cutLeafCount: int,
    cutTotalSteps: int,
    minLeafSteps: int,
    maxLeafSteps: int,
    onOwnedBox: bool
  ): bool =
    val prof = sched.profiles.get(b)
    val pSpec = prepRunSpec(pr)
    val cap = shapeStepCapOf(shapeIdx)
    val claim =
      if (MUTANT_ID == MUT_ELA001_SCHEDULE_CHANGES_CLAIM) (pSpec.expectedClaimPub + cutLeafCount)
      else pSpec.expectedClaimPub
    all {
      canScheduleForcedCut(sched, b, pr, shapeIdx, cutLeafCount, cutTotalSteps, minLeafSteps, maxLeafSteps, onOwnedBox, MUTANT_ID),
      val rec: ScheduledRunRec = {
        prepId: pr,
        boxId: b,
        boxOwned: onOwnedBox,
        loadedPathKind: ProfilePathDefaultBox,
        hasEnvProfileOverride: false,
        profileKeyRoot: prof.keyRoot,
        profileKeysD: prof.keysD,
        proverKeyRoot: pSpec.keyRoot,
        proverKeysD: sched.proverKeysD,
        calibratedForKeySet: (prof.keyRoot == pSpec.keyRoot and prof.keysD == sched.proverKeysD),
        progId: pSpec.progId,
        inputId: pSpec.inputId,
        planD: pSpec.planD,
        progD: pSpec.progD,
        runSteps: pSpec.runSteps,
        runSalt: pSpec.runSalt,
        hasMemLimitOverride: false,
        memLimitOverrideGb: 0,
        hasGpuCountOverride: false,
        gpuCountOverride: 0,
        hasGpuMemOverride: false,
        gpuMemOverrideGb: 0,
        effectiveMemBudgetGb: prof.memBudgetGb,
        effectiveGpuCount: prof.gpuEntries.length(),
        effectiveGpuMemGb: profileGpuMemGb(prof),
        shapeIdx: shapeIdx,
        shapeStepCap: cap,
        shapeInKeyRoot: pSpec.admittedShapes.contains(shapeIdx),
        devKind: DevCpu,
        hostGbPerLeaf: 4,
        deviceGbPerLeaf: 0,
        concurrency: 1,
        cutLeafCount: cutLeafCount,
        cutTotalSteps: cutTotalSteps,
        minLeafSteps: minLeafSteps,
        maxLeafSteps: maxLeafSteps,
        forcedOverride: true,
        overrideNamed: true,
        usedLegacyMenuCheck: sched.keyRootHasScheduleMenu,
        estWallMs: cutLeafCount * 2000,
        wrapPolicy: deriveWrapPolicy(profileLargestGpuMb(prof), MUTANT_ID),
        scheduleStaysLocal: onOwnedBox,
        usedKeySetTuple: keyTupleForRun(pSpec.progId, sched.proverKeysD, DevCpu, MUTANT_ID),
        recordWritten: true,
        shardCount: cutLeafCount,
        leafStageTimeMs: 7260,
        nodeStageTimeMs: 20920,
        peakHostMemGb: 4,
        lStockVerified: true,
        nodeAccepted: true,
        claimPub: claim,
      }
      sched' = {
        ...sched,
        provedRuns: sched.provedRuns.union(Set(rec)),
        lastRefusal: SchedRefusalNone,
      },
      lastOp' = "scheduleAndProveForcedCut",
    }

  // Implements STM-013, SPL-011: Refuse schedule failing `check_free` (unlisted shape, 0 leaves, incomplete cover, or > S steps)
  action refuseInvalidCheckFree(
    pr: PreparedRunId,
    shapeIdx: int,
    cutLeafCount: int,
    cutTotalSteps: int,
    minLeafSteps: int,
    maxLeafSteps: int
  ): bool =
    val pSpec = prepRunSpec(pr)
    all {
      not(checkFreeSchedule(
        pSpec.admittedShapes,
        shapeIdx,
        pSpec.runSteps,
        cutLeafCount,
        cutTotalSteps,
        minLeafSteps,
        maxLeafSteps
      )),
      sched' = {
        ...sched,
        lastRefusal: SchedRefusalCheckFreeFailed(shapeIdx),
      },
      lastOp' = "refuseInvalidCheckFree",
    }

  // Implements STM-019 (PF-008): Device guard checks `freeDeviceMb >= starkExtra + wrapTail + margin`
  action evaluateEarlyWrapGuard(b: BoxId, oneUnitShort: bool): bool =
    val prof = sched.profiles.get(b)
    all {
      isProfileValid(prof),
      val pol = deriveWrapPolicy(profileLargestGpuMb(prof), MUTANT_ID)
      val reqMb = earlyWrapRequiredMb(pol)
      val freeMb = if (oneUnitShort) (reqMb - 1) else (reqMb + 500)
      val admit =
        if (MUTANT_ID == MUT_STM019_ADMIT_EARLY_WRAP_SHORT) (freeMb >= reqMb - 1)
        else (freeMb >= reqMb)
      val chk: EarlyWrapCheckRec = {
        boxId: b,
        wrapPolicy: pol,
        freeDeviceMb: freeMb,
        requiredMb: reqMb,
        earlyWrapAdmitted: admit,
        startedAfterStarkStage: not(admit),
      }
      val nextRef =
        if (admit) sched.lastRefusal
        else SchedRefusalEarlyWrapShort((b, freeMb, reqMb))
      sched' = {
        ...sched,
        earlyWrapChecks: sched.earlyWrapChecks.union(Set(chk)),
        lastRefusal: nextRef,
      },
      lastOp' = "evaluateEarlyWrapGuard",
    }

  // Pure guard for cross-box join (`ELA-006`, `SC-014`)
  pure def canJoinLeavesTwoBoxes(
    s: SchedulingState,
    bA: BoxId,
    bB: BoxId,
    hasCorruptedLeaf: bool,
    mutId: int
  ): bool =
    val pA = s.profiles.get(bA)
    val pB = s.profiles.get(bB)
    val profilesMatch =
      isProfileValid(pA) and isProfileValid(pB)
        and pA.keysD == s.proverKeysD and pB.keysD == s.proverKeysD
    val leavesOk = (mutId == MUT_ELA006_SKIP_LEAF_VERIFY_ON_JOIN) or not(hasCorruptedLeaf)
    bA != bB and profilesMatch and leavesOk

  // Implements ELA-006 (SC-014): Combine stock-verified leaves from two owned boxes under one N1
  action joinLeavesTwoBoxes(bA: BoxId, bB: BoxId, pr: PreparedRunId, hasCorruptedLeaf: bool): bool =
    val pSpec = prepRunSpec(pr)
    all {
      canJoinLeavesTwoBoxes(sched, bA, bB, hasCorruptedLeaf, MUTANT_ID),
      val jRec: CrossBoxJoinRec = {
        boxA: bA,
        boxB: bB,
        bothBoxesOwned: boxHardware(bA).boxOwned and boxHardware(bB).boxOwned,
        manifestsEqualCombiner: true,
        allLeavesStockVerified: not(hasCorruptedLeaf),
        intervalsContiguous: true,
        nodeProofStarted: true,
        claimPub: pSpec.expectedClaimPub,
      }
      sched' = {
        ...sched,
        completedJoins: sched.completedJoins.union(Set(jRec)),
        lastRefusal: SchedRefusalNone,
      },
      lastOp' = "joinLeavesTwoBoxes",
    }

  // Implements ELA-006 Scenario: 2 leaves of odin + 1 corrupted leaf of huginn -> stop before N1 and name leaf
  action refuseJoinCorruptedLeaf(bA: BoxId, bB: BoxId, badLeafIdx: int): bool = all {
    bA != bB,
    isProfileValid(sched.profiles.get(bA)),
    isProfileValid(sched.profiles.get(bB)),
    sched' = {
      ...sched,
      lastRefusal: SchedRefusalJoinCorruptedLeaf((bA, bB, badLeafIdx)),
    },
    lastOp' = "refuseJoinCorruptedLeaf",
  }

  // =========================================================================
  // 6. Nondeterministic `step` Relation (Step 6)
  // =========================================================================

  action step: bool = {
    nondet boxChoice = ALL_BOXES.oneOf()
    nondet prepChoice = ALL_PREP_RUNS.oneOf()
    nondet presetChoice = Set(
      LimitNone,
      LimitRaiseMem80Gb,
      LimitLowMem4Gb,
      LimitGpuMemOneLeaf,
      LimitNoGpu
    ).oneOf()
    nondet badKindChoice = Set(BadFormatTag, BadMissingField, BadZeroLeafTime).oneOf()
    nondet envOverrideChoice = Set(false, true).oneOf()
    nondet shortGuardChoice = Set(false, true).oneOf()
    val corruptJoinFlag = (MUTANT_ID == MUT_ELA006_SKIP_LEAF_VERIFY_ON_JOIN) and shortGuardChoice
    val forcedShape = if (MUTANT_ID == MUT_SPL011_UNLISTED_SHAPE_OR_UNOWNED) 2 else 0
    val forcedCover = if (MUTANT_ID == MUT_STM013_BYPASS_CHECK_FREE) 3 else 5
    val forcedOwned = not(MUTANT_ID == MUT_SPL011_UNLISTED_SHAPE_OR_UNOWNED)
    any {
      runSetupOnBox(BoxHuginn),
      runSetupOnBox(BoxMuninn),
      runSetupOnBox(BoxOdin),
      all { lastOp == "init", abortSetupBadCalProof(boxChoice) },
      all { lastOp == "init", injectMalformedProfile(boxChoice, badKindChoice) },
      all { sched.lastRefusal == SchedRefusalNone, refuseMalformedProfile(boxChoice) },
      all { lastOp == "init", installStaleT10mProfile(boxChoice) },
      all { sched.lastRefusal == SchedRefusalNone, recordUncalibratedProfileKeyMismatch(boxChoice, prepChoice) },
      scheduleAndProveFromProfile(BoxHuginn, prepChoice, presetChoice, envOverrideChoice),
      scheduleAndProveFromProfile(BoxMuninn, prepChoice, presetChoice, envOverrideChoice),
      scheduleAndProveFromProfile(BoxOdin, prepChoice, presetChoice, envOverrideChoice),
      all { sched.lastRefusal == SchedRefusalNone, refuseUnfittableBudget(boxChoice, prepChoice) },
      scheduleAndProveForcedCut(BoxHuginn, PrepRunA_Short, forcedShape, 5, forcedCover, 1, 1, forcedOwned),
      scheduleAndProveForcedCut(BoxOdin, PrepRunA_Short, forcedShape, 5, forcedCover, 1, 1, forcedOwned),
      all { sched.lastRefusal == SchedRefusalNone, refuseInvalidCheckFree(PrepRunA_Short, 2, 5, 5, 1, 1) },
      all { sched.earlyWrapChecks.size() < 3, evaluateEarlyWrapGuard(boxChoice, shortGuardChoice) },
      joinLeavesTwoBoxes(BoxOdin, BoxHuginn, PrepRunA_Short, corruptJoinFlag),
      joinLeavesTwoBoxes(BoxMuninn, BoxHuginn, PrepRunA_Short, corruptJoinFlag),
      all { sched.lastRefusal == SchedRefusalNone, refuseJoinCorruptedLeaf(BoxOdin, BoxHuginn, 2) },
    }
  }

  // =========================================================================
  // 7. Safety Invariants (`val inv_<ID>` and Master Conjunction `val inv`)
  // =========================================================================

  val consistency_constants: bool = and {
    PROFILE_FIELD_COUNT == 11,
    GPU_ENTRY_FIELDS_REQUIRED == 7,
    WARMUP_REPS == 1,
    DEFAULT_CAL_REPS >= 1,
    CPU_CORES_PER_LEAF == 4,
    MAX_LEAVES_BOUND >= 5,
    PIPE_HEADROOM_DIV == 16,
    FULL_OVERLAP_MIN_MB > EVICT_OVERLAP_MIN_MB,
    EVICT_OVERLAP_MIN_MB > 0,
    STARK_EXTRA_MB > 0,
    WRAP_PEAK_MB > RESIDENT_COSETS_MB,
    RESIDENT_COSETS_MB >= 0,
    WRAP_MARGIN_MB > 0,
  }

  // STM-001: Written valid profiles record each GPU with all 7 required GPU metadata fields.
  val inv_STM_001: bool =
    ALL_BOXES.forall(b =>
      val prof = sched.profiles.get(b)
      val hw = boxHardware(b)
      isProfileValid(prof) implies (
        prof.gpuEntries.length() == hw.gpuCount
          and prof.gpuEntries.indices().forall(i =>
            val g = prof.gpuEntries.nth(i)
            g.entryFieldCount == GPU_ENTRY_FIELDS_REQUIRED
              and g.computeCap > 0
              and g.smClockMhz > 0
          )
      )
    )

  // STM-002: A GPU reporting no own memory (`vramTotalMb == -1`) is recorded as `unified == true`.
  val inv_STM_002: bool =
    ALL_BOXES.forall(b =>
      val prof = sched.profiles.get(b)
      prof.written implies prof.gpuEntries.indices().forall(i =>
        val g = prof.gpuEntries.nth(i)
        (g.vramTotalMb == -1) iff g.unified
      )
    )

  // STM-003: Profile `mem_budget` equals min(MemAvailable, cgroup headroom) when cgroup limit exists, else MemAvailable.
  val inv_STM_003: bool =
    ALL_BOXES.forall(b =>
      val prof = sched.profiles.get(b)
      val hw = boxHardware(b)
      val expectedBudget = if (hw.hasCgroupLimit) minInt(hw.memAvailableGb, hw.cgroupHeadroomGb) else hw.memAvailableGb
      isProfileValid(prof) implies (
        prof.memBudgetGb == expectedBudget
          and prof.cores == hw.cores
          and prof.ramTotalGb == hw.ramTotalGb
          and prof.diskFreeGb == hw.diskFreeGb
      )
    )

  // STM-004: Valid profiles hold CPU and GPU calibration entries with 1 warm-up and `>= 1` timed repetitions.
  val inv_STM_004: bool =
    ALL_BOXES.forall(b =>
      val prof = sched.profiles.get(b)
      isProfileValid(prof) implies (
        prof.calEntries.length() == 4
          and prof.calEntries.indices().forall(i =>
            val ce = prof.calEntries.nth(i)
            ce.warmupCount == WARMUP_REPS and ce.timedReps == DEFAULT_CAL_REPS
          )
      )
    )

  // STM-005: A refused calibration proof stops setup and writes no profile.
  val inv_STM_005: bool =
    ALL_BOXES.forall(b =>
      val prof = sched.profiles.get(b)
      prof.written implies prof.allCalProofsVerified
    )

  // STM-006: Valid profiles carry `format = sherlock-profile/1`, all 11 fields, `key_root`, `keys_d`, and `setup_s > 0`.
  val inv_STM_006: bool =
    ALL_BOXES.forall(b =>
      val prof = sched.profiles.get(b)
      isProfileValid(prof) implies (
        prof.formatTag == FormatSherlockProfileV1
          and prof.fieldCount == PROFILE_FIELD_COUNT
          and prof.keyRoot > 0
          and prof.keysD > 0
          and prof.setupSec > 0
      )
    )

  // STM-007: No run is scheduled or proved from a profile with bad `format`, missing fields, or non-positive `leaf_s`.
  val inv_STM_007: bool =
    sched.provedRuns.forall(r => isProfileValid(sched.profiles.get(r.boxId)))

  // STM-008: Any profile whose `key_root` or `keys_d` differs from the prover's key set is recorded as NOT calibrated.
  val inv_STM_008: bool = and {
    sched.uncalibratedRecords.forall(rec => not(rec._3)),
    sched.provedRuns.forall(r =>
      (r.profileKeyRoot != r.proverKeyRoot or r.profileKeysD != r.proverKeysD)
        implies not(r.calibratedForKeySet)
    ),
  }

  // STM-009: Non-forced schedules derive `ceil(n / S)` leaves of `1..S` steps covering `runSteps` from the profile.
  val inv_STM_009: bool =
    sched.provedRuns.forall(r =>
      not(r.forcedOverride) implies (
        r.cutLeafCount == ceilDiv(r.runSteps, r.shapeStepCap)
          and r.cutTotalSteps == r.runSteps
          and r.minLeafSteps >= 1
          and r.maxLeafSteps <= r.shapeStepCap
          and r.concurrency >= 1
      )
    )

  // STM-010: Operator limits (`SHERLOCK_MEM_BUDGET`, `SHERLOCK_GPUS`, `SHERLOCK_GPU_MEM`) only lower profile values.
  val inv_STM_010: bool =
    sched.provedRuns.forall(r =>
      val prof = sched.profiles.get(r.boxId)
      val profGpuGb = profileGpuMemGb(prof)
      r.effectiveMemBudgetGb <= prof.memBudgetGb
        and r.effectiveGpuCount <= prof.gpuEntries.length()
        and r.effectiveGpuMemGb <= profGpuGb
        and (r.hasMemLimitOverride implies r.effectiveMemBudgetGb == minInt(r.memLimitOverrideGb, prof.memBudgetGb))
    )

  // STM-011: Leaf concurrency fits device free memory, host memory budget, and `max(1, cores / 4)` on CPU.
  val inv_STM_011: bool =
    sched.provedRuns.forall(r =>
      val prof = sched.profiles.get(r.boxId)
      not(r.forcedOverride) implies match r.devKind {
        | DevCpu =>
            r.concurrency * r.hostGbPerLeaf <= r.effectiveMemBudgetGb
              and r.concurrency <= maxInt(1, prof.cores / CPU_CORES_PER_LEAF)
        | DevDiscreteGpu =>
            r.concurrency * r.deviceGbPerLeaf <= r.effectiveGpuMemGb
              and r.concurrency * r.hostGbPerLeaf <= r.effectiveMemBudgetGb
        | DevUnifiedGpu =>
            r.concurrency * (r.deviceGbPerLeaf + r.hostGbPerLeaf) <= minInt(r.effectiveMemBudgetGb, r.effectiveGpuMemGb)
      }
    )

  // STM-012: Every scheduled run fits at least one leaf in its memory budget (unfittable budgets never prove).
  val inv_STM_012: bool =
    sched.provedRuns.forall(r =>
      r.concurrency >= 1
        and r.concurrency * r.hostGbPerLeaf <= r.effectiveMemBudgetGb
    )

  // STM-013: Every accepted schedule satisfies `check_free`, and forced overrides are named in the run record.
  val inv_STM_013: bool =
    sched.provedRuns.forall(r =>
      val pSpec = prepRunSpec(r.prepId)
      checkFreeSchedule(
        pSpec.admittedShapes,
        r.shapeIdx,
        r.runSteps,
        r.cutLeafCount,
        r.cutTotalSteps,
        r.minLeafSteps,
        r.maxLeafSteps
      ) and (r.forcedOverride implies r.overrideNamed)
    )

  // STM-014: No schedule menu or shard-count list exists in keys or prover (`check_free` only).
  val inv_STM_014: bool =
    not(sched.keyRootHasScheduleMenu)
      and sched.provedRuns.forall(r => not(r.usedLegacyMenuCheck))

  // STM-015: Wrap pipeline policy matches largest profile GPU memory less 1/16 headroom.
  val inv_STM_015: bool =
    sched.provedRuns.forall(r =>
      val prof = sched.profiles.get(r.boxId)
      r.wrapPolicy == deriveWrapPolicy(profileLargestGpuMb(prof), MUT_NONE)
    )

  // STM-016: When `T16_PROFILE` is unset, the box profile is read from `~/sherlock/setup/profile.json`.
  val inv_STM_016: bool =
    sched.provedRuns.forall(r =>
      not(r.hasEnvProfileOverride) implies (r.loadedPathKind == ProfilePathDefaultBox)
    )

  // STM-017: Profile-scheduled pick minimizes `(estWallMs, cutLeafCount, shapeIdx, GPU-before-CPU)`.
  val inv_STM_017: bool =
    sched.provedRuns.forall(r =>
      not(r.forcedOverride) implies {
        val prof = sched.profiles.get(r.boxId)
        val pSpec = prepRunSpec(r.prepId)
        val refPick = pickFromCalList(
          prof.calEntries,
          pSpec.admittedShapes,
          prof.cores,
          r.runSteps,
          r.effectiveMemBudgetGb,
          r.effectiveGpuCount,
          r.effectiveGpuMemGb,
          false,
          MUT_NONE
        )
        refPick.ok
          and r.shapeIdx == refPick.ce.shapeIdx
          and r.devKind == refPick.ce.devKind
          and r.concurrency == refPick.conc
      }
    )

  // STM-018: Lowering the memory budget never lowers the estimated wall time of the scheduler's pick.
  val inv_STM_018: bool =
    sched.provedRuns.forall(rLow =>
      not(rLow.forcedOverride) implies {
        val prof = sched.profiles.get(rLow.boxId)
        val pSpec = prepRunSpec(rLow.prepId)
        val unloweredPick = pickFromCalList(
          prof.calEntries,
          pSpec.admittedShapes,
          prof.cores,
          rLow.runSteps,
          prof.memBudgetGb,
          rLow.effectiveGpuCount,
          rLow.effectiveGpuMemGb,
          false,
          MUT_NONE
        )
        (unloweredPick.ok and rLow.effectiveMemBudgetGb <= prof.memBudgetGb)
          implies (rLow.estWallMs >= unloweredPick.estMs)
      }
    )

  // STM-019: Early wrap start is admitted ONLY when free device bytes >= STARK extra + wrap tail + margin.
  val inv_STM_019: bool =
    sched.earlyWrapChecks.forall(chk =>
      chk.earlyWrapAdmitted implies (chk.freeDeviceMb >= chk.requiredMb)
    )

  // ELA-001: Byte-equal `claim_pub` across every schedule/cut/box of one prepared run; distinct salts -> distinct claims.
  val inv_ELA_001: bool =
    sched.provedRuns.forall(r1 =>
      sched.provedRuns.forall(r2 =>
        ((r1.prepId == r2.prepId) implies (r1.claimPub == r2.claimPub))
          and ((r1.progId == r2.progId and r1.inputId == r2.inputId and r1.runSalt != r2.runSalt)
                implies (r1.claimPub != r2.claimPub))
      )
    )

  // ELA-002: Identical `(app, leaf, N1, N2, N3, L)` keys across every device, concurrency, cut, and host of a program/keyset.
  val inv_ELA_002: bool =
    sched.provedRuns.forall(r1 =>
      sched.provedRuns.forall(r2 =>
        (r1.progId == r2.progId and r1.proverKeysD == r2.proverKeysD)
          implies (r1.usedKeySetTuple == r2.usedKeySetTuple)
      )
    )

  // ELA-003: Lowered memory/GPU caps prove with fitted concurrency >= 1 and unchanged `claim_pub`.
  val inv_ELA_003: bool =
    sched.provedRuns.forall(r =>
      (r.hasMemLimitOverride or r.hasGpuMemOverride) implies (
        r.concurrency >= 1
          and r.claimPub == prepRunSpec(r.prepId).expectedClaimPub
      )
    )

  // ELA-004: When a box has no usable GPU (`effectiveGpuCount == 0`), leaf proving runs on CPU and `L` verifies.
  val inv_ELA_004: bool =
    sched.provedRuns.forall(r =>
      (r.effectiveGpuCount == 0) implies (r.devKind == DevCpu and r.lStockVerified)
    )

  // ELA-005: `muninn`, `huginn`, and `odin` proving one prepared run from their own profiles give equal `claim_pub`.
  val inv_ELA_005: bool =
    sched.provedRuns.forall(r1 =>
      sched.provedRuns.forall(r2 =>
        (r1.prepId == r2.prepId and r1.boxId != r2.boxId)
          implies (r1.claimPub == r2.claimPub)
      )
    )

  // ELA-006: Cross-box leaf join under one N1 verifies every leaf with the stock verifier before starting N1.
  val inv_ELA_006: bool =
    sched.completedJoins.forall(j =>
      j.nodeProofStarted implies (
        j.bothBoxesOwned
          and j.manifestsEqualCombiner
          and j.allLeavesStockVerified
          and j.intervalsContiguous
      )
    )

  // ELA-007: Every completed elastic run writes a complete measured run record (`shardCount`, stage times, peak RAM, `L`, `claim_pub`).
  val inv_ELA_007: bool =
    sched.provedRuns.forall(r =>
      r.recordWritten
        and r.shardCount == r.cutLeafCount
        and r.leafStageTimeMs > 0
        and r.nodeStageTimeMs >= 0
        and r.peakHostMemGb > 0
        and r.lStockVerified
        and r.claimPub > 0
    )

  // LPC-006: Two run lengths under one `plan_d` are both accepted by the node and preserve `prog_d` and keys.
  val inv_LPC_006: bool =
    sched.provedRuns.forall(r1 =>
      sched.provedRuns.forall(r2 =>
        (r1.progId == r2.progId and r1.planD == r2.planD) implies (
          r1.nodeAccepted
            and r2.nodeAccepted
            and r1.progD == r2.progD
            and r1.proverKeyRoot == r2.proverKeyRoot
            and r1.usedKeySetTuple == r2.usedKeySetTuple
        )
      )
    )

  // Adopted SPL-011: Schedule stays on user's owned box, uses only shapes in `key_root`, and is outside `plan_d`.
  val inv_SPL_011: bool =
    sched.provedRuns.forall(r =>
      r.boxOwned
        and r.shapeInKeyRoot
        and r.scheduleStaysLocal
        and r.planD == prepRunSpec(r.prepId).planD
    )

  // Master conjunction of all safety invariants
  val inv: bool = and {
    inv_STM_001,
    inv_STM_002,
    inv_STM_003,
    inv_STM_004,
    inv_STM_005,
    inv_STM_006,
    inv_STM_007,
    inv_STM_008,
    inv_STM_009,
    inv_STM_010,
    inv_STM_011,
    inv_STM_012,
    inv_STM_013,
    inv_STM_014,
    inv_STM_015,
    inv_STM_016,
    inv_STM_017,
    inv_STM_018,
    inv_STM_019,
    inv_ELA_001,
    inv_ELA_002,
    inv_ELA_003,
    inv_ELA_004,
    inv_ELA_005,
    inv_ELA_006,
    inv_ELA_007,
    inv_LPC_006,
    inv_SPL_011,
  }

  // =========================================================================
  // 8. Non-Vacuity Reachability Witnesses (Negated Goals: MUST Exit 1 in Main)
  // =========================================================================

  val reach_three_boxes_profiled_and_proved: bool = not(
    ALL_BOXES.forall(b => sched.provedRuns.exists(r => r.boxId == b and r.prepId == PrepRunA_Short))
  )

  val reach_forced_cut_and_profile_cut_same_claim: bool = not(
    sched.provedRuns.exists(r1 => r1.prepId == PrepRunA_Short and r1.forcedOverride)
      and sched.provedRuns.exists(r2 => r2.prepId == PrepRunA_Short and not(r2.forcedOverride))
  )

  val reach_two_run_lengths_one_plan: bool = not(
    sched.provedRuns.exists(r1 => r1.prepId == PrepRunA_Short)
      and sched.provedRuns.exists(r2 => r2.prepId == PrepRunB_Long)
  )

  val reach_cpu_fallback_when_no_gpu: bool = not(
    sched.provedRuns.exists(r => r.hasGpuCountOverride and r.effectiveGpuCount == 0 and r.devKind == DevCpu)
  )

  val reach_cross_box_join_completed: bool = not(
    sched.completedJoins.size() > 0
  )

  val reach_early_wrap_refused_and_admitted: bool = not(
    sched.earlyWrapChecks.exists(c1 => c1.earlyWrapAdmitted)
      and sched.earlyWrapChecks.exists(c2 => not(c2.earlyWrapAdmitted))
  )

  val reach_budget_or_check_free_refused: bool = not(
    match sched.lastRefusal {
      | SchedRefusalNoShapeFitsBudget(_) => true
      | SchedRefusalCheckFreeFailed(_)   => true
      | _                                => false
    }
  )
}

// ===========================================================================
// 9. Concrete Instances: Main, 2 Edge Instances, and 22 Guard/Binding Mutants
// ===========================================================================

module scheduling_main {
  import scheduling(
    PROFILE_FIELD_COUNT = 11,
    GPU_ENTRY_FIELDS_REQUIRED = 7,
    WARMUP_REPS = 1,
    DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4,
    MAX_LEAVES_BOUND = 16777216,
    PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000,
    EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000,
    WRAP_PEAK_MB = 18000,
    RESIDENT_COSETS_MB = 7500,
    WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 0
  ).*
}

// Edge Instance 1: Minimum legal calibration repetitions (1) and minimum positive margin (1 MiB)
module scheduling_edge_min {
  import scheduling(
    PROFILE_FIELD_COUNT = 11,
    GPU_ENTRY_FIELDS_REQUIRED = 7,
    WARMUP_REPS = 1,
    DEFAULT_CAL_REPS = 1,
    CPU_CORES_PER_LEAF = 4,
    MAX_LEAVES_BOUND = 16777216,
    PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000,
    EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000,
    WRAP_PEAK_MB = 18000,
    RESIDENT_COSETS_MB = 7500,
    WRAP_MARGIN_MB = 1,
    MUTANT_ID = 0
  ).*
}

// Edge Instance 2: Exact boundary limits (`MAX_LEAVES_BOUND = 5` matching forced [1,1,1,1,1] cut,
// and `FULL_OVERLAP_MIN_MB = 30720` exact at 32 GiB less 1/16 headroom)
module scheduling_edge_at_limit {
  import scheduling(
    PROFILE_FIELD_COUNT = 11,
    GPU_ENTRY_FIELDS_REQUIRED = 7,
    WARMUP_REPS = 1,
    DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4,
    MAX_LEAVES_BOUND = 5,
    PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 30720,
    EVICT_OVERLAP_MIN_MB = 15360,
    STARK_EXTRA_MB = 6000,
    WRAP_PEAK_MB = 18000,
    RESIDENT_COSETS_MB = 7500,
    WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 0
  ).*
}

// --- 22 Guard / Binding Mutant Instances (`scheduling_m_<name>`) ---

module scheduling_m_stm002_unified_not_marked {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 1
  ).*
}

module scheduling_m_stm003_ignore_cgroup_cap {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 2
  ).*
}

module scheduling_m_stm005_write_on_bad_cal_proof {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 3
  ).*
}

module scheduling_m_stm007_accept_zero_leaf_s {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 4
  ).*
}

module scheduling_m_stm008_ignore_keyset_mismatch {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 5
  ).*
}

module scheduling_m_stm010_limit_raises_budget {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 6
  ).*
}

module scheduling_m_stm011_ignore_host_cap_on_gpu {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 7
  ).*
}

module scheduling_m_stm012_schedule_zero_fit {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 8
  ).*
}

module scheduling_m_stm013_bypass_check_free {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 9
  ).*
}

module scheduling_m_stm014_legacy_menu_check {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 10
  ).*
}

module scheduling_m_stm015_wrong_wrap_policy {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 11
  ).*
}

module scheduling_m_stm016_wrong_default_path {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 12
  ).*
}

module scheduling_m_stm017_wrong_tie_order {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 13
  ).*
}

module scheduling_m_stm018_lower_budget_faster {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 14
  ).*
}

module scheduling_m_stm019_admit_early_wrap_short {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 15
  ).*
}

module scheduling_m_ela001_schedule_changes_claim {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 16
  ).*
}

module scheduling_m_ela002_device_specific_l_key {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 17
  ).*
}

module scheduling_m_ela004_gpu_used_when_hidden {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 18
  ).*
}

module scheduling_m_ela006_skip_leaf_verify_on_join {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 19
  ).*
}

module scheduling_m_ela007_incomplete_run_record {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 20
  ).*
}

module scheduling_m_lpc006_longer_run_mutates_prog_d {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 21
  ).*
}

module scheduling_m_spl011_unlisted_shape_or_unowned_box {
  import scheduling(
    PROFILE_FIELD_COUNT = 11, GPU_ENTRY_FIELDS_REQUIRED = 7, WARMUP_REPS = 1, DEFAULT_CAL_REPS = 3,
    CPU_CORES_PER_LEAF = 4, MAX_LEAVES_BOUND = 16777216, PIPE_HEADROOM_DIV = 16,
    FULL_OVERLAP_MIN_MB = 24000, EVICT_OVERLAP_MIN_MB = 14000,
    STARK_EXTRA_MB = 6000, WRAP_PEAK_MB = 18000, RESIDENT_COSETS_MB = 7500, WRAP_MARGIN_MB = 1000,
    MUTANT_ID = 22
  ).*
}
