## ADDED Requirements

### Requirement: WRP-001 One BLS12-381 wrap proof for each ledger call
WRP-001: When an `LfrProofReady` event arrives for a ledger call, the wrap prover SHALL produce exactly 1 midnight-proofs PLONK proof with KZG over BLS12-381 and the Blake2b transcript.

Verification: T17 (a) honest call, `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:21`; rerun on the T25 keys in row T23 (STL-002); exists in part.

#### Scenario: One wrap proof settles one call
- **GIVEN** one exported `L_Fr` proof of program A
- **WHEN** `wrap <fixture> <out> 21` runs
- **THEN** the output folder holds one `wrap.proof` and the library verifier accepts it

#### Scenario: Status of WRP-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: MID-001, LW-002, LW-013 (sherlock-processor-final-20261111). Input event `LfrProofReady` is published by slice B (LTR). Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/src/bin/wrap.rs:23,79-81`. Check: T17 (a) honest call, `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:21`; rerun on the T25 keys in row T23 (STL-002).

### Requirement: WRP-002 Never k = 22
WRP-002: If a caller asks for a wrap circuit size above k = 21, then the wrap tool SHALL stop before keygen with the message `never k = 22`.

Verification: MISSING as a named test. The assert exists; task: add test `wrap_refuses_k22` in `tools/sherlock-wrap/tests/`; to build.

#### Scenario: k = 22 is refused
- **GIVEN** any wrap fixture
- **WHEN** `wrap <fixture> <out> 22 cost` runs
- **THEN** the process stops with `never k = 22` and writes no key

#### Scenario: Status of WRP-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: hard rule "never k = 22" (LOOP.md:102, Moriarty main working copy 2026-11-15); DG-001 and LW-013 (processor-final) state the limit; this requirement adds the refusal of the wrap tool. Decision D-001. Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/src/bin/wrap.rs:32`. Check: MISSING as a named test. The assert exists; task: add test `wrap_refuses_k22` in `tools/sherlock-wrap/tests/`.

### Requirement: WRP-003 Row gate at k = 21
WRP-003: If the cost model of the wrap relation gives more than 1,782,573 rows at k = 21, then the wrap tool SHALL stop before keygen.

Verification: `tools/sherlock-wrap/tests/i5_whir.rs:124` (cost model against the gate); RKY-003 on the T25 keys; exists in part.

#### Scenario: The measured wrap fits the gate
- **GIVEN** the T10m wrap relation
- **WHEN** the cost model runs
- **THEN** it reports 1,685,845 rows, below 1,782,573

#### Scenario: Status of WRP-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: DG-002 (processor-final) states the gate and RKY-003 (completion) measures it on the T25 keys; this requirement adds the stop of the wrap tool. Decision D-083. Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/src/util.rs:21-22`, `src/bin/wrap.rs:45-49`; rows MEASURED in LOOP.md:31. Check: `tools/sherlock-wrap/tests/i5_whir.rs:124` (cost model against the gate); RKY-003 on the T25 keys.

### Requirement: WRP-004 Verifying key size bound
WRP-004: If the serialized wrap verifying key is larger than 50,000 bytes, then the wrap tool SHALL stop without writing `wrap.vk`.

Verification: MISSING (the guard and a test `wrap_vk_size_guard`). Until then RKY-004 is the only check; to build.

#### Scenario: The measured key is far below the bound
- **GIVEN** the T10m key set
- **WHEN** the wrap key is serialized in `SerdeFormat::Processed`
- **THEN** the key is 1,325 bytes and `wrap.vk` is written

#### Scenario: Status of WRP-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: MID-003 and DG-003 (processor-final) state the bound and RKY-004 (completion) measures it; this requirement adds a guard in the wrap tool. Evidence: the size is measured, not guarded: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/src/bin/wrap.rs:82-87` writes the key with no size check; 1,325 bytes MEASURED in LOOP.md:31. Check: MISSING (the guard and a test `wrap_vk_size_guard`). Until then RKY-004 is the only check.

