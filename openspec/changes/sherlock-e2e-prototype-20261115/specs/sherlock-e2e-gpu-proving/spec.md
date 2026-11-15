## ADDED Requirements

### Requirement: GPV-001 The app proof runs on the GPU in a cuda build
GPV-001: Where the build has the cargo feature `cuda`, the leaf prover SHALL prove app shards on the GPU through `openvm-cuda-backend`.

Verification: gate `sg6` on huginn and odin for correctness. NEW: the run record names the backend of each proof, and the GPU test asserts it. Partly to build.

#### Scenario: A GPU app proof
- **GIVEN** huginn with the `cuda` build
- **WHEN** an app shard is proved
- **THEN** the record names the CUDA backend and the stock verifier accepts the proof

#### Scenario: Status of GPV-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED (huginn GPU app proof 0.225 s against 3.595 s on the CPU, Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:52`). The device record is a gap. Rerun on the T25 keys. Evidence: openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/harness.rs:145` to `:153` (the app prover, set `frozen::APP`, one engine behind a `Mutex`); openvm `t16-elastic` 1baf605 `t16.rs:706` selects the GPU for the app. Gate `sg6` (Moriarty 3dc9001a9 `research/sherlock-t10-bound-20261114/chain.sh:149`). The stock verifier accepts a CPU proof too, so it does not show the device. Split after Astra-B review R-24.

### Requirement: GPV-002 GPU and CPU give the same public values
GPV-002: When the GPU prover and the CPU prover each prove one shard, the leaf prover SHALL give byte-equal public values on both.

Verification: NEW: a test that proves one leaf on both devices and compares the public values; the G3 GPU leg (SQG-014). To build.

#### Scenario: One shard on both devices
- **GIVEN** one leaf shard of `t02 increment`
- **WHEN** odin proves it on the GPU and on the CPU
- **THEN** the public values are byte-equal and both proofs verify

#### Scenario: Status of GPV-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: equality is shown at `claim_pub` only. Owner lane: t12-gate (the G3 GPU leg on huginn). Evidence: Runs h-nogpu against the GPU run give an equal `claim_pub` (Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:119`). No test compares a BabyBear CUDA proof with a CPU proof: gate `sg6` only runs the stock verifier.

### Requirement: GPV-003 GPU hashing of L_Fr equals the CPU
GPV-003: Where the build has the cargo feature `gpu`, the `L_Fr` prover SHALL compute Poseidon hashes, the encoding and the fractional sumcheck with results equal to the CPU path.

Verification: tests `e4_gpu_poseidon_equals_cpu` (`crates/sherlock-lfr/tests/e4_gpu_hash.rs:49`), `e4_gpu_encode_equals_cpu` (`:84`), `e4_gpu_frac_sumcheck_equals_cpu` (`crates/sherlock-lfr/tests/e4_gpu_frac.rs:36`); gate `lfr-gpu` (`~/.cache/t1/run.sh:45`). Exists.

#### Scenario: The GPU is switched off
- **GIVEN** one `L_Fr` proof with `MORI_GPU=off` and one with the GPU
- **WHEN** both are made from one tape
- **THEN** the proofs are byte-identical

#### Scenario: Status of GPV-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (LOOP T1, T3). Rerun on the T25 keys. Evidence: `openvm-t14` `crates/sherlock-lfr/Cargo.toml:32` (feature `gpu`; `MORI_GPU=off` takes the CPU path). Moriarty 3dc9001a9 `research/sherlock-e4-gpu-20261112/RESULTS.md:68`: CPU and GPU `L_Fr` proofs byte-identical.

### Requirement: GPV-004 Nodes prove on the GPU
GPV-004: Where the build has the cargo feature `cuda` and `T7_NODE_GPU` is not `0` or `off`, the node prover SHALL prove N1, N2 and N3 on the GPU.

Verification: the T14 timing record on the T25 keys (LPC-005); the stock verifier accepts each node proof. Exists.

#### Scenario: The tree stage
- **GIVEN** a 3-leaf run on huginn with the variable unset
- **WHEN** the nodes are proved
- **THEN** the record names the CUDA backend, the stock verifier accepts N1 to N3 and `claim_pub` equals the CPU run

#### Scenario: The switch is off
- **GIVEN** the same run with `T7_NODE_GPU` = 0
- **WHEN** the nodes are proved
- **THEN** the record names the CPU backend

#### Scenario: Status of GPV-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT and MEASURED on huginn on the T10m keys. Evidence: `openvm-t14` `crates/sherlock-wrap-export/Cargo.toml:51` to `:58`; `crates/sherlock-wrap-export/examples/t7_chain.rs:109` (unset means enabled; `0` or `off` means CPU). Moriarty `t14-gpu` 3fd18b166 `research/sherlock-t14-gpu-20261114/RESULTS.md:146` to `:158`: `tree_s` warm 19.04 s to 1.56 s, kept after the retime. Trigger corrected after Astra-B review R-25.

### Requirement: GPV-005 L proves on the GPU
GPV-005: Where the build has the cargo feature `cuda` and `T14_L_GPU` is not `off`, the `L` prover SHALL prove `L` on the GPU.

Verification: NEW: a timed run on huginn and odin with the stock verifier on the `L` proof. To build.

#### Scenario: L on odin
- **GIVEN** a node root on odin
- **WHEN** `L` is proved with `T14_L_GPU`
- **THEN** the stock verifier accepts `L` and `L_Fr` verifies it

#### Scenario: Status of GPV-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: no timing on file. Owner lane: t14-gpu. Evidence: `openvm-t14` `crates/sherlock-wrap-export/Cargo.toml:51` to `:58`; `examples/t7_chain.rs:893` (`T14_L_GPU=off`). Moriarty `t14-gpu` 3fd18b166 `research/sherlock-t14-gpu-20261114/RESULTS.md`: item 1 (`L` on GPU) "not timed yet".

### Requirement: GPV-006 A GPU fault stops the run
GPV-006: If a GPU proof of a shard fails with a device error, then the Sherlock prover SHALL stop the run and name the shard and the device.

Verification: NEW: a fault-injection run (a GPU memory cap below one leaf with a forced GPU schedule). To build.

#### Scenario: Out of device memory
- **GIVEN** huginn with 1 GB free VRAM and a forced GPU schedule
- **WHEN** a leaf proof starts
- **THEN** the run stops with the shard index and writes no partial proof under a final name

#### Scenario: Status of GPV-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. Slice C owns the recovery after the stop (FRC). Owner lane: t16-elastic. Evidence: openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:457` (`admit_early_wrap` guards the device memory before the wrap starts). No record states what the prover does on a CUDA error in a leaf.

### Requirement: GPV-007 The leaf layer runs on the GPU
GPV-007: Where the build has the cargo feature `cuda`, the leaf prover SHALL prove the leaf-layer proof of each shard on the GPU.

Verification: NEW: a timed run on huginn and odin whose record names the CUDA backend for the leaf layer; the stock verifier accepts each proof. To build.

#### Scenario: A GPU leaf layer
- **GIVEN** huginn with the `cuda` build
- **WHEN** a leaf-layer proof is made
- **THEN** the record names the CUDA backend and the leaf time is below the 1.864 s CPU figure

#### Scenario: Status of GPV-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: not built. Owner lane: t14-gpu. Evidence: Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:52`: "the leaf layer still proves on the CPU" (leaf layer 1.864 s on huginn and 2.975 s on odin, DERIVED). openvm `t16-elastic` 1baf605 `t16.rs:711` runs the leaf layer apart from the GPU app proof. Split from GPV-001 after Astra-B review R-24.
