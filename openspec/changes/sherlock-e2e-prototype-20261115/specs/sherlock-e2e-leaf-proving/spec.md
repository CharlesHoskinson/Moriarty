## ADDED Requirements

### Requirement: LFP-001 Leaves are BabyBear STARKs
LFP-001: The leaf prover SHALL prove every app shard and every leaf-layer proof as a STARK over BabyBear with the Poseidon2 configuration `BabyBearPoseidon2Config`.

Verification: gate V.10 `scripts/qa/check-chain.sh chain` (Moriarty 3dc9001a9 `research/sherlock-t10-bound-20261114/chain.sh:88`): the stock stark-backend verifier accepts every leaf proof. Exists.

#### Scenario: A leaf proof verifies with the stock verifier
- **GIVEN** a leaf proof of the suite call `t02 increment`
- **WHEN** the stock stark-backend verifier checks the proof under the leaf key
- **THEN** the verifier accepts and the proof holds no BLS12-381 commitment

#### Scenario: Status of LFP-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys (LOOP T10m). Rerun on the T25 keys. Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/harness.rs:26` sets the engine. ARCHITECTURE.md section 1.6 row 1 names the field (Moriarty 3dc9001a9 `research/sherlock-final-design-20261111/ARCHITECTURE.md:184`).

### Requirement: LFP-002 One app proof and one leaf-layer proof per shard
LFP-002: When the leaf prover receives an `RslStatement` and its `ExecutionTrace`, the leaf prover SHALL make, for each shard, one app proof under R_SL chip set 3 and one leaf-layer proof whose N1 pre-hash equals the universal N1 pre-hash.

Verification: run `t7_chain rsl OUT` with `RSL_PROBE` set, for every suite call; the record field `n1_pre_hash_equals_universal_n1` is true (openvm `rekey-chips` dcf1b4c `t7_chain.rs:2040`). Exists.

#### Scenario: A suite call proves under the universal node
- **GIVEN** the R_SL program of suite call `d04 withdraw` with 9 AIRs
- **WHEN** the leaf prover proves the app shard and the leaf layer
- **THEN** the N1 pre-hash equals the universal N1 pre-hash

#### Scenario: A second program needs no new universal key
- **GIVEN** leaf-layer proofs of `a05` and `z02`
- **WHEN** N1 verifies each
- **THEN** both give the universal N1 pre-hash and one internal-recursive key verifies both N1 proofs

#### Scenario: Status of LFP-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL: 5 calls MEASURED (a05, t02, s01, d04, z02). 137 calls open. Owner lane: rekey-chips, then T25. Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/rsl/mod.rs:1` names chip set 3. The probe `t7_chain rsl OUT` (openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:2060`) makes one app key and one leaf-layer key per program (`:2024`, `:2030`) and compares the N1 pre-hash with the universal one (`:2034` to `:2040`). App and leaf-layer keys are per program under `key_root` (SPL-004); "universal leaf key" in LOOP T24 and T25 names this pre-hash property. LOOP T23 records "leaf over R_SL fails LogUp"; `~/.cache/rekey-t11/LIST.md:33` records the odin r10 probe as equal for 5 calls. Reworded after Astra-B review R-06.

### Requirement: LFP-003 An app shape over 8 AIRs is refused
LFP-003: If the app shape of an R_SL program needs more than 8 app AIRs, then the leaf prover SHALL refuse the shape before it makes a proof.

Verification: tests `rsl_air_count_in_universal_leaf_bucket` (openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/tests/rsl.rs:191`) and `rsl_sha_and_keccak_in_one_program_refused` (openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/tests/rsl_keccak.rs:150`). Exists.

#### Scenario: SHA-256 and Keccak-256 in one program
- **GIVEN** a program that calls `persistentHash` and `keccak256`
- **WHEN** the leaf prover builds the chip set
- **THEN** the leaf prover refuses the program and writes no proof

#### Scenario: Status of LFP-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT; no MEASURED run of the keccak tests on file (audit A3). GAP against GP-001: a program with both hash kits has no proving route. D-024 (slice A, OWNER 2026-11-15) holds this as a temporary limit; a route (a second chip-set bucket or a split) is open. Owner lane: rekey-chips. Astra-B review R-08, R-09. Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/rsl/trace.rs:49` sets `MAX_APP_AIRS = 8`; the assert is at `trace.rs:137`; a program with SHA-256 and Keccak-256 is refused at `trace.rs:114` (`~/.cache/rekey-t11/LIST.md:34`).

