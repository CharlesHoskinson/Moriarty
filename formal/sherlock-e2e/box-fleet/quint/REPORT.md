# Sherlock E2E Box Fleet — Independent Quint Formal Specification Report

- **Lane**: `e2e-box-fleet` (Gemini 4 Carbon)
- **Bounded Context**: Box Fleet (`sherlock-e2e-box-fleet`, `sherlock-e2e-gpu-proving`, `sherlock-e2e-prover-daemon`, completion `LPC-005`, `LPC-007`, adopted `PRD-001..004`)
- **Quint Artifacts**:
  - Model: `box_fleet.qnt` (`box_fleet`, `box_fleet_main`, `box_fleet_edge_min`, `box_fleet_edge_at_limit`, and 29 guard/binding mutants `box_fleet_m_*`)
  - Requirement tests: `box_fleet_test.qnt` (`10 passing` multi-step scenario `run` tests)
  - ITF replay traces: `../traces/box_fleet_main/t_0.itf.json` .. `t_19.itf.json` (20 traces, 15 steps max, seed `0x20261115`, covering all 32 `lastOp` values)

---

## 1. Requirement Coverage Table (ID Table)

Every own EARS ID (`BXF-001` .. `BXF-014`, `GPV-001` .. `GPV-007`, `PDM-001` .. `PDM-009`), completion EARS ID (`LPC-005`, `LPC-007`), and cited adopted EARS ID (`PRD-001` .. `PRD-004`) is accounted for below.

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
|---|---|---|---|
| `BXF-001` | `inv_BXF_001`; actions `addFourthOwnedBoxToRoster`, `refuseUnownedHostJob`, `admitAndStartBoxJob`; test `bf_Bxf001RosterRefusesRentalAndAdmitsFourthBoxTest`; mutant `box_fleet_m_bxf001_admit_unowned_host` | `MODELED` | Refuses hosts outside the user's `ownedRoster` (`HostRentalExternal`) and admits a newly added 4th owned box (`HostFourthOwned`). See Finding `Q-box-fleet-5`. |
| `BXF-002` | `inv_BXF_002`, `consistency_constants`; actions `admitAndStartBoxJob`, `enqueueMuninnSlotWaiter`, `releaseMuninnSlot`; test `bf_Bxf002And003HeavySlotThreeSharedWaitAnd70GbExclusiveTest`; mutant `box_fleet_m_bxf002_bypass_heavy_slot` | `MODELED` / `CONSTANT-CHECKED` | Requires all heavy jobs on `muninn` to run through `heavy-slot`, bounding concurrent shared jobs by `HEAVY_SLOTS_MAX` (`3`) and mutually excluding `--exclusive` jobs. See Finding `Q-box-fleet-4`. |
| `BXF-003` | `inv_BXF_003`, `consistency_constants`; actions `refuseSharedOverBudgetOnMuninn`, `admitAndStartBoxJob`; test `bf_Bxf002And003HeavySlotThreeSharedWaitAnd70GbExclusiveTest`; mutant `box_fleet_m_bxf003_admit_shared_over_budget` | `MODELED` / `CONSTANT-CHECKED` | Refuses shared `heavy-slot` jobs above `HEAVY_BUDGET_GB` (`64`), allowing a `70 GB` keygen to run only under `--exclusive`. See Finding `Q-box-fleet-4`. |
| `BXF-004` | `inv_BXF_004`; actions `startOrQueueGpuJobOnBox`, `finishGpuJobReleaseLock`; test `bf_Bxf004GpuFlockMutualExclusionAndWaitTest`; mutant `box_fleet_m_bxf004_gpu_job_without_lock` | `MODELED` | Requires exclusive `flock` on `~/sherlock/gpu.lock` (`activeGpuProvingJobs == 1` and `gpuLockHeld == true`) while a GPU proving job runs, queueing a second job. See Finding `Q-box-fleet-3`. |
| `BXF-005` | `inv_BXF_005`, `consistency_constants`; actions `stageLockHolderScenario`, `reapOrphanLockSigterm`; test `bf_Bxf005And006And013OrphanReapAndSlowProverProtectedTest`; mutant `box_fleet_m_bxf005_sigterm_without_log_or_early` | `MODELED` / `CONSTANT-CHECKED` | Sends `SIGTERM` only after logging on the box bus and only when `lockHolderRole == HolderOrphanSampler`, `not(parentWasAlive)`, and `stallMin > ORPHAN_STALL_MIN` (`10`). |
| `BXF-006` | `inv_BXF_006`; action `observeSlowProverLeaveRunning`; test `bf_Bxf005And006And013OrphanReapAndSlowProverProtectedTest`; mutant `box_fleet_m_bxf006_kill_slow_prover` | `MODELED` | Forbids any lock-reap signal (`SIGTERM` or `SIGKILL`) from targeting `HolderProver` or `HolderWrapd` even at 40+ minutes. |
| `BXF-007` | `inv_BXF_007`; actions `transferFileVerified`, `refuseDigestMismatchTransfer`; test `bf_Bxf007And014AndPrd003TransferDigestAndMuninnHubTest`; mutant `box_fleet_m_bxf007_accept_bad_digest_transfer` | `MODELED` | Verifies `actualSha256 == busMsgSha256` on every cross-box result file transfer and refuses truncated/corrupted proofs before wrap starts. See Finding `Q-box-fleet-2`. |
| `BXF-008` | `inv_BXF_008`; action `postAndRelayBusMessage`; test `bf_Bxf008To010RelayDurableRebootAndLinkDropFailoverTest` | `MODELED` | Appends every relayed outbox message to `~/.cache/box-bus/all.log` (`allLogMsgIds`) and delivers to recipient. See Finding `Q-box-fleet-5`. |
| `BXF-009` | `inv_BXF_009`, `consistency_constants`; actions `admitAndStartBoxJob`, `rebootAndResumeDurableUnits`; test `bf_Bxf008To010RelayDurableRebootAndLinkDropFailoverTest`; mutant `box_fleet_m_bxf009_long_run_not_durable` | `MODELED` / `CONSTANT-CHECKED` | Starts every job with `plannedMin > DURABLE_RUN_THRESHOLD_MIN` (`30`) as a `systemd-run --user` unit with persistent logs resumable across reboot. |
| `BXF-010` | `inv_BXF_010`; action `dropBoxLinkAndHandoverQueue`; test `bf_Bxf008To010RelayDurableRebootAndLinkDropFailoverTest`; mutant `box_fleet_m_bxf010_leave_dropped_queue_stranded` | `MODELED` | Moves queued suite jobs off a dropped box (`odin`) to another live box (`huginn`) and logs the move on the bus. See Finding `Q-box-fleet-5`. |
| `BXF-011` | `inv_BXF_011`; action `recordBoxTimingClaim`; test `bf_Bxf011And012AndLpc005And007TimingProbeAndScaleDemoTest`; mutant `box_fleet_m_bxf011_cross_box_speed_claim` | `MODELED` | Enforces `measuredHost == baselineHost` on every speed claim. See Finding `Q-box-fleet-5`. |
| `BXF-012` | `inv_BXF_012`; action `probeGpuWithoutConfCompute`; test `bf_Bxf011And012AndLpc005And007TimingProbeAndScaleDemoTest`; mutant `box_fleet_m_bxf012_call_conf_compute` | `MODELED` | Requires GPU probes to use `nvidia-smi --query-gpu` + CUDA memory calls and forbids `nvidia-smi conf-compute` (`usedConfCompute == false`). |
| `BXF-013` | `inv_BXF_013`, `consistency_constants`; action `escalateOrphanLockSigkill`; test `bf_Bxf005And006And013OrphanReapAndSlowProverProtectedTest`; mutant `box_fleet_m_bxf013_early_or_unrechecked_sigkill` | `MODELED` / `CONSTANT-CHECKED` | Requires every `SIGKILL` to follow `SIGTERM` on the same `pid` after `>= SIGKILL_GRACE_SEC` (`30` s) and to re-verify orphan role and dead parent. |
| `BXF-014` | `inv_BXF_014`; actions `transferFileVerified`, `refuseDirectSpokeTransfer`; test `bf_Bxf007And014AndPrd003TransferDigestAndMuninnHubTest`; mutant `box_fleet_m_bxf014_allow_direct_odin_huginn` | `MODELED` | Refuses direct `odin <-> huginn` file copies unless `routedViaMuninn == true`. See Finding `Q-box-fleet-5`. |
| `GPV-001` | `inv_GPV_001`; action `proveStagePipeline`; test `bf_Gpv001To007GpuStagesCpuEqualityAndFaultStopTest`; mutant `box_fleet_m_gpv001_cuda_app_on_cpu` | `MODELED` | Requires GPU-scheduled app shards on a `cuda` build to run on `BackendCuda` and pass stock verification. See Finding `Q-box-fleet-6`. |
| `GPV-002` | `inv_GPV_002`; action `proveStagePipeline`; test `bf_Gpv001To007GpuStagesCpuEqualityAndFaultStopTest`; mutant `box_fleet_m_gpv002_gpu_cpu_pubvals_differ` | `MODELED` | Enforces byte-equal `publicValuesDigest` across GPU (`BackendCuda`) and CPU (`BackendCpu`) proofs of the same shard. |
| `GPV-003` | `inv_GPV_003`; action `proveStagePipeline`; test `bf_Gpv001To007GpuStagesCpuEqualityAndFaultStopTest`; mutant `box_fleet_m_gpv003_lfr_gpu_differs_from_cpu` | `MODELED` | Enforces byte-identical `lfrProofBytesDigest` between GPU (`MORI_GPU` enabled) and CPU (`MORI_GPU=off`) on the same tape. |
| `GPV-004` | `inv_GPV_004`; action `proveStagePipeline`; test `bf_Gpv001To007GpuStagesCpuEqualityAndFaultStopTest`; mutant `box_fleet_m_gpv004_ignore_node_gpu_switch` | `MODELED` | Selects `BackendCuda` for `N1..N3` on a `cuda` build iff `T7_NODE_GPU` is not `0`/`off`, else `BackendCpu`. |
| `GPV-005` | `inv_GPV_005`; action `proveStagePipeline`; test `bf_Gpv001To007GpuStagesCpuEqualityAndFaultStopTest` | `MODELED` | Selects `BackendCuda` for `L` on a `cuda` build iff `T14_L_GPU` is not `off`, with `lStockVerified == true`. |
| `GPV-006` | `inv_GPV_006`; action `faultGpuShardOutOfMemory`; test `bf_Gpv001To007GpuStagesCpuEqualityAndFaultStopTest`; mutant `box_fleet_m_gpv006_write_partial_proof_on_fault` | `MODELED` | Stops the run on a GPU device error, names `(faultHost, shardIdx)`, and writes no partial proof under a final name (`not(finalProofWritten)`). |
| `GPV-007` | `inv_GPV_007`; action `proveStagePipeline`; test `bf_Gpv001To007GpuStagesCpuEqualityAndFaultStopTest`; mutant `box_fleet_m_gpv007_leaf_layer_stays_cpu` | `MODELED` | Requires GPU-scheduled leaf-layer proofs on a `cuda` build to run on `BackendCuda`. See Finding `Q-box-fleet-6`. |
| `PDM-001` | `inv_PDM_001`, `consistency_constants`; actions `startDaemonsOnBox`, `daemonServeCall`; test `bf_Pdm001To009AndPrdDaemonLifecycleTest`; mutant `box_fleet_m_pdm001_reload_srs_on_second_call` | `MODELED` / `CONSTANT-CHECKED` | Keeps all `WRAPD_RESIDENT_ITEMS = 6` items resident in `wrapd serve` with `keyReloadsAfterStart == 0` across calls. See Finding `Q-box-fleet-3`. |
| `PDM-002` | `inv_PDM_002`, `consistency_constants`; actions `startDaemonsOnBox`, `refuseUnauthorizedSocketPeer`; test `bf_Pdm001To009AndPrdDaemonLifecycleTest`; mutant `box_fleet_m_pdm002_world_readable_socket` | `MODELED` / `CONSTANT-CHECKED` | Creates socket under `$XDG_RUNTIME_DIR` with `SOCKET_MODE_OCTAL = 600` and refuses non-owner peers. |
| `PDM-003` | `inv_PDM_003`; action `daemonHandleUnknownOpDump`; test `bf_Pdm001To009AndPrdDaemonLifecycleTest`; mutant `box_fleet_m_pdm003_crash_on_unknown_op` | `MODELED` | Answers unknown operation `dump` with `FleetRefusalUnknownDaemonOp` while keeping `daemon.running == true`. |
| `PDM-004` | `inv_PDM_004`; actions `startDaemonsOnBox`, `daemonServeCall`; test `bf_Pdm001To009AndPrdDaemonLifecycleTest` | `MODELED` | Keeps STARK keys, `L` key, and `L_Fr` key resident on device in STARK worker serve mode with `0` key reloads. See Finding `Q-box-fleet-3`. |
| `PDM-005` | `inv_PDM_005`; action `daemonServeCall`; test `bf_Pdm001To009AndPrdDaemonLifecycleTest`; mutant `box_fleet_m_pdm005_reuse_static_blinder` | `MODELED` | Draws fresh in-process CSPRNG blinders per `prove`/`call` request so two proofs of one statement differ byte-for-byte and both verify. See Finding `Q-box-fleet-1`. |
| `PDM-006` | `inv_PDM_006`; actions `daemonServeCall`, `daemonRefuseManifestMismatch`; test `bf_Pdm001To009AndPrdDaemonLifecycleTest`; mutant `box_fleet_m_pdm006_serve_mismatched_manifest` | `MODELED` | Refuses requests whose `reqManifestSha` differs from `daemon.loadedManifestSha`, naming both digests. |
| `PDM-007` | `inv_PDM_007`; action `daemonServeCall`; test `bf_Pdm001To009AndPrdDaemonLifecycleTest`; mutant `box_fleet_m_pdm007_one_leaf_internal_nodes` | `MODELED` | Serves every 1-leaf call (`leafCount == 1`) on Path C (`usedPathC == true`, `internalNodeProofsMade == 0`). See Finding `Q-box-fleet-3`. |
| `PDM-008` | `inv_PDM_008`; action `daemonServeStats`; test `bf_Pdm001To009AndPrdDaemonLifecycleTest`; mutant `box_fleet_m_pdm008_stats_leaks_witness` | `MODELED` | Serves GUI bridge `stats` with stage, shard count, and stage time while leaking `0` witness/secret words. See Finding `Q-box-fleet-3`. |
| `PDM-009` | `inv_PDM_009`; action `daemonServeCall`; test `bf_Pdm001To009AndPrdDaemonLifecycleTest`; mutant `box_fleet_m_pdm009_carryover_prior_blinder` | `MODELED` | Enforces `priorBlinderCarryover == 0` so proving `X` after `Y` matches fresh `X` under identical seed. |
| `LPC-005` | `inv_LPC_005`; action `recordBoxTimingClaim`; test `bf_Bxf011And012AndLpc005And007TimingProbeAndScaleDemoTest` | `MODELED` | Completion ID: measures post-rekey (`T25`) warm/cold times and peak RAM on `huginn` and `odin` with host load recorded and same-box baseline only. |
| `LPC-007` | `inv_LPC_007`, `consistency_constants`; action `runElasticScaleDemo`; test `bf_Bxf011And012AndLpc005And007TimingProbeAndScaleDemoTest`; mutant `box_fleet_m_lpc007_scaleup_changes_keys` | `MODELED` / `CONSTANT-CHECKED` | Completion ID: proves `1x, 2x, 4x, 8x` run lengths (`MIN_REKEY_LEAF_RULES > 5`) under identical keys, including `muninn` + `huginn` join. |
| `PRD-001..004` *(adopted)* | `inv_PRD_001_004` (plus `inv_PDM_002`, `inv_PDM_005`, `inv_PDM_007`); actions `refuseUnauthorizedSocketPeer`, `daemonRefuseBadBlinder`, `refuseStarkExportOutsideRoster`; test `bf_Pdm001To009AndPrdDaemonLifecycleTest`; mutant `box_fleet_m_prd003_export_stark_outside_roster` | `MODELED` | Adopted (`sherlock-prover-daemon`): local same-user socket (`PRD-001`), one-use local blinders & helper refusal (`PRD-002`), STARK artifacts kept inside owned-box trust domain (`PRD-003`), and Path C on 1-leaf calls (`PRD-004`). See Findings `Q-box-fleet-1`, `Q-box-fleet-2`. |