### Requirement: WRP-006 Fresh blinders on every wrap proof
WRP-006: The wrap prover SHALL draw the blinders of each wrap proof from the operating system CSPRNG.

Verification: row T11 done-when "two blinded proofs differ byte for byte and both verify" (LOOP.md:33); status OPEN until T11 RESULTS section 4 lands; exists in part.

#### Scenario: Two proofs of one call differ
- **GIVEN** one `L_Fr` proof
- **WHEN** the wrap prover proves it twice in a production build
- **THEN** the two proofs differ byte for byte and both verify

#### Scenario: Status of WRP-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: LW-012, ZK-003 as fixed on spec-audit, ZK-014 (test-seed guard). Slice B (ZKE) owns the STARK side. Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/src/bin/wrap.rs:13,79`. Check: row T11 done-when "two blinded proofs differ byte for byte and both verify" (LOOP.md:33); status OPEN until T11 RESULTS section 4 lands.

### Requirement: WRP-007 The anchor slot is the key constant 15
WRP-007: When the wrap decoder checks the anchor prologue, the wrap circuit SHALL require the anchor read key in lane 6 and the chunk write key in lane 15 to equal the key constant 15.

Verification: test `t12_anchor_slot_15` (uncommitted); to build on the T25 keys: a control with a changed chunk write key and a control with an ordinary call that carries anchor writes (ANC-013, ANC-016); exists in part.

#### Scenario: A prover-chosen slot is refused
- **GIVEN** a call vector whose anchor read or chunk write addresses slot 0, 1 or 3
- **WHEN** the native decoder and MockProver run
- **THEN** both refuse at decode step 5b

#### Scenario: Status of WRP-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: ANC-001 as fixed on spec-audit, ANC-013 (read key and chunk write key), ANC-014, ANC-016 (an ordinary call with the anchor-write template in lanes 11 to 19 and 21 is refused), completion. Slice B (CLB) owns `iface.rs`. Decision D-021. Evidence: step 5b pins cell 0 today, `t10q-ntfix@95990a6d3:tools/sherlock-wrap/src/decode.rs:26`; NT-1 decoder refusal in the rekey list (`~/.cache/rekey-t11/LIST.md:6`); forged slots 1, 3, 15 refused on `t12-gate@90f9b4df9` (test `t12_anchor_slot`, 3 tests); the move to 15 is uncommitted on openvm `t12-slot15` (AUDIT.md item A7a). Check: test `t12_anchor_slot_15` (uncommitted); to build on the T25 keys: a control with a changed chunk write key and a control with an ordinary call that carries anchor writes (ANC-013, ANC-016).

### Requirement: WRP-008 The generated key equals the deployed key
WRP-008: If the generated wrap key differs from the key file named by `E6_EXPECT_VK`, then the wrap tool SHALL stop before writing an output file.

Verification: T10m devnet run (LOOP.md:31) used the check; MISSING a negative test with a different key file; exists in part.

#### Scenario: A stale key set is caught before submit
- **GIVEN** `E6_EXPECT_VK` names the key installed on the ledger
- **WHEN** the wrap tool runs on another key set
- **THEN** the run stops with "the wrap key differs"

#### Scenario: Status of WRP-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/src/bin/wrap.rs:84-86`. Check: T10m devnet run (LOOP.md:31) used the check; MISSING a negative test with a different key file.

### Requirement: WRP-009 Wrap output files
WRP-009: When the library verifier accepts a new wrap proof, the wrap tool SHALL write the 3 files `wrap.vk`, `wrap.proof` and `wrap.public.json` into the output folder.

Verification: T17 (a) and T10m runs. These three files are the published type `WrapOutput` in `bus/ddd-C.md`; exists.

