// Requirement-level `.expect(...)` scenario tests for Sherlock E2E Scheduling
// Lane: e2e-scheduling

module scheduling_test {
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
  ).* from "./scheduling"

  // Constant consistency check across spec bounds
  run sc_ConsistencyConstantsTest =
    init.expect(consistency_constants and inv)

  // STM-001 & STM-002: Setup reads GPU 7-field entry on huginn and marks muninn's GB10 unified (`vram_total = null`).
  run sc_Stm001And002GpusSevenFieldsAndUnifiedMuninnTest =
    init
      .then(runSetupOnBox(BoxHuginn))
      .then(runSetupOnBox(BoxMuninn))
      .expect(all {
        sched.profiles.get(BoxHuginn).gpuEntries.length() == 1,
        sched.profiles.get(BoxHuginn).gpuEntries.nth(0).entryFieldCount == 7,
        not(sched.profiles.get(BoxHuginn).gpuEntries.nth(0).unified),
        sched.profiles.get(BoxMuninn).gpuEntries.nth(0).unified,
        sched.profiles.get(BoxMuninn).gpuEntries.nth(0).vramTotalMb == -1,
        inv_STM_001,
        inv_STM_002,
      })

  // STM-003: Host probe sets `mem_budget` to lower of MemAvailable and cgroup headroom (60 vs 100 GB), or MemAvailable when unset.
  run sc_Stm003HostProbeCgroupMinAndNoCgroupTest =
    init
      .then(runSetupOnBox(BoxHuginn))
      .then(runSetupOnBox(BoxMuninn))
      .expect(all {
        computeSetupMemBudget(100, true, 60, 0) == 60,
        computeSetupMemBudget(60, true, 100, 0) == 60,
        computeSetupMemBudget(100, false, 0, 0) == 100,
        sched.profiles.get(BoxHuginn).memBudgetGb == 60,
        sched.profiles.get(BoxMuninn).memBudgetGb == 100,
        inv_STM_003,
      })

  // STM-004 & STM-006: Setup calibrates each shape on CPU + GPU with 1 warmup + 3 timed reps and writes 11-field `sherlock-profile/1`.
  run sc_Stm004And006CalibrateRepsAndElevenFieldsRoundTripTest =
    init
      .then(runSetupOnBox(BoxHuginn))
      .expect(all {
        sched.profiles.get(BoxHuginn).formatTag == FormatSherlockProfileV1,
        sched.profiles.get(BoxHuginn).fieldCount == 11,
        sched.profiles.get(BoxHuginn).calEntries.length() == 4,
        sched.profiles.get(BoxHuginn).calEntries.nth(0).warmupCount == 1,
        sched.profiles.get(BoxHuginn).calEntries.nth(0).timedReps == 3,
        sched.profiles.get(BoxHuginn).calEntries.nth(0).leafMs > 0,
        inv_STM_004,
        inv_STM_006,
      })

  // STM-005: Refused calibration proof stops setup and writes no profile (`written == false`).
  run sc_Stm005RefusedCalProofWritesNoProfileTest =
    init
      .then(abortSetupBadCalProof(BoxHuginn))
      .expect(all {
        not(sched.profiles.get(BoxHuginn).written),
        sched.lastRefusal == SchedRefusalBadCalibrationProof(BoxHuginn),
        inv_STM_005,
      })

  // STM-007: Profile with `leaf_s = 0` (or bad format / missing field) is refused and cannot schedule.
  run sc_Stm007ZeroLeafTimeAndBadFormatRefusedTest =
    init
      .then(injectMalformedProfile(BoxHuginn, BadZeroLeafTime))
      .expect(not(canScheduleFromProfile(sched, BoxHuginn, PrepRunA_Short, LimitNone, false, 0)))
      .then(refuseMalformedProfile(BoxHuginn))
      .expect(all {
        sched.lastRefusal == SchedRefusalMalformedProfile((BoxHuginn, BadZeroLeafTime)),
        sched.provedRuns.size() == 0,
        inv_STM_007,
      })

  // STM-008: T10m profile on T25 prover keys fails key-set check and is recorded as not calibrated.
  run sc_Stm008StaleT10mProfileRecordedNotCalibratedTest =
    init
      .then(installStaleT10mProfile(BoxHuginn))
      .expect(not(canScheduleFromProfile(sched, BoxHuginn, PrepRunA_Short, LimitNone, false, 0)))
      .then(recordUncalibratedProfileKeyMismatch(BoxHuginn, PrepRunA_Short))
      .expect(all {
        sched.uncalibratedRecords.contains((BoxHuginn, PrepRunA_Short, false)),
        sched.lastRefusal == SchedRefusalUncalibratedKeySet((BoxHuginn, T10M_KEYS_D, T25_KEYS_D)),
        inv_STM_008,
      })

  // STM-009: Run of `3 S` (`12`) steps under 4 GB budget (fitting Shape 0 of `S = 4`) cuts into 3 leaves of `1..4` steps.
  run sc_Stm009DeriveThreeLeafScheduleTest =
    init
      .then(runSetupOnBox(BoxHuginn))
      .then(scheduleAndProveFromProfile(BoxHuginn, PrepRunB_Long, LimitLowMem4Gb, false))
      .expect(all {
        sched.provedRuns.exists(r =>
          r.runSteps == 12
            and r.shapeStepCap == 4
            and r.cutLeafCount == 3
            and r.minLeafSteps == 4
            and r.maxLeafSteps == 4
        ),
        inv_STM_009,
      })

  // STM-010: Operator limit `SHERLOCK_MEM_BUDGET = 80 GB` above profile `60 GB` uses `60 GB`.
  run sc_Stm010OperatorLimitsOnlyLowerProfileValuesTest =
    init
      .then(runSetupOnBox(BoxHuginn))
      .then(scheduleAndProveFromProfile(BoxHuginn, PrepRunA_Short, LimitRaiseMem80Gb, false))
      .expect(all {
        sched.provedRuns.exists(r =>
          r.memLimitOverrideGb == 80 and r.effectiveMemBudgetGb == 60
        ),
        inv_STM_010,
      })

  // STM-011: Unified GPU shares one pool (`deviceGb + hostGb`); discrete GPU with VRAM for 8 leaves & RAM for 2 sets concurrency = 2.
  run sc_Stm011UnifiedPoolAndDiscreteHostCapTest =
    init
      .then(runSetupOnBox(BoxMuninn))
      .then(scheduleAndProveFromProfile(BoxMuninn, PrepRunB_Long, LimitGpuMemOneLeaf, false))
      .expect(all {
        // Discrete GPU with 16 GB VRAM (8x 2GB leaves) and 8 GB host RAM (2x 4GB leaves) -> concurrency 2
        computeLeafConcurrency(
          { shapeIdx: 0, devKind: DevDiscreteGpu, warmupCount: 1, timedReps: 3, leafMs: 2000, appMs: 500, hostGb: 4, deviceGb: 2 },
          16, 8, 1, 16, 0
        ) == 2,
        // Unified GPU with 6 GB pool and 2 GB device + 4 GB host -> concurrency 1
        sched.provedRuns.exists(r => r.boxId == BoxMuninn and r.concurrency == 1),
        inv_STM_011,
      })

  // STM-012: `SHERLOCK_MEM_BUDGET = 1 GB` fits no shape on any device -> refused before any proof.
  run sc_Stm012OneGbBudgetRefusedBeforeProofTest =
    init
      .then(runSetupOnBox(BoxHuginn))
      .expect(not(canScheduleFromProfile(sched, BoxHuginn, PrepRunA_Short, LimitTooSmall1Gb, false, 0)))
      .then(refuseUnfittableBudget(BoxHuginn, PrepRunA_Short))
      .expect(all {
        sched.provedRuns.size() == 0,
        sched.lastRefusal == SchedRefusalNoShapeFitsBudget((BoxHuginn, 1)),
        inv_STM_012,
      })

  // STM-013 & STM-014: Forced cut `T16_CUT = 1,1,1,1,1` passes `check_free` without any schedule menu and names override.
  run sc_Stm013And014ForcedFiveLeafCutAndNoMenuTest =
    init
      .then(runSetupOnBox(BoxHuginn))
      .then(scheduleAndProveForcedCut(BoxHuginn, PrepRunA_Short, 0, 5, 5, 1, 1, true))
      .expect(all {
        sched.provedRuns.exists(r => r.cutLeafCount == 5 and r.forcedOverride and r.overrideNamed and not(r.usedLegacyMenuCheck)),
        not(canScheduleForcedCut(sched, BoxHuginn, PrepRunA_Short, 0, 5, 4, 1, 1, true, 0)),
        inv_STM_013,
        inv_STM_014,
      })

  // STM-015: Wrap pipeline policy from largest GPU memory less 1/16 headroom (odin 32GB -> FullOverlap, huginn 16GB -> CosetEvict, 8GB -> NoOverlap).
  run sc_Stm015WrapPipelineOdinHuginnSmallGpuTest =
    init
      .then(runSetupOnBox(BoxOdin))
      .then(runSetupOnBox(BoxHuginn))
      .then(scheduleAndProveFromProfile(BoxOdin, PrepRunA_Short, LimitNone, false))
      .then(scheduleAndProveFromProfile(BoxHuginn, PrepRunA_Short, LimitNone, false))
      .expect(all {
        sched.provedRuns.exists(r => r.boxId == BoxOdin and r.wrapPolicy == PipeFullOverlap),
        sched.provedRuns.exists(r => r.boxId == BoxHuginn and r.wrapPolicy == PipeCosetEvict),
        deriveWrapPolicy(8192, 0) == PipeNoOverlap,
        inv_STM_015,
      })

  // STM-016: When `T16_PROFILE` is unset (`useEnvPathOverride == false`), prover loads `~/sherlock/setup/profile.json`.
  run sc_Stm016DefaultBoxProfilePathLoadedTest =
    init
      .then(runSetupOnBox(BoxOdin))
      .then(scheduleAndProveFromProfile(BoxOdin, PrepRunA_Short, LimitNone, false))
      .expect(all {
        sched.provedRuns.exists(r => not(r.hasEnvProfileOverride) and r.loadedPathKind == ProfilePathDefaultBox),
        inv_STM_016,
      })

  // STM-017: Equal estimated time tie breaks deterministically (fewer leaves, lower shapeIdx, GPU before CPU) regardless of entry order.
  run sc_Stm017LeastTimeAndReversedOrderTieBreakTest =
    init
      .then(runSetupOnBox(BoxOdin))
      .then(scheduleAndProveFromProfile(BoxOdin, PrepRunA_Short, LimitNone, false))
      .expect(all {
        // On BoxOdin for 5 steps, Shape 0 (2 leaves, 2000ms) ties Shape 1 (1 leaf, 2000ms) -> Shape 1 wins (fewer leaves)
        sched.provedRuns.exists(r => r.boxId == BoxOdin and r.shapeIdx == 1 and r.cutLeafCount == 1),
        // Reversed entry order produces identical pick
        val fwd = List(
          { shapeIdx: 0, devKind: DevCpu, warmupCount: 1, timedReps: 3, leafMs: 3000, appMs: 500, hostGb: 4, deviceGb: 0 },
          { shapeIdx: 0, devKind: DevDiscreteGpu, warmupCount: 1, timedReps: 3, leafMs: 3000, appMs: 500, hostGb: 4, deviceGb: 2 }
        )
        val rev = List(
          { shapeIdx: 0, devKind: DevDiscreteGpu, warmupCount: 1, timedReps: 3, leafMs: 3000, appMs: 500, hostGb: 4, deviceGb: 2 },
          { shapeIdx: 0, devKind: DevCpu, warmupCount: 1, timedReps: 3, leafMs: 3000, appMs: 500, hostGb: 4, deviceGb: 0 }
        )
        val pFwd = pickFromCalList(fwd, ADMITTED_KEYROOT_SHAPES, 16, 4, 32, 1, 16, false, 0)
        val pRev = pickFromCalList(rev, ADMITTED_KEYROOT_SHAPES, 16, 4, 32, 1, 16, false, 0)
        pFwd.ce.devKind == DevDiscreteGpu and pRev.ce.devKind == DevDiscreteGpu,
        inv_STM_017,
      })

  // STM-018: Sequence of budgets 64, 32, 16, and 4 GB on one profile never lowers estimated wall time.
  run sc_Stm018BudgetSequence64To32To16NeverFasterTest =
    init
      .then(runSetupOnBox(BoxHuginn))
      .expect({
        val prof = sched.profiles.get(BoxHuginn)
        val p64 = pickFromCalList(prof.calEntries, ADMITTED_KEYROOT_SHAPES, prof.cores, 12, 64, 1, 16, false, 0)
        val p32 = pickFromCalList(prof.calEntries, ADMITTED_KEYROOT_SHAPES, prof.cores, 12, 32, 1, 16, true, 0)
        val p16 = pickFromCalList(prof.calEntries, ADMITTED_KEYROOT_SHAPES, prof.cores, 12, 16, 1, 16, true, 0)
        val p4  = pickFromCalList(prof.calEntries, ADMITTED_KEYROOT_SHAPES, prof.cores, 12, 4, 1, 16, true, 0)
        p64.ok and p32.ok and p16.ok and p4.ok
          and p64.estMs <= p32.estMs
          and p32.estMs <= p16.estMs
          and p16.estMs <= p4.estMs
      })

  // STM-019: One unit below `starkExtra + wrapTail + margin` refuses early wrap start; sufficient free memory admits.
  run sc_Stm019OneShortRefusesEarlyWrapTest =
    init
      .then(runSetupOnBox(BoxOdin))
      .then(evaluateEarlyWrapGuard(BoxOdin, true))
      .expect(all {
        sched.earlyWrapChecks.exists(c => not(c.earlyWrapAdmitted) and c.startedAfterStarkStage and c.freeDeviceMb == c.requiredMb - 1),
        inv_STM_019,
      })
      .then(evaluateEarlyWrapGuard(BoxOdin, false))
      .expect(all {
        sched.earlyWrapChecks.exists(c => c.earlyWrapAdmitted and not(c.startedAfterStarkStage)),
        inv_STM_019,
      })

  // ELA-001..005 & ELA-007: Forced 5-leaf cut, 3 boxes (muninn, huginn, odin), 1-leaf GPU cap, and `SHERLOCK_GPUS=0` CPU run
  // all give byte-equal `claim_pub` under identical keys with full run records; fresh salt gives distinct `claim_pub`.
  run sc_Ela001To005And007ElasticSuiteAndRunRecordTest =
    init
      .then(runSetupOnBox(BoxHuginn))
      .then(runSetupOnBox(BoxMuninn))
      .then(runSetupOnBox(BoxOdin))
      .then(scheduleAndProveForcedCut(BoxHuginn, PrepRunA_Short, 0, 5, 5, 1, 1, true))
      .then(scheduleAndProveFromProfile(BoxHuginn, PrepRunA_Short, LimitNone, false))
      .then(scheduleAndProveFromProfile(BoxHuginn, PrepRunA_Short, LimitGpuMemOneLeaf, false))
      .then(scheduleAndProveFromProfile(BoxHuginn, PrepRunA_Short, LimitNoGpu, false))
      .then(scheduleAndProveFromProfile(BoxMuninn, PrepRunA_Short, LimitNone, false))
      .then(scheduleAndProveFromProfile(BoxOdin, PrepRunA_Short, LimitNone, false))
      .then(scheduleAndProveFromProfile(BoxHuginn, PrepRunC_FreshSalt, LimitNone, false))
      .expect(all {
        sched.provedRuns.size() == 7,
        inv_ELA_001,
        inv_ELA_002,
        inv_ELA_003,
        inv_ELA_004,
        inv_ELA_005,
        inv_ELA_007,
        inv,
      })

  // ELA-006: Corrupted leaf from huginn stops before N1 and names leaf; valid leaves from odin + huginn join under N1.
  run sc_Ela006CrossBoxJoinAndCorruptedLeafRefusedTest =
    init
      .then(runSetupOnBox(BoxOdin))
      .then(runSetupOnBox(BoxHuginn))
      .expect(not(canJoinLeavesTwoBoxes(sched, BoxOdin, BoxHuginn, true, 0)))
      .then(refuseJoinCorruptedLeaf(BoxOdin, BoxHuginn, 2))
      .expect(all {
        sched.completedJoins.size() == 0,
        sched.lastRefusal == SchedRefusalJoinCorruptedLeaf((BoxOdin, BoxHuginn, 2)),
        inv_ELA_006,
      })
      .then(joinLeavesTwoBoxes(BoxOdin, BoxHuginn, PrepRunA_Short, false))
      .expect(all {
        sched.completedJoins.size() == 1,
        inv_ELA_006,
      })

  // LPC-006 & SPL-011: Two run lengths under one `plan_d` both accepted with unchanged `prog_d` and keys; unlisted shape refused.
  run sc_Lpc006AndSpl011TwoRunLengthsOnePlanAndUnlistedShapeRefusedTest =
    init
      .then(runSetupOnBox(BoxHuginn))
      .then(scheduleAndProveFromProfile(BoxHuginn, PrepRunA_Short, LimitNone, false))
      .then(scheduleAndProveFromProfile(BoxHuginn, PrepRunB_Long, LimitNone, false))
      .expect(not(canScheduleForcedCut(sched, BoxHuginn, PrepRunA_Short, 2, 5, 5, 1, 1, true, 0)))
      .then(refuseInvalidCheckFree(PrepRunA_Short, 2, 5, 5, 1, 1))
      .expect(all {
        sched.lastRefusal == SchedRefusalCheckFreeFailed(2),
        inv_LPC_006,
        inv_SPL_011,
        inv,
      })
}
