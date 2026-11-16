# Sherlock E2E Scheduling — Independent Quint Formal Specification Report

- **Lane**: `e2e-scheduling` (Gemini 4 Carbon)
- **Bounded Context**: Scheduling (`sherlock-e2e-setup-mode`, `sherlock-e2e-elastic-proving`, completion `LPC-006`, adopted `SPL-011` and `SETUP-SPEC.md` `ST-001..008`, `PF-001..008`, `SC-001..008`, `SC-010..014`)
- **Quint Artifacts**:
  - Model: `scheduling.qnt` (`scheduling`, `scheduling_main`, `scheduling_edge_min`, `scheduling_edge_at_limit`, and 22 guard/binding mutants `scheduling_m_*`)
  - Requirement tests: `scheduling_test.qnt` (`20 passing` scenario `run` tests)
  - ITF replay traces: `../traces/scheduling_main/t_0.itf.json` .. `t_19.itf.json` (20 traces, 15 steps max, seed `0x20261115`)

---

## 1. Requirement Coverage Table (ID Table)

Every own EARS ID (`STM-001` .. `STM-019`, `ELA-001` .. `ELA-007`), completion EARS ID (`LPC-006`), and cited adopted EARS ID (`SPL-011`, `ST-001..008`, `PF-001..008`, `SC-001..008`, `SC-010..014`; note `SETUP-SPEC.md` has no `SC-009`) is listed below.

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
|---|---|---|---|
| `STM-001` | `inv_STM_001`, `consistency_constants`; action `runSetupOnBox`; test `sc_Stm001And002GpusSevenFieldsAndUnifiedMuninnTest` | `MODELED` / `CONSTANT-CHECKED` | Carries `ST-001`, `PF-002`. Checks `gpuEntries.length() == hw.gpuCount` and `entryFieldCount == GPU_ENTRY_FIELDS_REQUIRED` (`7`) with positive `computeCap` and `smClockMhz`. See Finding `Q-scheduling-1`. |
| `STM-002` | `inv_STM_002`; action `runSetupOnBox`; test `sc_Stm001And002GpusSevenFieldsAndUnifiedMuninnTest`; mutant `scheduling_m_stm002_unified_not_marked` | `MODELED` | Carries `ST-002`. Marks GPUs without own VRAM (`vramTotalMb == -1` modelling `null`, e.g. `muninn` GB10) as `unified == true`. |
| `STM-003` | `inv_STM_003`; action `runSetupOnBox`; test `sc_Stm003HostProbeCgroupMinAndNoCgroupTest`; mutant `scheduling_m_stm003_ignore_cgroup_cap` | `MODELED` | Carries `ST-003`, `PF-003`. Sets profile `memBudgetGb` to `min(MemAvailable, cgroupHeadroom)` when a cgroup limit exists (`huginn`: `min(64, 60) = 60`), else `MemAvailable` (`muninn`: `100`). |
| `STM-004` | `inv_STM_004`, `consistency_constants`; action `runSetupOnBox`; test `sc_Stm004And006CalibrateRepsAndElevenFieldsRoundTripTest` | `MODELED` / `CONSTANT-CHECKED` | Carries `ST-004`, `ST-006`. Calibrates each leaf shape on CPU and GPU with `WARMUP_REPS = 1` warm-up and `DEFAULT_CAL_REPS = 3` (`1` in `edge_min`) timed repetitions. See Findings `Q-scheduling-1`, `Q-scheduling-4`, `Q-scheduling-5`. |
| `STM-005` | `inv_STM_005`; action `abortSetupBadCalProof`; test `sc_Stm005RefusedCalProofWritesNoProfileTest`; mutant `scheduling_m_stm005_write_on_bad_cal_proof` | `MODELED` | Carries `ST-005`. Stops setup and leaves `written == false` if the stock verifier refuses any calibration proof. |
| `STM-006` | `inv_STM_006`, `consistency_constants`; action `runSetupOnBox`; test `sc_Stm004And006CalibrateRepsAndElevenFieldsRoundTripTest` | `MODELED` / `CONSTANT-CHECKED` | Carries `ST-008`, `PF-001..004`. Writes `format = sherlock-profile/1` with all `PROFILE_FIELD_COUNT = 11` fields. See Findings `Q-scheduling-1`, `Q-scheduling-3`. |
| `STM-007` | `inv_STM_007`; actions `injectMalformedProfile`, `refuseMalformedProfile`; test `sc_Stm007ZeroLeafTimeAndBadFormatRefusedTest`; mutant `scheduling_m_stm007_accept_zero_leaf_s` | `MODELED` | Carries `PF-005`. Refuses profiles with bad `format`, missing fields, or non-positive `leaf_s`, blocking run scheduling. See Finding `Q-scheduling-4`. |
| `STM-008` | `inv_STM_008`; actions `installStaleT10mProfile`, `recordUncalibratedProfileKeyMismatch`; test `sc_Stm008StaleT10mProfileRecordedNotCalibratedTest`; mutant `scheduling_m_stm008_ignore_keyset_mismatch` | `MODELED` | Carries `PF-006`. Records runs whose profile `key_root` or `keys_d` (`T10m` vs `T25`) differs from the prover's key set as `not calibrated`. See Finding `Q-scheduling-3`. |
| `STM-009` | `inv_STM_009`, `consistency_constants`; action `scheduleAndProveFromProfile`; test `sc_Stm009DeriveThreeLeafScheduleTest` | `MODELED` / `CONSTANT-CHECKED` | Carries `SC-001`, `SC-003`. Derives `ceil(n / S)` leaves (`<= MAX_LEAVES_BOUND`) of `1..S` steps covering `runSteps` from profile, run length, and runtime limits. |
| `STM-010` | `inv_STM_010`; action `scheduleAndProveFromProfile`; test `sc_Stm010OperatorLimitsOnlyLowerProfileValuesTest`; mutant `scheduling_m_stm010_limit_raises_budget` | `MODELED` | Carries `SC-002`. Operator limits (`SHERLOCK_MEM_BUDGET`, `SHERLOCK_GPUS`, `SHERLOCK_GPU_MEM`) clamp via `min(limit, profileValue)` and never raise profile values. See Findings `Q-scheduling-1`, `Q-scheduling-2`. |
| `STM-011` | `inv_STM_011`, `consistency_constants`; action `scheduleAndProveFromProfile`; test `sc_Stm011UnifiedPoolAndDiscreteHostCapTest`; mutant `scheduling_m_stm011_ignore_host_cap_on_gpu` | `MODELED` / `CONSTANT-CHECKED` | Carries `SC-004`. Bounds leaf concurrency by `max(1, cores / CPU_CORES_PER_LEAF)` (`4`) and host RAM on CPU, `min(VRAM, host RAM)` on discrete GPU, and single shared pool `deviceGb + hostGb` on unified GPU. See Findings `Q-scheduling-1`, `Q-scheduling-4`. |
| `STM-012` | `inv_STM_012`; action `refuseUnfittableBudget`; test `sc_Stm012OneGbBudgetRefusedBeforeProofTest`; mutant `scheduling_m_stm012_schedule_zero_fit` | `MODELED` | Carries `SC-006`. Refuses runs before any proof when no shape fits the memory budget on any device (e.g. `1 GB` budget). See Finding `Q-scheduling-4`. |
| `STM-013` | `inv_STM_013`; actions `scheduleAndProveForcedCut`, `refuseInvalidCheckFree`; test `sc_Stm013And014ForcedFiveLeafCutAndNoMenuTest`; mutant `scheduling_m_stm013_bypass_check_free` | `MODELED` | Carries `SC-008`, `SC-010`. Enforces `check_free` (admitted shape key, `>= 1` leaf, exact step cover, `1..S` steps per leaf) and records forced overrides (`T16_CUT = 1,1,1,1,1`). See Finding `Q-scheduling-5`. |
| `STM-014` | `inv_STM_014`; actions `scheduleAndProveFromProfile`, `scheduleAndProveForcedCut`; test `sc_Stm013And014ForcedFiveLeafCutAndNoMenuTest`; mutant `scheduling_m_stm014_legacy_menu_check` | `MODELED` | Accepts arbitrary `check_free`-valid shard counts (`1, 2, 3, 5`) with no schedule menu in `key_root` or prover. |
| `STM-015` | `inv_STM_015`, `consistency_constants`; action `scheduleAndProveFromProfile`; test `sc_Stm015WrapPipelineOdinHuginnSmallGpuTest`; mutant `scheduling_m_stm015_wrong_wrap_policy` | `MODELED` / `CONSTANT-CHECKED` | Carries `PF-007`. Derives `PipeFullOverlap` (`odin` 32 GB), `PipeCosetEvict` (`huginn` 16 GB), or `PipeNoOverlap` (small GPU) after `1 / PIPE_HEADROOM_DIV` (`1/16`) headroom. See Finding `Q-scheduling-2`. |
| `STM-016` | `inv_STM_016`; action `scheduleAndProveFromProfile`; test `sc_Stm016DefaultBoxProfilePathLoadedTest`; mutant `scheduling_m_stm016_wrong_default_path` | `MODELED` | Loads `~/sherlock/setup/profile.json` (`ProfilePathDefaultBox`) whenever `T16_PROFILE` is unset. See Finding `Q-scheduling-3`. |
| `STM-017` | `inv_STM_017`; action `scheduleAndProveFromProfile`; test `sc_Stm017LeastTimeAndReversedOrderTieBreakTest`; mutant `scheduling_m_stm017_wrong_tie_order` | `MODELED` | Carries `SC-005`. Picks least `(estWallMs, cutLeafCount, shapeIdx, GPU-before-CPU)` independent of calibration entry order. See Finding `Q-scheduling-1`. |
| `STM-018` | `inv_STM_018`; action `scheduleAndProveFromProfile`; test `sc_Stm018BudgetSequence64To32To16NeverFasterTest`; mutant `scheduling_m_stm018_lower_budget_faster` | `MODELED` | Carries `SC-007`. Lowering `effectiveMemBudgetGb` never yields a lower `estWallMs` than the higher budget's pick. See Finding `Q-scheduling-6`. |
| `STM-019` | `inv_STM_019`, `consistency_constants`; action `evaluateEarlyWrapGuard`; test `sc_Stm019OneShortRefusesEarlyWrapTest`; mutant `scheduling_m_stm019_admit_early_wrap_short` | `MODELED` / `CONSTANT-CHECKED` | Carries `PF-008`. Admits early wrap start iff `freeDeviceMb >= STARK_EXTRA_MB + wrapTail + WRAP_MARGIN_MB` (`wrapTail` subtracts `RESIDENT_COSETS_MB` under `PipeFullOverlap`), refusing at `reqMb - 1`. |
| `ELA-001` | `inv_ELA_001`; actions `scheduleAndProveFromProfile`, `scheduleAndProveForcedCut`; test `sc_Ela001To005And007ElasticSuiteAndRunRecordTest`; mutant `scheduling_m_ela001_schedule_changes_claim` | `MODELED` | Carries `SC-012`. Enforces byte-equal `claimPub` across all schedules/cuts of one prepared run (`[1,1,1,1,1]` vs profile cut) and distinct `claimPub` across fresh salts (`PrepRunA_Short` vs `PrepRunC_FreshSalt`). |
| `ELA-002` | `inv_ELA_002`; actions `scheduleAndProveFromProfile`, `scheduleAndProveForcedCut`; test `sc_Ela001To005And007ElasticSuiteAndRunRecordTest`; mutant `scheduling_m_ela002_device_specific_l_key` | `MODELED` | Carries `SC-011`. Enforces identical `(app, leaf, N1, N2, N3, L)` key tuple across CPU/GPU, cuts, concurrencies, and hosts. |
| `ELA-003` | `inv_ELA_003`; actions `scheduleAndProveFromProfile`, `refuseUnfittableBudget`; test `sc_Ela001To005And007ElasticSuiteAndRunRecordTest` | `MODELED` | Proves under lowered memory/GPU caps with fitted concurrency `>= 1` and unchanged `claimPub`, refusing when `< 1` leaf fits (`STM-012`). |
| `ELA-004` | `inv_ELA_004`; action `scheduleAndProveFromProfile`; test `sc_Ela001To005And007ElasticSuiteAndRunRecordTest`; mutant `scheduling_m_ela004_gpu_used_when_hidden` | `MODELED` | When `effectiveGpuCount == 0` (`SHERLOCK_GPUS = 0`), proves leaves on `DevCpu` with `lStockVerified == true`. |
| `ELA-005` | `inv_ELA_005`; action `scheduleAndProveFromProfile`; test `sc_Ela001To005And007ElasticSuiteAndRunRecordTest` | `MODELED` | Proving one prepared run on `muninn`, `huginn`, and `odin` from each box's own profile passes `check_free` and yields identical `claimPub`. |
| `ELA-006` | `inv_ELA_006`; actions `joinLeavesTwoBoxes`, `refuseJoinCorruptedLeaf`; test `sc_Ela006CrossBoxJoinAndCorruptedLeafRefusedTest`; mutant `scheduling_m_ela006_skip_leaf_verify_on_join` | `MODELED` | Carries `SC-014`. Checks every leaf with the stock verifier (plus manifest equality and contiguous `h` intervals) before starting `N1`, stopping and naming any corrupted leaf. See Finding `Q-scheduling-6`. |
| `ELA-007` | `inv_ELA_007`; actions `scheduleAndProveFromProfile`, `scheduleAndProveForcedCut`; test `sc_Ela001To005And007ElasticSuiteAndRunRecordTest`; mutant `scheduling_m_ela007_incomplete_run_record` | `MODELED` | Requires every completed elastic run to write a complete measured run record (`shardCount`, stage times, peak RAM, `L` result, `claimPub`). |
| `LPC-006` | `inv_LPC_006`; action `scheduleAndProveFromProfile`; test `sc_Lpc006AndSpl011TwoRunLengthsOnePlanAndUnlistedShapeRefusedTest`; mutant `scheduling_m_lpc006_longer_run_mutates_prog_d` | `MODELED` | Completion ID: two run lengths (`PrepRunA_Short` = 5 steps, `PrepRunB_Long` = 12 steps) under one `plan_d` (`501`) are both accepted by the node with invariant `prog_d` (`601`) and keys. |
| `SPL-011` *(adopted)* | `inv_SPL_011`; actions `scheduleAndProveFromProfile`, `scheduleAndProveForcedCut`, `refuseInvalidCheckFree`; test `sc_Lpc006AndSpl011TwoRunLengthsOnePlanAndUnlistedShapeRefusedTest`; mutant `scheduling_m_spl011_unlisted_shape_or_unowned_box` | `MODELED` | Adopted (`sherlock-shard-plan`): schedule is outside `plan_d`, stays on user's owned box, checks leaf shapes against `key_root`, and gives one `claim_pub` per prepared run. |
| `ST-001..008`, `PF-001..008`, `SC-001..008`, `SC-010..014` *(adopted)* | Covered via `inv_STM_001..019`, `inv_ELA_001..006`, `inv_SPL_011` (`ST-007` also modelled in Key Management under `UKS-008`; setup actions here mutate no key/`plan_d`) | `MODELED` | All 30 adopted `SETUP-SPEC.md` items (`ST-001..008`, `PF-001..008`, `SC-001..008`, `SC-010..014`; `SC-009` does not exist in `SETUP-SPEC.md`) are checked through their corresponding `STM`, `ELA`, and `SPL-011` invariants and tests. |

