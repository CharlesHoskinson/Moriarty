// Quint formal specification for Sherlock E2E Bounded Context: Box Fleet
// Lane: e2e-box-fleet
// Own IDs: BXF-001 .. BXF-014, GPV-001 .. GPV-007, PDM-001 .. PDM-009
// Completion IDs: LPC-005, LPC-007
// Adopted IDs modelled here: PRD-001, PRD-002, PRD-003, PRD-004

module box_fleet {
  // =========================================================================
  // 1. Module Constants (Spec Bounds & Mutant Selector)
  // =========================================================================

  const HEAVY_SLOTS_MAX: int             // BXF-002: shared slots on muninn (3)
  const HEAVY_BUDGET_GB: int             // BXF-003: max GiB per shared heavy-slot job (64)
  const ORPHAN_STALL_MIN: int            // BXF-005: > 10 minutes with no progress before SIGTERM
  const SIGKILL_GRACE_SEC: int           // BXF-013: >= 30 seconds after SIGTERM before SIGKILL
  const DURABLE_RUN_THRESHOLD_MIN: int   // BXF-009: > 30 minutes planned -> systemd-run --user
  const WRAPD_RESIDENT_ITEMS: int        // PDM-001: 6 resident items (SRS, relation, PK, VK, tables, CUDA ctx)
  const SOCKET_MODE_OCTAL: int           // PDM-002, PRD-001: Unix socket mode 0600
  const MIN_REKEY_LEAF_RULES: int        // LPC-007: > 5 rules per leaf after rekey (e.g. 16)
  const MUTANT_ID: int                   // 0 = sound; 1..25 = single-guard/binding mutants

  // Mutant ID tags
  pure val MUT_NONE: int                                  = 0
  pure val MUT_BXF001_ADMIT_UNOWNED_HOST: int             = 1
  pure val MUT_BXF002_BYPASS_HEAVY_SLOT: int              = 2
  pure val MUT_BXF003_ADMIT_SHARED_OVER_BUDGET: int       = 3
  pure val MUT_BXF004_GPU_JOB_WITHOUT_LOCK: int           = 4
  pure val MUT_BXF005_SIGTERM_WITHOUT_LOG_OR_EARLY: int   = 5
  pure val MUT_BXF006_KILL_SLOW_PROVER: int               = 6
  pure val MUT_BXF007_ACCEPT_BAD_DIGEST_TRANSFER: int     = 7
  pure val MUT_BXF009_LONG_RUN_NOT_DURABLE: int           = 8
  pure val MUT_BXF010_LEAVE_DROPPED_QUEUE_STRANDED: int   = 9
  pure val MUT_BXF011_CROSS_BOX_SPEED_CLAIM: int          = 10
  pure val MUT_BXF012_CALL_CONF_COMPUTE: int              = 11
  pure val MUT_BXF013_EARLY_OR_UNRECHECKED_SIGKILL: int   = 12
  pure val MUT_BXF014_ALLOW_DIRECT_ODIN_HUGINN: int       = 13
  pure val MUT_GPV001_CUDA_APP_ON_CPU: int                = 14
  pure val MUT_GPV002_GPU_CPU_PUBVALS_DIFFER: int         = 15
  pure val MUT_GPV003_LFR_GPU_DIFFERS_FROM_CPU: int       = 16
  pure val MUT_GPV004_IGNORE_NODE_GPU_SWITCH: int         = 17
  pure val MUT_GPV006_WRITE_PARTIAL_PROOF_ON_FAULT: int   = 18
  pure val MUT_GPV007_LEAF_LAYER_STAYS_CPU: int           = 19
  pure val MUT_PDM001_RELOAD_SRS_ON_SECOND_CALL: int      = 20
  pure val MUT_PDM002_WORLD_READABLE_SOCKET: int          = 21
  pure val MUT_PDM003_CRASH_ON_UNKNOWN_OP: int            = 22
  pure val MUT_PDM005_REUSE_STATIC_BLINDER: int           = 23
  pure val MUT_PDM006_SERVE_MISMATCHED_MANIFEST: int      = 24
  pure val MUT_PDM007_ONE_LEAF_INTERNAL_NODES: int        = 25
  pure val MUT_PDM008_STATS_LEAKS_WITNESS: int            = 26
  pure val MUT_PDM009_CARRYOVER_PRIOR_BLINDER: int        = 27
  pure val MUT_LPC007_SCALEUP_CHANGES_KEYS: int           = 28
  pure val MUT_PRD003_EXPORT_STARK_OUTSIDE_ROSTER: int    = 29

  // =========================================================================
  // 2. Domain Enums & Cohesive Record Types
  // =========================================================================

  type HostId =
    | HostMuninn
    | HostHuginn
    | HostOdin
    | HostFourthOwned
    | HostRentalExternal

  type LockHolderRole =
    | HolderNone
    | HolderProver
    | HolderWrapd
    | HolderOrphanSampler

  type SignalKind =
    | SigTerm
    | SigKill

  type BackendKind =
    | BackendCuda
    | BackendCpu

  type DaemonOpKind =
    | OpProve
    | OpCall
    | OpStats
    | OpQuit
    | OpUnknownDump

  type FleetRefusal =
    | FleetRefusalNone
    | FleetRefusalUnownedHost(HostId)
    | FleetRefusalSharedOverBudget(int)
    | FleetRefusalDigestMismatch((HostId, HostId))
    | FleetRefusalDirectSpokeTransfer((HostId, HostId))
    | FleetRefusalGpuDeviceFault((HostId, int))
    | FleetRefusalSocketPermissionOrRemote
    | FleetRefusalUnknownDaemonOp
    | FleetRefusalDaemonManifestMismatch((int, int))
    | FleetRefusalForeignOrReusedBlinder
    | FleetRefusalStarkExportOutsideTrustDomain(HostId)

  type BoxRuntimeState = {
    linkUp: bool,
    linkDroppedWhileOwnerAway: bool,
    failoverHandled: bool,
    queuedSuiteJobs: int,
    activeGpuProvingJobs: int,
    gpuWaitQueue: int,
    gpuLockHeld: bool,
    lockHolderRole: LockHolderRole,
    lockHolderPid: int,
    holderParentAlive: bool,
    lockStallMin: int,
    sigtermLoggedOnBus: bool,
    sigtermSent: bool,
    elapsedSinceSigtermSec: int,
    cudaBuildEnabled: bool,
    gpuFeatureEnabled: bool,
  }

  type AdmittedJobRec = {
    jobId: int,
    targetHost: HostId,
    admittedRosterSnapshot: Set[HostId],
    isHeavyMuninnJob: bool,
    usedHeavySlot: bool,
    isExclusiveSlot: bool,
    reqMemGb: int,
    plannedMin: int,
    startedAsSystemdUserUnit: bool,
    persistentLogEnabled: bool,
    resumedAfterReboot: bool,
  }

  type LockSignalEvent = {
    boxHost: HostId,
    pid: int,
    targetRole: LockHolderRole,
    parentWasAlive: bool,
    stallMin: int,
    sigKind: SignalKind,
    loggedOnBusBeforeSignal: bool,
    precededBySigtermSamePid: bool,
    elapsedAfterSigtermSec: int,
    recheckedEligible: bool,
  }

  type FileTransferRec = {
    srcHost: HostId,
    dstHost: HostId,
    routedViaMuninn: bool,
    busMsgSha256: int,
    actualSha256: int,
    digestCheckedOk: bool,
    isStark: bool,
    wrapStarted: bool,
  }

  type BusMessageRec = {
    msgId: int,
    senderHost: HostId,
    recipientHost: HostId,
    appendedToAllLog: bool,
    deliveredToRecipient: bool,
  }

  type GpuProbeRec = {
    boxHost: HostId,
    usedQueryGpu: bool,
    usedCudaMemCall: bool,
    usedConfCompute: bool,
  }

  type GpuStageProofRec = {
    boxHost: HostId,
    progId: int,
    shardIdx: int,
    cudaBuild: bool,
    gpuScheduled: bool,
    nodeGpuSwitchOff: bool,
    lGpuSwitchOff: bool,
    moriGpuOff: bool,
    appBackend: BackendKind,
    leafLayerBackend: BackendKind,
    nodeBackend: BackendKind,
    lBackend: BackendKind,
    publicValuesDigest: int,
    tapeId: int,
    lfrProofBytesDigest: int,
    stockVerified: bool,
    lStockVerified: bool,
  }

  type GpuFaultRec = {
    faultHost: HostId,
    shardIdx: int,
    runStopped: bool,
    namedShardAndDevice: bool,
    finalProofWritten: bool,
  }

  type WrapDaemonState = {
    running: bool,
    loadedManifestSha: int,
    residentItemCount: int,
    keyReloadsAfterStart: int,
    socketUnderXdgRuntime: bool,
    socketMode: int,
    localSameUserOnly: bool,
    otherUserServedCount: int,
    servedSeqCounter: int,
    lastServedPathC: bool,
  }

  type StarkWorkerState = {
    serveRunning: bool,
    starkKeysResident: bool,
    lKeyResident: bool,
    lfrKeyResident: bool,
    keyReloadsAfterStart: int,
    runsServedCount: int,
  }

  type ServedWrapProofRec = {
    reqSeq: int,
    stmtId: int,
    leafCount: int,
    reqManifestSha: int,
    daemonManifestSha: int,
    blinderId: int,
    blinderFromLocalCsprng: bool,
    helperBlinderUsed: bool,
    priorBlinderCarryover: int,
    usedPathC: bool,
    internalNodeProofsMade: int,
    wrapProofBytesDigest: int,
    verified: bool,
  }

