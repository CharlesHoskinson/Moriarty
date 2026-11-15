## ADDED Requirements

### Requirement: ANC-013 The anchor slot is the key constant 15
ANC-013: The wrap circuit SHALL pin the anchor read key (lane 6) and the chunk write key (lane 15) to the key constant 15.

#### Scenario: The anchor slot is the key constant 15
- **WHEN** a prover writes slot 0 in lane 6
- **THEN** the wrap refuses the witness at the template pin

#### Scenario: Verification and status of ANC-013
- **WHEN** the status is reported
- **THEN** the source is T12-SLOT15, owner-confirmed 2026-11-15 (wiki log 09:04), the verification is `t12_anchor_slot_15` in `crates/sherlock-lfr/src/iface.rs` and `t12_anchor_slot` (3 tests, Moriarty `t12-gate` 9260e8b75), and the status on 2026-11-15 is: OPEN: openvm `t12-slot15` holds the change uncommitted. `decode.rs` and SherlockLedger are not changed.

### Requirement: ANC-014 SherlockLedger stores the anchor in slot 15
ANC-014: When the SherlockLedger contract is deployed, the SherlockLedger contract SHALL store `C_P` in top-level slot 15.

#### Scenario: SherlockLedger stores the anchor in slot 15
- **WHEN** the contract is deployed on the local devnet
- **THEN** slot 15 holds `C_P` and slot 0 is a contract field

#### Scenario: Verification and status of ANC-014
- **WHEN** the status is reported
- **THEN** the source is T12-SLOT15, owner-confirmed 2026-11-15, the verification is a deploy test in `tools/sherlock-sdk/sherlock-ledger.test.mjs` that reads slot 15 back, and the status on 2026-11-15 is: OPEN: no commit.

### Requirement: ANC-015 Compiler refusal of slot 15
ANC-015: If a contract's ledger layout uses top-level index 15, then the Mycroft compiler SHALL refuse the contract before deploy and name the colliding field.

#### Scenario: Compiler refusal of slot 15
- **WHEN** a contract has 16 top-level fields
- **THEN** the compiler refuses it and names field 15

#### Scenario: Verification and status of ANC-015
- **WHEN** the status is reported
- **THEN** the source is MYC-SEAL-021 (T18 (d)), T12-SLOT15, the verification is T-ANCHOR-DISJOINT census over every suite contract, and the status on 2026-11-15 is: OPEN: the census test is missing on the mycroft branches.

### Requirement: ANC-016 An ordinary call writes no anchor
ANC-016: If an ordinary call (mode 0) carries the anchor-write template in lanes 11 to 19 and 21, then the wrap circuit SHALL refuse the witness.

#### Scenario: An ordinary call writes no anchor
- **WHEN** an ordinary call writes the anchor at lane 11
- **THEN** MockProver refuses the wrap witness

#### Scenario: Verification and status of ANC-016
- **WHEN** the status is reported
- **THEN** the source is T10Q-NTFIX-ORD (LEAD PICK, in the single rekey list that the owner confirmed on 2026-11-15), the verification is `decode.rs` step 5b, MockProver refusal of NT-1 (Moriarty `t10q-ntfix` bb9dc9ae0), `t13_t10q_chunk` 9 of 9 (openvm e3b407d), and the status on 2026-11-15 is: DONE on `t10q-ntfix`, not merged into `t10m`.

### Requirement: ANC-017 The lock operand binds the run
ANC-017: When a chunk call reads or writes a lock, the `L_Fr` circuit SHALL compute the lock operand `x_j` of ANC-002 from the run's `c`, `run_tag`, `k`, `d_in`, `d_out`, the FP-3 link `pack(C_(j+1))` and `j`.

#### Scenario: The lock operand binds the run
- **WHEN** two runs share `run_tag` and differ in input
- **THEN** a chunk of one run cannot read a lock of the other

#### Scenario: Verification and status of ANC-017
- **WHEN** the status is reported
- **THEN** the source is owner canon `x_j` (2026-11-14), extension confirmed 2026-11-15, the verification is `t13_t10q_chunk` NT-2 and NT-3 tests pass after and fail before (openvm 6a00df3, e3b407d), and the status on 2026-11-15 is: DONE on `t10q-ntfix`.

### Requirement: ANC-018 FP-3: call lanes bound to guest effects
ANC-018: The `L` circuit SHALL bind every effect lane of the call vector to the guest effect word that the run wrote at that lane.

#### Scenario: FP-3: call lanes bound to guest effects
- **WHEN** an ordinary call writes the anchor one lane later
- **THEN** the `L` constraint set refuses the proof

#### Scenario: Verification and status of ANC-018
- **WHEN** the status is reported
- **THEN** the source is FP-3 lane, LEAD PICK (an `L` key change, joins the single rekey), the verification is `nt1_residual_shifted_write_is_fp3` un-ignored, fails before and passes after (openvm `fp3` d2abc38), and the status on 2026-11-15 is: OPEN: `crates/sherlock-lfr/src/fp3.rs` untracked in the openvm `fp3` tree. The t10q push waits on this row.

### Requirement: ANC-019 FP-3: k is the true slice count
ANC-019: When a run is cut into `k` chunks, the `L` circuit SHALL bind `k` to the slice count of the run through the forward claim chain.

#### Scenario: FP-3: k is the true slice count
- **WHEN** a prover claims `k = 2` for a run of 3 slices
- **THEN** the `L` constraint set refuses the proof

#### Scenario: Verification and status of ANC-019
- **WHEN** the status is reported
- **THEN** the source is FP-3 lane, LEAD PICK, the verification is `fp3_k_is_the_true_slice_count` fails before and passes after (openvm `fp3` d2abc38), and the status on 2026-11-15 is: OPEN.
