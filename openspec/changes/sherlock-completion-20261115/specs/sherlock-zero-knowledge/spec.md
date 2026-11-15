## ADDED Requirements

### Requirement: ZK-012 r_P once per deployment
ZK-012: When the prover prepares a deployment, the prover SHALL draw `r_P` once from the OS CSPRNG and store it in `deploy.secret` with mode 0600.

#### Scenario: r_P once per deployment
- **WHEN** a `deploy.secret` file has group or world bits
- **THEN** the prover refuses the file

#### Scenario: Verification and status of ZK-012
- **WHEN** the status is reported
- **THEN** the source is T11-1, owner-confirmed 2026-11-15, the verification is unit tests of `crates/sherlock-leaf/src/blind.rs` (openvm `t11-zk`), and the status on 2026-11-15 is: OPEN: `blind.rs` untracked in the openvm `t11-zk` tree.

### Requirement: ZK-013 A fresh salt per run
ZK-013: When the prover prepares a run, the prover SHALL draw a fresh input salt and store it in `run.secret`.

#### Scenario: A fresh salt per run
- **WHEN** two runs of one input are prepared
- **THEN** their salts differ and their `claim_pub` differ

#### Scenario: Verification and status of ZK-013
- **WHEN** the status is reported
- **THEN** the source is T11-1, owner-confirmed 2026-11-15, the verification is `blind.rs` tests and the forbid list of `t7_chain t10`, and the status on 2026-11-15 is: OPEN.

### Requirement: ZK-014 Seeded builds refuse production proofs
ZK-014: If a build without the cargo feature `test-seed` receives `SHERLOCK_TEST_SEED`, `GW_DET`, `GW_CHIP_SEED` or `GW_SEED`, then the wrap prover SHALL exit with code 4 and write no proof.

#### Scenario: Seeded builds refuse production proofs
- **WHEN** a release build runs with `GW_DET=1`
- **THEN** it exits 4 and the output directory has no proof

#### Scenario: Verification and status of ZK-014
- **WHEN** the status is reported
- **THEN** the source is T11-4, owner-confirmed 2026-11-15, the verification is T11 RESULTS section 4 (`tools/sherlock-wrap/src/zkguard.rs`), and the status on 2026-11-15 is: OPEN: RESULTS-PENDING, `zkguard.rs` untracked.

### Requirement: ZK-015 EQ-ZK
ZK-015: When the wrap prover proves one statement twice, the wrap prover SHALL make two proofs that differ byte for byte and that the ledger 9.1.0.1 verifier accepts.

#### Scenario: EQ-ZK
- **WHEN** two proofs of one call vector are compared
- **THEN** every byte range of the proof differs and both verify

#### Scenario: Verification and status of ZK-015
- **WHEN** the status is reported
- **THEN** the source is row T11 done-when (LOOP.md line 33), the verification is T11 RESULTS section 3 on CPU and GPU, and the status on 2026-11-15 is: OPEN: RESULTS-PENDING.

### Requirement: ZK-016 Blinding slots are one-use
ZK-016: If a request to `wrapd serve` or to the STARK worker reuses a blinding slot, then the daemon SHALL refuse the request.

#### Scenario: Blinding slots are one-use
- **WHEN** a client replays a slot id
- **THEN** the daemon refuses the request and makes no proof

#### Scenario: Verification and status of ZK-016
- **WHEN** the status is reported
- **THEN** the source is PRD-002, T11-2 (owner-confirmed 2026-11-15), the verification is T11 RESULTS section 5, and the status on 2026-11-15 is: OPEN: RESULTS-PENDING.

### Requirement: ZK-017 The GPU wrap takes fresh blinders
ZK-017: While the wrap runs on a GPU of odin or huginn, the GPU wrap prover SHALL take its blinders from `OsRng`.

#### Scenario: The GPU wrap takes fresh blinders
- **WHEN** two GPU proofs of one statement are made
- **THEN** they differ and both verify on muninn

#### Scenario: Verification and status of ZK-017
- **WHEN** the status is reported
- **THEN** the source is row T11 ("show it on the GPU prover too"), the verification is T11 RESULTS section 7, EQ-ZK on the GPU, and the status on 2026-11-15 is: OPEN: odin queue "T11 GPU half" waits behind r10 and the R_SL suite prove.

### Requirement: ZK-018 Case B is refused
ZK-018: If a configuration sends a leaf, node, `L`, `L_Fr` or wrap job to a host that the user does not own, then the prover SHALL refuse the configuration.

#### Scenario: Case B is refused
- **WHEN** odin proves for a user who does not own odin
- **THEN** the prover refuses the job

#### Scenario: Verification and status of ZK-018
- **WHEN** the status is reported
- **THEN** the source is owner 2026-11-15: "we will start with case A and a future design will allow for outsources components using the Veritee model from the MPC Research thread", the verification is placement Quint property of ZK-001 and a configuration test, and the status on 2026-11-15 is: OPEN: no configuration test. Case B is a future Veritee-based design.

### Requirement: ZK-019 ZK-L2 in Lean
ZK-019: The SherlockSpec Lean library SHALL prove `pv2_inj` and `pv2_count_det` with no axiom outside `propext`, `Quot.sound` and `Classical.choice`.

#### Scenario: ZK-L2 in Lean
- **WHEN** the axiom report runs
- **THEN** it lists no other axiom

#### Scenario: Verification and status of ZK-019
- **WHEN** the status is reported
- **THEN** the source is row T11 done-when, the verification is `lake build` of SherlockSpec and `ZkPaddingAxioms.lean` (Moriarty `t11-zk` 71c4c001b), and the status on 2026-11-15 is: DONE on `t11-zk`, not merged.
