## ADDED Requirements

### Requirement: STM-001 Setup reads the GPUs
STM-001: When the operator starts a setup run, the Sherlock prover SHALL read the GPU count and, for each GPU, its model name, compute capability, total memory, free memory and maximum SM clock.

Verification: `t7_chain t16 setup OUT` on each box (openvm `t16-elastic` 1baf605 `crates/sherlock-wrap-export/examples/common/t16.rs:815`); profile records in Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/records-cde/`. Exists.

#### Scenario: huginn
- **GIVEN** huginn with one RTX 5080
- **WHEN** the setup mode runs
- **THEN** the profile has one `gpus` entry with 7 fields

#### Scenario: Status of STM-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED on huginn, muninn and odin (Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:72`). Evidence: Carries ST-001. openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:184` (`gpus`, one `nvidia-smi --query-gpu` call), parser at `profile.rs:161`.

### Requirement: STM-002 A GPU without own memory is unified
STM-002: When a setup run finds a GPU that reports no own memory, the Sherlock prover SHALL record that GPU as unified.

Verification: test `unified_pool_counts_device_and_host_bytes_once` (openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:544`); the muninn profile record. Exists.

#### Scenario: muninn
- **GIVEN** muninn with the GB10 GPU
- **WHEN** the setup mode runs
- **THEN** `unified` is true and `vram_total` is `null`

#### Scenario: Status of STM-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (muninn profile MEASURED on a loaded host, LOOP T16). Evidence: Carries ST-002. openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:23` to `:25`; `crates/sherlock-leaf/src/sched.rs:28` (GB10 class).

### Requirement: STM-003 Setup reads the host
STM-003: When the operator starts a setup run, the Sherlock prover SHALL read the CPU model, the usable core count, the total RAM, the free disk space of the output directory and, as the memory budget, the lower of `MemAvailable` and the cgroup headroom of the prover process.

Verification: the profile records of the three boxes. NEW: a unit test of the probe with (available, cgroup) values in both orders and with no cgroup limit. Partly to build.

#### Scenario: A cgroup limit below MemAvailable
- **GIVEN** `MemAvailable` of 100 GB and a cgroup headroom of 60 GB
- **WHEN** the setup run reads the budget
- **THEN** `mem_budget` is 60 GB

#### Scenario: No cgroup limit
- **GIVEN** `MemAvailable` of 100 GB and no cgroup limit
- **WHEN** the setup run reads the budget
- **THEN** `mem_budget` is 100 GB

#### Scenario: Status of STM-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. The probe test is a gap (Astra-B review R-17). Owner lane: t16-elastic. Evidence: Carries ST-003. openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:188`, `:197`, `:205`; `crates/sherlock-leaf/src/sched.rs:100` (`MemAvailable`), `sched.rs:172` (`Hardware::probe`), `sched.rs:177` (the minimum).

### Requirement: STM-004 Setup calibrates each leaf shape
STM-004: When the operator starts a setup run, the Sherlock prover SHALL prove one full-size leaf of each leaf shape of the universal key set on the CPU and on each GPU of a GPU build, with one warm-up proof and `T16_CAL_REPS` timed proofs.

Verification: the `calibration` entries of the three profile records (setup 117.3 s, 884 s, 158 s; see the status). Exists.

#### Scenario: Default repetitions
- **GIVEN** `T16_CAL_REPS` unset
- **WHEN** setup calibrates one shape on one GPU
- **THEN** the entry has `reps` 3 and a median `leaf_s`

#### Scenario: Status of STM-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED for boxes with one GPU (Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:72` to Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:80`). OPEN: calibration of each GPU of a multi-GPU box. The code loops the device kinds CPU and GPU with one GPU engine (openvm `t16-elastic` 1baf605 `t16.rs:689` to `:706`, `crates/sherlock-leaf/src/harness.rs:149`); a calibration entry has no device index. Astra-B review R-18. Owner lane: t16-elastic. Evidence: Carries ST-004 and ST-006. openvm `t16-elastic` 1baf605 `crates/sherlock-wrap-export/examples/common/t16.rs:670` (setup writer), `t16.rs:690` (GPU only with feature `cuda` and a GPU present).

### Requirement: STM-005 A refused calibration proof stops setup
STM-005: If the stock verifier refuses a calibration proof, then the Sherlock prover SHALL stop the setup mode and write no profile.

Verification: NEW: a setup run with a corrupted calibration proof (fault injection in the T16 runner). To build.