---

## 2. Findings (`Q-box-fleet-1` .. `Q-box-fleet-6`)

### `Q-box-fleet-1` — Normative contradiction between `PRD-002` / `PRD-004` (blinding-slot pool) and `PDM-005` / `PDM-009` (stateless per-proof CSPRNG draw with no pool)
- **IDs**: `PDM-005`, `PDM-009`, `PRD-002`, `PRD-004`
- **Quoted sentences**:
  - `PRD-002` (`sherlock-prover-daemon/spec.md:11`): *"The daemon SHALL hold a pool of one-use blinding slots with fresh CSPRNG blinders. Each slot SHALL be consumed by exactly one `wrap_prove`. A slot SHALL NOT be serialized off the machine. A slot made by a helper SHALL be refused."*
  - `PRD-004` (`sherlock-prover-daemon/spec.md:29`): *"The daemon and its slot pool SHALL be built with path C in build step 15."*
  - `PDM-005` (`sherlock-e2e-prover-daemon/spec.md:60, 71`): *"When the prover daemon serves a `prove` or `call` request, the prover daemon SHALL draw the blinders of that wrap proof from the OS CSPRNG inside the daemon process."* / Status: *"No blinding-slot pool exists in the code. D-040 keeps OD-13 and PRD-002; owner ruling 2026-11-15: the per-proof draw satisfies one-use slots, so no slot pool is built."*
