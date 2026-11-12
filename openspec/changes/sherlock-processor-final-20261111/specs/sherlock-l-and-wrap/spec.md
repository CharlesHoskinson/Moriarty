## ADDED Requirements

### Requirement: LW-001 `L` verifies the node root and exposes the statement
`L` SHALL verify the node root proof on the full tree or the leaf-layer proof on path C. `L` SHALL take the flattened field log and its tree shape as private witness. `L` SHALL expose the statement words, the anchor statement `(mode, j, k, a_in, a_out)`, `run_tag` for a chunk, `layout_d`, `pv` of `W_pv` words, the DAG commitment `X` and the call vector of `2 + L_TX` words as public values.

#### Scenario: Full tree and path C
- **WHEN** `L` proves a run on the full tree and a one-leaf call on path C
- **THEN** both proofs expose the same public-value layout, and path C adds the inclusion of `P2(DOM_KEYDAG, entry, path, enc(X))` into `key_root` (SPL-005)

#### Scenario: Log digest mismatch
- **WHEN** the private log does not hash to the onion digest `(d, n)` of the node root
- **THEN** the constraint checker refuses the trace of `L`

### Requirement: LW-002 The wrap verifies `L` under the key schedule
The wrap SHALL verify the proof of `L` under the frozen `L` key. The verifier of `L` in the wrap SHALL follow the key schedule `sched`, the transcript algorithm of HDM-004 and the one-sample WHIR order of HDM-006 (OD-18, OD-19). It SHALL NOT follow the structure of the proof.

#### Scenario: Verifier follows the proof
- **WHEN** the verifier of `L` takes its flush points from the proof and not from `sched`
- **THEN** the wrap circuit refuses the witness (mutant `wrap_l_transcript_from_proof`), and the transcript vectors T-FS give equal outputs on the CPU prover, the GPU prover, the stark-backend verifier and the wrap's verifier of `L` (HDM-009)

### Requirement: LW-003 The wrap public inputs are the call vector only
The wrap SHALL have exactly the call vector of `2 + L_TX` words as public inputs, with `L_TX` = 1,024 (OD-7, OD-15). Lane 0 SHALL be `binding_input`. Lane 1 SHALL be `comm`. Unused lanes SHALL be constrained zero.

#### Scenario: Forbidden public input
- **WHEN** a wrap key exposes `X`, `mode`, `j`, `k`, `run_tag`, `key_root`, plain `prog_d`, `pre_d`, `post_d`, a memory root, a tail digest, a length, an address or a word derived from the transcript of `L`
- **THEN** keygen refuses the key, and kit ZERO KNOWLEDGE records the leak (ZK-004)

#### Scenario: Public word flipped
- **WHEN** one public word of a real wrap proof is flipped
- **THEN** `validate_call` refuses the call (SRC-020)

### Requirement: LW-004 The wrap checks and replays the field log natively
The wrap SHALL check range and `Canon` on every logged value, a bounded quotient on every lazy reduction and a canonical recomposition on every squeeze. It SHALL absorb `pv` natively and bind it by one `eq` evaluation. It SHALL replay every entry natively. It SHALL check the offset sums and the padding rules. It SHALL require every entry of the reservation `R` (FLG-004, FLG-006).

#### Scenario: Log mutants
- **WHEN** a logged value is not canonical, a quotient is unbounded, a squeeze is not canonical, `pv` is unbound, a used entry follows a `NOP`, a padding value is not zero, or an anchor entry is removed
- **THEN** the wrap circuit refuses the witness (mutants `wrap_log_value_noncanon`, `wrap_quotient_unbounded`, `wrap_squeeze_noncanon`, `wrap_pv_unbound`, `wrap_nop_then_used`, `wrap_value_padding_nonzero`, `wrap_anchor_entry_removed`)

#### Scenario: Ordinary call and chunks
- **WHEN** the wrap proves an ordinary call, chunk 1, chunk `j` and chunk `k`, and the forced-overflow cases of frontend M4 with `R` reserved
- **THEN** MockProver accepts each wrap witness

### Requirement: LW-005 One wrap checks the anchor and the lanes for both modes
The wrap SHALL require the `popeq` lane to equal `a_in`. `a_in` and `a_out` SHALL be outputs of `DIGEST` entries of the log. The `a_in` tag SHALL be `SHK_PROGC` when `mode` is ORD or `j = 1`, else `SHK_LOCK`. The `a_out` tag SHALL be `SHK_PROGC` when `mode` is ORD or `j = k`, else `SHK_LOCK`. Lane 0 SHALL equal `bind`. `layout_d` SHALL equal the key constant (OD-5, OD-16).