### Requirement: LFP-004 App key bounds
LFP-004: If an app verifying key has more than 30,000 columns, then the leaf harness SHALL refuse the key.

Verification: NEW: a unit test that gives `check_vk_bounds` an app key of 30,001 columns and expects the refusal. To build.

#### Scenario: An app over the column bound
- **GIVEN** an app key of 30,001 columns
- **WHEN** the leaf harness checks the key
- **THEN** the harness refuses the key and names the column bound

#### Scenario: Status of LFP-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (the check). The negative test is a gap. Owner lane: rekey-chips. Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/frozen.rs:98` holds the bound; `frozen.rs:437` enforces it in `check_vk_bounds`; openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/harness.rs:55` calls it. The largest measured R_SL app has 16,346 columns (`~/.cache/rekey-t11/LIST.md:30`).

### Requirement: LFP-005 Fixed shard shapes
LFP-005: The leaf prover SHALL prove every leaf shard with the ordered AIR height vector of its admitted shape.

Verification: gate V.10 `scripts/qa/check-chain.sh chain`: the record field `fixed` is true for every leaf. NEW: negative cases with two swapped AIR heights and with one oversized AIR; a fixed-shape check for chip set 3. Partly to build.

#### Scenario: A longer run adds shards
- **GIVEN** a run that fills 2 leaf shards
- **WHEN** the run length doubles
- **THEN** the shard count grows and every shard has the same height vector

#### Scenario: Two heights are swapped
- **GIVEN** a leaf trace with the heights of AIR 1 and AIR 2 exchanged
- **WHEN** the leaf is proved
- **THEN** the proof does not verify under the leaf-layer key

#### Scenario: Status of LFP-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT for chip sets 0 to 2 on the T10m keys (LOOP T6). GAP: chip set 3 has no named shape vector and no negative case. Owner lane: rekey-chips. Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/leaf.rs:33` to `:35` (`HEIGHTS`, seven heights in AIR order) and `leaf.rs:631` to `:635` (`HEIGHTS_V`, chip set 2). The chain checks the heights at openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:908`; the slice chain asserts them at `crates/sherlock-wrap-export/examples/slice_chain.rs:574`. R_SL records its app and leaf shapes at `t7_chain.rs:2038`; no fixed vector is named for chip set 3. Reworded after Astra-B review R-32.

### Requirement: LFP-006 The leaf exposes prog_d
LFP-006: The leaf AIR SHALL constrain the public value slot `PROG_D` to the `prog_d` of the program that the shard runs.

Verification: test `anchor_d2_is_the_progc_operand` (openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/stmt.rs:2267`) and the cases of LFP-007. Exists.

#### Scenario: The slot holds the program digest
- **GIVEN** a leaf proof of program A
- **WHEN** N1 opens slot `PROG_D`
- **THEN** the value equals the `prog_d` of program A

#### Scenario: Status of LFP-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (LOOP T7). Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/core.rs:41` (slot 8), `core.rs:599` (constraint), `core.rs:1122` (fill). N1 opens the slot at openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/stmt.rs:339`.

### Requirement: LFP-007 A wrong prog_d is refused
LFP-007: If a leaf proof carries a `prog_d` that differs from the `prog_d` of the run, then the N1 statement AIR SHALL refuse the leaf.

Verification: `scripts/qa/check-chain.sh cases` runs `t7_chain cases`; both cases report refused. Exists.

#### Scenario: A leaf of another program
- **GIVEN** a run of program A and one leaf proof of program B
- **WHEN** N1 merges the leaves
- **THEN** N1 refuses the merge

#### Scenario: Status of LFP-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (LOOP T7). Rerun on the T25 keys. Evidence: Runtime cases `wrong_prog_d_merge_refused_at_N1` (openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:1138`) and `wrong_prog_d_opening_refused_at_N1` (`t7_chain.rs:1159`).

### Requirement: LFP-008 Longer runs keep the leaf key
LFP-008: When a run has more steps than one leaf shape holds, the leaf prover SHALL add leaf shards and leave every per-program key and every universal key unchanged.

Verification: run LPC-007: one program at 1x, 2x, 4x and 8x run length; `key_root` and the universal key manifest sha256 are equal in the four records. Exists.