- **Witness state**:
  - A daemon implementing `PDM-005` holds `slotPoolSize = 0` and draws fresh `OsRng` blinders inline on each `prove`/`call` request. This state satisfies `PDM-005` and `PDM-009`, yet violates `PRD-002`'s literal `SHALL hold a pool of one-use blinding slots` and leaves `PRD-002`'s two scenarios (*Reused slot* and *Helper-made slot*) without any slot-ID object to test against (`DDD.md:304`: *"'Blinding slot' is the PRD-002 term and names no code object today"*).
- **What would close it**:
  - Update `PRD-002` and `PRD-004` in `sherlock-prover-daemon/spec.md` to match owner ruling `D-040`: state that the daemon generates fresh one-use CSPRNG blinders in-process per `prove`/`call` request (never reusing blinders across calls, never exporting blinders, and never accepting caller- or helper-supplied blinders) rather than mandating a pre-allocated slot pool.

### `Q-box-fleet-2` — `PRD-003` Scenario ("only the wrap proof and the call vector leave the process") contradicts multi-process `PDM-001`/`PDM-004` and cross-box `BXF-007`/`BXF-014`/`LPC-007`
- **IDs**: `PRD-003`, `BXF-007`, `BXF-014`, `LPC-007`, `PDM-001`, `PDM-004`
- **Quoted sentences**:
  - `PRD-003` (`sherlock-prover-daemon/spec.md:22-26`): *"No STARK proof, prover checkpoint or shard plan of a private program SHALL leave the daemon's trust domain. The trust domain SHALL be the user's own devices and GPUs (OD-9)."* / Scenario (*Export request*): *"WHEN a client asks the daemon for its outputs THEN only the wrap proof and the call vector leave the process"*
  - `BXF-007` Scenario: *"GIVEN an `L_Fr` proof of odin that arrives short WHEN muninn checks the digest..."*
  - `BXF-014` Scenario: *"GIVEN a leaf proof on odin wanted on huginn ... the file goes odin to muninn to huginn with its sha256"*
  - `LPC-007` Scenario: *"WHEN leaves from muninn and huginn are joined THEN one N1 accepts them and `claim_pub` is byte-equal"*
- **Witness state**:
  - Moving a STARK leaf proof (`odin -> muninn -> huginn`, `BXF-014` / `LPC-007`), moving an `L_Fr` STARK proof (`odin -> muninn`, `BXF-007`), or even passing `L_Fr` from the STARK worker process (`PDM-004`) to the separate `wrapd serve` process (`PDM-001`) stays inside the user's **owned-box trust domain** (`BXF-001`), yet directly violates `PRD-003`'s scenario (*"only the wrap proof and the call vector leave the process"*).
- **What would close it**:
  - Amend `PRD-003`'s scenario from *"leave the process"* to *"leave the user's owned-box trust domain (`BXF-001` roster)"*, explicitly permitting authenticated intra-box IPC (`PDM-004 -> PDM-001`) and `sha256`-verified transfers between owned roster boxes (`BXF-007`, `BXF-014`, `LPC-007`).