#### Scenario: A bad calibration proof
- **GIVEN** a calibration proof with one flipped byte
- **WHEN** the stock verifier checks it
- **THEN** setup stops and `profile.json` does not exist

#### Scenario: Status of STM-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: the check is in the code path; no negative run is on file. Owner lane: t16-elastic. Evidence: Carries ST-005 (`SETUP-SPEC.md:13`).

### Requirement: STM-006 The profile format
STM-006: When a setup run finishes, the Sherlock prover SHALL write one JSON profile with the fields `format` = `sherlock-profile/1`, `host`, `created`, `key_root`, `keys_d`, `gpus`, `cpu`, `memory`, `disk`, `calibration` and `setup_s`.

Verification: test `profile_json_round_trip` (openvm `t16-elastic` 1baf605 `profile.rs:650`). Exists.

#### Scenario: Round trip
- **GIVEN** a profile written by setup
- **WHEN** the prover reads it back
- **THEN** the 11 fields are equal

#### Scenario: Status of STM-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: Carries ST-008 and PF-001 to PF-004. openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:21` (`FORMAT`), `profile.rs:79` (`to_json`), `t16.rs:754` (file write).

### Requirement: STM-007 A bad profile is refused
STM-007: If a profile has another `format` value, lacks a field or has a `leaf_s` that is not a finite positive time, then the Sherlock prover SHALL refuse the profile.

Verification: the counterexample tests `cx_f1` to `cx_f9` of openvm `t16-elastic` 1baf605 `profile.rs:575` to `:632`; lead Quint rerun 29 of 29 (Moriarty `qa-carbon` 463f44c72 `tests/quint-regressions/t16/COUNTEREXAMPLES.md:91`). Exists.

#### Scenario: A zero leaf time
- **GIVEN** a profile with `leaf_s` = 0
- **WHEN** the prover reads the profile
- **THEN** the prover refuses it and names the field

#### Scenario: Status of STM-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (LOOP T16: F-1, F-4 to F-8, F-10 CONFIRMED and fixed). Evidence: Carries PF-005. openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:93` (`from_json`).

### Requirement: STM-008 A profile of another key set is not a calibration
STM-008: If the `key_root` or `keys_d` of a profile differs from the universal key set of the prover, then the Sherlock prover SHALL record the run as not calibrated for this key set.

Verification: the `hw` run record case `hw_profile_key_set_is_this_key_set`. After T25 every box profile is stale: rerun setup on the three boxes. Exists.

#### Scenario: The profile after the rekey
- **GIVEN** a T10m profile and the T25 keys
- **WHEN** a run starts
- **THEN** the case fails and the record reads not calibrated

#### Scenario: Status of STM-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys. OPEN for T25: three setup reruns. Owner lane: t16-elastic. Evidence: Carries PF-006 (case `hw_profile_key_set_is_this_key_set`, `SETUP-SPEC.md:25`).

### Requirement: STM-009 The schedule comes from the profile
STM-009: When a run starts with a profile, the scheduler SHALL derive the leaf shape, the device, the leaf concurrency and the cut from the profile, the run length and the run-time limits.

Verification: tests `faster_device_wins_and_cut_is_fewest_leaves` (openvm `t16-elastic` 1baf605 `profile.rs:520`) and `no_gpu_and_memory_cap_scale_down` (`profile.rs:531`); the `hw` runs of Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md`. Exists.

#### Scenario: A run of 3 leaf sizes
- **GIVEN** a profile and a run of `3 S` steps
- **WHEN** the scheduler runs
- **THEN** the cut has 3 leaves of 1 to S steps each

#### Scenario: Status of STM-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED on huginn and odin. Evidence: Carries SC-001 and SC-003. openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:238` (`schedule`), `profile.rs:231` (`MAX_LEAVES`).

### Requirement: STM-010 Operator limits only lower the profile values
STM-010: Where the operator sets `SHERLOCK_MEM_BUDGET`, `SHERLOCK_GPUS` or `SHERLOCK_GPU_MEM`, the scheduler SHALL use the lower of each limit and the profile value.

Verification: runs h-cap and o-cap. NEW: a unit test with a limit above and a limit below the profile value. Partly to build.

#### Scenario: A limit above the profile
- **GIVEN** a profile budget of 60 GB and `SHERLOCK_MEM_BUDGET` of 80 GB
- **WHEN** the scheduler runs
- **THEN** the scheduler uses 60 GB