  type StatsReplyRec = {
    stageCode: int,
    shardCount: int,
    stageTimeMs: int,
    leakedSecretWords: int,
  }

  type TimingClaimRec = {
    measuredHost: HostId,
    baselineHost: HostId,
    keysSha: int,
    warmMs: int,
    coldMs: int,
    peakMemGb: int,
    hostLoadRecorded: bool,
  }

  type ScaleDemoRec = {
    scaleFactor: int,          // 1, 2, 4, or 8
    rulesPerLeaf: int,
    keysSha: int,
    joinedMuninnHuginn: bool,
    claimPub: int,
  }

  type BoxFleetState = {
    ownedRoster: Set[HostId],
    activeKeysSha: int,
    muninnSharedRunning: int,
    muninnExclusiveRunning: bool,
    muninnSlotWaiters: int,
    nextJobId: int,
    boxes: HostId -> BoxRuntimeState,
    admittedJobs: Set[AdmittedJobRec],
    killEvents: Set[LockSignalEvent],
    acceptedTransfers: Set[FileTransferRec],
    relayedMessages: Set[BusMessageRec],
    allLogMsgIds: Set[int],
    gpuProbeCalls: Set[GpuProbeRec],
    stageProofs: Set[GpuStageProofRec],
    gpuFaultRecords: Set[GpuFaultRec],
    daemon: WrapDaemonState,
    starkWorker: StarkWorkerState,
    consumedBlinderIds: Set[int],
    servedWrapProofs: Set[ServedWrapProofRec],
    statsReplies: Set[StatsReplyRec],
    timingRecords: Set[TimingClaimRec],
    scaleDemoRuns: Set[ScaleDemoRec],
    lastRefusal: FleetRefusal,
  }

  // =========================================================================
  // 3. Domain Constants & Pure Helper Functions (Step 3)
  // =========================================================================

  pure val INITIAL_OWNED_ROSTER: Set[HostId] = Set(HostMuninn, HostHuginn, HostOdin)
  pure val ALL_HOSTS: Set[HostId] = Set(HostMuninn, HostHuginn, HostOdin, HostFourthOwned, HostRentalExternal)
  pure val CORE_BOXES: Set[HostId] = Set(HostMuninn, HostHuginn, HostOdin)

  pure val T10M_KEYS_SHA: int = 100
  pure val T25_KEYS_SHA: int = 250

  // Deterministic seeded proof digest for PDM-009 (`X` after `Y` vs fresh `X` under same seed)
  pure def seededWrapProofDigest(stmtId: int, seedVal: int, priorBlinderCarry: int): int =
    10000 + 100 * stmtId + seedVal + priorBlinderCarry

  // Pure guard: can a job be admitted on `targetHost`? (BXF-001..003, BXF-009)
  pure def canAdmitJob(
    s: BoxFleetState,
    targetHost: HostId,
    isSharedOnMuninn: bool,
    reqMemGb: int,
    mutId: int
  ): bool =
    val rosterOk =
      if (mutId == MUT_BXF001_ADMIT_UNOWNED_HOST) true
      else s.ownedRoster.contains(targetHost)
    val muninnSlotOk =
      if (targetHost != HostMuninn) true
      else if (isSharedOnMuninn) {
        val budgetOk =
          if (mutId == MUT_BXF003_ADMIT_SHARED_OVER_BUDGET) true
          else (reqMemGb <= HEAVY_BUDGET_GB)
        budgetOk and not(s.muninnExclusiveRunning) and s.muninnSharedRunning < HEAVY_SLOTS_MAX
      } else {
        not(s.muninnExclusiveRunning) and s.muninnSharedRunning == 0
      }
    rosterOk and muninnSlotOk

  // Pure guard: is the current lock holder on `bx` an orphan eligible for SIGTERM? (BXF-005, BXF-006)
  pure def isEligibleOrphanForSigterm(bx: BoxRuntimeState, mutId: int): bool =
    if (mutId == MUT_BXF006_KILL_SLOW_PROVER)
      bx.gpuLockHeld
    else if (mutId == MUT_BXF005_SIGTERM_WITHOUT_LOG_OR_EARLY)
      (bx.gpuLockHeld and bx.lockHolderRole == HolderOrphanSampler and not(bx.holderParentAlive))
    else
      bx.gpuLockHeld
        and bx.lockHolderRole == HolderOrphanSampler
        and not(bx.holderParentAlive)
        and bx.lockStallMin > ORPHAN_STALL_MIN

  // Pure guard: is the current lock holder eligible for SIGKILL escalation? (BXF-013, BXF-006)
  pure def isEligibleOrphanForSigkill(bx: BoxRuntimeState, mutId: int): bool =
    if (mutId == MUT_BXF013_EARLY_OR_UNRECHECKED_SIGKILL)
      (bx.gpuLockHeld and bx.sigtermSent)
    else
      bx.gpuLockHeld
        and bx.sigtermSent
        and bx.sigtermLoggedOnBus
        and bx.elapsedSinceSigtermSec >= SIGKILL_GRACE_SEC
        and bx.lockHolderRole == HolderOrphanSampler
        and not(bx.holderParentAlive)

  // Pure guard: is a cross-box file transfer valid? (BXF-007, BXF-014, PRD-003)
  pure def canAcceptFileTransfer(
    s: BoxFleetState,
    srcHost: HostId,
    dstHost: HostId,
    viaMuninn: bool,
    busSha: int,
    actualSha: int,
    isStark: bool,
    mutId: int
  ): bool =
    val digestOk =
      if (mutId == MUT_BXF007_ACCEPT_BAD_DIGEST_TRANSFER) true
      else (busSha == actualSha)
    val spokePair =
      (srcHost == HostOdin and dstHost == HostHuginn)
        or (srcHost == HostHuginn and dstHost == HostOdin)
    val routeOk =
      if (mutId == MUT_BXF014_ALLOW_DIRECT_ODIN_HUGINN) true
      else (not(spokePair) or viaMuninn)
    val trustDomainOk =
      if (mutId == MUT_PRD003_EXPORT_STARK_OUTSIDE_ROSTER) true
      else (not(isStark) or (s.ownedRoster.contains(srcHost) and s.ownedRoster.contains(dstHost)))
    srcHost != dstHost and digestOk and routeOk and trustDomainOk

  // Pure guard: can `wrapd` serve a `prove` or `call` request? (PDM-001, PDM-005, PDM-006, PRD-001, PRD-002)
  pure def canDaemonServeCall(
    s: BoxFleetState,
    reqManifestSha: int,
    sameUserLocal: bool,
    helperSuppliedBlinder: bool,
    reusedBlinderId: bool,
    mutId: int
  ): bool =
    val manifestOk =
      if (mutId == MUT_PDM006_SERVE_MISMATCHED_MANIFEST) true
      else (reqManifestSha == s.daemon.loadedManifestSha)
    val peerOk =
      if (mutId == MUT_PDM002_WORLD_READABLE_SOCKET) true
      else sameUserLocal
    val blinderOk =
      if (mutId == MUT_PDM005_REUSE_STATIC_BLINDER) true
      else (not(helperSuppliedBlinder) and not(reusedBlinderId))
    s.daemon.running and manifestOk and peerOk and blinderOk

  // =========================================================================
  // 4. State Variables & Initialization (Step 4)
  // =========================================================================

  var fleet: BoxFleetState
  var lastOp: str

  pure val INIT_BOX_STATE: BoxRuntimeState = {
    linkUp: true,
    linkDroppedWhileOwnerAway: false,
    failoverHandled: false,
    queuedSuiteJobs: 5,
    activeGpuProvingJobs: 0,
    gpuWaitQueue: 0,
    gpuLockHeld: false,
    lockHolderRole: HolderNone,
    lockHolderPid: 0,
    holderParentAlive: false,
    lockStallMin: 0,
    sigtermLoggedOnBus: false,
    sigtermSent: false,
    elapsedSinceSigtermSec: 0,
    cudaBuildEnabled: true,
    gpuFeatureEnabled: true,
  }