### `Q-box-fleet-3` — "The prover daemon" conflates `wrapd` and the STARK worker across `PDM-001`, `PDM-004`, `PDM-007`, `PDM-008`, and `BXF-004` (`gpu.lock` deadlock hazard)
- **IDs**: `PDM-001`, `PDM-004`, `PDM-007`, `PDM-008`, `BXF-004`, `BXF-006`
- **Quoted sentences**:
  - `PDM-001`: *"While `wrapd serve` runs, the prover daemon SHALL keep the SRS, the relation, the proving key, the verifying key, the fixed-base tables and the CUDA context loaded."*
  - `PDM-004`: *"While the STARK worker runs in serve mode, the STARK worker SHALL keep the STARK keys, the `L` key and the `L_Fr` key loaded on the device."*
  - `PDM-007`: *"When a one-leaf call arrives at the prover daemon, the prover daemon SHALL serve the call on path C."*
  - `PDM-008`: *"When the prover GUI bridge sends a `stats` request, the prover daemon SHALL answer with the stage, the shard counts and the stage times of the running call and with no witness word."*
  - `BXF-004`: *"While a GPU proving job runs on a box, the box runner SHALL hold an exclusive `flock` on `~/sherlock/gpu.lock`."*
  - `BXF-006`: *"If the holder of `gpu.lock` is a prover or a `wrapd` process, then the box agent SHALL leave the holder running."*
- **Witness trace / state**:
  1. **Daemon identity split**: `PDM-001` identifies *"the prover daemon"* with `wrapd serve` (which wraps an already-proved `L_Fr` proof), yet `PDM-007` and `PDM-008` require *"the prover daemon"* to select Path C over 1-leaf calls and report STARK `node stage` and `shard counts` via `stats` — state owned by the separate STARK worker (`PDM-004`).
  2. **`gpu.lock` deadlock between resident `wrapd` and STARK worker**: Under `BXF-004` + `BXF-006`, if `wrapd serve` holds `~/sherlock/gpu.lock` while resident (`PDM-001`), no STARK GPU job can ever acquire `gpu.lock` on the same box; if instead the outer runner acquires exclusive `gpu.lock` for a full end-to-end call and then invokes `wrapd` (`call`/`prove`) which also acquires `gpu.lock` (`BXF-006`), the call self-deadlocks on `flock`!
- **What would close it**:
  - Define cleanly in `PDM-001..008` and `BXF-004` whether `wrapd` and the STARK worker are unified behind one coordinator socket (holding one per-call `gpu.lock` lease across leaf -> node/`Path C` -> `L` -> `L_Fr` -> `wrap` and aggregating `stats`), or specify exact `gpu.lock` handoff between the STARK worker and `wrapd` so neither starves nor self-deadlocks the other.

### `Q-box-fleet-4` — `BXF-002` and `BXF-003` omit the slot/budget numbers from EARS (`64 GB` vs `90 GB`) and allow 3 shared jobs to exceed `heavy.slice` `MemoryMax`
- **IDs**: `BXF-002`, `BXF-003`
- **Quoted sentences**:
  - `BXF-002`: *"When a prover, keygen or gate job starts on muninn, the job runner SHALL start the job through `heavy-slot`."*
  - `BXF-003`: *"If a shared job asks for more memory than the heavy-slot budget, then the `heavy-slot` tool SHALL refuse the job."*
  - `BXF-002` Status (`spec.md:34`): *"GAP: LOOP states 90G in one rule and 64G in another; the script is 64 ... `heavy-run` starts the job in `heavy.slice` (`MemoryHigh=64G`, `MemoryMax=72G`)."*
- **Witness state**:
  - (a) If `HEAVY_BUDGET_GB = 90` (per the unamended LOOP rule), the `70 GB` T25 keygen in `BXF-003`'s scenario (`70 <= 90`) is admitted as a **shared** job instead of refused!
  - (b) Even with `HEAVY_SLOTS_MAX = 3` and `HEAVY_BUDGET_GB = 64`, start 3 shared jobs on `muninn` each requesting `--mem 40G`. Each job individually satisfies `BXF-003` (`40 <= 64`) and `BXF-002` (`3 <= 3` slots), yet their concurrent demand is `3 * 40 = 120 GB > 72 GB` (`heavy.slice` `MemoryMax`), causing systemd OOM termination! In addition, `BXF-003` leaves `--exclusive` mutual exclusion and maximum host RAM cap unstated in normative EARS text.
- **What would close it**:
  - State `SLOTS = 3` and `BUDGET = 64 GB` normatively in `BXF-002`/`BXF-003`; require `heavy-slot` to track **aggregate** reserved memory across active shared slots (`sum(reqMemGb) <= HEAVY_BUDGET_GB`, queueing a job whose memory does not fit alongside currently running shared jobs); and state normatively that `--exclusive` drains/blocks all shared slots and caps `--mem` at `muninn`'s physical `MemAvailable`.

### `Q-box-fleet-5` — `BXF-010` unconditional queue failover violates `BXF-011` / `LPC-005` for box-pinned jobs, and `BXF-008` / `BXF-014` hardcode 3 host names against `BXF-001`'s dynamic roster
- **IDs**: `BXF-001`, `BXF-008`, `BXF-010`, `BXF-011`, `BXF-014`, `LPC-005`
- **Quoted sentences**:
  - `BXF-010`: *"If the link to a box drops while the owner is away, then the lead agent SHALL move the queued jobs of that box to another box."*
  - `BXF-011`: *"The timing runner SHALL state a speed gain from two runs on the same box."*
  - `LPC-005`: *"When the rekey completes, the timing runner SHALL measure warm and cold call time and peak memory on huginn and odin, each against its own baseline."*
  - `BXF-001` Scenario (*A fourth owned box*): *"GIVEN a roster of muninn, huginn, odin and a new owned box WHEN a job for the new box starts THEN the runner admits the job"*
  - `BXF-014`: *"If a file transfer between huginn and odin does not pass through muninn, then the box bus SHALL refuse the transfer."*
- **Witness state**:
  1. `odin` has a queued `LPC-005` timing baseline job (or box-specific `setup` job) when `odin`'s link drops overnight. Moving ALL queued jobs to `huginn` per `BXF-010` runs `odin`'s timing job on `huginn` (violating `BXF-011` / `LPC-005` if compared against `odin`, or leaving `odin` unmeasured), or moves proving jobs onto a box (`muninn` or un-rekeyed box) whose key manifest does not match (`PDM-006`).
  2. Adding `HostFourthOwned` to the roster (`BXF-001`): `BXF-014` only restricts transfers *"between huginn and odin"*, permitting direct un-relayed spoke transfers `HostFourthOwned <-> HostOdin` and `HostFourthOwned <-> HostHuginn`, while `box-relay` (`BXF-008` Status: `to:odin`, `to:huginn`, `to:all`) has no routing rule for a 4th roster host.
- **What would close it**:
  - Scope `BXF-010` to **host-agnostic proving jobs** (holding host-pinned timing/setup jobs until `odin` relinks, and requiring target-box roster + manifest eligibility), and generalize `BXF-008` and `BXF-014` from hardcoded `huginn`/`odin` names to *"any two non-hub worker boxes in the factory roster"*.

