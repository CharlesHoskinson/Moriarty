## ADDED Requirements

### Requirement: GPH-001 GP-1: both rule families under one key set
GPH-001: The Sherlock prover SHALL prove programs of the interpreted rule family and programs of the straight-line family `R_SL_*` under the same universal key set.

Verification: RSL-001 probe for the R_SL family; LPC-007 run for the interpreted family; one key manifest sha256 in both records. Exists.

#### Scenario: Two rule families
- **GIVEN** one interpreted program and one R_SL program
- **WHEN** both are proved
- **THEN** N1, N2, N3, `L` and `L_Fr` use the same keys

#### Scenario: Status of GPH-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. GAP: the interpreted family has 5 rules in the leaf, so the proving layer does not yet cover the universal interpreter of EUI. Owner lane: T25 with t16-elastic. Evidence: R_SL is chip set 3 (openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/rsl/mod.rs:1`). The interpreted leaf proves 5 rules today (Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:121` to Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:123`; LOOP T16 (c) BLOCKED); EUI-010 (slice A) counts 5 + 2 of the 120 enabled rules (`crates/sherlock-leaf/src/dec.rs:23` to `:41`).

### Requirement: GPH-002 GP-2: a run consumes the claim of another run
GPH-002: When a run consumes the claim of an earlier run, the node prover SHALL verify the consumed claim through the GP-2 node modes MERGE, SEAL, SEAL-WITH-DEPS, DL-STEP, DESCEND and JOIN.

