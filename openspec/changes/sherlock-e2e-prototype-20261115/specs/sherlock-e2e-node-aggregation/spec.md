## ADDED Requirements

### Requirement: NAG-001 Nodes are BabyBear STARKs at the internal set
NAG-001: The node prover SHALL prove N1, N2 and N3 as STARKs over BabyBear at the `INTERNAL` parameter set.

Verification: gate V.10 `scripts/qa/check-chain.sh chain`: the stock verifier accepts N1, N2 and N3 (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:89`). Exists.

#### Scenario: The chain of one run
- **GIVEN** a run of 3 leaves
- **WHEN** the node prover proves N1, N2 and N3
- **THEN** the stock verifier accepts the three proofs under the internal-recursive key

#### Scenario: Status of NAG-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys. Rerun on the T25 keys. Evidence: openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/fixture.rs:28` (`NodeSC`); openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/frozen.rs:130` (`INTERNAL`: log blowup 3, queries 97/82/81). Node kinds: openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/stmt.rs:585` (`enum Kind`: `First` is N1, `Node` is N2 and N3).

### Requirement: NAG-002 A proof without a required AIR is refused
NAG-002: If a child proof omits `StmtAir`, `ProofShapeAir` or `PublicValuesAir`, then the parent node SHALL refuse the child.

Verification: gate V.9 `scripts/qa/check-chain.sh omit`: 70 cases (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:89`). Exists.

#### Scenario: N1 without the statement AIR
- **GIVEN** an N1 proof made without `StmtAir`
- **WHEN** N3 verifies its children
- **THEN** N3 refuses the child and the stock verifier refuses the proof

#### Scenario: Status of NAG-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys. Open part: LPC-018 (the parent constraints that never fired). Rerun on the T25 keys. Evidence: openvm `t10mm` 5b7ada8 `crates/sherlock-wrap-export/examples/t7_chain.rs:1209` (mode `omit`), `t7_chain.rs:1263` (the omitted list). Moriarty 3dc9001a9 `research/sherlock-t3-airs-20261113/RESULTS.md:10`: 30 of 30 refused on the fp8 tree.

### Requirement: NAG-003 N1 opens prog_d and forwards key_root
NAG-003: The N1 statement AIR SHALL open `prog_d` as `P2(DOM_PROG, image_d, plan_d, key_root, machine_version)` and forward `key_root` to its parent.

Verification: cases `wrong_prog_d_opening_refused_at_N1` (openvm `t10mm` 5b7ada8 `crates/sherlock-wrap-export/examples/t7_chain.rs:1118`) and `wrong_prog_d_merge_refused_at_N1` (`t7_chain.rs:1097`) in `scripts/qa/check-chain.sh cases`. Exists.

#### Scenario: A changed key_root
- **GIVEN** an N1 witness with one changed word of `key_root`
- **WHEN** N1 opens `prog_d`
- **THEN** the opening constraint fails

#### Scenario: Status of NAG-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (LOOP T7). Evidence: openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/stmt.rs:909` to `:915`.

### Requirement: NAG-004 Mixed chip sets under one key_root
NAG-004: When the leaves of one run use more than one chip set of one `key_root`, the node prover SHALL merge the leaves under N2 and N3.

Verification: `scripts/qa/check-chain.sh cases`: both cases accept. NEW after T25: one case with chip set 3 and chip set 0 leaves. Partly to build.

#### Scenario: An R_SL leaf beside a base leaf
- **GIVEN** one run with a chip set 0 leaf and a chip set 3 leaf
- **WHEN** N2 merges them
- **THEN** N2 accepts and the stock verifier accepts N2

#### Scenario: Status of NAG-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT for chip sets 0 to 2. The chip set 3 case is a gap. Owner lane: rekey-chips. Evidence: Cases `mixed_chip_sets_one_key_root_N2_accepts` (openvm `t10mm` 5b7ada8 `crates/sherlock-wrap-export/examples/t7_chain.rs:1081`) and `mixed_chip_sets_N3_accepts` (`t7_chain.rs:1084`).

### Requirement: NAG-005 MERGE-WORK has at most 5,400 columns
NAG-005: If a MERGE-WORK key has more than 5,400 columns, then the key generator SHALL refuse the key.

Verification: gate `d3work` (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/run.sh:33`) and the probe at openvm `t10mm` 5b7ada8 `crates/sherlock-wrap-export/examples/t8_gp.rs:785`. NEW: a refusal check with a 5,401-column key. Partly to build.

#### Scenario: A key over the bound
- **GIVEN** a MERGE-WORK key of 5,401 columns
- **WHEN** the key generator checks the bounds
- **THEN** the key generator refuses the key with `G1Error::Columns`

#### Scenario: Status of NAG-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. OPEN: the width of the complete kernel with STACKS and VALUE DIGEST is DERIVED, not MEASURED (LPC-002). Owner lane: T8c. The bound does not change without the owner. Evidence: openvm `rekey-int` 3826e38 `crates/sherlock-lfr/src/frozen.rs:169` (`MERGE_WORK_COLUMNS = 5400`; `t10mm` 5b7ada8 `frozen.rs:166`), enforced by `check_vk_bounds` at `frozen.rs:429` (`t10mm` 5b7ada8 `frozen.rs:424`). Owner ruling 2026-11-14: "MERGE-WORK bound 5,400 columns" (LOOP, "Owner rulings 2026-11-14"). Width: the current MEASURED width is 4,988 columns (the T10q Work kind without the GP block and `fv_state`, `t7_chain shape`); with STACKS (71 words, ESTIMATE) and the value-set CORE (+62, MEASURED) it is 5,121 DERIVED (openvm `rekey-int` 3826e38 and `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/frozen.rs:166` to `:168`). Earlier layouts: 5,128 MEASURED in T8 (`frozen.rs:165`) and 5,284 MEASURED in T8b on the T10m keys (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:51`, `:96`). The earlier figure 5,417 (LOOP lead decisions) is superseded. Corrected after Astra-B review R-10.

### Requirement: NAG-006 The T10q forgeries are refused
NAG-006: If a node witness is one of the forged inputs MW-F01, AFK-F7, GP2-F4.2 or the unlinked one-child MERGE-WORK node of AFK-F9(b), then the node AIRs SHALL refuse the witness.

Verification: `cargo test -p sherlock-leaf --test quint_regressions` on the T25 tree: the 6 refusal tests pass and none has `#[ignore]`. Exists.

#### Scenario: A false carry in the work statement
- **GIVEN** the MW-F01 witness with `h_hi` outside 16 bits
- **WHEN** the Work AIR checks the row
- **THEN** the range check fails

#### Scenario: Status of NAG-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on `t10q` (LOOP T10q). Rerun on the T25 keys. Evidence: openvm `t10q` 0082832 `crates/sherlock-leaf/tests/quint_regressions.rs` (fix commit beba730): `mw_f01_work_whi_false_carry_refused` (`:165`), `mw_f01_work_whi_wrap_bypass_refused` (`:170`), `mw_f01_work_whi_wrap_bypass_run_refused` (`:175`), `afk_f7_n2_shape_binds_the_cut` (`:185`), `gp2_f42_const_vd_binds_the_type_name` (`:195`), `afk_f9_work_one_child_unlinked_refused` (`:348`). Moriarty 3dc9001a9 `tests/quint-regressions/COUNTEREXAMPLES.md:56`, `:61`. Corrected after Astra-B review R-02: AFK-F6 and the linked one-child node are honest inputs (NAG-011).

### Requirement: NAG-007 The open counterexample GP2-C2.1
NAG-007: If a node witness is the forged input of counterexample GP2-C2.1, then the node AIRs SHALL refuse the witness.

Verification: the test `gp2_c21_pubv_checks_the_constant_row` without `#[ignore]`, failing before the fix and passing after. Exists.

#### Scenario: The constant row
- **GIVEN** the GP2-C2.1 witness
- **WHEN** the node AIR checks the public value row
- **THEN** the constraint fails

#### Scenario: Status of NAG-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: a CONFIRMED defect with no fix. If the fix changes a key, the fix joins `~/.cache/rekey-t11/LIST.md` before T25. Owner lane: t10q. Evidence: openvm `t10q` 0082832 `crates/sherlock-leaf/tests/quint_regressions.rs:201` (`gp2_c21_pubv_checks_the_constant_row`, still `#[ignore]`). Moriarty 3dc9001a9 `tests/quint-regressions/COUNTEREXAMPLES.md`: verdict CONFIRMED for GP2-C2.1.

### Requirement: NAG-008 The outcome is bound in L
NAG-008: If the outcome word of a run differs from the outcome that the leaves proved, then the `L` circuit SHALL refuse the node root.

Verification: `scripts/qa/check-chain.sh chain`: the three cases report refused. Exists.

#### Scenario: A changed outcome
- **GIVEN** an honest node root and an outcome word changed by 1
- **WHEN** `L` is proved
- **THEN** the `L` constraint check fails

#### Scenario: Status of NAG-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (LOOP T7). Evidence: Cases `outcome_word_changed_refused_at_L` (openvm `t10mm` 5b7ada8 `crates/sherlock-wrap-export/examples/t7_chain.rs:908`), `non_terminal_outcome_refused_at_L` (`t7_chain.rs:912`) and `forged_outcome_operand_refused_at_L_Fr` (`t7_chain.rs:935`). Moriarty 3dc9001a9 `research/sherlock-t7-bind-20261113/RESULTS.md:13`: X10 is in `L`, not in a SEAL node; `RESULTS.md:42`: FP-2 node statement done.

### Requirement: NAG-009 Children link by step interval and state digest
NAG-009: The N1 and N2 merge constraints SHALL check that the step interval of each child starts where the interval of the child before it ends and that its `pre_d` equals the `post_d` of the child before it.

Verification: run `t7_chain t16 join` (openvm `t16-elastic` 1baf605 `t16.rs:815`); cases `join_L_proof_stock_verifier` (`t16.rs:797`) and `join_claim_pub_equals_both_hosts` (`t16.rs:799`). NEW: a negative case with two leaves in the wrong order. Partly to build.

#### Scenario: Leaves in the wrong order
- **GIVEN** leaf 2 placed before leaf 1
- **WHEN** N1 merges them
- **THEN** the `post_d` to `pre_d` link constraint fails

#### Scenario: Status of NAG-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT for odin with huginn. The negative case and the muninn join are gaps. Owner lane: t16-elastic. Evidence: SC-014 of `SETUP-SPEC.md` (Moriarty `t16-elastic` a7a7b8867, `research/sherlock-t16-elastic-20261114/`) (`SETUP-SPEC.md:46`). The join run of leaves from odin and huginn: openvm `t16-elastic` 1baf605 `crates/sherlock-wrap-export/examples/common/t16.rs:762` (`fn join`), Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:66` (5 of 5 cases).

### Requirement: NAG-010 Node proofs enter the proof bundle
NAG-010: When the node root of a run is proved, the node prover SHALL add each node proof path, its sha256 and its public values to the `ProofBundle` record of the run.

Verification: NEW: a schema check of `ProofBundle` in the T23 chain runner (fields: leaf, N1, N2, N3, `L`, `L_Fr` entries, `claim_pub`, key manifest sha256). To build.

#### Scenario: The user flow reads one record
- **GIVEN** a completed run
- **WHEN** slice A reads the result
- **THEN** one `ProofBundle` names every proof of the run

#### Scenario: Status of NAG-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: the record exists, the published type does not. Owner lane: t23-suite. Evidence: The chain record of `t7_chain chain OUT`; example Moriarty 3dc9001a9 `research/sherlock-t7-bind-20261113/runs/chain-A.json`. No record has the name `ProofBundle` today.

### Requirement: NAG-011 The honest T10q inputs are accepted
NAG-011: When an N1 witness carries a nonzero `reg_root` or a MERGE-WORK node has one linked child, the node AIRs SHALL accept the witness.

Verification: the two tests above on the T25 tree; honest shards still prove and verify (LOOP T10q done-when). Exists.

#### Scenario: A registry epoch after the first
- **GIVEN** an honest N1 witness with `reg_root` not zero
- **WHEN** the N1 AIR checks the row
- **THEN** the constraints hold and the stock verifier accepts N1

#### Scenario: Status of NAG-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on `t10q`. Rerun on the T25 keys. Evidence: openvm `t10q` 0082832 `crates/sherlock-leaf/tests/quint_regressions.rs:180` (`afk_f6_first_carries_nonzero_reg_root`) and `:190` (`afk_f9_work_one_child`); the manifest rows have `expect = accept` (Moriarty 3dc9001a9 `tests/quint-regressions/manifest.json:214`, `:235`). LOOP T10q: "AFK-F6 nonzero `reg_root` accepted in N1; AFK-F9(b) one-child MERGE-WORK node". Added after Astra-B review R-02.