### `Q-box-fleet-6` — `GPV-001` and `GPV-007` mandate GPU execution unconditionally in `cuda` builds, forbidding CPU calibration (`STM-004`) and CPU fallback (`SHERLOCK_GPUS=0`)
- **IDs**: `GPV-001`, `GPV-007` (compare `GPV-004`, `GPV-005`)
- **Quoted sentences**:
  - `GPV-001`: *"Where the build has the cargo feature `cuda`, the leaf prover SHALL prove app shards on the GPU through `openvm-cuda-backend`."*
  - `GPV-007`: *"Where the build has the cargo feature `cuda`, the leaf prover SHALL prove the leaf-layer proof of each shard on the GPU."*
  - `GPV-004`: *"Where the build has the cargo feature `cuda` and `T7_NODE_GPU` is not `0` or `off`, the node prover SHALL prove N1, N2 and N3 on the GPU."*
- **Witness state**:
  - On `huginn` compiled with `--features cuda`, run `GPV-002`'s own scenario (*"WHEN odin proves it on the GPU **and on the CPU**"*), `setup` CPU calibration, or `SHERLOCK_GPUS = 0` (`h-nogpu`). Because the binary has feature `cuda`, proving the shard on CPU (`appBackend == BackendCpu`, `leafLayerBackend == BackendCpu`) violates the literal `Where` clauses of `GPV-001` and `GPV-007`!
- **What would close it**:
  - Add *"and the run schedule selects a GPU device"* to the `Where` clauses of `GPV-001` and `GPV-007` (mirroring the runtime-switch qualification in `GPV-004` and `GPV-005`).

---

## 3. Mutant Table

Each of the 29 refusal/binding guards has a dedicated mutant module `box_fleet_m_<name>` (`MUTANT_ID = 1..29`). Under `quint run` (`--max-samples=1000 --max-steps=15 --seed=0x20261115`), all 29 mutants exited `1` (`[violation]`) on their target invariant, and all 29 also violated `reach_spoke_transfer_and_drop_failover` (`exit 1`), confirming non-vacuity.

