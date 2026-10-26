# Recursion phase profile

Status: PROPOSED, 2026-10-26. All scenarios are proposed acceptance contracts, not execution receipts. Source: the reviewed draft `wiki-llm/recursion-perf/clusters/s9-orchestration/wave-2/OPENSPEC-DRAFT.md` and consensus row C0.

## ADDED Requirements

### Requirement: RPP-001 Exact workload and implementation identity

The profiling harness SHALL target published midnight-proofs 0.8.3 first. Each profile SHALL record the release archive checksum, the patch checksum, the `Cargo.lock` checksum, the resolved packages and features, the compiler, the binary checksum, the public workload manifest and the VK and SRS identities of each proof. Each proof record SHALL state its circuit shape, k, extended k when applicable, transcript and ordered children. The reducer SHALL reject a comparison between mismatched identities unless the comparison is labeled as a version or candidate comparison.

#### Scenario: RPP-001-01 one dependency closure for all Sherlock controls

- **GIVEN** A102, A80, P and D fixtures built with pinned 0.8.3 and the official SRS
- **WHEN** the harness builds the pristine, observation-off and observation-on variants
- **THEN** each binary is bound to its manifest and the observation feature is the only controlled difference

#### Scenario: RPP-001-02 historical recl9 offered as a control

- **GIVEN** a recl9 receipt that resolves midnight-proofs 0.8.2
- **WHEN** it is grouped with 0.8.3 profiles
- **THEN** the reducer refuses matched-control status, keeps it only as a labeled historical reference, and transfers no phase fraction from it

### Requirement: RPP-002 Complete phase ownership

The observer SHALL time key load, `setup_pk`, witness synthesis per advice phase, rational resolution and blinding, every advice and fixed commitment batch, lookup permutation and grand products, copy-permutation products, quotient evaluation with all extended-domain NTTs, quotient commitments, evaluations, multiopen and transcript work. It SHALL also account for instance preparation, vanishing, verification and command wrappers. Every nested MSM and NTT SHALL have exactly one accounting owner.

#### Scenario: RPP-002-01 advice challenges interleave with synthesis

- **GIVEN** a circuit with more than one advice phase
- **WHEN** the prover synthesizes, resolves and commits each phase before it squeezes the next challenge
- **THEN** the profile keeps that sequence and does not classify all trace computation as synthesis

#### Scenario: RPP-002-02 multiopen contains commitment MSMs

- **GIVEN** `multi_open` commits two polynomials with MSMs
- **WHEN** the reducer exports fractions for rows C3 and C9
- **THEN** it gives inclusive multiopen and exclusive non-MSM parts and names the shared MSM intervals, so no composed gain credits them twice

#### Scenario: RPP-002-03 a warm invocation builds no VK

- **GIVEN** a proof that uses an already prepared VK
- **WHEN** no fixed-column MSM batch runs
- **THEN** the profile marks that batch not applicable and invents no fixed-commitment cost

### Requirement: RPP-003 Reconciled wall and CPU accounting

The observer SHALL use a monotonic wall clock. The exclusive partition plus the unexplained bucket SHALL sum to 100% of one named proof. For each proof and each command, the absolute phase-sum discrepancy SHALL be at most 5% of the measured total, and unexplained time SHALL be at most 5%. Isolated-proof CPU SHALL include all worker threads. Overlapping jobs SHALL NOT each claim the same process CPU delta.

#### Scenario: RPP-003-01 parallel NTT tasks join inside one phase

- **GIVEN** several worker NTT diagnostics overlap inside one joined quotient phase
- **WHEN** the reducer sums wall time
- **THEN** it counts the joined interval once and counts the CPU of all workers once

#### Scenario: RPP-003-02 wrapper work in a command

- **GIVEN** B3 spends time on compilation, fixture generation, mock runs and verification
- **WHEN** whole-command reconciliation runs
- **THEN** those intervals appear explicitly, and proof-only coverage does not stand in for command coverage

#### Scenario: RPP-003-03 coverage below the bound

- **GIVEN** a proof whose phases plus unexplained time differ from its total by 7%
- **WHEN** the reducer validates it
- **THEN** the profile is marked invalid for coverage and acceleration claims

### Requirement: RPP-004 Honest memory and byte semantics

Each phase record SHALL include wall seconds, CPU core-seconds or a reason for absence, RSS at entry and exit, a sampled peak and a peak delta. The peak delta SHALL be `max(0, sampled_peak - entry)` and SHALL be labeled a sampled lower bound. Lifetime maxrss, cgroup memory and device memory SHALL stay distinct. Logical copy bytes SHALL NOT be labeled DRAM traffic. An unsupported hardware counter SHALL be null with a reason, never zero.

#### Scenario: RPP-004-01 a short phase between RSS samples

- **GIVEN** a phase shorter than the sampler period
- **WHEN** only boundary RSS readings exist
- **THEN** the record marks boundary-only quality and certifies no instantaneous peak or memory admission

#### Scenario: RPP-004-02 memory-controller counters are absent