---

## 2. Findings (`Q-scheduling-1` .. `Q-scheduling-6`)

### `Q-scheduling-1` — Multi-GPU boxes break the `calibration` schema, `STM-017` tie-breaking, and `SHERLOCK_GPUS` selection
- **IDs**: `STM-001`, `STM-004`, `STM-006` (`PF-004`), `STM-010`, `STM-011`, `STM-017`
- **Quoted sentences**:
  - `STM-001`: *"When the operator starts a setup run, the Sherlock prover SHALL read the GPU count and, for each GPU, its model name, compute capability, total memory, free memory and maximum SM clock."*
  - `STM-004`: *"When the operator starts a setup run, the Sherlock prover SHALL prove one full-size leaf of each leaf shape of the universal key set on the CPU and on each GPU of a GPU build, with one warm-up proof and `T16_CAL_REPS` timed proofs."*
  - `PF-004` (`SETUP-SPEC.md:23`, carried by `STM-006`): *"Each `calibration` entry shall hold `shape` (the leaf shape index), `device` (`"CPU"` or `"GPU"`), `reps`, `leaf_s`, `app_s`, `host_bytes` and `device_bytes`."*
  - `STM-010`: *"Where the operator sets `SHERLOCK_MEM_BUDGET`, `SHERLOCK_GPUS` or `SHERLOCK_GPU_MEM`, the scheduler SHALL use the lower of each limit and the profile value."*
  - `STM-017`: *"The scheduler SHALL pick the shape and device with the least estimated wall time, then the fewer leaves, then the lower shape index, then the GPU before the CPU."*
