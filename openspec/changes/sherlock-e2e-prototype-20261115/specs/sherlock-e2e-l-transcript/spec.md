## ADDED Requirements

### Requirement: LTR-001 L is a BabyBear STARK with Fr hashing
LTR-001: The `L` prover SHALL prove `L` as a STARK over BabyBear whose Merkle tree and transcript use Midnight Poseidon over the BLS12-381 scalar field.

Verification: gate V.10 `scripts/qa/check-chain.sh chain`: the stock verifier accepts the `L` proof (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:89`). Exists.

#### Scenario: The stock verifier accepts L
- **GIVEN** an `L` proof of a 3-leaf run
- **WHEN** the stock stark-backend verifier checks it under the `L` key
- **THEN** the verifier accepts

#### Scenario: Status of LTR-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys. Rerun on the T25 keys. Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/fixture.rs:26` sets `LSC = BabyBearBls12381PoseidonConfig`. The `L` parameter set is at openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/frozen.rs:180`.

### Requirement: LTR-002 L_Fr is a STARK over Fr with no extension
LTR-002: The `L_Fr` prover SHALL prove `L_Fr` as a STARK whose base field is the BLS12-381 scalar field with no extension field.

Verification: tests `i10_gate_at_lfr_params` (openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/tests/i10_gate.rs:83`) and `j5_gate_at_lfr_params` (openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/tests/j5_gate.rs:180`). Exists.

#### Scenario: L_Fr verifies L
- **GIVEN** an accepted `L` proof
- **WHEN** the `L_Fr` prover proves the verification of `L`
- **THEN** the stock verifier accepts the `L_Fr` proof under the `L_Fr` key

#### Scenario: Status of LTR-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (LOOP T1, T3). Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/testing.rs:23` (`Bls12381FrPoseidonConfig`); feature `bls12381-fr-poseidon` at openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/Cargo.toml:11`; parameter set `LFR` at openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/frozen.rs:203`.

### Requirement: LTR-003 L key bounds
LTR-003: If an `L` key has more than 2,200 columns or an AIR with more than 100 interactions, then the key generator SHALL refuse the key.

Verification: T25 keygen record: the `L` column count and the largest interaction count, checked by `check_vk_bounds`. Exists.

#### Scenario: The rekey adds columns
- **GIVEN** an `L` key of 2,201 columns after the FP-3 change
- **WHEN** the key generator checks the bounds
- **THEN** the key generator refuses the key and the rekey stops

#### Scenario: Status of LTR-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. The bound is 2,200 columns (F-B-BR-1, LEAD PICK FP3-LCOLS for T15; the bound was 2,000 before FP-3). FP-3 replaces the bare `Call` AIR with `EffectAir` of 77 columns (MEASURED, `~/.cache/rekey-t11/LIST.md:44`), so `L` with FP-3 is 2,071 columns (DERIVED, `LIST.md:47`), 129 columns below the bound. The width after T25 is not measured. T25 measures it, reruns D3 at that width (`LIST.md:69`), and keeps k <= 21 and the wrap VK at or below 50,000 B. Owner lane: T25. Evidence: openvm `rekey-int` 3826e38 `crates/sherlock-lfr/src/frozen.rs:202` (`max_columns: 2200` in the `L` set at `frozen.rs:183`); `check_vk_bounds` is at `frozen.rs:429`. Measured on the T10m keys: 1,995 columns and at most 99 interactions (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:11`, Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:49`).

### Requirement: LTR-004 Public words go through Tape expose
LTR-004: If an `L_Fr` tape does not expose the public words through `Tape::expose`, then the `L_Fr` machine SHALL refuse the tape.

Verification: runs `e4_profile_lfr_prove`, `exp-lfr-real` and `i5_lfr_real_native` (LOOP T3: pass) as the positive side. NEW: a negative test with one valid tape built without `Tape::expose`, paired with the same tape after the call. Partly to build.

#### Scenario: A tape without the expose call
- **GIVEN** a tape that writes the public words with a plain write
- **WHEN** the machine is built
- **THEN** the machine refuses the tape

#### Scenario: Status of LTR-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (LOOP T3). The negative test is a gap (Astra-B review R-14). Owner lane: rekey-chips. Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/tape.rs:327` (`Tape::expose`). LOOP T1 records the panic "expose the public words with Tape::expose"; LOOP T3 records the three runs that pass after the call was added.

### Requirement: LTR-005 The tape replays the L transcript
LTR-005: The `L_Fr` tape SHALL replay the Fiat-Shamir transcript of `L` with outputs equal to the outputs of the stock transcript.

Verification: tests `i5_transcript_equals_stock_and_proves` (openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/tests/i5_transcript.rs:132`) and `i5_l_transcript_on_tape` (openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/tests/i5_l_transcript.rs:60`). Exists.

#### Scenario: One challenge differs
- **GIVEN** an `L` proof and a tape with one changed challenge word
- **WHEN** the `L_Fr` machine checks the tape
- **THEN** the constraint check fails

#### Scenario: Status of LTR-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Part of G3 (SQG-014, SSG-006). Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/transcript.rs:31` (`TapeTranscript`), used at openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/lverify.rs:147`.

### Requirement: LTR-006 L_Fr result record
LTR-006: When the `L_Fr` prover completes a proof, the `L_Fr` prover SHALL publish one `LfrProofReady` record with the proof file path, the field `proof_sha256`, the `L_Fr` key pre-hash, the public words and `claim_pub`.

Verification: NEW in the T23 chain runner: sha256 of the proof bytes equals `proof_sha256`; the public words and the key pre-hash of the record equal those of the verified proof; cases with a corrupted proof and a wrong path. To build.

#### Scenario: The wrap reads the record
- **GIVEN** a completed `L_Fr` proof on odin
- **WHEN** the file moves to muninn
- **THEN** the sha256 of the moved proof bytes equals `proof_sha256` of the record

#### Scenario: A corrupted transfer
- **GIVEN** a proof file with one changed byte
- **WHEN** muninn checks the record
- **THEN** the digest differs and the wrap does not start

#### Scenario: Status of LTR-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: the files exist, the published type does not. Owner lane: t23-suite. Evidence: The chain driver writes `lfr_pk.bitcode` and the call files in mode `t10 key|args|call` (openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:1637`). No record has the name `LfrProofReady` today.

### Requirement: LTR-007 One L key on path C and on the full tree
LTR-007: The `L` prover SHALL use the same `L` key and the same `L_Fr` key for a one-leaf run on path C and for a run on the full tree.

Verification: gates `pathc` and `pathc-d3` (Moriarty 3dc9001a9 `research/sherlock-t10-bound-20261114/chain.sh:142`) and gate V.10 on one key directory. Exists.

#### Scenario: Two tree forms
- **GIVEN** a one-leaf call and a 3-leaf run
- **WHEN** both are proved
- **THEN** the `L` key sha256 and the `L_Fr` key sha256 are equal in both records

#### Scenario: Status of LTR-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys (path C 25/25, Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:93`). Rerun on the T25 keys. Evidence: LOOP T7c: "25 path C cases, one universal L and L_Fr key" (openvm 3607e95). Mode `t7_chain pathc` is `fn pathc` at openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:1775`.

### Requirement: LTR-008 The stock verifier checks L_Fr before the hand-off
LTR-008: If the stock verifier refuses an `L_Fr` proof, then the `L_Fr` prover SHALL publish no `LfrProofReady` record for that proof.

Verification: `scripts/qa/check-chain.sh cases`: the case reports refused. NEW: the runner test that no record is written. Partly to build.

#### Scenario: A forged anchor
- **GIVEN** an `L_Fr` proof made with a wrong `prog_d` anchor
- **WHEN** the stock verifier checks it
- **THEN** the verifier refuses and the wrap receives no record

#### Scenario: Status of LTR-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL: the refusal is BUILT; the no-record rule is a gap. Owner lane: t23-suite. Evidence: Runtime case `wrong_prog_d_anchor_refused_by_stock_L_Fr_verifier` (openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:1040`).