  action init: bool = all {
    fleet' = {
      ownedRoster: INITIAL_OWNED_ROSTER,
      activeKeysSha: T25_KEYS_SHA,
      muninnSharedRunning: 0,
      muninnExclusiveRunning: false,
      muninnSlotWaiters: 0,
      nextJobId: 1,
      boxes: ALL_HOSTS.mapBy(_ => INIT_BOX_STATE),
      admittedJobs: Set(),
      killEvents: Set(),
      acceptedTransfers: Set(),
      relayedMessages: Set(),
      allLogMsgIds: Set(),
      gpuProbeCalls: Set(),
      stageProofs: Set(),
      gpuFaultRecords: Set(),
      daemon: {
        running: false,
        loadedManifestSha: 0,
        residentItemCount: 0,
        keyReloadsAfterStart: 0,
        socketUnderXdgRuntime: false,
        socketMode: 0,
        localSameUserOnly: true,
        otherUserServedCount: 0,
        servedSeqCounter: 0,
        lastServedPathC: false,
      },
      starkWorker: {
        serveRunning: false,
        starkKeysResident: false,
        lKeyResident: false,
        lfrKeyResident: false,
        keyReloadsAfterStart: 0,
        runsServedCount: 0,
      },
      consumedBlinderIds: Set(),
      servedWrapProofs: Set(),
      statsReplies: Set(),
      timingRecords: Set(),
      scaleDemoRuns: Set(),
      lastRefusal: FleetRefusalNone,
    },
    lastOp' = "init",
  }

  // =========================================================================
  // 5. Guarded Actions (Tagged with EARS Requirement IDs)
  // =========================================================================

  // Implements BXF-001 (Scenario: A fourth owned box added to user's roster is admitted)
  action addFourthOwnedBoxToRoster: bool = all {
    not(fleet.ownedRoster.contains(HostFourthOwned)),
    fleet' = {
      ...fleet,
      ownedRoster: fleet.ownedRoster.union(Set(HostFourthOwned)),
      lastRefusal: FleetRefusalNone,
    },
    lastOp' = "addFourthOwnedBoxToRoster",
  }

  // Implements BXF-001 (Scenario: A rented host `gpu-rental-1` not in roster is refused)
  action refuseUnownedHostJob(h: HostId): bool = all {
    not(fleet.ownedRoster.contains(h)),
    fleet' = {
      ...fleet,
      lastRefusal: FleetRefusalUnownedHost(h),
    },
    lastOp' = "refuseUnownedHostJob",
  }

  // Implements BXF-001, BXF-002, BXF-003, BXF-009:
  // Start an admitted job on an owned box (using `heavy-slot` on muninn, and `systemd-run --user` if > 30 min)
  action admitAndStartBoxJob(h: HostId, reqMemGb: int, plannedMin: int, isExclusive: bool): bool =
    val isSharedMuninn = (h == HostMuninn and not(isExclusive))
    val useSlot =
      if (h != HostMuninn) false
      else not(MUTANT_ID == MUT_BXF002_BYPASS_HEAVY_SLOT)
    val needsDurable = plannedMin > DURABLE_RUN_THRESHOLD_MIN
    val durableUnit =
      if (MUTANT_ID == MUT_BXF009_LONG_RUN_NOT_DURABLE) false
      else needsDurable
    all {
      canAdmitJob(fleet, h, isSharedMuninn, reqMemGb, MUTANT_ID),
      fleet.nextJobId <= 8,
      val jRec: AdmittedJobRec = {
        jobId: fleet.nextJobId,
        targetHost: h,
        admittedRosterSnapshot: fleet.ownedRoster,
        isHeavyMuninnJob: (h == HostMuninn),
        usedHeavySlot: useSlot,
        isExclusiveSlot: (h == HostMuninn and isExclusive),
        reqMemGb: reqMemGb,
        plannedMin: plannedMin,
        startedAsSystemdUserUnit: durableUnit,
        persistentLogEnabled: durableUnit,
        resumedAfterReboot: false,
      }
      fleet' = {
        ...fleet,
        nextJobId: fleet.nextJobId + 1,
        muninnSharedRunning:
          if (h == HostMuninn and not(isExclusive)) (fleet.muninnSharedRunning + 1)
          else fleet.muninnSharedRunning,
        muninnExclusiveRunning:
          if (h == HostMuninn and isExclusive) true
          else fleet.muninnExclusiveRunning,
        admittedJobs: fleet.admittedJobs.union(Set(jRec)),
        lastRefusal: FleetRefusalNone,
      },
      lastOp' = "admitAndStartBoxJob",
    }

  // Implements BXF-002 (Scenario: 3 running jobs on muninn -> 4th waits for a slot)
  action enqueueMuninnSlotWaiter: bool = all {
    fleet.muninnSharedRunning == HEAVY_SLOTS_MAX or fleet.muninnExclusiveRunning,
    fleet.muninnSlotWaiters < 2,
    fleet' = {
      ...fleet,
      muninnSlotWaiters: fleet.muninnSlotWaiters + 1,
    },
    lastOp' = "enqueueMuninnSlotWaiter",
  }

  // Release a finished muninn slot (promoting a waiting job if any)
  action releaseMuninnSlot: bool = all {
    fleet.muninnSharedRunning > 0 or fleet.muninnExclusiveRunning,
    val promote = fleet.muninnSlotWaiters > 0
    fleet' = {
      ...fleet,
      muninnSharedRunning:
        if (fleet.muninnExclusiveRunning) (if (promote) 1 else 0)
        else (if (promote) fleet.muninnSharedRunning else (fleet.muninnSharedRunning - 1)),
      muninnExclusiveRunning: false,
      muninnSlotWaiters: if (promote) (fleet.muninnSlotWaiters - 1) else fleet.muninnSlotWaiters,
    },
    lastOp' = "releaseMuninnSlot",
  }

  // Implements BXF-003 (Scenario: 70 GB T25 keygen refused as shared -> restarted with `--exclusive`)
  action refuseSharedOverBudgetOnMuninn(reqMemGb: int): bool = all {
    reqMemGb > HEAVY_BUDGET_GB,
    fleet' = {
      ...fleet,
      lastRefusal: FleetRefusalSharedOverBudget(reqMemGb),
    },
    lastOp' = "refuseSharedOverBudgetOnMuninn",
  }

  // Implements BXF-009 (Scenario: Reboot during durable unit -> restart from resumable state with surviving first-part log)
  action rebootAndResumeDurableUnits: bool = all {
    fleet.admittedJobs.exists(j => j.startedAsSystemdUserUnit and not(j.resumedAfterReboot)),
    val updatedJobs = fleet.admittedJobs.map(j =>
      if (j.startedAsSystemdUserUnit) { ...j, resumedAfterReboot: true } else j
    )
    fleet' = {
      ...fleet,
      muninnSharedRunning: 0,
      muninnExclusiveRunning: false,
      admittedJobs: updatedJobs,
    },
    lastOp' = "rebootAndResumeDurableUnits",
  }

  // Implements BXF-004: Start GPU proving job holding exclusive `flock` on `~/sherlock/gpu.lock` (or queue waiter)
  action startOrQueueGpuJobOnBox(h: HostId): bool =
    val bx = fleet.boxes.get(h)
    all {
      fleet.ownedRoster.contains(h),
      if (not(bx.gpuLockHeld)) {
        val acquireLock = not(MUTANT_ID == MUT_BXF004_GPU_JOB_WITHOUT_LOCK)
        val updatedBx: BoxRuntimeState = {
          ...bx,
          activeGpuProvingJobs: bx.activeGpuProvingJobs + 1,
          gpuLockHeld: acquireLock,
          lockHolderRole: if (acquireLock) HolderProver else HolderNone,
          lockHolderPid: 4001,
          holderParentAlive: true,
          lockStallMin: 0,
          sigtermLoggedOnBus: false,
          sigtermSent: false,
          elapsedSinceSigtermSec: 0,
        }
        fleet' = {
          ...fleet,
          boxes: fleet.boxes.put(h, updatedBx),
          lastRefusal: FleetRefusalNone,
        }
      } else {
        all {
          bx.gpuWaitQueue < 2,
          val updatedBx: BoxRuntimeState = {
            ...bx,
            gpuWaitQueue: bx.gpuWaitQueue + 1,
          }
          fleet' = {
            ...fleet,
            boxes: fleet.boxes.put(h, updatedBx),
          }
        }
      },
      lastOp' = "startOrQueueGpuJobOnBox",
    }

  // Implements BXF-004: Finish GPU job and release `~/sherlock/gpu.lock` (or hand to waiting job)
  action finishGpuJobReleaseLock(h: HostId): bool =
    val bx = fleet.boxes.get(h)
    all {
      bx.activeGpuProvingJobs > 0,
      val hasWaiter = bx.gpuWaitQueue > 0
      val updatedBx: BoxRuntimeState = {
        ...bx,
        activeGpuProvingJobs: if (hasWaiter) 1 else 0,
        gpuWaitQueue: if (hasWaiter) (bx.gpuWaitQueue - 1) else 0,
        gpuLockHeld: hasWaiter,
        lockHolderRole: if (hasWaiter) HolderProver else HolderNone,
        lockHolderPid: if (hasWaiter) 4002 else 0,
        holderParentAlive: hasWaiter,
        lockStallMin: 0,
      }
      fleet' = {
        ...fleet,
        boxes: fleet.boxes.put(h, updatedBx),
      },
      lastOp' = "finishGpuJobReleaseLock",
    }

  // Precondition setup for BXF-005, BXF-006, BXF-013:
  // Either an orphan sampler (`pid 592115`, parent dead, stalled 12 min) or a slow prover (`40 min`) holds `gpu.lock`
  action stageLockHolderScenario(h: HostId, role: LockHolderRole, stallMinutes: int): bool =
    val bx = fleet.boxes.get(h)
    all {
      CORE_BOXES.contains(h),
      not(bx.gpuLockHeld),
      bx.activeGpuProvingJobs == 0,
      Set(HolderOrphanSampler, HolderProver).contains(role),
      val updatedBx: BoxRuntimeState = {
        ...bx,
        activeGpuProvingJobs: if (role == HolderProver) 1 else 0,
        gpuLockHeld: true,
        lockHolderRole: role,
        lockHolderPid: if (role == HolderOrphanSampler) 592115 else 610001,
        holderParentAlive: (role == HolderProver),
        lockStallMin: stallMinutes,
        sigtermLoggedOnBus: false,
        sigtermSent: false,
        elapsedSinceSigtermSec: 0,
      }
      fleet' = {
        ...fleet,
        boxes: fleet.boxes.put(h, updatedBx),
      },
      lastOp' = "stageLockHolderScenario",
    }

  // Implements BXF-006 (Scenario: Slow leaf prover holding lock 40 min is left running and posted on bus)
  action observeSlowProverLeaveRunning(h: HostId): bool =
    val bx = fleet.boxes.get(h)
    all {
      bx.gpuLockHeld,
      Set(HolderProver, HolderWrapd).contains(bx.lockHolderRole),
      val nextMsgId = fleet.allLogMsgIds.size() + 100
      val msg: BusMessageRec = {
        msgId: nextMsgId,
        senderHost: h,
        recipientHost: HostMuninn,
        appendedToAllLog: true,
        deliveredToRecipient: true,
      }
      fleet' = {
        ...fleet,
        relayedMessages: fleet.relayedMessages.union(Set(msg)),
        allLogMsgIds: fleet.allLogMsgIds.union(Set(nextMsgId)),
      },
      lastOp' = "observeSlowProverLeaveRunning",
    }

  // Implements BXF-005 (Scenario: Orphan sampler stalled > 10 min with dead parent -> log on bus, then SIGTERM)
  action reapOrphanLockSigterm(h: HostId, trapSigterm: bool): bool =
    val bx = fleet.boxes.get(h)
    all {
      isEligibleOrphanForSigterm(bx, MUTANT_ID),
      not(bx.sigtermSent),
      val logFirst = not(MUTANT_ID == MUT_BXF005_SIGTERM_WITHOUT_LOG_OR_EARLY)
      val ev: LockSignalEvent = {
        boxHost: h,
        pid: bx.lockHolderPid,
        targetRole: bx.lockHolderRole,
        parentWasAlive: bx.holderParentAlive,
        stallMin: bx.lockStallMin,
        sigKind: SigTerm,
        loggedOnBusBeforeSignal: logFirst,
        precededBySigtermSamePid: false,
        elapsedAfterSigtermSec: 0,
        recheckedEligible: true,
      }
      val updatedBx: BoxRuntimeState =
        if (trapSigterm) {
          ...bx,
          sigtermLoggedOnBus: logFirst,
          sigtermSent: true,
          elapsedSinceSigtermSec: 35,
        } else {
          ...bx,
          activeGpuProvingJobs: 0,
          gpuLockHeld: false,
          lockHolderRole: HolderNone,
          lockHolderPid: 0,
          lockStallMin: 0,
          sigtermLoggedOnBus: false,
          sigtermSent: false,
          elapsedSinceSigtermSec: 0,
        }
      fleet' = {
        ...fleet,
        boxes: fleet.boxes.put(h, updatedBx),
        killEvents: fleet.killEvents.union(Set(ev)),
      },
      lastOp' = "reapOrphanLockSigterm",
    }

  // Implements BXF-013 (Scenario: Orphan traps SIGTERM -> after >= 30s, recheck pid/parent/role and send SIGKILL)
  action escalateOrphanLockSigkill(h: HostId): bool =
    val bx = fleet.boxes.get(h)
    all {
      isEligibleOrphanForSigkill(bx, MUTANT_ID),
      val elapsed = if (MUTANT_ID == MUT_BXF013_EARLY_OR_UNRECHECKED_SIGKILL) 10 else bx.elapsedSinceSigtermSec
      val recheckOk =
        if (MUTANT_ID == MUT_BXF013_EARLY_OR_UNRECHECKED_SIGKILL) false
        else (bx.lockHolderRole == HolderOrphanSampler and not(bx.holderParentAlive))
      val ev: LockSignalEvent = {
        boxHost: h,
        pid: bx.lockHolderPid,
        targetRole: bx.lockHolderRole,
        parentWasAlive: bx.holderParentAlive,
        stallMin: bx.lockStallMin,
        sigKind: SigKill,
        loggedOnBusBeforeSignal: true,
        precededBySigtermSamePid: bx.sigtermSent,
        elapsedAfterSigtermSec: elapsed,
        recheckedEligible: recheckOk,
      }
      val updatedBx: BoxRuntimeState = {
        ...bx,
        gpuLockHeld: false,
        lockHolderRole: HolderNone,
        lockHolderPid: 0,
        lockStallMin: 0,
        sigtermLoggedOnBus: false,
        sigtermSent: false,
        elapsedSinceSigtermSec: 0,
      }
      fleet' = {
        ...fleet,
        boxes: fleet.boxes.put(h, updatedBx),
        killEvents: fleet.killEvents.union(Set(ev)),
      },
      lastOp' = "escalateOrphanLockSigkill",
    }

  // Implements BXF-007, BXF-014, PRD-003: Move result file across owned boxes with verified sha256 (through muninn)
  action transferFileVerified(
    srcHost: HostId,
    dstHost: HostId,
    viaMuninn: bool,
    busSha: int,
    actualSha: int,
    isStark: bool
  ): bool = all {
    canAcceptFileTransfer(fleet, srcHost, dstHost, viaMuninn, busSha, actualSha, isStark, MUTANT_ID),
    val tf: FileTransferRec = {
      srcHost: srcHost,
      dstHost: dstHost,
      routedViaMuninn: viaMuninn,
      busMsgSha256: busSha,
      actualSha256: actualSha,
      digestCheckedOk: (busSha == actualSha),
      isStark: isStark,
      wrapStarted: (dstHost == HostMuninn and busSha == actualSha),
    }
    fleet' = {
      ...fleet,
      acceptedTransfers: fleet.acceptedTransfers.union(Set(tf)),
      lastRefusal: FleetRefusalNone,
    },
    lastOp' = "transferFileVerified",
  }

  // Implements BXF-007 (Scenario: Truncated L_Fr proof from odin fails digest check on muninn -> wrap does not start)
  action refuseDigestMismatchTransfer(srcHost: HostId, dstHost: HostId, busSha: int, badSha: int): bool = all {
    busSha != badSha,
    fleet' = {
      ...fleet,
      lastRefusal: FleetRefusalDigestMismatch((srcHost, dstHost)),
    },
    lastOp' = "refuseDigestMismatchTransfer",
  }

  // Implements BXF-014 (Scenario: Direct odin-to-huginn copy refused by box bus)
  action refuseDirectSpokeTransfer(srcHost: HostId, dstHost: HostId): bool = all {
    (srcHost == HostOdin and dstHost == HostHuginn)
      or (srcHost == HostHuginn and dstHost == HostOdin),
    fleet' = {
      ...fleet,
      lastRefusal: FleetRefusalDirectSpokeTransfer((srcHost, dstHost)),
    },
    lastOp' = "refuseDirectSpokeTransfer",
  }

  // Implements PRD-003: Refuse exporting STARK proof/checkpoint outside the user's owned-box trust domain
  action refuseStarkExportOutsideRoster(dstHost: HostId): bool = all {
    not(fleet.ownedRoster.contains(dstHost)),
    fleet' = {
      ...fleet,
      lastRefusal: FleetRefusalStarkExportOutsideTrustDomain(dstHost),
    },
    lastOp' = "refuseStarkExportOutsideRoster",
  }

  // Implements BXF-008: `box-relay` appends outbox line to `~/.cache/box-bus/all.log` and delivers to target box
  action postAndRelayBusMessage(senderHost: HostId, recipientHost: HostId): bool = all {
    CORE_BOXES.contains(senderHost),
    CORE_BOXES.contains(recipientHost),
    senderHost != recipientHost,
    fleet.relayedMessages.size() < 6,
    val mId = fleet.allLogMsgIds.size() + 1
    val msg: BusMessageRec = {
      msgId: mId,
      senderHost: senderHost,
      recipientHost: recipientHost,
      appendedToAllLog: true,
      deliveredToRecipient: true,
    }
    fleet' = {
      ...fleet,
      relayedMessages: fleet.relayedMessages.union(Set(msg)),
      allLogMsgIds: fleet.allLogMsgIds.union(Set(mId)),
    },
    lastOp' = "postAndRelayBusMessage",
  }

  // Implements BXF-010 (Scenario: odin link drops overnight -> lead moves 71 queued calls to huginn and logs move)
  action dropBoxLinkAndHandoverQueue(droppedHost: HostId, backupHost: HostId): bool =
    val dBx = fleet.boxes.get(droppedHost)
    val bBx = fleet.boxes.get(backupHost)
    all {
      CORE_BOXES.contains(droppedHost),
      CORE_BOXES.contains(backupHost),
      droppedHost != backupHost,
      dBx.linkUp,
      bBx.linkUp,
      dBx.queuedSuiteJobs > 0,
      val keepStranded = (MUTANT_ID == MUT_BXF010_LEAVE_DROPPED_QUEUE_STRANDED)
      val movedCount = if (keepStranded) 0 else dBx.queuedSuiteJobs
      val updatedDropped: BoxRuntimeState = {
        ...dBx,
        linkUp: false,
        linkDroppedWhileOwnerAway: true,
        failoverHandled: true,
        queuedSuiteJobs: if (keepStranded) dBx.queuedSuiteJobs else 0,
      }
      val updatedBackup: BoxRuntimeState = {
        ...bBx,
        queuedSuiteJobs: bBx.queuedSuiteJobs + movedCount,
      }
      val mId = fleet.allLogMsgIds.size() + 500
      val logMsg: BusMessageRec = {
        msgId: mId,
        senderHost: HostMuninn,
        recipientHost: backupHost,
        appendedToAllLog: true,
        deliveredToRecipient: true,
      }
      fleet' = {
        ...fleet,
        boxes: fleet.boxes.put(droppedHost, updatedDropped).put(backupHost, updatedBackup),
        relayedMessages: fleet.relayedMessages.union(Set(logMsg)),
        allLogMsgIds: fleet.allLogMsgIds.union(Set(mId)),
      },
      lastOp' = "dropBoxLinkAndHandoverQueue",
    }

  // Implements BXF-012: Read GPU data via `nvidia-smi --query-gpu` + CUDA memory call, never `conf-compute`
  action probeGpuWithoutConfCompute(h: HostId): bool = all {
    CORE_BOXES.contains(h),
    fleet.gpuProbeCalls.size() < 4,
    val rec: GpuProbeRec = {
      boxHost: h,
      usedQueryGpu: true,
      usedCudaMemCall: true,
      usedConfCompute: (MUTANT_ID == MUT_BXF012_CALL_CONF_COMPUTE),
    }
    fleet' = {
      ...fleet,
      gpuProbeCalls: fleet.gpuProbeCalls.union(Set(rec)),
    },
    lastOp' = "probeGpuWithoutConfCompute",
  }

  // Implements GPV-001, GPV-002, GPV-003, GPV-004, GPV-005, GPV-007:
  // Prove shard/node/L/L_Fr stages respecting CUDA feature & runtime switches, matching CPU/GPU digests
  action proveStagePipeline(
    h: HostId,
    gpuScheduled: bool,
    nodeGpuOff: bool,
    lGpuOff: bool,
    moriGpuOff: bool
  ): bool =
    val bx = fleet.boxes.get(h)
    all {
      CORE_BOXES.contains(h),
      fleet.stageProofs.size() < 8,
      val appBk =
        if (not(bx.cudaBuildEnabled) or not(gpuScheduled)) BackendCpu
        else if (MUTANT_ID == MUT_GPV001_CUDA_APP_ON_CPU) BackendCpu
        else BackendCuda
      val leafLayerBk =
        if (not(bx.cudaBuildEnabled) or not(gpuScheduled)) BackendCpu
        else if (MUTANT_ID == MUT_GPV007_LEAF_LAYER_STAYS_CPU) BackendCpu
        else BackendCuda
      val nodeBk =
        if (not(bx.cudaBuildEnabled)) BackendCpu
        else if (MUTANT_ID == MUT_GPV004_IGNORE_NODE_GPU_SWITCH) BackendCuda
        else if (nodeGpuOff) BackendCpu
        else BackendCuda
      val lBk =
        if (not(bx.cudaBuildEnabled) or lGpuOff) BackendCpu else BackendCuda
      val pubValDig =
        if (MUTANT_ID == MUT_GPV002_GPU_CPU_PUBVALS_DIFFER and appBk == BackendCuda) 8889 else 8888
      val lfrDig =
        if (MUTANT_ID == MUT_GPV003_LFR_GPU_DIFFERS_FROM_CPU and not(moriGpuOff)) 7778 else 7777
      val sp: GpuStageProofRec = {
        boxHost: h,
        progId: 1,
        shardIdx: 0,
        cudaBuild: bx.cudaBuildEnabled,
        gpuScheduled: gpuScheduled,
        nodeGpuSwitchOff: nodeGpuOff,
        lGpuSwitchOff: lGpuOff,
        moriGpuOff: moriGpuOff,
        appBackend: appBk,
        leafLayerBackend: leafLayerBk,
        nodeBackend: nodeBk,
        lBackend: lBk,
        publicValuesDigest: pubValDig,
        tapeId: 1,
        lfrProofBytesDigest: lfrDig,
        stockVerified: true,
        lStockVerified: true,
      }
      fleet' = {
        ...fleet,
        stageProofs: fleet.stageProofs.union(Set(sp)),
        lastRefusal: FleetRefusalNone,
      },
      lastOp' = "proveStagePipeline",
    }

  // Implements GPV-006 (Scenario: Out of device memory -> stop run with shard index & device, write no partial proof)
  action faultGpuShardOutOfMemory(h: HostId, shardIdx: int): bool = all {
    CORE_BOXES.contains(h),
    shardIdx >= 0,
    fleet.gpuFaultRecords.size() < 3,
    val partialWritten = (MUTANT_ID == MUT_GPV006_WRITE_PARTIAL_PROOF_ON_FAULT)
    val fRec: GpuFaultRec = {
      faultHost: h,
      shardIdx: shardIdx,
      runStopped: true,
      namedShardAndDevice: true,
      finalProofWritten: partialWritten,
    }
    fleet' = {
      ...fleet,
      gpuFaultRecords: fleet.gpuFaultRecords.union(Set(fRec)),
      lastRefusal: FleetRefusalGpuDeviceFault((h, shardIdx)),
    },
    lastOp' = "faultGpuShardOutOfMemory",
  }

  // Implements PDM-001, PDM-002, PDM-004, PRD-001:
  // Start `wrapd serve` (mode 0600 under `$XDG_RUNTIME_DIR`, 6 resident items) and STARK worker in serve mode
  action startDaemonsOnBox: bool = all {
    not(fleet.daemon.running),
    val sockMode = if (MUTANT_ID == MUT_PDM002_WORLD_READABLE_SOCKET) 666 else SOCKET_MODE_OCTAL
    fleet' = {
      ...fleet,
      daemon: {
        running: true,
        loadedManifestSha: fleet.activeKeysSha,
        residentItemCount: WRAPD_RESIDENT_ITEMS,
        keyReloadsAfterStart: 0,
        socketUnderXdgRuntime: true,
        socketMode: sockMode,
        localSameUserOnly: (sockMode == SOCKET_MODE_OCTAL),
        otherUserServedCount: 0,
        servedSeqCounter: 0,
        lastServedPathC: false,
      },
      starkWorker: {
        serveRunning: true,
        starkKeysResident: true,
        lKeyResident: true,
        lfrKeyResident: true,
        keyReloadsAfterStart: 0,
        runsServedCount: 0,
      },
      lastRefusal: FleetRefusalNone,
    },
    lastOp' = "startDaemonsOnBox",
  }

  // Implements PDM-002, PRD-001 (Scenario: Second local user or remote peer connection refused by 0600 socket)
  action refuseUnauthorizedSocketPeer: bool = all {
    fleet.daemon.running,
    fleet' = {
      ...fleet,
      lastRefusal: FleetRefusalSocketPermissionOrRemote,
    },
    lastOp' = "refuseUnauthorizedSocketPeer",
  }

  // Implements PDM-003 (Scenario: Unknown operation `dump` returns error and daemon keeps serving)
  action daemonHandleUnknownOpDump: bool = all {
    fleet.daemon.running,
    val stayUp = not(MUTANT_ID == MUT_PDM003_CRASH_ON_UNKNOWN_OP)
    fleet' = {
      ...fleet,
      daemon: { ...fleet.daemon, running: stayUp },
      lastRefusal: FleetRefusalUnknownDaemonOp,
    },
    lastOp' = "daemonHandleUnknownOpDump",
  }

  // Implements PDM-006 (Scenario: Daemon loaded with old keys refuses mismatched manifest digest)
  action daemonRefuseManifestMismatch(reqManifestSha: int): bool = all {
    fleet.daemon.running,
    reqManifestSha != fleet.daemon.loadedManifestSha,
    fleet' = {
      ...fleet,
      lastRefusal: FleetRefusalDaemonManifestMismatch((fleet.daemon.loadedManifestSha, reqManifestSha)),
    },
    lastOp' = "daemonRefuseManifestMismatch",
  }

  // Implements PRD-002: Refuse helper-made or reused blinder slot
  action daemonRefuseBadBlinder: bool = all {
    fleet.daemon.running,
    fleet' = {
      ...fleet,
      lastRefusal: FleetRefusalForeignOrReusedBlinder,
    },
    lastOp' = "daemonRefuseBadBlinder",
  }

  // Implements PDM-001, PDM-004, PDM-005, PDM-006, PDM-007, PDM-009, PRD-001, PRD-002, PRD-004:
  // Serve a `prove` / `call` request with resident SRS/keys, fresh CSPRNG blinders, no carryover, and Path C for 1-leaf calls
  action daemonServeCall(
    stmtId: int,
    leafCount: int,
    reqManifestSha: int,
    sameUserLocal: bool
  ): bool =
    val seqNext = fleet.daemon.servedSeqCounter + 1
    val freshBlinderId =
      if (MUTANT_ID == MUT_PDM005_REUSE_STATIC_BLINDER) 999
      else (5000 + seqNext)
    val alreadyUsed = fleet.consumedBlinderIds.contains(freshBlinderId)
    all {
      leafCount >= 1,
      seqNext <= 6,
      canDaemonServeCall(fleet, reqManifestSha, sameUserLocal, false, alreadyUsed, MUTANT_ID),
      val reloadInc = if (MUTANT_ID == MUT_PDM001_RELOAD_SRS_ON_SECOND_CALL and seqNext > 1) 1 else 0
      val takePathC =
        if (leafCount == 1) not(MUTANT_ID == MUT_PDM007_ONE_LEAF_INTERNAL_NODES)
        else false
      val intNodes = if (leafCount == 1) (if (takePathC) 0 else 1) else (leafCount - 1)
      val carryOver =
        if (MUTANT_ID == MUT_PDM009_CARRYOVER_PRIOR_BLINDER and seqNext > 1) 17 else 0
      val proofDig = 30000 + 100 * stmtId + freshBlinderId + carryOver
      val wp: ServedWrapProofRec = {
        reqSeq: seqNext,
        stmtId: stmtId,
        leafCount: leafCount,
        reqManifestSha: reqManifestSha,
        daemonManifestSha: fleet.daemon.loadedManifestSha,
        blinderId: freshBlinderId,
        blinderFromLocalCsprng: true,
        helperBlinderUsed: false,
        priorBlinderCarryover: carryOver,
        usedPathC: takePathC,
        internalNodeProofsMade: intNodes,
        wrapProofBytesDigest: proofDig,
        verified: true,
      }
      fleet' = {
        ...fleet,
        daemon: {
          ...fleet.daemon,
          keyReloadsAfterStart: fleet.daemon.keyReloadsAfterStart + reloadInc,
          otherUserServedCount: fleet.daemon.otherUserServedCount + (if (sameUserLocal) 0 else 1),
          servedSeqCounter: seqNext,
          lastServedPathC: takePathC,
        },
        starkWorker: {
          ...fleet.starkWorker,
          runsServedCount: fleet.starkWorker.runsServedCount + 1,
        },
        consumedBlinderIds: fleet.consumedBlinderIds.union(Set(freshBlinderId)),
        servedWrapProofs: fleet.servedWrapProofs.union(Set(wp)),
        lastRefusal: FleetRefusalNone,
      },
      lastOp' = "daemonServeCall",
    }

  // Implements PDM-008: Serve GUI bridge `stats` reply with stage, shard counts, stage times, and 0 witness words
  action daemonServeStats: bool = all {
    fleet.daemon.running,
    fleet.statsReplies.size() < 4,
    val leaked = if (MUTANT_ID == MUT_PDM008_STATS_LEAKS_WITNESS) 3 else 0
    val rep: StatsReplyRec = {
      stageCode: 2,
      shardCount: 3,
      stageTimeMs: 1560,
      leakedSecretWords: leaked,
    }
    fleet' = {
      ...fleet,
      statsReplies: fleet.statsReplies.union(Set(rep)),
      lastRefusal: FleetRefusalNone,
    },
    lastOp' = "daemonServeStats",
  }

  // Implements BXF-011, LPC-005: Measure warm/cold call time & peak memory on huginn/odin against same-box baseline
  action recordBoxTimingClaim(measuredHost: HostId): bool = all {
    Set(HostHuginn, HostOdin).contains(measuredHost),
    fleet.timingRecords.size() < 4,
    val baseHost =
      if (MUTANT_ID == MUT_BXF011_CROSS_BOX_SPEED_CLAIM) HostMuninn
      else measuredHost
    val tRec: TimingClaimRec = {
      measuredHost: measuredHost,
      baselineHost: baseHost,
      keysSha: fleet.activeKeysSha,
      warmMs: if (measuredHost == HostOdin) 27770 else 71590,
      coldMs: if (measuredHost == HostOdin) 48000 else 113010,
      peakMemGb: 24,
      hostLoadRecorded: true,
    }
    fleet' = {
      ...fleet,
      timingRecords: fleet.timingRecords.union(Set(tRec)),
      lastRefusal: FleetRefusalNone,
    },
    lastOp' = "recordBoxTimingClaim",
  }

  // Implements LPC-007: Prove one program at 1x, 2x, 4x, and 8x run length under the same rekeyed keys (> 5 rules/leaf)
  // including muninn + huginn cross-host join with byte-equal `claim_pub`
  action runElasticScaleDemo: bool = all {
    fleet.scaleDemoRuns.size() == 0,
    val k8 = if (MUTANT_ID == MUT_LPC007_SCALEUP_CHANGES_KEYS) (fleet.activeKeysSha + 1) else fleet.activeKeysSha
    val runs = Set(
      { scaleFactor: 1, rulesPerLeaf: MIN_REKEY_LEAF_RULES, keysSha: fleet.activeKeysSha, joinedMuninnHuginn: true, claimPub: 91001 },
      { scaleFactor: 2, rulesPerLeaf: MIN_REKEY_LEAF_RULES, keysSha: fleet.activeKeysSha, joinedMuninnHuginn: false, claimPub: 91002 },
      { scaleFactor: 4, rulesPerLeaf: MIN_REKEY_LEAF_RULES, keysSha: fleet.activeKeysSha, joinedMuninnHuginn: false, claimPub: 91004 },
      { scaleFactor: 8, rulesPerLeaf: MIN_REKEY_LEAF_RULES, keysSha: k8, joinedMuninnHuginn: false, claimPub: 91008 }
    )
    fleet' = {
      ...fleet,
      scaleDemoRuns: runs,
      lastRefusal: FleetRefusalNone,
    },
    lastOp' = "runElasticScaleDemo",
  }

  // =========================================================================
  // 6. Nondeterministic `step` Relation (Step 6)
  // =========================================================================

  action step: bool = {
    nondet coreBox = CORE_BOXES.oneOf()
    nondet anyHost = ALL_HOSTS.oneOf()
    nondet boolFlag = Set(false, true).oneOf()
    nondet stmtChoice = Set(1, 2).oneOf()
    nondet leafCntChoice = Set(1, 3).oneOf()
    val jobHost = if (MUTANT_ID == MUT_BXF001_ADMIT_UNOWNED_HOST) anyHost else coreBox
    val muninnSharedMem = if (MUTANT_ID == MUT_BXF003_ADMIT_SHARED_OVER_BUDGET) 70 else 32
    val spokeViaMuninn = not(MUTANT_ID == MUT_BXF014_ALLOW_DIRECT_ODIN_HUGINN)
    val transferActualSha = if (MUTANT_ID == MUT_BXF007_ACCEPT_BAD_DIGEST_TRANSFER) 999 else 555
    val transferDst = if (MUTANT_ID == MUT_PRD003_EXPORT_STARK_OUTSIDE_ROSTER) HostRentalExternal else HostHuginn
    val reqManifest = if (MUTANT_ID == MUT_PDM006_SERVE_MISMATCHED_MANIFEST) T10M_KEYS_SHA else T25_KEYS_SHA
    val sameUserFlag = not(MUTANT_ID == MUT_PDM002_WORLD_READABLE_SOCKET)
    any {
      addFourthOwnedBoxToRoster,
      all { fleet.lastRefusal == FleetRefusalNone, refuseUnownedHostJob(HostRentalExternal) },
      admitAndStartBoxJob(jobHost, muninnSharedMem, 45, false),
      admitAndStartBoxJob(HostFourthOwned, 16, 15, false),
      enqueueMuninnSlotWaiter,
      releaseMuninnSlot,
      all { fleet.lastRefusal == FleetRefusalNone, refuseSharedOverBudgetOnMuninn(70) },
      admitAndStartBoxJob(HostMuninn, 70, 60, true),
      rebootAndResumeDurableUnits,
      startOrQueueGpuJobOnBox(coreBox),
      finishGpuJobReleaseLock(coreBox),
      stageLockHolderScenario(HostHuginn, HolderOrphanSampler, 12),
      stageLockHolderScenario(HostOdin, HolderProver, 40),
      observeSlowProverLeaveRunning(HostOdin),
      reapOrphanLockSigterm(HostHuginn, boolFlag),
      reapOrphanLockSigterm(HostOdin, false),
      escalateOrphanLockSigkill(HostHuginn),
      transferFileVerified(HostOdin, transferDst, spokeViaMuninn, 555, transferActualSha, true),
      all { fleet.lastRefusal == FleetRefusalNone, refuseDigestMismatchTransfer(HostOdin, HostMuninn, 555, 550) },
      all { fleet.lastRefusal == FleetRefusalNone, refuseDirectSpokeTransfer(HostOdin, HostHuginn) },
      all { fleet.lastRefusal == FleetRefusalNone, refuseStarkExportOutsideRoster(HostRentalExternal) },
      postAndRelayBusMessage(HostHuginn, HostOdin),
      dropBoxLinkAndHandoverQueue(HostOdin, HostHuginn),
      probeGpuWithoutConfCompute(HostMuninn),
      proveStagePipeline(coreBox, true, boolFlag, boolFlag, boolFlag),
      proveStagePipeline(coreBox, false, true, true, true),
      all { fleet.lastRefusal == FleetRefusalNone, faultGpuShardOutOfMemory(HostHuginn, 0) },
      startDaemonsOnBox,
      all { fleet.lastRefusal == FleetRefusalNone, refuseUnauthorizedSocketPeer },
      all { fleet.lastRefusal == FleetRefusalNone, daemonHandleUnknownOpDump },
      all { fleet.lastRefusal == FleetRefusalNone, daemonRefuseManifestMismatch(T10M_KEYS_SHA) },
      all { fleet.lastRefusal == FleetRefusalNone, daemonRefuseBadBlinder },
      daemonServeCall(stmtChoice, leafCntChoice, reqManifest, sameUserFlag),
      daemonServeStats,
      recordBoxTimingClaim(HostHuginn),
      recordBoxTimingClaim(HostOdin),
      runElasticScaleDemo,
    }
  }

  // =========================================================================
  // 7. Safety Invariants (`val inv_<ID>` and Master Conjunction `val inv`)
  // =========================================================================

  val consistency_constants: bool = and {
    HEAVY_SLOTS_MAX >= 1,
    HEAVY_BUDGET_GB >= 16,
    ORPHAN_STALL_MIN == 10,
    SIGKILL_GRACE_SEC >= 30,
    DURABLE_RUN_THRESHOLD_MIN == 30,
    WRAPD_RESIDENT_ITEMS == 6,
    SOCKET_MODE_OCTAL == 600,
    MIN_REKEY_LEAF_RULES > 5,
  }

  // BXF-001: Only hosts in the user's owned-box roster are admitted for proving jobs.
  val inv_BXF_001: bool =
    fleet.admittedJobs.forall(j => j.admittedRosterSnapshot.contains(j.targetHost))

  // BXF-002: Heavy jobs on muninn run through `heavy-slot` within `HEAVY_SLOTS_MAX` (or 1 exclusive).
  val inv_BXF_002: bool = and {
    fleet.admittedJobs.forall(j => j.isHeavyMuninnJob implies j.usedHeavySlot),
    fleet.muninnSharedRunning <= HEAVY_SLOTS_MAX,
    fleet.muninnExclusiveRunning implies (fleet.muninnSharedRunning == 0),
  }

  // BXF-003: Shared `heavy-slot` jobs on muninn never exceed `HEAVY_BUDGET_GB`.
  val inv_BXF_003: bool =
    fleet.admittedJobs.forall(j =>
      (j.isHeavyMuninnJob and not(j.isExclusiveSlot)) implies (j.reqMemGb <= HEAVY_BUDGET_GB)
    )

  // BXF-004: While a GPU proving job runs on a box, it holds an exclusive `flock` on `~/sherlock/gpu.lock`.
  val inv_BXF_004: bool =
    ALL_HOSTS.forall(h =>
      val bx = fleet.boxes.get(h)
      (bx.activeGpuProvingJobs > 0) implies (bx.gpuLockHeld and bx.activeGpuProvingJobs == 1)
    )

  // BXF-005: Every SIGTERM lock-reap logged on the bus first and targeted a dead-parent sampler stalled > 10 min.
  val inv_BXF_005: bool =
    fleet.killEvents.forall(ev =>
      (ev.sigKind == SigTerm) implies (
        ev.loggedOnBusBeforeSignal
          and ev.targetRole == HolderOrphanSampler
          and not(ev.parentWasAlive)
          and ev.stallMin > ORPHAN_STALL_MIN
      )
    )

  // BXF-006: Neither a prover nor a `wrapd` lock holder is ever ended by the orphan-lock rule.
  val inv_BXF_006: bool =
    fleet.killEvents.forall(ev =>
      ev.targetRole != HolderProver and ev.targetRole != HolderWrapd
    )

  // BXF-007: Every accepted cross-box result file verified `actualSha256 == busMsgSha256` before use/wrap.
  val inv_BXF_007: bool =
    fleet.acceptedTransfers.forall(tf =>
      tf.digestCheckedOk and tf.actualSha256 == tf.busMsgSha256
    )

  // BXF-008: Every relayed outbox message is appended to `~/.cache/box-bus/all.log`.
  val inv_BXF_008: bool =
    fleet.relayedMessages.forall(m =>
      m.appendedToAllLog and fleet.allLogMsgIds.contains(m.msgId)
    )

  // BXF-009: Runs planned > 30 minutes start as `systemd-run --user` units with persistent logs.
  val inv_BXF_009: bool =
    fleet.admittedJobs.forall(j =>
      (j.plannedMin > DURABLE_RUN_THRESHOLD_MIN)
        implies (j.startedAsSystemdUserUnit and j.persistentLogEnabled)
    )

  // BXF-010: A box whose link drops while the owner is away hands over all queued suite jobs to another box.
  val inv_BXF_010: bool =
    CORE_BOXES.forall(h =>
      val bx = fleet.boxes.get(h)
      (bx.linkDroppedWhileOwnerAway and bx.failoverHandled) implies (bx.queuedSuiteJobs == 0)
    )

  // BXF-011: Speed claims compare two runs on the same box only.
  val inv_BXF_011: bool =
    fleet.timingRecords.forall(t => t.measuredHost == t.baselineHost)

  // BXF-012: GPU probing uses `--query-gpu` + CUDA memory calls and NEVER calls `nvidia-smi conf-compute`.
  val inv_BXF_012: bool =
    fleet.gpuProbeCalls.forall(p => p.usedQueryGpu and not(p.usedConfCompute))

  // BXF-013: Every SIGKILL follows SIGTERM on the same pid after >= 30s and repeats the orphan eligibility check.
  val inv_BXF_013: bool =
    fleet.killEvents.forall(ev =>
      (ev.sigKind == SigKill) implies (
        ev.precededBySigtermSamePid
          and ev.elapsedAfterSigtermSec >= SIGKILL_GRACE_SEC
          and ev.recheckedEligible
          and ev.targetRole == HolderOrphanSampler
          and not(ev.parentWasAlive)
      )
    )

  // BXF-014: Transfers between `huginn` and `odin` pass through `muninn`.
  val inv_BXF_014: bool =
    fleet.acceptedTransfers.forall(tf =>
      ((tf.srcHost == HostOdin and tf.dstHost == HostHuginn)
        or (tf.srcHost == HostHuginn and tf.dstHost == HostOdin))
        implies tf.routedViaMuninn
    )

  // GPV-001: On a `cuda` build, GPU-scheduled app shards prove on `BackendCuda` and pass stock verification.
  val inv_GPV_001: bool =
    fleet.stageProofs.forall(sp =>
      (sp.cudaBuild and sp.gpuScheduled) implies (sp.appBackend == BackendCuda and sp.stockVerified)
    )

  // GPV-002: GPU and CPU shard provers produce byte-equal public values on the same program shard.
  val inv_GPV_002: bool =
    fleet.stageProofs.forall(s1 =>
      fleet.stageProofs.forall(s2 =>
        (s1.progId == s2.progId and s1.shardIdx == s2.shardIdx)
          implies (s1.publicValuesDigest == s2.publicValuesDigest)
      )
    )

  // GPV-003: GPU `L_Fr` proving (`MORI_GPU` enabled) produces byte-identical proof bytes to CPU (`MORI_GPU=off`).
  val inv_GPV_003: bool =
    fleet.stageProofs.forall(s1 =>
      fleet.stageProofs.forall(s2 =>
        (s1.tapeId == s2.tapeId) implies (s1.lfrProofBytesDigest == s2.lfrProofBytesDigest)
      )
    )

  // GPV-004: On a `cuda` build, N1..N3 prove on `BackendCuda` iff `T7_NODE_GPU` is not `0`/`off`.
  val inv_GPV_004: bool =
    fleet.stageProofs.forall(sp =>
      sp.cudaBuild implies (
        sp.nodeBackend == (if (sp.nodeGpuSwitchOff) BackendCpu else BackendCuda)
      )
    )

  // GPV-005: On a `cuda` build, `L` proves on `BackendCuda` iff `T14_L_GPU` is not `off`, and `L` verifies.
  val inv_GPV_005: bool =
    fleet.stageProofs.forall(sp =>
      sp.cudaBuild implies (
        sp.lBackend == (if (sp.lGpuSwitchOff) BackendCpu else BackendCuda)
          and sp.lStockVerified
      )
    )

  // GPV-006: A GPU device fault stops the run, names shard + device, and writes no partial proof under a final name.
  val inv_GPV_006: bool =
    fleet.gpuFaultRecords.forall(f =>
      f.runStopped and f.namedShardAndDevice and not(f.finalProofWritten)
    )

  // GPV-007: On a `cuda` build, GPU-scheduled leaf-layer proofs run on `BackendCuda`.
  val inv_GPV_007: bool =
    fleet.stageProofs.forall(sp =>
      (sp.cudaBuild and sp.gpuScheduled) implies (sp.leafLayerBackend == BackendCuda)
    )

  // PDM-001: While `wrapd serve` runs, all 6 wrap items stay resident with 0 reloads across calls.
  val inv_PDM_001: bool =
    fleet.daemon.running implies (
      fleet.daemon.residentItemCount == WRAPD_RESIDENT_ITEMS
        and fleet.daemon.keyReloadsAfterStart == 0
    )

  // PDM-002 (and PRD-001): Running daemon creates Unix socket mode 0600 under `$XDG_RUNTIME_DIR` for same user only.
  val inv_PDM_002: bool =
    fleet.daemon.running implies (
      fleet.daemon.socketUnderXdgRuntime
        and fleet.daemon.socketMode == SOCKET_MODE_OCTAL
        and fleet.daemon.localSameUserOnly
        and fleet.daemon.otherUserServedCount == 0
    )

  // PDM-003: Unknown operation (`dump`) returns an error and leaves the daemon running.
  val inv_PDM_003: bool =
    (lastOp == "daemonHandleUnknownOpDump")
      implies (fleet.daemon.running and fleet.lastRefusal == FleetRefusalUnknownDaemonOp)

  // PDM-004: While STARK worker runs in serve mode, STARK, `L`, and `L_Fr` keys stay resident on device.
  val inv_PDM_004: bool =
    fleet.starkWorker.serveRunning implies (
      fleet.starkWorker.starkKeysResident
        and fleet.starkWorker.lKeyResident
        and fleet.starkWorker.lfrKeyResident
        and fleet.starkWorker.keyReloadsAfterStart == 0
    )

  // PDM-005 (and PRD-002): Each served `prove`/`call` draws fresh one-use local CSPRNG blinders; distinct calls differ.
  val inv_PDM_005: bool =
    fleet.servedWrapProofs.forall(p1 =>
      p1.blinderFromLocalCsprng
        and not(p1.helperBlinderUsed)
        and p1.verified
        and fleet.servedWrapProofs.forall(p2 =>
          (p1.reqSeq != p2.reqSeq) implies (
            p1.blinderId != p2.blinderId
              and p1.wrapProofBytesDigest != p2.wrapProofBytesDigest
          )
        )
    )

  // PDM-006: Daemon serves ONLY requests matching its loaded key manifest `sha256`.
  val inv_PDM_006: bool =
    fleet.servedWrapProofs.forall(p => p.reqManifestSha == p.daemonManifestSha)

  // PDM-007 (and PRD-004): Every one-leaf call arriving at the daemon is served on Path C with 0 internal node proofs.
  val inv_PDM_007: bool =
    fleet.servedWrapProofs.forall(p =>
      (p.leafCount == 1) implies (p.usedPathC and p.internalNodeProofsMade == 0)
    )

  // PDM-008: Prover GUI `stats` replies contain zero witness/secret words.
  val inv_PDM_008: bool =
    fleet.statsReplies.forall(sr => sr.leakedSecretWords == 0 and sr.shardCount > 0)

  // PDM-009: No blinder state crosses requests (`priorBlinderCarryover == 0`).
  val inv_PDM_009: bool =
    fleet.servedWrapProofs.forall(p => p.priorBlinderCarryover == 0)

  // LPC-005: Post-rekey warm/cold timing and peak memory on huginn/odin record load and compare same box only.
  val inv_LPC_005: bool =
    fleet.timingRecords.forall(t =>
      t.keysSha == T25_KEYS_SHA
        and t.warmMs > 0
        and t.coldMs >= t.warmMs
        and t.peakMemGb > 0
        and t.hostLoadRecorded
        and t.measuredHost == t.baselineHost
    )

  // LPC-007: Scale-up demo proves 1x, 2x, 4x, 8x (> 5 rules/leaf) under one keyset, including muninn+huginn join.
  val inv_LPC_007: bool =
    (fleet.scaleDemoRuns.size() > 0) implies (
      fleet.scaleDemoRuns.size() == 4
        and fleet.scaleDemoRuns.forall(r1 =>
          r1.rulesPerLeaf > 5
            and fleet.scaleDemoRuns.forall(r2 => r1.keysSha == r2.keysSha)
        )
    )

  // Adopted PRD-001..004 trust-domain & one-use-blinder invariant
  val inv_PRD_001_004: bool = and {
    inv_PDM_002,
    inv_PDM_005,
    inv_PDM_007,
    fleet.acceptedTransfers.forall(tf =>
      tf.isStark implies (
        fleet.ownedRoster.contains(tf.srcHost) and fleet.ownedRoster.contains(tf.dstHost)
      )
    ),
  }

  // Master safety conjunction
  val inv: bool = and {
    inv_BXF_001,
    inv_BXF_002,
    inv_BXF_003,
    inv_BXF_004,
    inv_BXF_005,
    inv_BXF_006,
    inv_BXF_007,
    inv_BXF_008,
    inv_BXF_009,
    inv_BXF_010,
    inv_BXF_011,
    inv_BXF_012,
    inv_BXF_013,
    inv_BXF_014,
    inv_GPV_001,
    inv_GPV_002,
    inv_GPV_003,
    inv_GPV_004,
    inv_GPV_005,
    inv_GPV_006,
    inv_GPV_007,
    inv_PDM_001,
    inv_PDM_002,
    inv_PDM_003,
    inv_PDM_004,
    inv_PDM_005,
    inv_PDM_006,
    inv_PDM_007,
    inv_PDM_008,
    inv_PDM_009,
    inv_LPC_005,
    inv_LPC_007,
    inv_PRD_001_004,
  }

  // =========================================================================
  // 8. Non-Vacuity Reachability Witnesses (Negated Goals: MUST Exit 1 in Main)
  // =========================================================================

  val reach_fourth_box_and_exclusive_slot_used: bool = not(
    fleet.admittedJobs.exists(j1 => j1.targetHost == HostFourthOwned)
      and fleet.admittedJobs.exists(j2 => j2.isExclusiveSlot)
  )

  val reach_orphan_sigterm_and_sigkill_chain: bool = not(
    fleet.killEvents.exists(e => e.sigKind == SigKill)
  )

  val reach_spoke_transfer_and_drop_failover: bool = not(
    fleet.acceptedTransfers.size() > 0
      and fleet.boxes.get(HostOdin).failoverHandled
  )

  val reach_gpu_cpu_stage_proofs_and_fault: bool = not(
    fleet.stageProofs.exists(s1 => s1.gpuScheduled)
      and fleet.stageProofs.exists(s2 => not(s2.gpuScheduled))
      and fleet.gpuFaultRecords.size() > 0
  )

  val reach_daemon_two_calls_and_path_c: bool = not(
    fleet.servedWrapProofs.size() >= 2
      and fleet.servedWrapProofs.exists(p => p.usedPathC)
  )

  val reach_timing_and_scale_demo_recorded: bool = not(
    fleet.timingRecords.size() >= 2 and fleet.scaleDemoRuns.size() == 4
  )

  val reach_refusals_exercised: bool = not(
    match fleet.lastRefusal {
      | FleetRefusalUnownedHost(_)          => true
      | FleetRefusalDirectSpokeTransfer(_)  => true
      | FleetRefusalDaemonManifestMismatch(_) => true
      | _                                   => false
    }
  )
}