#### Scenario: Status of STM-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED (Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:72` to Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:80`). Evidence: Carries SC-002. openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:213` (`Limits`), `profile.rs:220` (`from_env`).

### Requirement: STM-011 Leaf concurrency fits the measured memory
STM-011: The scheduler SHALL set the leaf concurrency to the number of leaves whose measured device bytes fit the free memory of the device and whose measured host bytes fit the host memory budget, with at most max(1, cores / 4) leaves on the CPU.

Verification: tests `unified_pool_counts_device_and_host_bytes_once` and `cx_f6_mixed_gpus_keep_each_discrete_gpu_in_its_vram` (`profile.rs:544`, `:600`). NEW: the VRAM-for-8, RAM-for-2 case. Partly to build.

#### Scenario: A unified GPU
- **GIVEN** muninn with one shared memory pool
- **WHEN** the scheduler counts a leaf
- **THEN** the device bytes and the host bytes are both taken from the one pool

#### Scenario: Much VRAM and little RAM
- **GIVEN** a discrete GPU with VRAM for 8 leaves and a host budget for 2
- **WHEN** the scheduler sets the concurrency
- **THEN** the concurrency is 2

#### Scenario: Status of STM-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: Carries SC-004. openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:227` (`CPU_THREADS_PER_LEAF = 4`), `profile.rs:293` to `:298` (the host cap on a discrete GPU).

### Requirement: STM-012 A run that fits nothing is refused
STM-012: If no leaf shape fits the memory budget on any device, then the scheduler SHALL refuse the run and name the reason.

Verification: NEW: a unit test with a budget below one leaf on every device. To build.

#### Scenario: A 1 GB budget
- **GIVEN** `SHERLOCK_MEM_BUDGET` = 1 GB
- **WHEN** the scheduler runs
- **THEN** the run stops before any proof with the reason

#### Scenario: Status of STM-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (the refusal). The named test is a gap. Owner lane: t16-elastic. Evidence: Carries SC-006. openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:333`.

### Requirement: STM-013 Only check_free limits a schedule
STM-013: If a schedule has a shape with no key in the universal key set, a cut with no leaf, a cut that does not cover the run or a leaf outside 1 to S steps, then the scheduler SHALL refuse the schedule.

Verification: test `check_free_has_no_count_list` (openvm `t16-elastic` 1baf605 `profile.rs:555`); forced cut [1,1,1,1,1] proves on huginn and odin (Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:113`). Exists.

#### Scenario: A forced cut of five leaves
- **GIVEN** `T16_CUT` = 1,1,1,1,1
- **WHEN** `check_free` checks the schedule
- **THEN** the schedule passes and the run record names the override

#### Scenario: Status of STM-013
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED (Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:72` to Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:80`). Evidence: Carries SC-008 and SC-010. openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:339` (`check_free`).

### Requirement: STM-014 No schedule menu in the keys or the prover
STM-014: The Sherlock prover SHALL accept a schedule without a list of admitted shard counts and without a menu of schedules.

Verification: NEW: `rg "sched::check|admitted\(" crates/` gives no hit on the T25 tree; then the part (b) run of LPC-006 through `check_free`. To build.

#### Scenario: Part (b) through the free check
- **GIVEN** two run lengths under one `plan_d`
- **WHEN** the runner checks both schedules
- **THEN** both pass `check_free` and no call reaches `sched::check`

#### Scenario: Status of STM-014
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: the code and the result file disagree. Owner lane: t16-elastic. Evidence: Owner ruling F-3 REPLACED (LOOP loop rules). SPL-011 as fixed by the spec audit. T16 still holds the menu code: `crates/sherlock-leaf/src/sched.rs:331` (`admitted`) and `sched.rs:359` (`check`), called by the part (b) runner at `crates/sherlock-wrap-export/examples/common/t16.rs:224`, `:231`, `:246`. Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:13` says the menu code is removed.

### Requirement: STM-015 The wrap pipeline policy comes from the profile
STM-015: When the scheduler derives a schedule from a profile, the scheduler SHALL pick the wrap pipeline policy from the largest GPU memory of the profile less 1/16 headroom: full overlap with no eviction, else coset eviction with overlap after the quotient, else no overlap.

Verification: test `pipeline_from_the_three_boxes` (openvm `t16-elastic` 1baf605 `profile.rs:662`). Exists.

#### Scenario: odin and huginn
- **GIVEN** the profiles of a 32 GB GPU and a 16 GB GPU
- **WHEN** the policy is derived
- **THEN** odin gets full overlap and huginn gets eviction or no overlap

#### Scenario: A small GPU
- **GIVEN** a profile whose largest GPU memory holds neither one STARK call with one wrap nor the evicted form
- **WHEN** the policy is derived
- **THEN** the policy is no overlap

#### Scenario: Status of STM-015
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: Carries PF-007 (`SETUP-SPEC.md:26`). openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:407` (`PIPE_HEADROOM_DIV = 16`), `profile.rs:414` (`pipeline`). Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:3` names PF-001 to PF-006 only.

### Requirement: STM-016 One profile path per box
STM-016: Where `T16_PROFILE` is not set, the Sherlock prover SHALL read the box profile from `~/sherlock/setup/profile.json`.

Verification: NEW: a prover start with no `T16_PROFILE` on each box in the T23 chain runner. To build.

#### Scenario: A T23 call on odin
- **GIVEN** odin with a profile at the picked path and no variable set
- **WHEN** a call starts
- **THEN** the prover loads that profile

#### Scenario: Status of STM-016
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. Owner lane: t23-suite. Evidence: LEAD PICK of LOOP T23 ("box profile path ~/sherlock/setup/profile.json"), confirmed by the owner 2026-11-15 (`~/Projects/Moriarty/llm-wiki/wiki/log.md:4107`). The code writes `OUT/profile.json` (ST-008); the runners use `$O/setup/profile.json` (Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/run-odin.sh:19`, `run-huginn.sh:34`). No code reads the picked path.