- **Witness state**:
  - Consider a 2-GPU host (`gpus = [GPU 0 (32 GB), GPU 1 (16 GB)]`).
    1. `STM-004` calibrates *each GPU*, producing two `"GPU"` calibration entries per shape, yet `PF-004` (`STM-006`) has no `gpu_index` field — only `device in {"CPU", "GPU"}`.
    2. If both GPU entries are stored and tie on `(estWallMs, leaves, shapeIdx)` (e.g., 1 leaf of 4 steps taking 2.0 s on both GPUs, or two identical cards with different free VRAM), `STM-017`'s tie order (*"then the GPU before the CPU"*) does not distinguish GPU `0` from GPU `1`, making the pick depend on calibration list order in violation of `SC-005` / `STM-017`'s scenario!
    3. When `SHERLOCK_GPUS = 1` on a 2-GPU profile (`STM-010`), neither `STM-010` nor `STM-011` states which GPU is retained or whether leaf concurrency when `SHERLOCK_GPUS = 2` sums across GPUs (subject to aggregate host `mem_budget`) or selects one GPU.
- **What would close it**:
  - Add `gpu_index: Option<u32>` (`null` for `"CPU"`, `0..G-1` for `"GPU"`) to `calibration` in `STM-006` / `PF-004`; extend `STM-017`'s tie-breaker with *"then the lower `gpu_index`"* after `"GPU before CPU"`; and specify in `STM-010` / `STM-011` which GPUs `SHERLOCK_GPUS = k` enables (e.g. indices `0..k-1` or explicit device list) and how multi-GPU concurrency aggregates under the shared host `mem_budget`.

### `Q-scheduling-2` — `STM-015` derives wrap pipeline policy from un-lowered profile GPU memory and is undefined on unified GPUs (`vram_total = null`)
- **IDs**: `STM-015`, `STM-002`, `STM-010`, `ELA-004` (`PF-007`)
- **Quoted sentences**:
  - `STM-015`: *"When the scheduler derives a schedule from a profile, the scheduler SHALL pick the wrap pipeline policy from the largest GPU memory of the profile less 1/16 headroom: full overlap with no eviction, else coset eviction with overlap after the quotient, else no overlap."*
  - `STM-002` Scenario (`muninn`): *"GIVEN muninn with the GB10 GPU WHEN the setup mode runs THEN `unified` is true and `vram_total` is `null`"*
  - `STM-010`: *"Where the operator sets `SHERLOCK_MEM_BUDGET`, `SHERLOCK_GPUS` or `SHERLOCK_GPU_MEM`, the scheduler SHALL use the lower of each limit and the profile value."*
  - `PF-007` (`SETUP-SPEC.md:26`): *"... from the largest GPU memory of the profile (VRAM, or the memory budget for a unified GPU, lowered by any limit) less 1/16 headroom ..."*