#### Scenario: Anchor or lane forged
- **WHEN** an anchor tag is swapped, the `popeq` lane is not `a_in`, lane 0 is not `bind`, or `layout_d` differs from the key constant
- **THEN** the wrap circuit refuses the witness (mutants `wrap_anchor_tag_swapped`, `wrap_popeq_lane_not_a_in`, `wrap_lane0_not_bind`, `wrap_layout_d_other`)

### Requirement: LW-006 The wrap checks the DAG key commitment
The wrap SHALL check `Canon` on the limbs of `X`. The wrap SHALL require `X` to equal the cached-trace commitment of `L` that it reads.

#### Scenario: Other DAG commitment
- **WHEN** `X` differs from the cached-trace commitment of `L`
- **THEN** the wrap circuit refuses the witness (mutant `wrap_x_not_cached_commitment`)

### Requirement: LW-007 A general run writes its run statement as effect words
A general run SHALL be a Sherlock program whose effect words write its run statement `[C_P, input_pub, outcome_pub]` (OD-15), with the public digest forms of FPR-010. The statement SHALL refuse Running. `n_steps` SHALL NOT be public (FPR-013). No state digest, memory root, tail digest, length, address, plain `prog_d` or `result_be` SHALL be public (LW-003, ZK-002). The public program identity `C_P` is LEAD F-7 under owner item N-11.

#### Scenario: Every outcome
- **WHEN** a sealed Halted, Rejected or Exhausted run is proved with every admitted rejection code and exhaustion kind
- **THEN** the effect words carry exactly the three values (positive `pos_decider_all_outcomes`), and DEC-L1 (O34, `OutcomeStatement.lean`) is open

#### Scenario: Forged statement
- **WHEN** `n_steps` enters the claim or the call vector, a state digest is public, or `n_steps = 2^64`
- **THEN** the proof or the wrap key is refused (mutants `decider_steps_in_claim`, `decider_public_state_digest`, `decider_nsteps_2p64`, keygen refusal `decider_nsteps_public`)

### Requirement: LW-008 The run statement comes from a sealed SET head
The run statement SHALL come from a SET root sealed by NODE INTEGRATION. The opening layer SHALL recompute `claim_d`. It SHALL open the root with one hash as `LEAF(claim_d, n_steps)` or `DEP(claim_d, n_steps, cover_root, in_link)`. It SHALL refuse a SEG, `JOIN`, `FOREIGN`, `ADM` or `VIEW` root. It SHALL check no seal predicate. The opening layer SHALL be `L` (FPR-007, LEAD F-6).

#### Scenario: Unsealed root or forged head
- **WHEN** a SEG root is offered as a run, or the SET head preimage differs from the run statement
- **THEN** the proof is refused (mutants `decider_seg_root_as_run`, `decider_set_head_forged`), and the seal predicates stay in NODE INTEGRATION (NODE-003)

### Requirement: LW-009 The collection statement carries no claim count
A collection SHALL publish `[reg_root, set_root, shape_d]` through its effect words, with a schema separate from the run statement. No claim count SHALL be public, range-checked or used for soundness (AD-2, OWNER). The entry label SHALL discriminate the collection statement from the run statement (FPR-008). The call-vector lanes and the contract-side form of a collection are open (FPR-009).

#### Scenario: Three runs joined
- **WHEN** three sealed runs of different programs are joined and wrapped
- **THEN** the statement is exactly `[reg_root, set_root, shape_d]` (positive `pos_decider_collection_three_runs` as a real proof), and each claim opens through the LEDGER CONTRACT (LDG-004)

#### Scenario: Stale registry root
- **WHEN** the call carries a registry root other than the contract's
- **THEN** the call is refused (Quint invariant `StaleOrForeignRegRootRefused`)

### Requirement: LW-010 Replay is refused by `claim_d` only
The replay identity SHALL be `claim_d = P2(SHK_RUN, 1, prog_d, input_d, outcome_d)`, without `n_steps` and without any physical field (SRC-007, SRC-023). The contract SHALL key it by its public image `claim_pub = transientHash([C_P, input_pub, outcome_pub])` (FPR-010). Every partition of one run SHALL have the salt of its `input_d`, so every partition SHALL have one `claim_pub`.

#### Scenario: Second partition
- **WHEN** the same run is proved under a second leaf partition and submitted
- **THEN** the second call is refused, and the Quint invariant `PartitionReplayRefusedByClaimD` holds

### Requirement: LW-011 Mid-run consumption binds the registry root
If AD-20 selects option (b), the run statement SHALL either refuse a `DEP` head whose cover holds a FOREIGN member, or check `reg_root` read from contract state. The three run values SHALL NOT change. Under option (a) this requirement SHALL have no effect.

