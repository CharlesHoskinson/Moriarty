## ADDED Requirements

### Requirement: EUI-001 One machine, two rule families
EUI-001: The Sherlock machine SHALL run each program under exactly one rule family, the interpreted family of 120 step rules or the straight-line family `R_SL`, that the `machine_version` inside `prog_d` names.

Verification: test T-MACHINE-VERSION (a leaf proof of an `R_SL` program under a `prog_d` that names the interpreted family is refused); to build.

#### Scenario: Rule family grows
- **WHEN** a kit rule is added to the `R_SL` family
- **THEN** the machine version changes, and the keys of programs planned under the old version stay valid

#### Scenario: Status of EUI-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: design (`research/sherlock-final-design-20261111/ARCHITECTURE.md:225-227`, machine version `sherlock-k-a/2`); OPEN in code, because `R_SL` programs have no `prog_d` (ERS-006). References: GP-001 (`sherlock-vm-consensus-20261107`), CORE-011, CPR-005, D-008, D-017.

### Requirement: EUI-002 Both families prove under one universal key set
EUI-002: When the prototype demonstration runs, the Sherlock prover SHALL prove one interpreted-family program and one `R_SL` program under one universal key set.

Verification: the T25 gate rerun (every T10m, T10q and T8b gate on the new keys) plus one T23 call on the same key set, each wrap checked on muninn by the ledger 9.1.0.1 verifier; to build.

#### Scenario: Second program
- **WHEN** the demonstration proves the interpreted program `entry x;` (outcome Rejected, code 3), the interpreted Halted program `unitctor` and then `t02` `increment`
- **THEN** the node, `L`, `L_Fr` and wrap keys are byte-equal for the three runs

#### Scenario: Status of EUI-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. On the T10m keys the leaf proves 5 rules and no `R_SL` rule (LOOP.md rows T16 and T18); the universal leaf key takes `R_SL` apps in the odin r10 probe (`LIST.md` row 6f773b2). References: RKY-001, RSL-001, CPR-003, LPC-008, D-004.

### Requirement: EUI-003 A program is data
EUI-003: When a developer adds a program, the Sherlock prover SHALL need no new universal key, contract class or ceremony.

Verification: rsl probe field `n1_pre_hash_equals_universal_n1` true for every suite program (`crates/sherlock-wrap-export/examples/t7_chain.rs:2003-2045`); exists.

#### Scenario: Program the keys were not built from
- **WHEN** a contract outside the suite compiles to `R_SL`
- **THEN** its leaf proof verifies under the universal leaf key

#### Scenario: Status of EUI-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED true for 7 call kinds on huginn at 699539c: a05, t02, s01 th, d04 withdraw, z02, s01 ec, s02 keccak (wiki log 2026-11-15 10:01); OPEN over the full suite. The app key is per program, the preprocessed traces (`crates/sherlock-leaf/src/rsl/mod.rs:12-13`). References: RSL-001, MYC-LOW-013, MYC-KEY-001.

### Requirement: EUI-004 An `R_SL` call ends in one of two outcomes
EUI-004: When the `R_SL` executor finishes a call, the `R_SL` executor SHALL return either one outcome with `comm`, the effect words and the outputs, or one reject code.

Verification: Lean guards of `Mycroft.Spike.T18c` (28 guards over `SL.run`) and tests `tests_lowering.rs`; exists.

#### Scenario: Failed range check
- **WHEN** a witness word fails a `Range` instruction
- **THEN** the executor returns `MYC-REF-037/rangeFailed` and no outcome

#### Scenario: Status of EUI-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (`mycroft/compiler/src/sem.rs:19`, `:37-41`, `:592-618`; reject codes at `pipeline.rs:139-156`). References: MYC-LOW-005, MYC-LOW-006, MYC-LOW-007.

### Requirement: EUI-005 The interpreted family keeps three outcome kinds
EUI-005: When an interpreted-family run ends, the Sherlock machine SHALL report exactly one of `Halted`, `Rejected` and `Exhausted`.

Verification: one real proof per outcome kind with a mutation of its tag and of its payload: Rejected by `entry x;` (T6), Halted by `unitctor` (T8b, gp2 48 of 48), Exhausted by a fuel-bound run; Rejected and Halted exist, the Exhausted case and the mutations are to build.

#### Scenario: Rejected run
- **WHEN** `entry x;` runs
- **THEN** the outcome is `Rejected` with code 3 and `L` accepts the chain

#### Scenario: Status of EUI-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED on the T10m keys for two kinds: `entry x;` gives Rejected code 3 (`research/sherlock-t6-leaf-20261113/RESULTS.md:47-50`), `unitctor` gives Halted (`research/sherlock-t8b-gp-20261114/RESULTS.md:35-43`). No Exhausted proof is on file. Reruns on the T25 keys. References: GP-001, CORE-003, CORE-012, CORE-015.