- **Witness state**:
  - (a) On `BoxMuninn` (`unified = true`, `vram_total = null` per `STM-002`), *"the largest GPU memory of the profile"* in `STM-015` is `null` unless the unified host `mem_budget` rule from `PF-007` is restored into `STM-015`.
  - (b) On `BoxOdin` (`32 GB` discrete GPU profile), suppose the operator sets `SHERLOCK_GPUS = 0` (`ELA-004`) or `SHERLOCK_GPU_MEM = 8 GB` (`STM-010`). Following `STM-015` literally (*"from the largest GPU memory of the profile"*), the scheduler still computes `32 GB * 15/16 = 30 GB` and selects `PipeFullOverlap` even though `0` GPUs (or only `8 GB` of GPU memory) are permitted at run time!
- **What would close it**:
  - Amend `STM-015` to match `PF-007` explicitly: compute wrap pipeline policy from the largest *effective* GPU memory among enabled GPUs (`0` -> `no overlap` when `SHERLOCK_GPUS = 0`; `min(vram_total, SHERLOCK_GPU_MEM)` on a discrete GPU; `effective_mem_budget` lowered by `SHERLOCK_MEM_BUDGET` and `SHERLOCK_GPU_MEM` on a unified GPU) less `1/16` headroom.

### `Q-scheduling-3` — Per-box `profile.json` path & per-program `key_root` conflict across `STM-006` (`ST-008`), `STM-008`, `STM-016`, and `SPL-011`
- **IDs**: `STM-006` (`ST-008`), `STM-008`, `STM-016`, `SPL-011`
- **Quoted sentences**:
  - `ST-008` (`SETUP-SPEC.md:16`, carried by `STM-006`): *"When the setup mode completes, the prover shall write the machine profile to `OUT/profile.json`."*
  - `STM-016`: *"Where `T16_PROFILE` is not set, the Sherlock prover SHALL read the box profile from `~/sherlock/setup/profile.json`."*
  - `STM-008`: *"If the `key_root` or `keys_d` of a profile differs from the universal key set of the prover, then the Sherlock prover SHALL record the run as not calibrated for this key set."*
  - `SPL-011`: *"Each leaf shape SHALL be checked against `key_root`..."* (and `DDD.md:144`: `key_root` is per-program, whereas `DDD.md:228`: setup is *"per box"*).
- **Witness state**:
  1. **Write vs read path mismatch**: Operator runs `setup OUT` writing `OUT/profile.json` (`ST-008`), then runs a T23 call with `T16_PROFILE` unset (`STM-016`), which reads `~/sherlock/setup/profile.json` (missing or stale).
  2. **Per-box profile vs per-program `key_root`**: Box `odin` runs setup once under Program 1 (`key_root = 701`), writing `~/sherlock/setup/profile.json`. Next, Program 2 (`d04 withdraw`, `key_root = 802 != 701`) runs on `odin` under the **same** T25 universal key set (`keys_d = 250`). Because `STM-006` stores `key_root` in the single per-box profile and `STM-008` triggers whenever *"`key_root` or `keys_d` of a profile differs"*, Program 2 (and 207 of 208 Compact suite programs) is flagged **not calibrated** (`STM-008` also misstates `key_root` as part of *"the universal key set of the prover"*).
  3. **Refusal vs degraded run unspecified**: Unlike `STM-007`/`STM-012`/`STM-013` (*"SHALL refuse"*), `STM-008` only says *"SHALL record the run as not calibrated"*, leaving unspecified whether production proving refuses to start on a stale-keyset profile or risks OOM by scheduling with stale byte peaks.
- **What would close it**:
  - Have `setup` default to writing `~/sherlock/setup/profile.json` when no override path is given (`STM-006`/`STM-016`); key box calibration by universal `keys_d` + chip-set/bucket shape parameters rather than a single program's `key_root` (or store per-bucket calibration so one box profile serves all suite programs); and state in `STM-008` whether an uncalibrated profile is refused in non-test runs.

### `Q-scheduling-4` — `STM-007` allows zero/negative `host_bytes` and `device_bytes`, causing division-by-zero or false admission in `STM-011` and `STM-012`
- **IDs**: `STM-007`, `STM-011`, `STM-012` (`PF-005`, `ST-006`)
- **Quoted sentences**:
  - `STM-007`: *"If a profile has another `format` value, lacks a field or has a `leaf_s` that is not a finite positive time, then the Sherlock prover SHALL refuse the profile."*
  - `ST-006` (`SETUP-SPEC.md:14`, carried by `STM-004`): *"... and on a GPU the device bytes: the GPU free bytes just before the first GPU proof of the process minus the least free bytes after a proof of this shape ... less the process's VmRSS growth on a unified GPU."*
  - `STM-011`: *"The scheduler SHALL set the leaf concurrency to the number of leaves whose measured device bytes fit the free memory of the device and whose measured host bytes fit the host memory budget, with at most max(1, cores / 4) leaves on the CPU."*
  - `STM-012`: *"If no leaf shape fits the memory budget on any device, then the scheduler SHALL refuse the run and name the reason."*
- **Witness state**:
  - Take a profile with `format = "sherlock-profile/1"`, all 11 fields present, `cores = 8`, `leaf_s = 2.0 > 0`, and `host_bytes = 0` (or on a unified GPU under `ST-006` on a loaded host, `device_bytes <= 0` when `VmRSS` growth exceeds the free-memory delta). `STM-007` **accepts** this profile. Under `SHERLOCK_MEM_BUDGET = 1 GB` (`STM-012` scenario), `STM-011` either divides `mem_budget / 0` (panic/undefined) or finds that `2 * 0 <= 1 GB` fits on CPU (`max(1, 8/4) = 2` leaves), admitting a `1 GB` run instead of refusing it under `STM-012`!
- **What would close it**:
  - Strengthen `STM-007` (`PF-005`) to refuse any profile with `cores < 1`, `mem_budget <= 0`, `reps < 1`, `host_bytes <= 0`, `device_bytes <= 0` on a discrete GPU entry, or `host_bytes + device_bytes <= 0` on a unified GPU entry (and clamp/refuse noisy non-positive deltas during setup in `ST-006`).

