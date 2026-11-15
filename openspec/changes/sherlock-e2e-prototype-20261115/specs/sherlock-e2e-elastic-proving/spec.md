## ADDED Requirements

### Requirement: ELA-001 One claim for every schedule of a prepared run
ELA-001: The Sherlock prover SHALL give a byte-equal `claim_pub` for every schedule of one prepared run of one deployment.

Verification: the `claim_pub` column of Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md` over the runs h-c5, o-c5, h-nogpu, h-cap, o-nogpu, o-cap and the join. Exists.

#### Scenario: Five leaves against one
- **GIVEN** one prepared run cut as [1,1,1,1,1] and cut by the profile
- **WHEN** both are proved
- **THEN** the two `claim_pub` values are byte-equal

#### Scenario: Two fresh runs
- **GIVEN** one program and input prepared twice
- **WHEN** both are proved
- **THEN** the salts differ and the two `claim_pub` values differ

#### Scenario: Status of ELA-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED on huginn and odin. muninn `hw` run: NOT RUN (unit t16-cde-run6). Rerun on the T25 keys. Evidence: Carries SC-012. A prepared run fixes the program, the input, `r_P` and the input salt (ZK-012, ZK-013; openvm `t11-zk` cce8c1d `crates/sherlock-leaf/src/blind.rs:180` to `:187`): a new run draws a new salt and gives another claim; a retry reuses the stored salt of its run (D-077, slice C). Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:119`: lanes 20 and 34 are equal in every run of the three-box table (seeded T10m chain). `research/sherlock-final-design-20261111/ARCHITECTURE.md:215` (`claim_d` excludes `n_steps`). Narrowed after Astra-B review R-21.

### Requirement: ELA-002 One key set for every schedule
ELA-002: The Sherlock prover SHALL use the same app, leaf, N1, N2, N3 and `L` keys for every device, concurrency, cut and host of a run.

Verification: every T16 run record names one key manifest sha256; the stock verifier accepts `L` in each run. Exists.

#### Scenario: CPU and GPU
- **GIVEN** the run on huginn with a GPU and with no GPU
- **WHEN** both `L` proofs are checked
- **THEN** one `L` key verifies both

#### Scenario: Status of ELA-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED on the T10m keys. Rerun on the T25 keys. Evidence: Carries SC-011 (`SETUP-SPEC.md:43`). LOOP T16: "one claim_pub over all, L stock-verified, T10m keys".

### Requirement: ELA-003 A lower memory budget lowers the concurrency
ELA-003: While at least one leaf shape fits the lowered memory budget, when the operator lowers the memory budget of a run, the Sherlock prover SHALL prove the run with the leaf concurrency that fits the lower budget.

Verification: runs h-cap and o-cap of Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md`; equal `claim_pub` (ELA-001). Exists.

#### Scenario: A GPU memory cap of one leaf
- **GIVEN** huginn with `SHERLOCK_GPU_MEM` set to one leaf
- **WHEN** the run is proved
- **THEN** 1 leaf runs at a time and `claim_pub` is unchanged

#### Scenario: A budget below one leaf
- **GIVEN** a budget that holds no leaf on any device
- **WHEN** the run starts
- **THEN** the scheduler refuses the run (STM-012)

#### Scenario: Status of ELA-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED on huginn and odin. muninn: NOT RUN. Evidence: Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:97` to Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:103`: h-cap proves 1 leaf at a time, 4.73 s, host peak 0.78 GB lower; o-cap 6.49 s, host peak 0.58 GB lower.

### Requirement: ELA-004 A box without a GPU proves on the CPU
ELA-004: While at least one leaf shape fits the CPU memory budget, if a box has no usable GPU, then the Sherlock prover SHALL prove the leaves on the CPU.

Verification: runs h-nogpu and o-nogpu; test `no_gpu_and_memory_cap_scale_down` (openvm `t16-elastic` 1baf605 `profile.rs:531`). Exists.

#### Scenario: The GPU is hidden
- **GIVEN** huginn with `SHERLOCK_GPUS` = 0
- **WHEN** the run is proved
- **THEN** the leaf stage runs on the CPU and the stock verifier accepts `L`

#### Scenario: Status of ELA-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED. Evidence: openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:271`; runs h-nogpu (8.81 s) and o-nogpu (10.95 s) at Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:97` to Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:103`.

### Requirement: ELA-005 Each box proves the prepared run with a schedule from its own profile
ELA-005: When muninn, huginn and odin each prove one prepared run with the profile of that box, the Sherlock prover SHALL give the same `claim_pub` on the three boxes.

Verification: the three-box table of Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md` with the muninn `hw` row filled. Exists.

#### Scenario: Unified and discrete memory
- **GIVEN** muninn (GB10) and huginn (RTX 5080) and one prepared run moved between them
- **WHEN** each proves the run
- **THEN** each schedule passes `check_free` and `claim_pub` is byte-equal

#### Scenario: Status of ELA-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL: muninn row open (unit t16-cde-run6 behind the T10q gate). Owner lane: t16-elastic. Evidence: Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:72` to Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:80`: GPU leaf 2.094 s (huginn), 23.729 s (muninn), 3.158 s (odin); leaf stage of the `hw` run 3.97 s (huginn), 4.80 s (odin), muninn NOT RUN.

### Requirement: ELA-006 Leaves from two boxes join under one node
ELA-006: When leaves proved on two owned boxes are combined under one N1, the Sherlock prover SHALL check each leaf with the stock verifier before it starts a node proof.

Verification: `t7_chain t16 join` with leaves of odin and huginn; then muninn with huginn. Exists.

#### Scenario: A corrupted leaf from one box
- **GIVEN** 2 leaves of odin and 1 corrupted leaf of huginn
- **WHEN** the join starts
- **THEN** the prover stops before N1 and names the leaf

#### Scenario: Status of ELA-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED for odin with huginn. The muninn join: NOT RUN. Owner lane: t16-elastic. Evidence: Carries SC-014. openvm `t16-elastic` 1baf605 `crates/sherlock-wrap-export/examples/common/t16.rs:762` (`fn join`), cases at `t16.rs:771`, `:797`, `:799`. Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:66`: odin with huginn, 5 of 5 cases, 283.6 s.

### Requirement: ELA-007 Each elastic run leaves a measured record
ELA-007: When an elastic run completes, the Sherlock prover SHALL write a run record with the box, the schedule, the shard count, the device and the wall time of each stage, the peak host memory, the stock verifier result and `claim_pub`.

Verification: the records of `records-cde/`; `sum.py` beside them builds the table. Exists.

#### Scenario: The forced cut on huginn
- **GIVEN** run h-c5
- **WHEN** the record is written
- **THEN** it reads leaf stage 7.26 s, nodes 20.92 s, `L` accepted

#### Scenario: Status of ELA-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT for the times. GAP: the record does not name the device per stage; Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:52` states that the leaf layer stays on the CPU in a "GPU leaf" run (Astra-B draft G-17). Owner lane: t16-elastic. Evidence: LOOP T16 done-when: "MEASURED wall time and peak memory against shard count and against hardware, every proof verified by the stock verifier, one claim_pub per run". Records: Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/records-cde/`.
