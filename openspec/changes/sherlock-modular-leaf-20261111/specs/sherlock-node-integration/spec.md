## ADDED Requirements

### Requirement: NODE-001 Segment merge links adjacent segments
MERGE SHALL take two SEGs with equal `prog_d`, `L.post_d == R.pre_d` and `L.h_hi == R.h_lo`, and SHALL output `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)`. A unary MERGE SHALL pass its child through. A LEAF child SHALL count as the SEG `(prog_d, pre_d, post_d, h_pre, h_pre + steps)`.

#### Scenario: Adjacent segments
- **WHEN** two adjacent SEGs are merged in either bracketing, or one slot is off
- **THEN** the constraint checker accepts the node with the output above (positives `pos_node_merge_adjacent`, `pos_node_merge_bracketing_same_claim`, `pos_node_unary_merge`), the Quint invariant `SegCoversExactInterval` of `nodeIntegration.qnt` holds, and O31 (NODE-L1, `BinaryNode.lean`) is open

#### Scenario: Broken link
- **WHEN** `L.post_d != R.pre_d`, the intervals gap or overlap, or the programs differ
- **THEN** the node is unsatisfiable (mutants `node_broken_boundary`, `node_gap_interval`, `node_overlap_interval`, `node_cross_program_merge`)

### Requirement: NODE-002 Heights and steps are range-checked
NODE SHALL range-check `h_lo` and `h_hi` below `2^64` with 8-bit byte decomposition only, so it adds no range-table tag (SRC-004). For a LEAF child NODE SHALL check `1 <= steps <= S_MAX` and `h_pre + steps < 2^64`.

#### Scenario: Height edge
- **WHEN** `h = 2^64 - 1`
- **THEN** the constraint checker accepts the node (positive `pos_node_height_2p64_minus_1`, a GP-001 acceptance line)

#### Scenario: Height wrap or bad step count
- **WHEN** `h = 2^64`, `steps = 0` or `steps = S_MAX + 1`
- **THEN** the node is unsatisfiable (mutants `node_height_wrap`, `node_steps_zero`, `node_steps_gt_smax`)

### Requirement: NODE-003 Seal requires genesis and a terminal state and erases physical fields
SEAL SHALL require `h_lo == 0`, a terminal opened `outcome_tag` and `deps_d == EMPTY_DEPS` from `open_state_header(post_d)`. It SHALL output `set_root = H(SHK_SET_LEAF, claim_d, n_steps)` with `claim_d = H(SHK_RUN, 1, prog_d, input_d, outcome_d)` and `n_steps = h_hi`, and no physical state field. The opening SHALL be measured by X10 (AD-19). SEAL and SEAL-WITH-DEPS SHALL be the only seal path. `L` SHALL open the sealed SET head and SHALL NOT check a seal predicate again (FPR-007, FPR-013).

#### Scenario: Seal of every outcome
- **WHEN** a SEG with `h_lo == 0` and a Halted, Rejected or Exhausted state is sealed
- **THEN** the SET binds `claim_d` and `n_steps` and no `pre_d`, `post_d` or memory root (positives `pos_node_seal_all_outcomes`, `pos_node_header_opening_all_outcomes`), the Quint invariants `SealRequiresGenesisAndTerminal` and `SealErasesPhysicalAddresses` hold, and O32 (NODE-L2) is open

#### Scenario: Running, non-genesis or dependent seal
- **WHEN** the state is Running, `h_lo != 0`, a state digest enters the SET, an opened header word is forged, or `deps_d` is not `EMPTY_DEPS` in a SEAL
- **THEN** the node is unsatisfiable (mutants `node_seal_running`, `node_seal_nonzero_hlo`, `node_seal_state_digest_in_set`, `node_seal_header_word_forged`, `node_seal_nonempty_deps`)

### Requirement: NODE-004 Child keys are routed by one selector and disabled slots are neutral
Each child slot SHALL have `en_j` boolean and a one-hot `key_sel_j` over {LEAF, NODE, FOREIGN} that sums to `en_j`. One selector SHALL bind the child key, its permitted `stmt_tag` and its statement lanes. The recursion verifier SHALL verify every enabled child proof, and each child key commitment SHALL have an inclusion path into `key_root`. A disabled slot SHALL use the canonical neutral statement.