#### Scenario: Foreign member under another root
- **WHEN** AD-20 option (b) is taken and a `DEP` head has a FOREIGN member lifted under another registry root
- **THEN** the wrap or the contract refuses it (mutant `decider_dep_foreign_other_reg_root`)

### Requirement: LW-012 Fresh blinders on every wrap proof
Every wrap proof SHALL use fresh CSPRNG blinders. A seeded wrap proof SHALL exist only in a test mode (ZK-003).

#### Scenario: Two wraps of one statement
- **WHEN** two wraps of the same statement are made with fresh blinders
- **THEN** their bytes differ and the ledger 9 verifier accepts both (EQ-ZK)

### Requirement: LW-013 The wrap has the G4 shape
Under OD-21 option A, and for the first layer of option C, the wrap SHALL have 4 identical lanes of 8 advice columns over shared fixed columns, rotations within `{-1, 0, +1}`, every gate and lookup of degree at most 5, the 19-row Poseidon layout over Fr, and one fixed 2^16-row range table with at most 4 lookups of input degree 2. It SHALL have no foreign-field, pairing or SHA chip (LW-017).

#### Scenario: Degree-6 gate
- **WHEN** the 70-row Poseidon chip with its degree-6 gate is placed in the wrap
- **THEN** the shape check refuses the wrap key, because the extended domain doubles

### Requirement: LW-014 Keygen refusals and the fallback layer
Wrap keygen SHALL refuse a log capacity that does not hold the largest reservation `R`, and an `L` key whose query vector or shape bounds differ from the frozen set. An Fr-terminal layer between `L` and the wrap SHALL be built only on a fallback trigger. Its form A SHALL NOT be built (OD-11) unless OD-21 selects option E, which reopens OD-11.

#### Scenario: Fallback trigger
- **WHEN** the wrap rows exceed the gate after M7, the measured GPU wrap exceeds 8 s at the chosen k after K1 to K5, or the owner asks for a larger log capacity
- **THEN** the trigger is recorded, the owner decides the layer under OD-11, and k stays at or below 21 (CPR-001)

### Requirement: LW-015 Lean obligations of L AND WRAP
The kit SHALL be reported complete only when BR-L2, BR-L4 and DEC-L1 are proved with axiom reports that hold only `propext`, `Quot.sound` and `Classical.choice`. Premises A1 to A7 and `vkL_ok` SHALL stay named hypotheses.

#### Scenario: Obligation status
- **WHEN** the kit status is reported
- **THEN** BR-L1 (`bridge_sound`, `wrap_view_only_claim`) and BR-L3 (`pv_binding_count`, transcript-derived `pvFirstT`, `d_0 = P2(TAG_EMPTY)`, fixed `pv` length) are built at `bdb92973a` against the older `ProvenRegime`, and are not evidence for the recommended parameter set

#### Scenario: Open obligations
- **WHEN** the open obligations are listed
- **THEN** BR-L2 (the restatement: `ProvenRegime` in the unique decoding regime with the frozen query vector, the PoW constants, one OOD sample per round and the `w_stack` bound, `encode` on the two-table `pv`, `wrap_refines` with the DAG key check and the anchor checks, the two Fr sponge algorithms named), BR-L4 (`ReduceRel` in place of `ArithGadget`) and DEC-L1 (O34) are open

### Requirement: LW-016 Production qualification of the wrap
The wrap SHALL be qualified on the OD-21 path over the complete `L` interface (12,164 public words, log AIRs, root and anchor statements, DAG key check), with `R` as real non-`NOP` entries, on the full tree and on path C at the worst admitted shapes. Rows and k SHALL come from keygen, and the serialized ledger key SHALL be measured against 50,000 bytes. A benchmark on `L_cached` with an all-`NOP` log SHALL qualify no gate (LEAD FZ-3).

#### Scenario: Benchmark offered as qualification
- **WHEN** wrap rows measured on `L_cached` with an all-`NOP` log are offered for DG-002, LW-013 or the key freeze
- **THEN** the gate stays open

### Requirement: LW-017 The wrap path waits on the ledger key reader
The G4 shape SHALL NOT freeze, and no G4 figure SHALL be reported as ledger-accepted, before owner decision OD-21 selects a ledger-readable path. Ledger 9 refuses a key with G4 lane gates (M7-0, MEASURED on a toy key).

#### Scenario: G4 key at the unchanged ledger
- **WHEN** a G4 wrap key is installed on ledger 9 without a change to `midnight-zk-stdlib`
- **THEN** `VerifierKey::verify` refuses it at key parsing, and the path is one of the OD-21 options A, C or E