#### Scenario: Run length times 8
- **GIVEN** one R_SL program and inputs of 1x and 8x length
- **WHEN** both runs are proved
- **THEN** the shard counts differ and `key_root` is equal

#### Scenario: Status of LFP-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BLOCKED until T25. Owner lane: t16-elastic. Evidence: LOOP T16 (c) is BLOCKED: "leaf proves 5 rules; longer programs need a rekey". SC-003 of `~/Projects/Moriarty-wt/t16-elastic/research/sherlock-t16-elastic-20261114/SETUP-SPEC.md:33` cuts a run into `ceil(n / S)` leaves.

### Requirement: LFP-009 Leaf result event
LFP-009: When the leaf prover completes the leaves of a run, the leaf prover SHALL publish one `LeafProofsReady` record that names each leaf proof file, its sha256, its public values and the `LeafKeyRef` of the run.

Verification: NEW: a schema check of the run record in the T23 chain runner. To build.

#### Scenario: The node stage reads the record
- **GIVEN** a completed leaf stage of 3 shards
- **WHEN** the node prover starts
- **THEN** the node prover reads 3 proof paths with sha256 from the record

#### Scenario: Status of LFP-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: the record exists, the published type does not. Owner lane: t23-suite. Evidence: The chain driver writes one JSON record per run (openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:2008` for the rsl mode). No record has the name `LeafProofsReady` today.

### Requirement: LFP-010 The app proof binds its statement into the recursion statement
LFP-010: If the outcome commitment in the FP-1 public values of an R_SL app proof differs from the digest of `comm` and the ordered W of the PUBLIC rows of that proof, then the R_SL app AIR SHALL refuse the trace.

Verification: the negative test T-RSL-STMT-BIND of ERS-012 on a real proof; to build.

#### Scenario: FP-1 words of a second outcome
- **GIVEN** an honest app trace of `t02 increment` with the FP-1 words of another outcome
- **WHEN** the app proof is made
- **THEN** the constraint check fails

#### Scenario: Status of LFP-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN, risk HIGH: the change adds constraints to the R_SL AIR 0 and joins `~/.cache/rekey-t11/LIST.md` before T25. Owner lane: rekey-chips. Evidence: Proving side of ERS-012 (slice A): FP-1 has 54 words, so the binding is a digest. openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/rsl/air.rs:199` to `:202`: "binding `comm ++ W` into the FP-1 words is the statement lane's work (GAP)"; the FP-1 probe carries zeros (`air.rs:216` to `:222`). Test `rsl_fp1_public_values_on_air0_only` (openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/tests/rsl.rs:171`) checks the position only. No recursion proof authenticates a Compact outcome today.

### Requirement: LFP-011 The R_SL leaf carries the prog_d of the program
LFP-011: When the leaf prover proves a shard of an R_SL program, the leaf prover SHALL set the public value slot `PROG_D` to the `prog_d` of ERS-006 for that program.

Verification: the test T-RSL-PROGD of ERS-006 and the cases `wrong_prog_d_*` of LFP-007 on an R_SL chain; to build.

#### Scenario: Two R_SL programs
- **GIVEN** leaf proofs of `t02 increment` and `a05`
- **WHEN** N1 opens slot `PROG_D`
- **THEN** the two values differ and each equals the planner value

#### Scenario: Status of LFP-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. Owner lanes: rekey-chips (leaf), mycroft-factory (the planner). Evidence: Proving side of ERS-006 (slice A): R_SL programs have no `prog_d` today (bus log 09:56, Fable-A). The slot and its constraint exist for the interpreted leaf (openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/core.rs:41`, `core.rs:599`).

### Requirement: LFP-012 The R_SL leaf binds the input digest
LFP-012: If the digest of the tape that an R_SL app consumed differs from the `input_d` of the leaf statement, then the leaf AIR SHALL refuse the shard.

Verification: the negative test T-RSL-INPUT of ERS-007; to build.

#### Scenario: Another tape
- **GIVEN** an R_SL leaf statement with `input_d` of tape A and a trace that consumed tape B
- **WHEN** the leaf is proved
- **THEN** the constraint check fails

#### Scenario: Status of LFP-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. Owner lane: rekey-chips. Evidence: Proving side of ERS-007 (slice A): no INIT or `input_d` binding exists for R_SL (bus log 09:56, Fable-A). INIT kit: change `sherlock-modular-leaf-20261111`, spec `sherlock-init-admission` (INIT-001 to INIT-008).
