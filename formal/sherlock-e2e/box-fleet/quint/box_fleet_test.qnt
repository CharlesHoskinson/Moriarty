// Requirement-level `.expect(...)` scenario tests for Sherlock E2E Box Fleet
// Lane: e2e-box-fleet

module box_fleet_test {
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
  ).* from "./box_fleet"

  // Constant consistency check across spec bounds
  run bf_ConsistencyConstantsTest =
    init.expect(consistency_constants and inv)

  // BXF-001: Rented host `gpu-rental-1` is refused; adding a 4th owned box admits jobs on it.
  run bf_Bxf001RosterRefusesRentalAndAdmitsFourthBoxTest =
    init
      .expect(not(canAdmitJob(fleet, HostRentalExternal, false, 16, 0)))
      .then(refuseUnownedHostJob(HostRentalExternal))
      .expect(fleet.lastRefusal == FleetRefusalUnownedHost(HostRentalExternal))
      .expect(not(canAdmitJob(fleet, HostFourthOwned, false, 16, 0)))
      .then(addFourthOwnedBoxToRoster)
      .then(admitAndStartBoxJob(HostFourthOwned, 16, 15, false))
      .expect(all {
        fleet.admittedJobs.exists(j => j.targetHost == HostFourthOwned),
        inv_BXF_001,
      })

  // BXF-002 & BXF-003: 3 shared jobs fill muninn heavy-slots (4th waits); 70 GB shared job is refused and runs `--exclusive`.
  run bf_Bxf002And003HeavySlotThreeSharedWaitAnd70GbExclusiveTest =
    init
      .then(admitAndStartBoxJob(HostMuninn, 32, 20, false))
      .then(admitAndStartBoxJob(HostMuninn, 32, 20, false))
      .then(admitAndStartBoxJob(HostMuninn, 32, 20, false))
      .expect(all {
        fleet.muninnSharedRunning == 3,
        not(canAdmitJob(fleet, HostMuninn, true, 32, 0)),
        inv_BXF_002,
      })
      .then(enqueueMuninnSlotWaiter)
      .expect(fleet.muninnSlotWaiters == 1)
      .then(releaseMuninnSlot)
      .then(releaseMuninnSlot)
      .then(releaseMuninnSlot)
      .then(releaseMuninnSlot)
      .expect(all {
        fleet.muninnSharedRunning == 0,
        fleet.muninnSlotWaiters == 0,
        not(canAdmitJob(fleet, HostMuninn, true, 70, 0)),
      })
      .then(refuseSharedOverBudgetOnMuninn(70))
      .expect(fleet.lastRefusal == FleetRefusalSharedOverBudget(70))
      .then(admitAndStartBoxJob(HostMuninn, 70, 45, true))
      .expect(all {
        fleet.muninnExclusiveRunning,
        inv_BXF_002,
        inv_BXF_003,
      })

  // BXF-004: Two GPU jobs on huginn -> first holds `gpu.lock`, second waits until first finishes.
  run bf_Bxf004GpuFlockMutualExclusionAndWaitTest =
    init
      .then(startOrQueueGpuJobOnBox(HostHuginn))
      .expect(all {
        fleet.boxes.get(HostHuginn).gpuLockHeld,
        fleet.boxes.get(HostHuginn).activeGpuProvingJobs == 1,
        fleet.boxes.get(HostHuginn).gpuWaitQueue == 0,
        inv_BXF_004,
      })
      .then(startOrQueueGpuJobOnBox(HostHuginn))
      .expect(all {
        fleet.boxes.get(HostHuginn).activeGpuProvingJobs == 1,
        fleet.boxes.get(HostHuginn).gpuWaitQueue == 1,
        inv_BXF_004,
      })
      .then(finishGpuJobReleaseLock(HostHuginn))
      .expect(all {
        fleet.boxes.get(HostHuginn).activeGpuProvingJobs == 1,
        fleet.boxes.get(HostHuginn).gpuWaitQueue == 0,
        fleet.boxes.get(HostHuginn).gpuLockHeld,
        inv_BXF_004,
      })
      .then(finishGpuJobReleaseLock(HostHuginn))
      .expect(not(fleet.boxes.get(HostHuginn).gpuLockHeld) and inv_BXF_004)

  // BXF-005, BXF-006 & BXF-013: Slow prover (40 min) is left running; dead-parent orphan sampler (>10 min)
  // is logged then SIGTERMed, and if trapping SIGTERM for >=30s is rechecked and SIGKILLed.
  run bf_Bxf005And006And013OrphanReapAndSlowProverProtectedTest =
    init
      .then(stageLockHolderScenario(HostOdin, HolderProver, 40))
      .expect(not(isEligibleOrphanForSigterm(fleet.boxes.get(HostOdin), 0)))
      .then(observeSlowProverLeaveRunning(HostOdin))
      .expect(all {
        fleet.boxes.get(HostOdin).gpuLockHeld,
        fleet.killEvents.size() == 0,
        inv_BXF_006,
      })
      .then(stageLockHolderScenario(HostHuginn, HolderOrphanSampler, 12))
      .then(reapOrphanLockSigterm(HostHuginn, true))
      .expect(all {
        fleet.boxes.get(HostHuginn).sigtermSent,
        fleet.boxes.get(HostHuginn).sigtermLoggedOnBus,
        fleet.killEvents.exists(e => e.sigKind == SigTerm and e.loggedOnBusBeforeSignal),
        inv_BXF_005,
      })
      .then(escalateOrphanLockSigkill(HostHuginn))
      .expect(all {
        not(fleet.boxes.get(HostHuginn).gpuLockHeld),
        fleet.killEvents.exists(e => e.sigKind == SigKill and e.elapsedAfterSigtermSec >= 30 and e.recheckedEligible),
        inv_BXF_005,
        inv_BXF_006,
        inv_BXF_013,
      })

  // BXF-007, BXF-014 & PRD-003: Truncated proof fails sha256 check; direct odin->huginn refused;
  // export to rental host refused; verified transfer via muninn succeeds.
  run bf_Bxf007And014AndPrd003TransferDigestAndMuninnHubTest =
    init
      .expect(not(canAcceptFileTransfer(fleet, HostOdin, HostMuninn, true, 555, 550, true, 0)))
      .then(refuseDigestMismatchTransfer(HostOdin, HostMuninn, 555, 550))
      .expect(fleet.lastRefusal == FleetRefusalDigestMismatch((HostOdin, HostMuninn)))
      .expect(not(canAcceptFileTransfer(fleet, HostOdin, HostHuginn, false, 555, 555, true, 0)))
      .then(refuseDirectSpokeTransfer(HostOdin, HostHuginn))
      .expect(fleet.lastRefusal == FleetRefusalDirectSpokeTransfer((HostOdin, HostHuginn)))
      .expect(not(canAcceptFileTransfer(fleet, HostOdin, HostRentalExternal, true, 555, 555, true, 0)))
      .then(refuseStarkExportOutsideRoster(HostRentalExternal))
      .expect(fleet.lastRefusal == FleetRefusalStarkExportOutsideTrustDomain(HostRentalExternal))
      .then(transferFileVerified(HostOdin, HostHuginn, true, 555, 555, true))
      .expect(all {
        fleet.acceptedTransfers.size() == 1,
        inv_BXF_007,
        inv_BXF_014,
        inv_PRD_001_004,
      })

  // BXF-008, BXF-009 & BXF-010: Bus relay logs to `all.log`; >30 min run uses durable systemd unit across reboot;
  // dropped link on odin hands queued suite calls to huginn.
  run bf_Bxf008To010RelayDurableRebootAndLinkDropFailoverTest =
    init
      .then(postAndRelayBusMessage(HostHuginn, HostOdin))
      .expect(fleet.relayedMessages.size() == 1 and inv_BXF_008)
      .then(admitAndStartBoxJob(HostMuninn, 64, 45, true))
      .expect(fleet.admittedJobs.exists(j => j.startedAsSystemdUserUnit and j.persistentLogEnabled) and inv_BXF_009)
      .then(rebootAndResumeDurableUnits)
      .expect(fleet.admittedJobs.exists(j => j.resumedAfterReboot) and inv_BXF_009)
      .then(dropBoxLinkAndHandoverQueue(HostOdin, HostHuginn))
      .expect(all {
        fleet.boxes.get(HostOdin).queuedSuiteJobs == 0,
        fleet.boxes.get(HostHuginn).queuedSuiteJobs == 10,
        inv_BXF_008,
        inv_BXF_010,
      })

  // BXF-011, BXF-012, LPC-005 & LPC-007: Safe `--query-gpu` probe (no `conf-compute`), same-box timing claims,
  // and 1x/2x/4x/8x elastic scale-up demo under one keyset.
  run bf_Bxf011And012AndLpc005And007TimingProbeAndScaleDemoTest =
    init
      .then(probeGpuWithoutConfCompute(HostMuninn))
      .expect(fleet.gpuProbeCalls.size() == 1 and inv_BXF_012)
      .then(recordBoxTimingClaim(HostHuginn))
      .then(recordBoxTimingClaim(HostOdin))
      .expect(all {
        fleet.timingRecords.size() == 2,
        inv_BXF_011,
        inv_LPC_005,
      })
      .then(runElasticScaleDemo)
      .expect(all {
        fleet.scaleDemoRuns.size() == 4,
        inv_LPC_007,
      })

  // GPV-001 .. GPV-007: GPU/CPU stage backends, switches (`T7_NODE_GPU`, `T14_L_GPU`, `MORI_GPU`),
  // public-value & `L_Fr` byte equality, and device OOM stop without partial proof.
  run bf_Gpv001To007GpuStagesCpuEqualityAndFaultStopTest =
    init
      .then(proveStagePipeline(HostHuginn, true, false, false, false))
      .then(proveStagePipeline(HostOdin, false, true, true, true))
      .then(faultGpuShardOutOfMemory(HostHuginn, 0))
      .expect(all {
        fleet.stageProofs.exists(s =>
          s.gpuScheduled
            and s.appBackend == BackendCuda
            and s.leafLayerBackend == BackendCuda
            and s.nodeBackend == BackendCuda
            and s.lBackend == BackendCuda
        ),
        fleet.stageProofs.exists(s =>
          not(s.gpuScheduled)
            and s.nodeGpuSwitchOff
            and s.nodeBackend == BackendCpu
            and s.lBackend == BackendCpu
        ),
        fleet.gpuFaultRecords.size() == 1,
        inv_GPV_001,
        inv_GPV_002,
        inv_GPV_003,
        inv_GPV_004,
        inv_GPV_005,
        inv_GPV_006,
        inv_GPV_007,
      })

  // PDM-001 .. PDM-009 & PRD-001..004: Resident `wrapd` + STARK worker, 0600 socket, `dump` error keeps serving,
  // manifest mismatch refused, 2 calls on statement 1 differ byte-for-byte with 0 reloads & 0 carryover,
  // 1-leaf call takes Path C, and GUI `stats` leaks 0 secret words.
  run bf_Pdm001To009AndPrdDaemonLifecycleTest =
    init
      .then(startDaemonsOnBox)
      .expect(all {
        fleet.daemon.running,
        fleet.daemon.socketMode == 600,
        fleet.daemon.residentItemCount == 6,
        fleet.starkWorker.serveRunning,
        not(canDaemonServeCall(fleet, T25_KEYS_SHA, false, false, false, 0)),
        not(canDaemonServeCall(fleet, T10M_KEYS_SHA, true, false, false, 0)),
        not(canDaemonServeCall(fleet, T25_KEYS_SHA, true, true, false, 0)),
      })
      .then(refuseUnauthorizedSocketPeer)
      .expect(fleet.lastRefusal == FleetRefusalSocketPermissionOrRemote)
      .then(daemonHandleUnknownOpDump)
      .expect(fleet.daemon.running and fleet.lastRefusal == FleetRefusalUnknownDaemonOp and inv_PDM_003)
      .then(daemonRefuseManifestMismatch(T10M_KEYS_SHA))
      .expect(fleet.lastRefusal == FleetRefusalDaemonManifestMismatch((T25_KEYS_SHA, T10M_KEYS_SHA)))
      .then(daemonRefuseBadBlinder)
      .expect(fleet.lastRefusal == FleetRefusalForeignOrReusedBlinder)
      .then(daemonServeCall(1, 1, T25_KEYS_SHA, true))
      .then(daemonServeCall(1, 3, T25_KEYS_SHA, true))
      .then(daemonServeStats)
      .expect(all {
        fleet.servedWrapProofs.size() == 2,
        fleet.daemon.keyReloadsAfterStart == 0,
        // Seeded determinism check for PDM-009: X after Y with carry=0 equals fresh X
        seededWrapProofDigest(1, 42, 0) == seededWrapProofDigest(1, 42, 0),
        inv_PDM_001,
        inv_PDM_002,
        inv_PDM_004,
        inv_PDM_005,
        inv_PDM_006,
        inv_PDM_007,
        inv_PDM_008,
        inv_PDM_009,
        inv_PRD_001_004,
        inv,
      })
}