### Requirement: STM-017 The pick is the least estimated time, with a fixed tie order
STM-017: The scheduler SHALL pick the shape and device with the least estimated wall time, then the fewer leaves, then the lower shape index, then the GPU before the CPU.

Verification: tests `faster_device_wins_and_cut_is_fewest_leaves` (openvm `t16-elastic` 1baf605 `profile.rs:520`) and `cx_f8_ties_do_not_depend_on_the_calibration_order` (`profile.rs:623`). Exists.

#### Scenario: A tie with reversed entries
- **GIVEN** two calibration entries with equal estimated time, given in both orders
- **WHEN** the scheduler picks
- **THEN** the pick is the same in both orders

#### Scenario: Status of STM-017
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (LOOP T16: F-8 CONFIRMED and fixed). Evidence: Carries SC-005 (`SETUP-SPEC.md:35`). openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:324` to `:326`. Added after Astra-B review R-19.

### Requirement: STM-018 A lower budget never gives a faster pick
STM-018: When the memory budget is lowered, the scheduler SHALL give a pick whose estimated wall time is not lower than the pick at the higher budget.

Verification: test `cx_f9_a_lower_budget_never_gives_a_faster_pick` (openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:632`). Exists.

#### Scenario: A sequence of budgets
- **GIVEN** budgets of 64, 32 and 16 GB on one profile
- **WHEN** the scheduler picks at each
- **THEN** the estimated wall times do not fall

#### Scenario: Status of STM-018
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: Carries SC-007 as amended (`SETUP-SPEC.md:37`; F-9 resolved by amending SC-007, Moriarty `qa-carbon` 463f44c72). For one shape on one device the leaf concurrency does not rise. Added after Astra-B review R-19.

### Requirement: STM-019 The early wrap start is admitted against the profile tail
STM-019: If the free device bytes are below the STARK extra bytes plus the wrap tail plus the margin, then the device guard SHALL refuse the early wrap start.

Verification: test `cx_odin_h70_wrap_tail_counts_resident_cosets` (openvm `t16-elastic` 1baf605 `profile.rs:690`). NEW: a refusal test of `admit_early_wrap` one byte below the sum. Partly to build.

#### Scenario: One byte short
- **GIVEN** free device bytes one below STARK extra + tail + margin
- **WHEN** the guard is asked for an early wrap start
- **THEN** the guard refuses and the wrap starts after the STARK stage

#### Scenario: Status of STM-019
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. The odin test log predates `cx_odin_h70`: rerun. Owner lane: t16-elastic. Evidence: Carries PF-008 (`SETUP-SPEC.md:27`). The wrap tail is the wrap peak less the resident cosets with no eviction, or the whole wrap peak with eviction: openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:446` (`wrap_tail`), `profile.rs:457` (`admit_early_wrap`). Split from STM-015 after Astra-B review R-20, with the response of the Astra-B draft.