### `Q-scheduling-5` — `STM-004` and `STM-013` check leaf-shape keys against "the universal key set" instead of per-program `key_root`, and `"cover the run"` leaves overshoot (`sum(cut) > n`) ambiguous
- **IDs**: `STM-004`, `STM-013`, `SPL-011` (`SC-010`, `SC-013`, `SC-014`)
- **Quoted sentences**:
  - `STM-013`: *"If a schedule has a shape with no key in the universal key set, a cut with no leaf, a cut that does not cover the run or a leaf outside 1 to S steps, then the scheduler SHALL refuse the schedule."*
  - `SPL-011`: *"Each leaf shape SHALL be checked against `key_root`..."*
- **Witness state**:
  - Suppose shapes `{0, 1, 2}` exist across programs, Program `P` commits only shapes `{0, 1}` in its `key_root`, and an operator forces `T16_SHAPE = 2` (`STM-013`). Checking against *"the universal key set"* (which holds no per-program leaf shape keys per `DDD.md:143-144, 293`) instead of Program `P`'s `key_root` (`SPL-011`) lets `check_free` accept shape `2`, wasting leaf proving before `N1` refuses the unlisted leaf key (`SC-013`). Similarly, for `runSteps = 5` and shape `S = 4`, a forced cut `[3, 3]` (`sum = 6 > 5`, each leaf in `1..4`) "covers" 5 steps under `sum(cut) >= n` yet fails `N1`/`N2` exact interval linking (`SC-014`).
- **What would close it**:
  - Amend `STM-004` and `STM-013` to check leaf shapes against the program's `key_root` shape menu (`SPL-011`), and state explicitly in `STM-013` that `check_free` requires overflow-free exact partition `sum(cut) == run_steps` (with `run_steps >= 1`).

### `Q-scheduling-6` — `STM-018` omits `ELA-003`'s feasibility precondition, and `ELA-006` omits `SC-014`'s manifest-equality and owned-roster checks on cross-box join
- **IDs**: `STM-018`, `STM-012`, `ELA-003`, `ELA-006` (`SC-014`, `SPL-011`)
- **Quoted sentences**:
  - `STM-018`: *"When the memory budget is lowered, the scheduler SHALL give a pick whose estimated wall time is not lower than the pick at the higher budget."*
  - `ELA-003`: *"While at least one leaf shape fits the lowered memory budget, when the operator lowers the memory budget of a run, the Sherlock prover SHALL prove the run with the leaf concurrency that fits the lower budget."*
  - `ELA-006`: *"When leaves proved on two owned boxes are combined under one N1, the Sherlock prover SHALL check each leaf with the stock verifier before it starts a node proof."*
  - `SC-014` (`SETUP-SPEC.md:46`): *"When leaves proved on different hosts are combined under one N1, the prover shall check that each host's key manifest equals the key set of the combining process and that every leaf passes the stock verifier, and shall stop before any node proof when one check fails..."*