- **GIVEN** proc I/O and explicit copy counters exist but DRAM counters do not
- **WHEN** a phase is serialized
- **THEN** DRAM bytes are null with a reason and bandwidth conclusions are marked unsupported

### Requirement: RPP-005 Bounded and qualified overhead

Profiling SHALL be behind a compile-time feature that is off by default. The enabled path SHALL use bounded event buffers, SHALL avoid per-cell events, and SHALL record flush time, sampler cost and dropped events. Qualification SHALL require a one-sided 95% upper bound of at most 1% wall overhead, from at least five paired on/off runs per claimed workload and cache stratum, with the analysis fixed before the runs.

#### Scenario: RPP-005-01 noise exceeds the margin

- **GIVEN** a paired mean overhead below 1% and an upper bound above 1%
- **WHEN** the report is written
- **THEN** overhead qualification is reported as inconclusive or failed, never passed

#### Scenario: RPP-005-02 the event buffer fills

- **GIVEN** a proof that exceeds the event capacity
- **WHEN** the observer cannot keep a new event
- **THEN** proving behaves normally, the dropped-event count increases, and the profile is invalid

### Requirement: RPP-006 Unchanged proof behavior

Instrumentation SHALL preserve PLONK/KZG, BLS12-381, the official SRS, every transcript, commitment and challenge order, and production randomness. It SHALL NOT record witness values, scalars, challenge contents, RNG seeds, wallet material or environment values. The existing stock-verifier and installed-key checks SHALL still pass.

#### Scenario: RPP-006-01 on and off prove the same fixture

- **GIVEN** identical public inputs, keys and protocol settings
- **WHEN** the observation-off and observation-on variants prove the fixture
- **THEN** stock verification accepts both proofs and rejects the same malformed-proof and malformed-statement controls

#### Scenario: RPP-006-02 a sidecar with secret material

- **GIVEN** a sidecar that contains a field matching a witness, seed or wallet pattern
- **WHEN** the validator checks it
- **THEN** the validator rejects the sidecar and the profile is not stored

### Requirement: RPP-007 Production build isolation

The patched crate SHALL live in the separate workspace `tools/profile/` with its own `Cargo.lock`. The main workspace `Cargo.lock` and every factory check SHALL be unchanged by this change.

#### Scenario: RPP-007-01 main workspace after the change

- **GIVEN** the change is applied
- **WHEN** `cargo metadata --locked` runs in the main workspace
- **THEN** midnight-proofs resolves to the registry 0.8.3 source and `Cargo.lock` is byte-identical to its prior commit

#### Scenario: RPP-007-02 the feature leaks into production

- **GIVEN** a main-workspace build that enables `recursion-phase-profile`
- **WHEN** T0 runs
- **THEN** T0 fails and names the crate that enabled the feature

### Requirement: RPP-008 Factory receipts and failure preservation

Profiling builds and proofs SHALL run through DoctorWatson preflight and measured caps. Only `doctorwatson heavy` and `doctorwatson check` SHALL append to `qa/sherlock/runs.jsonl`. The run trace SHALL carry the sidecar path, hash and schema. Failed, capped, interrupted and incomplete profiles SHALL remain visible.

#### Scenario: RPP-008-01 a profile proof hits its cap

- **GIVEN** an A3 observation run exceeds its factory cap
- **WHEN** heavy terminates it
- **THEN** the failure receipt and partial trace remain, no successful profile is written, and the next attempt carries a trace-grounded hypothesis

### Requirement: RPP-009 Run matrix and decision output

Qualification SHALL include process-cold and resident-PK second-proof profiles for A102, A80, P and D, the A2, A3 and A4 proof records, and the B3 one-batch command. The reducer SHALL give one named deciding metric for each row C1–C10, with its denominator and contributing spans. It SHALL separate arithmetic feasibility from measured gain, SHALL reject overlapping gain credit, and SHALL leave rows without candidate data unresolved.

#### Scenario: RPP-009-01 no fused-terminal candidate or GPU run

- **GIVEN** only CPU profiles of the current recursion topology
- **WHEN** rows C1, C2 and C5 are evaluated
- **THEN** the reducer reports necessary bounds only and marks no 10× candidate or scheduler gain confirmed

#### Scenario: RPP-009-02 row C8 on 0.8.3

- **GIVEN** the observed 0.8.3 dispatcher always uses BLST
- **WHEN** the 0.8.2 dispatch backport gain is requested
- **THEN** the output marks the row inapplicable to this version and credits no gain

### Requirement: RPP-010 Queued and Finalized semantics

The profile SHALL distinguish admission, active service, proof-ready, local verification, installed-ledger verification, submission, inclusion and finality timestamps. Finalized SHALL require chain finality and state readback evidence.

#### Scenario: RPP-010-01 A4 verifies locally against the installed key

- **GIVEN** a terminal proof accepted by the installed-key path with no submitted transaction
- **WHEN** its sidecar is written
- **THEN** the state may be ledger_verified, the submission, inclusion and finality timestamps are null, and no Finalized throughput is claimed