| Mutant Module | `MUTANT_ID` | Guard / Binding Removed or Weakened | Target Invariant | Observed Exit Code | Non-Vacuity (`reach_spoke_transfer_and_drop_failover`) |
|---|---:|---|---|---:|---:|
| `box_fleet_m_bxf001_admit_unowned_host` | `1` | Removes `ownedRoster.contains(targetHost)` check in `canAdmitJob`, admitting `HostRentalExternal` | `inv_BXF_001` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf002_bypass_heavy_slot` | `2` | Starts heavy jobs on `muninn` with `usedHeavySlot = false` | `inv_BXF_002` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf003_admit_shared_over_budget` | `3` | Removes `reqMemGb <= HEAVY_BUDGET_GB` (`64`) check on shared `muninn` jobs, admitting `70 GB` shared | `inv_BXF_003` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf004_gpu_job_without_lock` | `4` | Starts a GPU proving job without acquiring `~/sherlock/gpu.lock` (`gpuLockHeld = false`) | `inv_BXF_004` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf005_sigterm_without_log_or_early` | `5` | Sends `SIGTERM` to an orphan without logging on the bus first (`loggedOnBusBeforeSignal = false`) | `inv_BXF_005` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf006_kill_slow_prover` | `6` | Allows orphan-lock `SIGTERM` to kill a slow `HolderProver` (`40` min) | `inv_BXF_006` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf007_accept_bad_digest_transfer` | `7` | Removes `busSha == actualSha` check in `canAcceptFileTransfer` | `inv_BXF_007` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf009_long_run_not_durable` | `8` | Starts `> 30` min runs without `systemd-run --user` or persistent log | `inv_BXF_009` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf010_leave_dropped_queue_stranded` | `9` | Marks link drop handled while leaving `queuedSuiteJobs > 0` stranded on dropped `odin` | `inv_BXF_010` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf011_cross_box_speed_claim` | `10` | Compares `huginn`/`odin` run times against `HostMuninn` baseline | `inv_BXF_011` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf012_call_conf_compute` | `11` | Invokes `nvidia-smi conf-compute` (`usedConfCompute = true`) during GPU probe | `inv_BXF_012` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf013_early_or_unrechecked_sigkill` | `12` | Sends `SIGKILL` `10s < 30s` after `SIGTERM` without re-verifying orphan eligibility | `inv_BXF_013` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_bxf014_allow_direct_odin_huginn` | `13` | Allows direct `HostOdin -> HostHuginn` file transfer (`routedViaMuninn = false`) | `inv_BXF_014` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_gpv001_cuda_app_on_cpu` | `14` | Runs GPU-scheduled app shard on `BackendCpu` in a `cuda` build | `inv_GPV_001` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_gpv002_gpu_cpu_pubvals_differ` | `15` | Emits diverging `publicValuesDigest` (`8889` vs `8888`) between CUDA and CPU shard proofs | `inv_GPV_002` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_gpv003_lfr_gpu_differs_from_cpu` | `16` | Emits diverging `lfrProofBytesDigest` (`7778` vs `7777`) between GPU and `MORI_GPU=off` | `inv_GPV_003` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_gpv004_ignore_node_gpu_switch` | `17` | Proves `N1..N3` on `BackendCuda` even when `T7_NODE_GPU = 0` (`nodeGpuSwitchOff = true`) | `inv_GPV_004` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_gpv006_write_partial_proof_on_fault` | `18` | Writes partial proof under a final name (`finalProofWritten = true`) upon GPU OOM fault | `inv_GPV_006` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_gpv007_leaf_layer_stays_cpu` | `19` | Leaves GPU-scheduled leaf-layer proof on `BackendCpu` in a `cuda` build | `inv_GPV_007` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_pdm001_reload_srs_on_second_call` | `20` | Reloads SRS/keys on the second served daemon call (`keyReloadsAfterStart > 0`) | `inv_PDM_001` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_pdm002_world_readable_socket` | `21` | Creates daemon socket with mode `0666` and serves another user | `inv_PDM_002` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_pdm003_crash_on_unknown_op` | `22` | Terminates `daemon.running = false` on unknown operation `dump` | `inv_PDM_003` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_pdm005_reuse_static_blinder` | `23` | Reuses static `blinderId = 999` across served wrap proofs | `inv_PDM_005` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_pdm006_serve_mismatched_manifest` | `24` | Serves `T10m` manifest request on a `T25`-loaded daemon | `inv_PDM_006` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_pdm007_one_leaf_internal_nodes` | `25` | Serves a 1-leaf call via internal nodes (`usedPathC = false`, `internalNodeProofsMade = 1`) | `inv_PDM_007` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_pdm008_stats_leaks_witness` | `26` | Includes `3` secret/witness words in GUI `stats` reply | `inv_PDM_008` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_pdm009_carryover_prior_blinder` | `27` | Carries over blinder state (`priorBlinderCarryover = 17`) from statement `Y` into `X` | `inv_PDM_009` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_lpc007_scaleup_changes_keys` | `28` | Changes `keysSha` at `8x` scale-up run length | `inv_LPC_007` | `1` (`[violation]`) | `1` (live) |
| `box_fleet_m_prd003_export_stark_outside_roster` | `29` | Transfers STARK proof artifact to external `HostRentalExternal` outside owned-box roster | `inv_PRD_001_004` | `1` (`[violation]`) | `1` (live) |

---

## 4. State Vocabulary for Replay

### 4.1 Top-Level State Variables (`var`)
In ITF JSON traces (`../traces/box_fleet_main/t_{seq}.itf.json`), Quint qualifies variables under `box_fleet_main::box_fleet::<var>`:

| Quint `var` | ITF JSON Key | Quint Type | Meaning |
|---|---|---|---|
| `fleet` | `box_fleet_main::box_fleet::fleet` | `BoxFleetState` | Cohesive observable state of the Box Fleet bounded context (roster, `heavy-slot` state, per-box `gpu.lock` & link state, jobs, signal logs, transfers, bus relay, GPU stage proofs & faults, `wrapd` & STARK worker daemons, served proofs, `stats` replies, timing records, scale-up demo runs, and latest refusal). |
| `lastOp` | `box_fleet_main::box_fleet::lastOp` | `str` | Action name that produced the state (`"init"` at index `0`, exact action identifier on every step). |

#### Fields of `fleet: BoxFleetState`
- `ownedRoster: Set[HostId]` — User-owned boxes (`HostMuninn`, `HostHuginn`, `HostOdin`, optionally `HostFourthOwned`; `HostRentalExternal` is never owned) (`BXF-001`, `PRD-003`).
- `activeKeysSha: int` (`250` = T25) — Active universal keyset digest (`PDM-006`, `LPC-005`, `LPC-007`).
- `muninnSharedRunning: int`, `muninnExclusiveRunning: bool`, `muninnSlotWaiters: int` — `heavy-slot` admission state on `muninn` (`BXF-002`, `BXF-003`).
- `nextJobId: int`, `admittedJobs: Set[AdmittedJobRec]` — Admitted jobs with `targetHost`, `admittedRosterSnapshot`, `isHeavyMuninnJob`, `usedHeavySlot`, `isExclusiveSlot`, `reqMemGb`, `plannedMin`, `startedAsSystemdUserUnit`, `persistentLogEnabled`, and `resumedAfterReboot` (`BXF-001..003`, `BXF-009`).
- `boxes: HostId -> BoxRuntimeState` — Per-host `linkUp`, `linkDroppedWhileOwnerAway`, `failoverHandled`, `queuedSuiteJobs`, `activeGpuProvingJobs`, `gpuWaitQueue`, `gpuLockHeld`, `lockHolderRole` (`HolderNone | HolderProver | HolderWrapd | HolderOrphanSampler`), `lockHolderPid`, `holderParentAlive`, `lockStallMin`, `sigtermLoggedOnBus`, `sigtermSent`, `elapsedSinceSigtermSec`, `cudaBuildEnabled`, and `gpuFeatureEnabled` (`BXF-004..006`, `BXF-010`, `BXF-013`).
- `killEvents: Set[LockSignalEvent]` — Signal audit trail (`SigTerm` / `SigKill`, `loggedOnBusBeforeSignal`, `precededBySigtermSamePid`, `elapsedAfterSigtermSec`, `recheckedEligible`) (`BXF-005`, `BXF-006`, `BXF-013`).
- `acceptedTransfers: Set[FileTransferRec]` — Verified cross-box file transfers (`srcHost`, `dstHost`, `routedViaMuninn`, `busMsgSha256`, `actualSha256`, `digestCheckedOk`, `isStark`, `wrapStarted`) (`BXF-007`, `BXF-014`, `PRD-003`).
- `relayedMessages: Set[BusMessageRec]`, `allLogMsgIds: Set[int]` — `box-relay` outbox messages appended to `~/.cache/box-bus/all.log` (`BXF-008`, `BXF-010`).
- `gpuProbeCalls: Set[GpuProbeRec]` — GPU query audit records (`usedQueryGpu`, `usedCudaMemCall`, `usedConfCompute`) (`BXF-012`).
- `stageProofs: Set[GpuStageProofRec]`, `gpuFaultRecords: Set[GpuFaultRec]` — GPU/CPU stage backend records (`appBackend`, `leafLayerBackend`, `nodeBackend`, `lBackend`, `publicValuesDigest`, `lfrProofBytesDigest`, `stockVerified`, `lStockVerified`) and device OOM fault stops (`GPV-001..007`).
- `daemon: WrapDaemonState`, `starkWorker: StarkWorkerState`, `consumedBlinderIds: Set[int]`, `servedWrapProofs: Set[ServedWrapProofRec]`, `statsReplies: Set[StatsReplyRec]` — Resident `wrapd serve` and STARK worker state, `0600` socket metadata, one-use CSPRNG blinders, Path C counters, and secret-free GUI `stats` replies (`PDM-001..009`, `PRD-001..004`).
- `timingRecords: Set[TimingClaimRec]`, `scaleDemoRuns: Set[ScaleDemoRec]` — Same-box warm/cold timing claims (`BXF-011`, `LPC-005`) and `1x/2x/4x/8x` elastic scale-up records (`LPC-007`).
- `lastRefusal: FleetRefusal` — Latest refusal diagnostic variant.

### 4.2 `lastOp` Values and Implemented Requirements

All 32 `lastOp` values (`"init"` + 31 actions; every single one appears in the 20 generated ITF traces):

| `lastOp` Value | Requirement ID(s) | Effect Summary |
|---|---|---|
| `"init"` | All (`BXF`, `GPV`, `PDM`, `LPC`, `PRD`) | Initializes 3-box owned roster (`muninn`, `huginn`, `odin`), T25 keys, idle locks/slots, and stopped daemons. |
| `"addFourthOwnedBoxToRoster"` | `BXF-001` | Adds `HostFourthOwned` to `ownedRoster`. |
| `"refuseUnownedHostJob"` | `BXF-001` | Refuses a job targeting `HostRentalExternal`; sets `FleetRefusalUnownedHost`. |
| `"admitAndStartBoxJob"` | `BXF-001..003`, `BXF-009` | Admits job on owned box, acquires `heavy-slot` on `muninn`, and wraps `> 30` min runs in `systemd-run --user`. |
| `"enqueueMuninnSlotWaiter"` | `BXF-002` | Enqueues a waiting job on `muninn` when all `HEAVY_SLOTS_MAX` slots (or `--exclusive`) are busy. |
| `"releaseMuninnSlot"` | `BXF-002` | Releases a completed `muninn` slot and promotes a waiting job if present. |
| `"refuseSharedOverBudgetOnMuninn"` | `BXF-003` | Refuses a shared `heavy-slot` job requesting `> HEAVY_BUDGET_GB` (`70 GB > 64 GB`). |
| `"rebootAndResumeDurableUnits"` | `BXF-009` | Simulates host reboot and resumes durable `systemd-run --user` units with persistent logs intact. |
| `"startOrQueueGpuJobOnBox"` | `BXF-004` | Acquires exclusive `~/sherlock/gpu.lock` for a GPU job or queues a waiter when held. |
| `"finishGpuJobReleaseLock"` | `BXF-004` | Releases `gpu.lock` or hands it directly to the next queued GPU job on that box. |
| `"stageLockHolderScenario"` | `BXF-005`, `BXF-006`, `BXF-013` | Stages either a stalled dead-parent orphan sampler (`12` min) or a slow prover (`40` min) holding `gpu.lock`. |
| `"observeSlowProverLeaveRunning"` | `BXF-006`, `BXF-008` | Leaves a slow `HolderProver` / `HolderWrapd` running untouched and logs status on the box bus. |
| `"reapOrphanLockSigterm"` | `BXF-005` | Logs dead-parent stalled orphan sampler on the bus and sends `SIGTERM` (optionally trapped by holder). |
| `"escalateOrphanLockSigkill"` | `BXF-013` | After `>= SIGKILL_GRACE_SEC` (`30` s), re-verifies orphan role & dead parent and sends `SIGKILL`. |
| `"transferFileVerified"` | `BXF-007`, `BXF-014`, `PRD-003` | Moves a result file across owned boxes (`odin <-> huginn` via `muninn`) after checking `busSha == actualSha`. |
| `"refuseDigestMismatchTransfer"` | `BXF-007` | Refuses a truncated/corrupted transfer whose `actualSha` differs from `busSha`, blocking wrap. |
| `"refuseDirectSpokeTransfer"` | `BXF-014` | Refuses a direct `odin <-> huginn` transfer bypassing `muninn`. |
| `"refuseStarkExportOutsideRoster"` | `PRD-003` | Refuses exporting a STARK artifact to a host outside `ownedRoster`. |
| `"postAndRelayBusMessage"` | `BXF-008` | Relays an outbox message, appending to `~/.cache/box-bus/all.log` and delivering to recipient. |
| `"dropBoxLinkAndHandoverQueue"` | `BXF-010`, `BXF-008` | Marks `droppedHost` link down while owner is away, moves queued suite jobs to `backupHost`, and logs on bus. |
| `"probeGpuWithoutConfCompute"` | `BXF-012` | Reads GPU info via `nvidia-smi --query-gpu` + CUDA memory call with zero `conf-compute` calls. |
| `"proveStagePipeline"` | `GPV-001..005`, `GPV-007` | Proves app, leaf-layer, `N1..N3`, `L`, and `L_Fr` stages under CUDA/CPU switches with matching digests. |
| `"faultGpuShardOutOfMemory"` | `GPV-006` | Stops run on GPU device error, records `(faultHost, shardIdx)`, and writes no partial final proof. |
| `"startDaemonsOnBox"` | `PDM-001`, `PDM-002`, `PDM-004`, `PRD-001` | Starts `wrapd serve` (6 resident items, mode `0600` under `$XDG_RUNTIME_DIR`) and resident STARK worker. |
| `"refuseUnauthorizedSocketPeer"` | `PDM-002`, `PRD-001` | Refuses socket connection from another local user or remote peer. |
| `"daemonHandleUnknownOpDump"` | `PDM-003` | Replies with error to unknown operation `dump` while keeping `daemon.running = true`. |
| `"daemonRefuseManifestMismatch"` | `PDM-006` | Refuses a daemon request with mismatched key manifest `sha256`, naming both digests. |
| `"daemonRefuseBadBlinder"` | `PRD-002` | Refuses a reused or helper-supplied blinder. |
| `"daemonServeCall"` | `PDM-001`, `PDM-004..007`, `PDM-009`, `PRD-001..002`, `PRD-004` | Serves `prove`/`call` with 0 reloads, fresh local CSPRNG blinders, 0 carryover, and Path C when `leafCount == 1`. |
| `"daemonServeStats"` | `PDM-008` | Replies to GUI bridge `stats` with stage, shard count, stage time, and `0` witness words. |
| `"recordBoxTimingClaim"` | `BXF-011`, `LPC-005` | Records post-rekey warm/cold call times, peak RAM, and load on `huginn`/`odin` against same-box baseline. |
| `"runElasticScaleDemo"` | `LPC-007` | Proves `1x, 2x, 4x, 8x` run lengths (`> 5` rules/leaf) under one keyset, including `muninn + huginn` join. |

### 4.3 Instance Constants

| Constant | `box_fleet_main` | `box_fleet_edge_min` | `box_fleet_edge_at_limit` | `box_fleet_m_*` | Meaning |
|---|---:|---:|---:|---:|---|
| `HEAVY_SLOTS_MAX` | `3` | `1` | `3` | `3` | Max concurrent shared `heavy-slot` jobs on `muninn` (`BXF-002`) |
| `HEAVY_BUDGET_GB` | `64` | `32` | `64` | `64` | Max GiB per shared `heavy-slot` job (`BXF-003`) |
| `ORPHAN_STALL_MIN` | `10` | `10` | `10` | `10` | No-progress minutes required before orphan `SIGTERM` (`BXF-005`) |
| `SIGKILL_GRACE_SEC` | `30` | `30` | `35` | `30` | Seconds required after `SIGTERM` before `SIGKILL` (`BXF-013`; `35` = exact at `35s` trapped wait) |
| `DURABLE_RUN_THRESHOLD_MIN` | `30` | `30` | `30` | `30` | Planned minutes above which `systemd-run --user` is required (`BXF-009`) |
| `WRAPD_RESIDENT_ITEMS` | `6` | `6` | `6` | `6` | Resident objects in `wrapd serve` (`PDM-001`) |
| `SOCKET_MODE_OCTAL` | `600` | `600` | `600` | `600` | Required Unix socket permission mode `0600` (`PDM-002`) |
| `MIN_REKEY_LEAF_RULES` | `16` | `6` | `16` | `16` | Rules per leaf after rekey (`> 5` in `LPC-007`; `6` = smallest legal value) |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 29` | `0` = sound; `1..29` = single-guard/binding mutant |

