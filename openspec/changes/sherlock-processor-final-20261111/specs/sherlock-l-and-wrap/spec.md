## ADDED Requirements

### Requirement: LW-001 `L` verifies the node root and exposes the statement
`L` SHALL verify the node root proof on the full tree or the leaf-layer proof on path C. `L` SHALL take the flattened field log and its tree shape as private witness. `L` SHALL expose the statement words, the anchor statement `(mode, j, k, a_in, a_out)`, `run_tag` for a chunk, `layout_d`, `pv` of `W_pv` words, the DAG commitment `X` and the call vector of `2 + L_TX` words as public values.

#### Scenario: Full tree and path C
- **WHEN** `L` proves a run on the full tree and a one-leaf call on path C
- **THEN** both proofs expose the same public-value layout, and path C adds the inclusion of `P2(DOM_KEYDAG, entry, path, enc(X))` into `key_root` (SPL-005)

#### Scenario: Log digest mismatch
- **WHEN** the private log does not hash to the onion digest `(d, n)` of the node root
- **THEN** the constraint checker refuses the trace of `L`

### Requirement: LW-002 `L_Fr` verifies `L` under the key schedule, and the wrap verifies `L_Fr`
`L_Fr` SHALL verify the proof of `L` under the frozen `L` key (OD-24 option E). The verifier of `L` inside `L_Fr` SHALL follow the key schedule `sched`, the transcript algorithm of HDM-004 and the one-sample WHIR order of HDM-006 (OD-18, OD-19). It SHALL NOT follow the structure of the proof. The wrap SHALL verify the proof of `L_Fr` natively in Fr under the frozen `L_Fr` key and the `L_Fr` transcript (u = 7, HDM-001).

#### Scenario: Verifier follows the proof
- **WHEN** the verifier of `L` inside `L_Fr` takes its flush points from the proof and not from `sched`
- **THEN** the trace of `L_Fr` is refused (mutant `wrap_l_transcript_from_proof`), and the transcript vectors T-FS give equal outputs on the CPU prover, the GPU prover, the stark-backend verifier and the `L_Fr` tape (HDM-009)

#### Scenario: Changed `L` proof
- **WHEN** one byte of the proof of `L` or one public word of `L_Fr` is changed
- **THEN** `L_Fr` or its verifier refuses it (MEASURED at gate E2: `LogupZerocheck(NonZeroRootSum)` and `BatchConstraintError(InconsistentClaims)`, `research/sherlock-e2-lfr-20261112/RESULTS.md` in the `e2-lfr` worktree)

### Requirement: LW-003 The wrap public inputs are the call vector only
The wrap SHALL have exactly the call vector of `2 + L_TX` words as public inputs, with `L_TX` = 1,024 (OD-7, OD-15). Lane 0 SHALL be `binding_input`. Lane 1 SHALL be `comm`. Unused lanes SHALL be constrained zero.

#### Scenario: Forbidden public input
- **WHEN** a wrap key exposes `X`, `mode`, `j`, `k`, `run_tag`, `key_root`, plain `prog_d`, `pre_d`, `post_d`, a memory root, a tail digest, a length, an address or a word derived from the transcript of `L` or of `L_Fr`
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
A collection SHALL publish `[reg_root, set_root]` through its effect words, with a schema separate from the run statement. `shape_d` SHALL NOT be public (owner ruling O9, 2026-11-13). No claim count SHALL be public, range-checked or used for soundness (AD-2, OWNER). The entry label SHALL discriminate the collection statement from the run statement (FPR-008). The contract entry SHALL be `acceptCollection(reg_pub, set_pub)`.

#### Scenario: Three runs joined
- **WHEN** three sealed runs of different programs are joined and wrapped
- **THEN** the statement is exactly `[reg_root, set_root]` (positive `pos_decider_collection_three_runs` as a real proof), and each claim opens through the LEDGER CONTRACT (LDG-004)

#### Scenario: Stale registry root
- **WHEN** the call carries a registry root other than the contract's
- **THEN** the call is refused (Quint invariant `StaleOrForeignRegRootRefused`)

#### Scenario: No shape digest in the collection call
- **WHEN** the compiled `acceptCollection` entry is inspected
- **THEN** its inputs are exactly `[reg_pub, set_pub]`, it changes only `verified_roots` by adding `set_pub`, and a control entry that takes `shape_d` fails the same check (test `(e) LW-009` and its control in `tools/sherlock-sdk/sherlock-ledger.test.mjs`)

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

### Requirement: LW-013 The wrap has the `ZkStdLib` architecture
The wrap SHALL be one `ZkStdLib`-architecture circuit (OD-24 option E) that ledger 9.1.0.1 reads with `MidnightVK::read`, at k at most 21. It SHALL verify `L_Fr` with the stdlib chips (Poseidon over Fr, range checks, Fr Merkle paths, the sumcheck and WHIR verifier, the Fr transcript replay). It SHALL have no foreign-field, pairing or SHA chip, and no G4 lane gate (LW-017).

#### Scenario: Rows against the gate
- **WHEN** the wrap rows of `L_Fr` over the full interface are reported before E5
- **THEN** they are ESTIMATE, 684,894 / 879,472 / 976,410 rows (low / mid / high), at most 55 percent of the k = 21 gate (`research/sherlock-m7-wrap-20261112/RESULTS.md` section 2b.2), and E5 replaces them by `MockProver` rows at the measured `L_Fr` widths

### Requirement: LW-014 Keygen refusals and the fallback layer
Wrap keygen SHALL refuse a log capacity that does not hold the largest reservation `R`, and an `L` or `L_Fr` key whose query vector or shape bounds differ from the frozen set. The Fr-native layer between `L` and the wrap SHALL be `L_Fr` of OD-24 option E. No second Fr proving engine SHALL be built as a call-time fallback (OD-11, owner decision 2026-11-12); `L_Fr` is part of the one proving path.