// ===========================================================================
// 9. Concrete Instances: Main, 2 Edge Instances, and 29 Guard/Binding Mutants
// ===========================================================================

module box_fleet_main {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3,
    HEAVY_BUDGET_GB = 64,
    ORPHAN_STALL_MIN = 10,
    SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30,
    WRAPD_RESIDENT_ITEMS = 6,
    SOCKET_MODE_OCTAL = 600,
    MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 0
  ).*
}

// Edge Instance 1: Minimum legal shared slot count (1) and minimum >5 leaf rules (6)
module box_fleet_edge_min {
  import box_fleet(
    HEAVY_SLOTS_MAX = 1,
    HEAVY_BUDGET_GB = 32,
    ORPHAN_STALL_MIN = 10,
    SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30,
    WRAPD_RESIDENT_ITEMS = 6,
    SOCKET_MODE_OCTAL = 600,
    MIN_REKEY_LEAF_RULES = 6,
    MUTANT_ID = 0
  ).*
}

// Edge Instance 2: Exact boundary limits (`HEAVY_BUDGET_GB = 64`, `SIGKILL_GRACE_SEC = 35` exact at trapped wait)
module box_fleet_edge_at_limit {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3,
    HEAVY_BUDGET_GB = 64,
    ORPHAN_STALL_MIN = 10,
    SIGKILL_GRACE_SEC = 35,
    DURABLE_RUN_THRESHOLD_MIN = 30,
    WRAPD_RESIDENT_ITEMS = 6,
    SOCKET_MODE_OCTAL = 600,
    MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 0
  ).*
}