---

## 5. Model Abstractions

1. **OS Primitives (`flock`, `systemd-run --user`, Unix Sockets, Signals)**: Kernel `flock` descriptors (`BXF-004`), `SIGTERM`/`SIGKILL` process signals (`BXF-005`, `BXF-013`), `systemd-run --user` units (`BXF-009`), and `$XDG_RUNTIME_DIR` `0600` Unix socket permissions (`PDM-002`) are modelled as explicit state fields and audit records rather than POSIX syscalls.
2. **Wall-Clock Durations**: The `10`-minute orphan stall threshold (`BXF-005`), `30`-second `SIGKILL` grace period (`BXF-013`), and `30`-minute durable-unit threshold (`BXF-009`) are represented as integer duration attributes (`lockStallMin`, `elapsedSinceSigtermSec`, `plannedMin`).
3. **Cryptographic Digests, Proofs, and CSPRNG Blinders**: File `sha256` digests (`BXF-007`), BabyBear shard public values (`GPV-002`), `L_Fr` proof bytes (`GPV-003`), key manifest digests (`PDM-006`), and OS CSPRNG blinder draws (`PDM-005`, `PDM-009`) are represented as injective integers (`blinderId = 5000 + seqNext`, `wrapProofBytesDigest = 30000 + 100 * stmtId + blinderId + carryOver`).
4. **Host Topology**: The user's dynamic roster (`BXF-001`) is modelled over 5 symbolic hosts: the 3 factory fixtures (`HostMuninn`, `HostHuginn`, `HostOdin`), a 4th user-owned host (`HostFourthOwned`), and an external unowned rental host (`HostRentalExternal`).
5. **Bounded Audit Sets**: Set growth in random simulation is bounded (`nextJobId <= 8`, `servedSeqCounter <= 6`, `stageProofs.size() < 8`) so 15-step random walks explore all 31 actions without state explosion.