#### Scenario: Fallback trigger
- **WHEN** the wrap rows exceed the gate after E5, the measured GPU wrap exceeds 8 s at the chosen k after K1 to K5, or the owner asks for a larger log capacity
- **THEN** the trigger is recorded, the owner decides, and k stays at or below 21 (CPR-001)

### Requirement: LW-015 Lean obligations of L AND WRAP
The kit SHALL be reported complete only when BR-L2, BR-L4 and DEC-L1 are proved with axiom reports that hold only `propext`, `Quot.sound` and `Classical.choice`. Premises A1 to A7, A2' (`lfrSound`) and the static checks `vkL_ok` and `vkLFr_ok` SHALL stay named hypotheses or kernel-checked conditions.

#### Scenario: Obligation status
- **WHEN** the kit status is reported
- **THEN** BR-L1 (`bridge_sound`, `wrap_view_only_claim`, with the wrap relation over `L_Fr`, A2' and `lfrSound_of_emul`) builds at `6661325d4` of branch `bridge-lean` (146 theorems, 0 sorry, standard axioms only), the unique decoding `ProvenRegime` of BR-L2 with the exact error terms of the `L`, internal, app and leaf-aggregation sets builds there, and BR-L3 (`pv_binding_count`, transcript-derived `pvFirstT`, `d_0 = P2(TAG_EMPTY)`, fixed `pv` length) is built at `bdb92973a`

#### Scenario: Open obligations
- **WHEN** the open obligations are listed
- **THEN** the rest of BR-L2 (the leaf-aggregation entry at LogUp PoW 17, the parameter gate of `L_Fr`, `encode` on the two-table `pv`, `wrap_refines` with the DAG key check and the anchor checks), BR-L4 (`ReduceRel` in place of `ArithGadget`) and DEC-L1 (O34) are open

### Requirement: LW-016 Production qualification of the wrap
The wrap SHALL be qualified on OD-24 option E (step E5), as the stdlib wrap of `L_Fr` over the complete `L` interface (12,166 public words at 5 lane-map pairs (O7), log AIRs, root and anchor statements, DAG key check), with `R` as real non-`NOP` entries, on the full tree and on path C at the worst admitted shapes. Rows and k SHALL come from keygen, and the serialized ledger key SHALL be measured against 50,000 bytes. A benchmark on `L_cached` with an all-`NOP` log SHALL qualify no gate (FZ-3).

#### Scenario: Benchmark offered as qualification
- **WHEN** wrap rows measured on `L_cached` with an all-`NOP` log are offered for DG-002, LW-013 or the key freeze
- **THEN** the gate stays open

### Requirement: LW-017 The wrap key is ledger-readable
The wrap key SHALL be one that ledger 9.1.0.1 reads unchanged: a `ZkStdLib`-architecture key (OD-24 option E). No G4 figure SHALL be reported as ledger-accepted. Ledger 9 refuses a key with G4 lane gates (M7-0, MEASURED on a toy key).

#### Scenario: G4 key at the unchanged ledger
- **WHEN** a G4 wrap key is installed on ledger 9 without a change to `midnight-zk-stdlib`
- **THEN** `VerifierKey::verify` refuses it at key parsing, and the design keeps option E

### Requirement: LW-018 The Fr-native layer `L_Fr`
`L_Fr` SHALL be a STARK on stark-backend over BLS12-381 Fr with no extension field. It SHALL take the proof of `L` as private witness, emulate the BabyBear and `EF` arithmetic of its verifier with a bounded quotient on every reduction, and hash the Merkle paths and transcript of `L` natively. Its own hashes SHALL use u = 5, 6, 7 (HDM-001). Its public values SHALL be the `L` key pre-hash, then the public values of `L` packed 8 per Fr word. Its key SHALL pin the `L` key and heights (LW-019).

#### Scenario: One real `L` proof
- **WHEN** `L_Fr` proves the whole verifier of one real 430,073-byte `L` proof at its parameter set
- **THEN** the stock verifier accepts it: 15 AIRs, every AIR at most 2^19 rows, 2 commitments, proof 2,516,530 bytes, and D3 at the measured key gives 101.1435 bits (query), MEASURED (gate E2, `research/sherlock-e2-lfr-20261112/RESULTS.md` section 8g in the `e2-lfr` worktree)

#### Scenario: Full interface
- **WHEN** `L_Fr` carries the complete `L` interface (12,166 public words at 5 lane-map pairs, the log AIRs, `R` as real non-`NOP` entries, an all-non-`NOP` case and a forced-overflow case)
- **THEN** the proof verifies and D3 reruns at that key (step E3, PENDING)

#### Scenario: Source of the MEM_BUS type
- **WHEN** the status is reported
- **THEN** every `MEM_BUS` message of `L_Fr` is typed as LW-020 and LW-021 require, the type element is C-A2c and PK1 (openvm `rekey-chips` fe3601c), a LEAD PICK in the single rekey list that the owner confirmed on 2026-11-15

### Requirement: LW-019 The parameter set of `L_Fr`
The `L_Fr` key SHALL use unique decoding at log blowup 3, `l_skip` 2, `n_stack` 18, `w_stack` at most 48, `k_whir` 4, queries 98/83/82 and query PoW 20, with one OOD sample per round, every term at 100 bits or more and every grindable term at 101 bits or more (SQG-004).

#### Scenario: Terms at the measured key
- **WHEN** D3 runs at the measured `L_Fr` key of gate E2
- **THEN** the weakest term is 101.1435 bits (query and shift), every field-size term is above 215 bits (LogUp 215.9502 with no PoW), and the set needs no folding, `mu` or LogUp PoW, MEASURED