// --- 29 Guard / Binding Mutant Instances (`box_fleet_m_<name>`) ---

module box_fleet_m_bxf001_admit_unowned_host {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 1
  ).*
}

module box_fleet_m_bxf002_bypass_heavy_slot {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 2
  ).*
}

module box_fleet_m_bxf003_admit_shared_over_budget {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 3
  ).*
}

module box_fleet_m_bxf004_gpu_job_without_lock {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 4
  ).*
}

module box_fleet_m_bxf005_sigterm_without_log_or_early {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 5
  ).*
}

module box_fleet_m_bxf006_kill_slow_prover {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 6
  ).*
}

module box_fleet_m_bxf007_accept_bad_digest_transfer {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 7
  ).*
}

module box_fleet_m_bxf009_long_run_not_durable {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 8
  ).*
}

module box_fleet_m_bxf010_leave_dropped_queue_stranded {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 9
  ).*
}

module box_fleet_m_bxf011_cross_box_speed_claim {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 10
  ).*
}

module box_fleet_m_bxf012_call_conf_compute {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 11
  ).*
}

module box_fleet_m_bxf013_early_or_unrechecked_sigkill {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 12
  ).*
}

module box_fleet_m_bxf014_allow_direct_odin_huginn {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 13
  ).*
}

module box_fleet_m_gpv001_cuda_app_on_cpu {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 14
  ).*
}