Verification: gate `gp2`: `t8_gp gp2 OUT`, 48 of 48 cases (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:94`). Exists.

#### Scenario: A forged dependency
- **GIVEN** a run that names a claim that no node proved
- **WHEN** the node checks the dependency
- **THEN** the node refuses the run

#### Scenario: Status of GPH-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys. Open: `L` over a SET root (Moriarty 3dc9001a9 `research/sherlock-t8b-gp-20261114/RESULTS.md:23`), the X12 discharge model in the circuit (`RESULTS.md:52`); both are LPC-003. Owner lane: T8c. Evidence: openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/gp.rs:1` to `:26` (the modes); openvm `t10mm` 5b7ada8 `crates/sherlock-wrap-export/examples/t8_gp.rs:564` (`fn gp2`).

### Requirement: GPH-003 GP-3: a foreign-verifier unit under N3
GPH-003: When a foreign-verifier unit proof is a child of N3, the N3 node SHALL accept the unit under the foreign rows of `keys_d`.

Verification: `scripts/qa/check-chain.sh chain` with the FV unit case; gate `pathc-d3` on `fv_vk.bitcode` (Moriarty 3dc9001a9 `research/sherlock-t10-bound-20261114/chain.sh:144`). Exists.

#### Scenario: An FV unit of another key
- **GIVEN** an FV unit whose key is not in the foreign rows
- **WHEN** N3 verifies its children
- **THEN** N3 refuses the unit

#### Scenario: Status of GPH-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys (LOOP T7). Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/x14_units.rs` (X14 units); openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:459` (2 foreign rows). LOOP T7 done-when: "an FV unit under N3 accepted".

### Requirement: GPH-004 GP-3: the Compact adapter class
GPH-004: Where the X8 in-circuit Compact verifier is built, the foreign registry SHALL name the adapter class `compact-zkir3-kzg-bls12381-blake2b/1`.

Verification: NEW: a registry test that reads the class name; LPC-003 real-proof run. To build.

#### Scenario: A native Compact proof as input
- **GIVEN** a Compact proof of class `compact-zkir3-kzg-bls12381-blake2b/1`
- **WHEN** a Sherlock run consumes it
- **THEN** the foreign verifier checks it in circuit and the registry row names the class

#### Scenario: Status of GPH-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. Owner lane: T8c. Evidence: Moriarty 3dc9001a9 `research/sherlock-t8-gp-20261113/RESULTS.md:33` and `:167` name the class (owner ruling O8, lead decision confirmed 2026-11-14). No code holds the string. Moriarty 3dc9001a9 `research/sherlock-t8b-gp-20261114/RESULTS.md:76` to `:83`: the in-circuit verifier is open.

### Requirement: GPH-005 GP-4: a tree of any shape over MERGE-WORK
GPH-005: When a proof-carrying-data tree of MERGE-WORK units of any shape is proved, the N3 node SHALL accept the root of the tree under the universal key set.

Verification: gate `x7`: `t8_gp x7 OUT`, 22 of 22 cases on the stock verifier (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:95`). Exists.

#### Scenario: Two tree shapes
- **GIVEN** a balanced tree and a chain of the same 4 work units
- **WHEN** both are proved
- **THEN** both roots verify under the same keys and give the same claim

#### Scenario: Status of GPH-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys. Open: the parent with STACKS and VALUE DIGEST in the inline kernel (Moriarty 3dc9001a9 `research/sherlock-t8b-gp-20261114/RESULTS.md:69`, `:71`; LPC-002). Owner lane: T8c. Evidence: openvm `t10mm` 5b7ada8 `crates/sherlock-wrap-export/examples/t8_gp.rs:863` (`fn x7`), `t8_gp.rs:349` to `:357` (NW, NW2); rows 4 to 6 of `keys_d` (openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/stmt.rs:89`).

### Requirement: GPH-006 A suspended run gives the same claim
GPH-006: When a run suspends and resumes, the Sherlock prover SHALL give the `claim_pub` of the run without the suspension.

Verification: the case `pos_plan_schedules` of the T16 part (b) run; `quint test` on `continuation.qnt`. Exists.

#### Scenario: Resume on another box
- **GIVEN** a run that suspends on huginn and resumes on odin
- **WHEN** the run completes
- **THEN** `claim_pub` equals the value of the run made on one box

#### Scenario: Status of GPH-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL: one suspension and resume MEASURED (LOOP T16 (b) S1 to S5); two run lengths open (LPC-006). Owner lane: t16-elastic. Evidence: Quint model Moriarty 3dc9001a9 `models/sherlock/quint/continuation.qnt` (N-4, commit 8ea5655ca). openvm `t16-elastic` 1baf605 `crates/sherlock-wrap-export/examples/common/t16.rs:235` (the case label `pos_plan_schedules`). `research/sherlock-final-design-20261111/ARCHITECTURE.md:215`.

### Requirement: GPH-007 A gate change does not narrow GP
GPH-007: If a change removes a case of the GP case map or changes its expected outcome, then the gate runner SHALL fail the gate.

Verification: NEW: a case map file that names, for each of GP-001 to GP-004, the cases of `gp2`, `x7`, the FV unit and the run-length cases with their expected outcome (accept or refuse); the gate compares the run against it. The count floors 48 and 22 stay as a smoke check. To build.

#### Scenario: A refusal case becomes a positive case
- **GIVEN** a tree where the forged-dependency case of `gp2` is replaced by a copy of an accept case
- **WHEN** the gate runs
- **THEN** the count is still 48 and the gate fails on the case map

#### Scenario: Status of GPH-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: the counts are reported, no floor is checked. Owner lane: T25. Evidence: Case counts today: `gp2` 48, `x7` 22 (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:94`, Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:95`). No map from GP-001 to GP-004 to named cases exists. CPR-005 and DG-010. LOOP hard rules: "GP-1 to GP-4 never narrowed". A count floor alone passes when a refusal case is replaced by a copy of a positive case (Astra-B review R-27).

### Requirement: GPH-008 The L collection form
GPH-008: When the node root of a run is the SET root of a collection of claims, the `L` circuit SHALL expose the `claims_root` of that SET as a public word.

Verification: LPC-003 real-proof run with refusal cases (a claim outside the SET, a changed `claims_root`); the Quint model of fix 2 on `t20-batched`; to build.

#### Scenario: A collection of 3 claims
- **GIVEN** a SET root over 3 run claims
- **WHEN** `L` is proved in the collection form
- **THEN** the public words hold `claims_root` and a membership path of each claim verifies against it

#### Scenario: Status of GPH-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN and required for the complete prototype: BAT (slice C) depends on it for the batched-write hard rule (D-009), and LPC-003 requires the real-proof run. It is not in `~/.cache/rekey-t11/LIST.md`, so the T25 key set as listed has no collection form; D-049 records the order (T25 first, T8c as a recorded second rekey before T15) and waives nothing. Owner lane: T8c. Evidence: Row T8c (no LOOP row; named in LOOP T8b). BAT-009 (slice C) refuses `acceptCollection` until this form has a key. Moriarty `t20-batched` 5dc1be2ee `research/sherlock-t20-batched-20261114/RESULTS.md:70`: "no code on this branch; it belongs to T8b L over SET, which is not built". Owner ruling 2026-11-14, T20 picks: `[reg_pub, claims_root]`, depth 20.