#### Scenario: Wrong key destination
- **WHEN** a child proved under one key is routed to another key, a child key has no inclusion path into `key_root`, or the selector and the key disagree
- **THEN** the node is unsatisfiable (mutants `node_wrong_key_destination`, `node_selector_repr_mismatch`), and O33 (NODE-L3, `NamedAccumulator.lean`) is open

#### Scenario: Garbage in a disabled slot
- **WHEN** a disabled slot carries a non-neutral statement, the genesis statement is not neutral, a child log digest is dropped from the `TAG_NODE` merge, or a child instance is unbound
- **THEN** the node is unsatisfiable (mutants `node_disabled_slot_garbage`, `node_genesis_non_neutral`, `node_drop_child_carry`, `node_unbound_child_instance`), and the positives `pos_node_genesis_neutral_slots` and `pos_node_key_kinds_each_slot` pass. The design keeps these names from the halo2 prototype, and their mapping onto the STARK node AIRs is open work

### Requirement: NODE-005 The node relation table is complete
The node SHALL accept only the rows MERGE, MERGE-WORK, SEAL, SEAL-WITH-DEPS, DL-STEP, DESCEND, JOIN, FV-WORK, ADMIT-WORK and VIEW-WORK of the kit's relation table. Every other combination of `stmt_tag`s, enables and work kind SHALL be unsatisfiable. Administrative modes SHALL execute no owner step and SHALL NOT change a height.

#### Scenario: Unsupported combination
- **WHEN** a SEG and a SET are merged, owner work runs in a JOIN, or an administrative mode changes a height
- **THEN** the node is unsatisfiable (mutants `graph_unsupported_combination`, `node_admin_mode_changes_height`)

#### Scenario: Inline work not at the right end
- **WHEN** inline work has a `pre_d` or a height that is not the right end of the enabled children, or does not advance the height
- **THEN** the node is unsatisfiable (mutants `node_work_not_adjacent`, `node_work_height_not_advanced`), and the positives `pos_node_merge_work_two_children_one_step` and `pos_node_merge_work_no_children` pass

### Requirement: NODE-006 The node relation serializes `S_MAX` and every keyed capacity
`write_relation` SHALL serialize the magic, the column count, `k`, `S_MAX`, every keyed capacity and the key representations (SRC-014). The landed lane name `rule` SHALL be renamed `rules_d`. The leaf entry form (direct leaf key or one-leaf lift) SHALL be decided by X7 (N-1).

#### Scenario: Relation without `S_MAX`
- **WHEN** a relation file lacks `S_MAX`
- **THEN** deserialization is refused (host refusal `node_relation_missing_smax`, recorded apart from constraint mutants)

### Requirement: NODE-007 The node stays inside the shape bounds of the internal set
The STARK node SHALL have a fixed arity and SHALL stay inside the shape bounds of the internal set: height at most `2^19`, 1,000 constraints and 100 interactions per AIR, 50 AIRs and 2,000 columns. Keygen SHALL refuse a node outside them. Its cost SHALL be recorded in seconds per proof with a run id (DG-004, DG-006 of `sherlock-processor-final-20261111`).

#### Scenario: Node cost measured
- **WHEN** X5 and X7 measure the node over the integrated leaf key
- **THEN** the receipt records the node shape against the bounds and its cost in seconds per proof, and until then every node figure is ESTIMATE

#### Scenario: Node outside the bounds
- **WHEN** the node needs a height of `2^20`
- **THEN** keygen refuses the node key (superseded 2026-11-11: the halo2 gate of 950,000 rows, text at commit `0ab8cd79d`)

### Requirement: NODE-008 The registry root is equal across child slots
`reg_root` SHALL be equal in every enabled NODE or FOREIGN child and in the parent. A LEAF child SHALL carry no `reg_root`. With no enabled NODE or FOREIGN child, the parent SHALL take `reg_root` from its witness. The constraint SHALL be `en_j * (1 - is_leaf_j) * (child_j.reg_root - parent.reg_root) == 0` for `j` in {0, 1}.

#### Scenario: Registry roots mixed across slots
- **WHEN** in JOIN, DL-STEP or SEAL-WITH-DEPS an enabled FOREIGN or NODE child has a `reg_root` that differs from the parent's `reg_root`
- **THEN** the node is unsatisfiable (mutant `node_reg_root_mixed`), and the Quint invariant `RegRootEqualAcrossSlots` of `nodeIntegration.qnt` holds