module box_fleet_m_gpv002_gpu_cpu_pubvals_differ {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 15
  ).*
}

module box_fleet_m_gpv003_lfr_gpu_differs_from_cpu {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 16
  ).*
}

module box_fleet_m_gpv004_ignore_node_gpu_switch {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 17
  ).*
}

module box_fleet_m_gpv006_write_partial_proof_on_fault {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 18
  ).*
}

module box_fleet_m_gpv007_leaf_layer_stays_cpu {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 19
  ).*
}

module box_fleet_m_pdm001_reload_srs_on_second_call {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 20
  ).*
}

module box_fleet_m_pdm002_world_readable_socket {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 21
  ).*
}

module box_fleet_m_pdm003_crash_on_unknown_op {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 22
  ).*
}

module box_fleet_m_pdm005_reuse_static_blinder {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 23
  ).*
}

module box_fleet_m_pdm006_serve_mismatched_manifest {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 24
  ).*
}

module box_fleet_m_pdm007_one_leaf_internal_nodes {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 25
  ).*
}

module box_fleet_m_pdm008_stats_leaks_witness {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 26
  ).*
}

module box_fleet_m_pdm009_carryover_prior_blinder {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 27
  ).*
}

module box_fleet_m_lpc007_scaleup_changes_keys {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 28
  ).*
}

module box_fleet_m_prd003_export_stark_outside_roster {
  import box_fleet(
    HEAVY_SLOTS_MAX = 3, HEAVY_BUDGET_GB = 64, ORPHAN_STALL_MIN = 10, SIGKILL_GRACE_SEC = 30,
    DURABLE_RUN_THRESHOLD_MIN = 30, WRAPD_RESIDENT_ITEMS = 6, SOCKET_MODE_OCTAL = 600, MIN_REKEY_LEAF_RULES = 16,
    MUTANT_ID = 29
  ).*
}