- **Witness state**:
  - (a) Lowering the budget on `BoxHuginn` from `60 GB` to `1 GB` requires `STM-012` to *refuse* the run (no pick exists), contradicting `STM-018`'s unconditional *"SHALL give a pick"*.
  - (b) Combining a valid `T10m` leaf from `BoxHuginn` with a valid `T25` leaf from `BoxOdin` (or a leaf from an unowned host outside the user's roster): each leaf passes the stock verifier under its own VK (`ELA-006`), so without `SC-014`'s manifest-equality check and `SPL-011`'s owned-box roster check, `join` starts `N1` instead of stopping beforehand.
- **What would close it**:
  - Prefix `STM-018` with `ELA-003`'s `While` clause (*"While at least one leaf shape fits the lowered memory budget, ..."*), and restore `SC-014`'s manifest-equality check, `h`-interval continuity check, and owned-box roster refusal into `ELA-006`.

---

## 3. Mutant Table

Each of the 22 refusal/binding guards has a dedicated mutant module `scheduling_m_<name>` (`MUTANT_ID = 1..22`). Under `quint run` (`--max-samples=1000 --max-steps=15 --seed=0x20261115`), all 22 mutants exited `1` (`[violation]`) on their named invariant, and all 22 also violated `reach_two_run_lengths_one_plan` (`exit 1`), confirming that the honest path stays live and no mutant kill is vacuous.

| Mutant Module | `MUTANT_ID` | Guard / Binding Removed or Weakened | Target Invariant | Observed Exit Code | Non-Vacuity (`reach_two_run_lengths_one_plan`) |
|---|---:|---|---|---:|---:|
| `scheduling_m_stm002_unified_not_marked` | `1` | Records `unified = false` on a GPU with `vramTotalMb == -1` (`null`) | `inv_STM_002` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm003_ignore_cgroup_cap` | `2` | Uses `memAvailableGb` (`64`) instead of `min(64, cgroup 60)` during setup | `inv_STM_003` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm005_write_on_bad_cal_proof` | `3` | Writes `profile.json` even when `abortSetupBadCalProof` rejects a calibration proof | `inv_STM_005` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm007_accept_zero_leaf_s` | `4` | Schedules runs from malformed profiles (`leafMs == 0`, missing field, or bad `format`) | `inv_STM_007` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm008_ignore_keyset_mismatch` | `5` | Marks `calibratedForKeySet = true` when profile `keys_d` (`T10m`) differs from prover `keys_d` (`T25`) | `inv_STM_008` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm010_limit_raises_budget` | `6` | Applies `SHERLOCK_MEM_BUDGET = 80 GB` directly above `60 GB` profile instead of `min(80, 60)` | `inv_STM_010` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm011_ignore_host_cap_on_gpu` | `7` | Computes discrete GPU concurrency from VRAM alone, ignoring host `effectiveMemBudgetGb` | `inv_STM_011` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm012_schedule_zero_fit` | `8` | Forces CPU concurrency `1` when `0` leaves fit in `effectiveMemBudgetGb` | `inv_STM_012` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm013_bypass_check_free` | `9` | Bypasses `check_free`, accepting a forced cut covering `3 != 5` steps | `inv_STM_013` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm014_legacy_menu_check` | `10` | Enables legacy schedule menu flag (`keyRootHasScheduleMenu = true`) | `inv_STM_014` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm015_wrong_wrap_policy` | `11` | Assigns `PipeFullOverlap` regardless of GPU memory / `1/16` headroom | `inv_STM_015` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm016_wrong_default_path` | `12` | Reads `ProfilePathWrongFallback` instead of `~/sherlock/setup/profile.json` when `T16_PROFILE` is unset | `inv_STM_016` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm017_wrong_tie_order` | `13` | Breaks equal-time ties by lower `shapeIdx` / CPU before fewer leaves (`BoxOdin` 5-step tie) | `inv_STM_017` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm018_lower_budget_faster` | `14` | Underestimates `estWallMs` on lowered budgets so lower budget reports faster time than full budget | `inv_STM_018` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_stm019_admit_early_wrap_short` | `15` | Admits early wrap start at `freeDeviceMb == requiredMb - 1` | `inv_STM_019` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_ela001_schedule_changes_claim` | `16` | Perturbs `claimPub` by `cutLeafCount`, breaking schedule invariance of `claim_pub` | `inv_ELA_001` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_ela002_device_specific_l_key` | `17` | Uses a distinct `L` key (`keysD + 99`) when proving on `DevCpu` vs GPU | `inv_ELA_002` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_ela004_gpu_used_when_hidden` | `18` | Allows GPU scheduling when `SHERLOCK_GPUS = 0` (`effectiveGpuCount == 0`) | `inv_ELA_004` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_ela006_skip_leaf_verify_on_join` | `19` | Starts `N1` join over cross-box leaves without rejecting a corrupted leaf | `inv_ELA_006` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_ela007_incomplete_run_record` | `20` | Completes an elastic run with `recordWritten = false` and zeroed stage metrics | `inv_ELA_007` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_lpc006_longer_run_mutates_prog_d` | `21` | Mutates `progD` (`+ 77`) on the longer 12-step run under the same `plan_d` | `inv_LPC_006` | `1` (`[violation]`) | `1` (live) |
| `scheduling_m_spl011_unlisted_shape_or_unowned_box` | `22` | Accepts unlisted Shape `2` (outside `key_root`) on an unowned host | `inv_SPL_011` | `1` (`[violation]`) | `1` (live) |

---

## 4. State Vocabulary for Replay

### 4.1 Top-Level State Variables (`var`)
In ITF JSON traces (`../traces/scheduling_main/t_{seq}.itf.json`), Quint qualifies variables under `scheduling_main::scheduling::<var>`:

| Quint `var` | ITF JSON Key | Quint Type | Meaning |
|---|---|---|---|
| `sched` | `scheduling_main::scheduling::sched` | `SchedulingState` | Cohesive observable state of the Scheduling bounded context (box profiles, proved runs, uncalibrated records, early-wrap guard checks, cross-box joins, and latest refusal). |
| `lastOp` | `scheduling_main::scheduling::lastOp` | `str` | Action name that produced the state (`"init"` at step `0`, exact action identifier on every transition). |

#### Fields of `sched: SchedulingState`
- `proverKeyEpoch: int`, `proverKeysD: int` (`250` = T25), `proverKeyRoot: int` (`701`) — Active prover keyset identity (`STM-008`, `ELA-002`).
- `keyRootHasScheduleMenu: bool` — `false` in all honest instances (`STM-014`, `SPL-011`).
- `profiles: BoxId -> BoxProfileRec` — Map over `BoxHuginn`, `BoxMuninn`, `BoxOdin` holding `written`, `atDefaultBoxPath`, `formatTag` (`FormatNone | FormatSherlockProfileV1 | FormatBadVersion`), `fieldCount`, `hostBox`, `keyRoot`, `keysD`, `gpuEntries: List[GpuProfileEntry]`, `cpuModelCode`, `cores`, `ramTotalGb`, `memBudgetGb`, `diskFreeGb`, `calEntries: List[CalEntry]`, `setupSec`, and `allCalProofsVerified` (`STM-001..008`, `STM-016`).
- `provedRuns: Set[ScheduledRunRec]` — Completed scheduled/proved run records with profile/prover key digests, `calibratedForKeySet`, `progId`, `inputId`, `planD`, `progD`, `runSteps`, `runSalt`, operator limit overrides and effective caps (`effectiveMemBudgetGb`, `effectiveGpuCount`, `effectiveGpuMemGb`), chosen `shapeIdx`, `shapeStepCap`, `shapeInKeyRoot`, `devKind` (`DevDiscreteGpu | DevUnifiedGpu | DevCpu`), `hostGbPerLeaf`, `deviceGbPerLeaf`, `concurrency`, cut metrics (`cutLeafCount`, `cutTotalSteps`, `minLeafSteps`, `maxLeafSteps`), `forcedOverride`, `overrideNamed`, `usedLegacyMenuCheck`, `estWallMs`, `wrapPolicy` (`PipeFullOverlap | PipeCosetEvict | PipeNoOverlap`), `scheduleStaysLocal`, `usedKeySetTuple`, `recordWritten`, `shardCount`, `leafStageTimeMs`, `nodeStageTimeMs`, `peakHostMemGb`, `lStockVerified`, `nodeAccepted`, and `claimPub` (`STM-009..018`, `ELA-001..005`, `ELA-007`, `LPC-006`, `SPL-011`).
- `uncalibratedRecords: Set[(BoxId, PreparedRunId, bool)]` — Recorded keyset-mismatch calibration checks `(box, prepRun, calibratedFlag)` (`STM-008`).
- `earlyWrapChecks: Set[EarlyWrapCheckRec]` — Runtime device-guard checks `{ boxId, wrapPolicy, freeDeviceMb, requiredMb, earlyWrapAdmitted, startedAfterStarkStage }` (`STM-019`).
- `completedJoins: Set[CrossBoxJoinRec]` — Cross-box `N1` joins `{ boxA, boxB, bothBoxesOwned, manifestsEqualCombiner, allLeavesStockVerified, intervalsContiguous, nodeProofStarted, claimPub }` (`ELA-006`).
- `lastRefusal: RefusalReason` — Latest refusal diagnostic (`SchedRefusalNone`, `SchedRefusalBadCalibrationProof`, `SchedRefusalMalformedProfile`, `SchedRefusalUncalibratedKeySet`, `SchedRefusalNoShapeFitsBudget`, `SchedRefusalCheckFreeFailed`, `SchedRefusalEarlyWrapShort`, `SchedRefusalJoinCorruptedLeaf`, `SchedRefusalUnownedBox`).

### 4.2 `lastOp` Values and Implemented Requirements

| `lastOp` Value | Requirement ID(s) | Effect Summary |
|---|---|---|
| `"init"` | All (`STM-001..019`, `ELA-001..007`, `LPC-006`, `SPL-011`) | Initializes T25 prover keyset (`keysD = 250`, `keyRoot = 701`), empty box profiles on `huginn`, `muninn`, `odin`, and empty run/join sets. |
| `"runSetupOnBox"` | `STM-001`, `STM-002`, `STM-003`, `STM-004`, `STM-006` (`ST-001..004`, `ST-006..008`, `PF-001..004`) | Probes GPUs & host (with cgroup minimum and unified-GPU detection), runs warm-up + `DEFAULT_CAL_REPS` calibrations, and writes 11-field `sherlock-profile/1` at default box path. |
| `"abortSetupBadCalProof"` | `STM-005` (`ST-005`) | Stops setup on a verifier-refused calibration proof, leaving `written = false` and setting `SchedRefusalBadCalibrationProof(b)`. |
| `"injectMalformedProfile"` | `STM-007` (`PF-005`) | Precondition helper writing a corrupted profile (`BadFormatTag`, `BadMissingField`, or `BadZeroLeafTime`) on box `b`. |
| `"refuseMalformedProfile"` | `STM-007` (`PF-005`) | Refuses a malformed profile on box `b` and records `SchedRefusalMalformedProfile((b, kind))`. |
| `"installStaleT10mProfile"` | `STM-008` (`PF-006`) | Precondition helper installing a valid pre-rekey `T10m` profile (`keysD = 100`) on box `b`. |
| `"recordUncalibratedProfileKeyMismatch"` | `STM-008` (`PF-006`) | Detects `profile.keysD != proverKeysD` (or `keyRoot` mismatch), records `(b, pr, false)` in `uncalibratedRecords`, and sets `SchedRefusalUncalibratedKeySet`. |
| `"scheduleAndProveFromProfile"` | `STM-009..011`, `STM-013..018`, `ELA-001..005`, `ELA-007`, `LPC-006`, `SPL-011` | Applies operator limits (`min`), computes device concurrency, selects optimal `(shape, device)` by `STM-017`, derives wrap policy (`STM-015`), verifies `check_free`, proves, and writes run record. |
| `"refuseUnfittableBudget"` | `STM-012`, `ELA-003` (`SC-006`) | Refuses a run under `1 GB` budget before any proof when no shape fits on any device; sets `SchedRefusalNoShapeFitsBudget`. |
| `"scheduleAndProveForcedCut"` | `STM-013`, `STM-014`, `ELA-001`, `ELA-007`, `SPL-011` (`SC-008`, `SC-010`) | Validates an operator-forced cut (`T16_CUT = 1,1,1,1,1`) via `check_free`, names the override, proves, and writes run record. |
| `"refuseInvalidCheckFree"` | `STM-013`, `SPL-011` (`SC-010`) | Refuses a schedule whose shape is outside `key_root`, has 0 leaves, fails to cover `runSteps`, or exceeds `shapeStepCap`. |
| `"evaluateEarlyWrapGuard"` | `STM-019` (`PF-008`) | Evaluates `freeDeviceMb >= STARK_EXTRA_MB + wrapTail + WRAP_MARGIN_MB`; admits early wrap start or refuses and defers wrap until after STARK stage. |
| `"joinLeavesTwoBoxes"` | `ELA-006` (`SC-014`) | Verifies all leaves from two owned boxes with the stock verifier (plus manifest equality and contiguous `h` intervals) and combines them under `N1`. |
| `"refuseJoinCorruptedLeaf"` | `ELA-006` (`SC-014`) | Stops cross-box join before `N1` when a leaf fails stock verification and names `(boxA, boxB, badLeafIdx)`. |

### 4.3 Instance Constants

| Constant | `scheduling_main` | `scheduling_edge_min` | `scheduling_edge_at_limit` | `scheduling_m_*` | Meaning |
|---|---:|---:|---:|---:|---|
| `PROFILE_FIELD_COUNT` | `11` | `11` | `11` | `11` | Required JSON fields in `sherlock-profile/1` (`STM-006`) |
| `GPU_ENTRY_FIELDS_REQUIRED` | `7` | `7` | `7` | `7` | Required fields per `gpus` entry (`STM-001`) |
| `WARMUP_REPS` | `1` | `1` | `1` | `1` | Warm-up proofs per calibrated shape/device (`STM-004`) |
| `DEFAULT_CAL_REPS` | `3` | `1` | `3` | `3` | Timed calibration repetitions (`T16_CAL_REPS`, `STM-004`) |
| `CPU_CORES_PER_LEAF` | `4` | `4` | `4` | `4` | CPU core divisor `max(1, cores / 4)` (`STM-011`) |
| `MAX_LEAVES_BOUND` | `16777216` | `16777216` | `5` | `16777216` | `MAX_LEAVES` cut bound (`2^24` vs exact `5` at forced `[1,1,1,1,1]`) |
| `PIPE_HEADROOM_DIV` | `16` | `16` | `16` | `16` | `1/16` GPU headroom divisor for wrap pipeline (`STM-015`) |
| `FULL_OVERLAP_MIN_MB` | `24000` | `24000` | `30720` | `24000` | MiB needed after `1/16` headroom for `PipeFullOverlap` (`30720` = exact `32768 * 15/16`) |
| `EVICT_OVERLAP_MIN_MB` | `14000` | `14000` | `15360` | `14000` | MiB needed after `1/16` headroom for `PipeCosetEvict` (`15360` = exact `16384 * 15/16`) |
| `STARK_EXTRA_MB` | `6000` | `6000` | `6000` | `6000` | STARK extra device MiB (`STM-019`) |
| `WRAP_PEAK_MB` | `18000` | `18000` | `18000` | `18000` | Wrap peak device MiB (`STM-019`) |
| `RESIDENT_COSETS_MB` | `7500` | `7500` | `7500` | `7500` | Resident coset MiB subtracted under `PipeFullOverlap` (`STM-019`) |
| `WRAP_MARGIN_MB` | `1000` | `1` | `1000` | `1000` | Device guard margin MiB (`STM-019`) |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 22` | `0` = sound; `1..22` = single-guard/binding mutant |

---

## 5. Model Abstractions

1. **JSON Serialization & Timings**: `sherlock-profile/1` JSON formatting (`STM-006`) and floating-point seconds (`leaf_s`, `app_s`, `setup_s`) are abstracted to typed records (`BoxProfileRec`) with `fieldCount: int` and integer milliseconds (`leafMs`, `estWallMs`), avoiding floating-point nondeterminism while preserving strict positivity (`leafMs > 0`) and exact tie comparisons (`STM-007`, `STM-017`).
2. **Memory Units and `null` VRAM**: Host and leaf memory footprints use integer GiB (`hostGb`, `deviceGb`, `memBudgetGb`), wrap pipeline and early-wrap device-guard arithmetic uses integer MiB (`32768`, `16384`, `reqMb - 1` for the one-unit-short test in `STM-019`), and JSON `null` for `vram_total`/`vram_free` on unified GPUs (`STM-002`) is represented by `-1`.
3. **Fleet Boxes, Shapes, and Prepared Runs**: Hardware diversity is represented by the three concrete fleet archetypes (`BoxHuginn`: 16 GB discrete + 60 GB cgroup cap; `BoxMuninn`: GB10 unified 100 GB pool; `BoxOdin`: 32 GB discrete 128 GB RAM), three leaf shapes (`0` = 4 steps, `1` = 8 steps in `key_root`, plus `2` = 16 steps unlisted in `key_root`), and three prepared runs (`PrepRunA_Short` = 5 steps, `PrepRunB_Long` = 12 steps under the same `plan_d`, and `PrepRunC_FreshSalt` = 5 steps with a distinct salt).
4. **Cryptographic Proofs, Digests, and `claim_pub`**: STARK leaf/node/`L` proofs, `key_root`, `keys_d`, `plan_d`, `prog_d`, salts, and `claim_pub` are modelled as injective positive integers and verifier booleans (`allCalProofsVerified`, `lStockVerified`, `allLeavesStockVerified`).
5. **Cut Representation**: Rather than allocating arbitrary-length `List[int]` cuts up to `2^24` elements, each cut is summarized by `(cutLeafCount, cutTotalSteps, minLeafSteps, maxLeafSteps)`, which suffices to evaluate `check_free` (`STM-013`) and wave count (`ceilDiv(cutLeafCount, concurrency)` in `STM-017`) exactly.

---

## 6. Commands

Run every command below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/scheduling/quint`:

```bash
# 1. Typecheck model and test suite
quint typecheck scheduling.qnt
# Expected exit code: 0
# Printed line: (no output on success)

quint typecheck scheduling_test.qnt
# Expected exit code: 0
# Printed line: (no output on success)

# 2. Requirement scenario unit tests (20 tests)
quint test --main=scheduling_test --match='sc_.*Test' --seed=0x20261115 scheduling_test.qnt
# Expected exit code: 0
# Printed line: 20 passing (14148ms)

# 3. Main and edge instance safety invariant checks
quint run --main=scheduling_main --invariant=inv --max-samples=2000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (21759ms at 92 traces/second).

quint run --main=scheduling_edge_min --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (14417ms at 69 traces/second).

quint run --main=scheduling_edge_at_limit --invariant=inv --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 0
# Printed line: [ok] No violation found (12493ms at 80 traces/second).

# 4. Non-vacuity reachability witnesses on main instance (negated goals: all MUST exit 1)
quint run --main=scheduling_main --invariant=reach_three_boxes_profiled_and_proved --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (361ms at 30 traces/second).

quint run --main=scheduling_main --invariant=reach_forced_cut_and_profile_cut_same_claim --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (310ms at 3 traces/second).

quint run --main=scheduling_main --invariant=reach_two_run_lengths_one_plan --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (509ms at 6 traces/second).

quint run --main=scheduling_main --invariant=reach_cpu_fallback_when_no_gpu --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (263ms at 4 traces/second).

quint run --main=scheduling_main --invariant=reach_cross_box_join_completed --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (664ms at 2 traces/second).

quint run --main=scheduling_main --invariant=reach_early_wrap_refused_and_admitted --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (1505ms at 11 traces/second).

quint run --main=scheduling_main --invariant=reach_budget_or_check_free_refused --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (629ms at 2 traces/second).

# 5. Guard and binding mutants (all 22 MUST exit 1)
quint run --main=scheduling_m_stm002_unified_not_marked --invariant=inv_STM_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (877ms at 1 traces/second).

quint run --main=scheduling_m_stm003_ignore_cgroup_cap --invariant=inv_STM_003 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (324ms at 3 traces/second).

quint run --main=scheduling_m_stm005_write_on_bad_cal_proof --invariant=inv_STM_005 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (793ms at 10 traces/second).

quint run --main=scheduling_m_stm007_accept_zero_leaf_s --invariant=inv_STM_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (947ms at 49 traces/second).

quint run --main=scheduling_m_stm008_ignore_keyset_mismatch --invariant=inv_STM_008 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (330ms at 9 traces/second).

quint run --main=scheduling_m_stm010_limit_raises_budget --invariant=inv_STM_010 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (783ms at 6 traces/second).

quint run --main=scheduling_m_stm011_ignore_host_cap_on_gpu --invariant=inv_STM_011 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (716ms at 6 traces/second).

quint run --main=scheduling_m_stm012_schedule_zero_fit --invariant=inv_STM_012 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (806ms at 2 traces/second).

quint run --main=scheduling_m_stm013_bypass_check_free --invariant=inv_STM_013 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (588ms at 2 traces/second).

quint run --main=scheduling_m_stm014_legacy_menu_check --invariant=inv_STM_014 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (677ms at 1 traces/second).

quint run --main=scheduling_m_stm015_wrong_wrap_policy --invariant=inv_STM_015 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (324ms at 3 traces/second).

quint run --main=scheduling_m_stm016_wrong_default_path --invariant=inv_STM_016 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (374ms at 3 traces/second).

quint run --main=scheduling_m_stm017_wrong_tie_order --invariant=inv_STM_017 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (336ms at 6 traces/second).

quint run --main=scheduling_m_stm018_lower_budget_faster --invariant=inv_STM_018 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (923ms at 2 traces/second).

quint run --main=scheduling_m_stm019_admit_early_wrap_short --invariant=inv_STM_019 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (786ms at 1 traces/second).

quint run --main=scheduling_m_ela001_schedule_changes_claim --invariant=inv_ELA_001 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (365ms at 3 traces/second).

quint run --main=scheduling_m_ela002_device_specific_l_key --invariant=inv_ELA_002 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (450ms at 2 traces/second).

quint run --main=scheduling_m_ela004_gpu_used_when_hidden --invariant=inv_ELA_004 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (802ms at 1 traces/second).

quint run --main=scheduling_m_ela006_skip_leaf_verify_on_join --invariant=inv_ELA_006 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (960ms at 1 traces/second).

quint run --main=scheduling_m_ela007_incomplete_run_record --invariant=inv_ELA_007 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (455ms at 2 traces/second).

quint run --main=scheduling_m_lpc006_longer_run_mutates_prog_d --invariant=inv_LPC_006 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (316ms at 9 traces/second).

quint run --main=scheduling_m_spl011_unlisted_shape_or_unowned_box --invariant=inv_SPL_011 --max-samples=1000 --max-steps=15 --seed=0x20261115 --verbosity=1 scheduling.qnt
# Expected exit code: 1
# Printed line: [violation] Found an issue (259ms at 4 traces/second).

# 6. Generate 20 ITF replay traces
quint run scheduling.qnt --main scheduling_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/scheduling/traces/scheduling_main/t_{seq}.itf.json'
# Expected exit code: 0
# Printed line: [ok] No violation found (1983ms at 10 traces/second).
```