### Requirement: NODE-009 Each key kind has a fixed set of permitted statement tags
The key selector SHALL permit `LEAF -> {SEG, LG}` (LG for the discharge child only, FPR-006), `FOREIGN -> {SET}` and `NODE -> {SEG, SET, SUB, DL, FV, AW, VW}`. The FV payload SHALL be `(fv_state_d)` only. The registry root of an FV statement SHALL be the node's `reg_root` lane.

#### Scenario: Statement tag outside the key kind
- **WHEN** a LEAF child offers a SET, or a FOREIGN child offers a SEG
- **THEN** the node is unsatisfiable (mutant `node_selector_repr_mismatch`)

### Requirement: NODE-010 A seal requires an admitted program
SEAL SHALL take a verified SET or SUB child in slot 1 whose root `prem_root` opens with one hash as `ADM(adm_d)`, with `adm_d == H(SHK_ADM, 1, prog_d)` for the sealed `prog_d`. SEAL-WITH-DEPS SHALL apply the same check to `DL.prem_root`. This check SHALL be the discharge point of the PROG canonicality premise.

#### Scenario: Unadmitted program
- **WHEN** a seal has no `ADM` head in its premise, or an `ADM` head of another `prog_d`
- **THEN** the node is unsatisfiable (mutants `node_seal_unadmitted_prog`, `node_seal_admission_other_prog`), the Quint invariant `SealRequiresAdmittedProgram` holds, and NODE-L4 (`BinaryNode.lean`) is open

### Requirement: NODE-011 ADMIT-WORK checks every ROM row in bounded units
ADMIT-WORK SHALL continue `adm_state_d = H(SHK_ADM_STATE, prog_d, prog_root, tbl, row)` of its child, or open `prog_d` at the first unit. Each unit SHALL check, in order, at most the unit budget of ROM rows: constant `vd`, `ty_d` of the AD-14 normal form, constructor ordinal and arity, reference ranges and canonical padding. It SHALL emit `ADM(adm_d)` only after the last row.

#### Scenario: Forged row or early claim
- **WHEN** a constant `vd`, a `ty_d`, a constructor arity or a pad row is not canonical, or the `ADM` SET is emitted before the last row
- **THEN** the unit is unsatisfiable (mutants `admit_const_vd_forged`, `admit_ty_d_forged`, `admit_ctor_arity_forged`, `admit_pad_noncanonical`, `admit_claim_before_done`)

### Requirement: NODE-012 VIEW-WORK certifies a read-only view of the input
Under input admission form (b) (AD-17, OWNER), VIEW-WORK SHALL continue `view_state_d = H(SHK_VIEW_STATE, val_d, view_root, cursor, n_rec, fold_d)` of its child. Each unit SHALL check view records in index order against `sherlock-value/1`, with record 0 equal to `val_d`. It SHALL emit `VIEW(vw_d)`, `vw_d = H(SHK_VIEW, 1, val_d, view_root)`, only after the last record.

#### Scenario: Forged record or early claim
- **WHEN** a view record or child-vd leaf has a `vd` that does not follow from its children, or the `VIEW` SET is emitted before the last record
- **THEN** the unit is unsatisfiable (mutants `view_record_forged`, `view_claim_before_done`), and NODE-L5 (`BinaryNode.lean`) is open

### Requirement: NODE-013 A seal with an input requires the view of that input
Under form (b), for every re-hashed `val_d` other than the absent-input constant, the seal premise SHALL open as `JOIN(ADM(adm_d), VIEW(vw_d))`. `vw_d` SHALL equal `H(SHK_VIEW, 1, val_d, view_root)`, with `val_d` from the re-hashed `input_d` preimage and `view_root` the header word that `open_state_header` returns (FPR-012).

#### Scenario: View of another input
- **WHEN** the `VIEW` claim names another `val_d` or another `view_root`, or a run with a present input seals with an `ADM`-only premise
- **THEN** the node is unsatisfiable (mutants `seal_view_other_input`, `seal_view_omitted`), and the Quint invariant `SealRequiresCertifiedInputView` holds