---

## 6. Commands

Run every command below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/box-fleet/quint`:

```bash
# 1. Typecheck model and test suite
quint typecheck box_fleet.qnt
# Expected exit code: 0
# Printed line: (no output on success)

quint typecheck box_fleet_test.qnt
# Expected exit code: 0
# Printed line: (no output on success)

# 2. Requirement scenario unit tests (10 tests)
quint test --main=box_fleet_test --match='bf_.*Test' --seed=0x20261115 box_fleet_test.qnt
# Expected exit code: 0
# Printed line: 10 passing (4891ms)

# 3. Main and edge instance safety invariant checks
quint run --main=box_fleet_main --invariant=inv --max-samples=2000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (2952ms at 678 traces/second).

quint run --main=box_fleet_edge_min --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (1417ms at 706 traces/second).

quint run --main=box_fleet_edge_at_limit --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (1644ms at 608 traces/second).

# 4. Non-vacuity reachability witnesses on main instance (negated goals: all MUST exit 1)
quint run --main=box_fleet_main --invariant=reach_fourth_box_and_exclusive_slot_used --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (234ms at 124 traces/second).

quint run --main=box_fleet_main --invariant=reach_orphan_sigterm_and_sigkill_chain --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (469ms at 2 traces/second).

quint run --main=box_fleet_main --invariant=reach_spoke_transfer_and_drop_failover --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (610ms at 5 traces/second).

quint run --main=box_fleet_main --invariant=reach_gpu_cpu_stage_proofs_and_fault --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (667ms at 6 traces/second).

quint run --main=box_fleet_main --invariant=reach_daemon_two_calls_and_path_c --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (507ms at 49 traces/second).

quint run --main=box_fleet_main --invariant=reach_timing_and_scale_demo_recorded --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (334ms at 24 traces/second).

quint run --main=box_fleet_main --invariant=reach_refusals_exercised --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (273ms at 4 traces/second).

# 5. Guard and binding mutants (all 29 MUST exit 1)
quint run --main=box_fleet_m_bxf001_admit_unowned_host --invariant=inv_BXF_001 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (556ms at 2 traces/second).

quint run --main=box_fleet_m_bxf002_bypass_heavy_slot --invariant=inv_BXF_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (306ms at 7 traces/second).

quint run --main=box_fleet_m_bxf003_admit_shared_over_budget --invariant=inv_BXF_003 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (290ms at 10 traces/second).

quint run --main=box_fleet_m_bxf004_gpu_job_without_lock --invariant=inv_BXF_004 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (229ms at 9 traces/second).

quint run --main=box_fleet_m_bxf005_sigterm_without_log_or_early --invariant=inv_BXF_005 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (253ms at 4 traces/second).

quint run --main=box_fleet_m_bxf006_kill_slow_prover --invariant=inv_BXF_006 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (223ms at 13 traces/second).

quint run --main=box_fleet_m_bxf007_accept_bad_digest_transfer --invariant=inv_BXF_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (279ms at 4 traces/second).

quint run --main=box_fleet_m_bxf009_long_run_not_durable --invariant=inv_BXF_009 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (288ms at 7 traces/second).

quint run --main=box_fleet_m_bxf010_leave_dropped_queue_stranded --invariant=inv_BXF_010 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (552ms at 5 traces/second).

quint run --main=box_fleet_m_bxf011_cross_box_speed_claim --invariant=inv_BXF_011 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (638ms at 2 traces/second).

quint run --main=box_fleet_m_bxf012_call_conf_compute --invariant=inv_BXF_012 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (1085ms at 1 traces/second).

quint run --main=box_fleet_m_bxf013_early_or_unrechecked_sigkill --invariant=inv_BXF_013 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (568ms at 2 traces/second).

quint run --main=box_fleet_m_bxf014_allow_direct_odin_huginn --invariant=inv_BXF_014 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (409ms at 2 traces/second).

quint run --main=box_fleet_m_gpv001_cuda_app_on_cpu --invariant=inv_GPV_001 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (357ms at 11 traces/second).

quint run --main=box_fleet_m_gpv002_gpu_cpu_pubvals_differ --invariant=inv_GPV_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (252ms at 16 traces/second).

quint run --main=box_fleet_m_gpv003_lfr_gpu_differs_from_cpu --invariant=inv_GPV_003 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (699ms at 6 traces/second).

quint run --main=box_fleet_m_gpv004_ignore_node_gpu_switch --invariant=inv_GPV_004 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (1163ms at 2 traces/second).

quint run --main=box_fleet_m_gpv006_write_partial_proof_on_fault --invariant=inv_GPV_006 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (255ms at 12 traces/second).

quint run --main=box_fleet_m_gpv007_leaf_layer_stays_cpu --invariant=inv_GPV_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (551ms at 7 traces/second).

quint run --main=box_fleet_m_pdm001_reload_srs_on_second_call --invariant=inv_PDM_001 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (726ms at 34 traces/second).

quint run --main=box_fleet_m_pdm002_world_readable_socket --invariant=inv_PDM_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (375ms at 3 traces/second).

quint run --main=box_fleet_m_pdm003_crash_on_unknown_op --invariant=inv_PDM_003 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (832ms at 23 traces/second).

quint run --main=box_fleet_m_pdm005_reuse_static_blinder --invariant=inv_PDM_005 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (436ms at 57 traces/second).

quint run --main=box_fleet_m_pdm006_serve_mismatched_manifest --invariant=inv_PDM_006 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (609ms at 10 traces/second).

quint run --main=box_fleet_m_pdm007_one_leaf_internal_nodes --invariant=inv_PDM_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (738ms at 8 traces/second).

quint run --main=box_fleet_m_pdm008_stats_leaks_witness --invariant=inv_PDM_008 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (339ms at 3 traces/second).

quint run --main=box_fleet_m_pdm009_carryover_prior_blinder --invariant=inv_PDM_009 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (192ms at 130 traces/second).

quint run --main=box_fleet_m_lpc007_scaleup_changes_keys --invariant=inv_LPC_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (811ms at 1 traces/second).

quint run --main=box_fleet_m_prd003_export_stark_outside_roster --invariant=inv_PRD_001_004 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 box_fleet.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (489ms at 2 traces/second).

# 6. Generate 20 ITF replay traces
quint run box_fleet.qnt --main box_fleet_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/box-fleet/traces/box_fleet_main/t_{seq}.itf.json'
# Expected exit code: 0
# Printed line: [ok] No violation found (1016ms at 20 traces/second).
```