#### Scenario: The ledger check reads the three files
- **GIVEN** an output folder of the wrap tool
- **WHEN** `sherlock-wrap-ledger-check <dir>` runs
- **THEN** the tool reads the three files and verifies the proof

#### Scenario: Status of WRP-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t10q-ntfix@95990a6d3:tools/sherlock-wrap/src/bin/wrap.rs:81-92`; `ledger-check/src/main.rs:37-41`. Check: T17 (a) and T10m runs. These three files are the published type `WrapOutput` in `bus/ddd-C.md`.

### Requirement: WRP-010 One universal wrap key
WRP-010: The Sherlock key set SHALL hold exactly 1 wrap verifying key for every program, both modes and every ledger deployment of one era.

Verification: STL-003 readback per deploy in row T23; status OPEN; exists in part.

#### Scenario: Two programs deploy with one key
- **GIVEN** the ledgers of two suite programs
- **WHEN** their installed `acceptRun` keys are read back from the indexer
- **THEN** both are byte-equal to the one wrap VK

#### Scenario: Status of WRP-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: MID-002 as narrowed by SLC-003 and SLC-004, LW-003 (1,026 public inputs), LW-005, STL-003; hard rule "one universal key set" (LOOP.md:102). Slice B (UKS) owns the STARK keys. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:17`. Check: STL-003 readback per deploy in row T23; status OPEN.

### Requirement: WRP-011 GPU and CPU wrap share one key
WRP-011: Where the wrap prover runs on a GPU, the wrap prover SHALL use a verifying key byte-equal to the CPU wrap key.

Verification: the w3b record; rerun on the T25 keys in LPC-005 (sherlock-completion-20261115); exists in part.

#### Scenario: The huginn GPU configuration keeps the key
- **GIVEN** the wrap daemon in configuration `all` on huginn
- **WHEN** muninn checks its proofs
- **THEN** 24 of 24 proofs pass ledger 9.1.0.1 and the VK is byte-equal to the CPU run

#### Scenario: Status of WRP-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: GPW-001, GPW-003, GPW-007 (sherlock-processor-final-20261111). Evidence: llm-wiki `wiki/log.md:4108` (2026-11-15 09:04, w3b KEEP, MEASURED on the T10m keys). Check: the w3b record; rerun on the T25 keys in LPC-005 (sherlock-completion-20261115).

### Requirement: WRP-012 The wrap checks are qualified on the final keys
WRP-012: When the wrap is qualified on the T25 key set, the wrap qualification record SHALL hold a pass result for each of the 5 requirements LW-003, LW-004, LW-005, LW-006 and LW-014.

Verification: MISSING on the T25 keys (no final-key record). Exists on T10m keys: wrap suite and E5 decoder Quint lane with 6 of 6 mutants caught (LOOP.md:35); to build.

#### Scenario: A missing replay check blocks the record
- **GIVEN** a T25 wrap whose record has no mutant result for the native log replay of LW-004
- **WHEN** the qualification record is closed
- **THEN** the record is incomplete and no deploy uses the key

#### Scenario: Status of WRP-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: LW-003 (call vector only, lane 0, lane 1, constrained zero padding), LW-004 (native replay of the field log, PATH included, T4), LW-005 (anchor and lanes, both modes), LW-006 (DAG key commitment), LW-014 (keygen refusals), LW-016 (qualification on the real interface), processor-final. This requirement adds only the final-key gate; the LW IDs stay normative. The deploy runner reads these 5 passes from the one gate record of PVW-013 (FAL-WRAP-1, LEAD PICK for T15). Evidence: T4 budget with every check, 1,676,222 rows (LOOP.md:21); T10m wrap 1,685,845 rows (LOOP.md:31); wrap suite 25/25 on T3 keys (LOOP.md:20). Check: MISSING on the T25 keys (no final-key record). Exists on T10m keys: wrap suite and E5 decoder Quint lane with 6 of 6 mutants caught (LOOP.md:35).