### Requirement: EUI-006 Three executors agree on every suite call
EUI-006: When the suite runs a call, the `R_SL` executor of openvm SHALL give the `comm` and W that the Mycroft reference `sem::sl_run` gives.

Verification: test `rsl_suite_exec_equal_native` and the Mycroft compare gate (207 equal, 1 could-not-apply, 0 different); exists.

#### Scenario: One executor differs
- **WHEN** openvm gives a W word that the Mycroft reference does not give
- **THEN** the call counts as a mismatch and blocks the suite

#### Scenario: Status of EUI-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: MEASURED (odin 142 of 142; `mycroft/factory/REPORT.md:18`). The Lean oracle agrees with the Quint model on 1,226 of 1,226 transitions (`REPORT.md:10`). References: ECC-011, MYC-TRN-008, RFB-001.

### Requirement: EUI-007 A bad tape starts no proof
EUI-007: If a tape has a wrong selector, a non-canonical word, a missing word or a leftover word, then the R_SL prover wrapper SHALL return the status `refused` and start no proof.

Verification: test `rsl_rejected_tapes_refused` (`crates/sherlock-leaf/tests/rsl.rs:75`) for the refusal; test T-NO-PROOF-START (a proof-start counter stays 0 on the four bad tapes) to build.

#### Scenario: Leftover input
- **WHEN** the tape holds one word more than the ABI `tape_len`
- **THEN** the status is `refused` with a tape length reason, and no key or trace commitment is made

#### Scenario: Status of EUI-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: refusal BUILT at `rsl::prove_call` (`src/rsl/mod.rs:43-45`). `compile::build` records the failure in `Built.failed` and still builds micro-ops (`compile.rs:598-614`); the stable public error codes (`inputLeftover` and the others of `pipeline.rs:139-156`) are not returned at this boundary: normalization to build. References: MYC-TAPE-006, MYC-TAPE-007, MYC-REF-036, MYC-REF-038, LW-021.

### Requirement: EUI-008 The step count of an entry is static
EUI-008: The `R_SL` executor SHALL take the same count of micro-ops for any two tapes of one entry.

Verification: test `rsl_key_independent_of_tape` (`tests/rsl.rs:60`); exists.

#### Scenario: Guarded-off effect
- **WHEN** a guard is false for one tape and true for a second tape
- **THEN** both runs have equal trace heights and the false guard writes zeros

#### Scenario: Status of EUI-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: CF-004, MYC-LOW-012, MYC-TRN-011, ZK-008.

### Requirement: EUI-009 processor-final governs a demonstrated conflict
EUI-009: Where a requirement of change `sherlock-universal-interpreter-design` conflicts with a named requirement of `sherlock-processor-final-20261111`, the Sherlock machine SHALL follow `sherlock-processor-final-20261111`.

Verification: the UIREQ retention map (each UIREQ ID: retained by reference, mapped to CORE, RFB or GP IDs, or superseded with the conflicting ID); to build.

#### Scenario: Missing stage is not a conflict
- **WHEN** a UIREQ requirement names behaviour that processor-final does not mention
- **THEN** the UIREQ requirement stays in force until the retention map rules on it

#### Scenario: Status of EUI-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. No UIREQ requirement is retired by this change. UIREQ-003, 004, 005, 008, 014 and 016 (lexical scope, evaluation order, recursion, matching, resumable execution) have host tests (`crates/mori-core/tests/sherlock_original_ears_host_positive.rs:196`, `:257`, `:315`; `sherlock_technical_chunks_positive.rs:255`) and no proof evidence; they stay in force by reference. OWNER 2026-11-15 (D-025). References: UIREQ-001 to UIREQ-035.

### Requirement: EUI-010 The interpreted family covers every enabled rule
EUI-010: The interpreted-family leaf SHALL prove each of the 120 enabled step rules of decision M-4.

Verification: the RFB-003 and RFB-004 gates plus, for every enabled rule, one honest proof case and one forged-transition case, or a written reason why the rule cannot be reached; to build.

#### Scenario: Rule with no proof case
- **WHEN** the coverage gate finds an enabled rule with no chip row
- **THEN** the gate fails and GP-1 stays open

#### Scenario: Status of EUI-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. The built chip sets hold 5 subset rules (`crates/sherlock-leaf/src/dec.rs:23-29`) and 2 value rules (`dec.rs:40-41`) at openvm `rekey-chips` dcf1b4c; T6 proves the subset chain only (LOOP.md row T6, Group V). Gap owner: slice B, modular-leaf groups after V. Found by the Astra-A draft. References: RFB-003, RFB-004, CORE-011, GP-001, D-008.
