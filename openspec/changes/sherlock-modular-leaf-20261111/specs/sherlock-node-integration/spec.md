## ADDED Requirements

### Requirement: NODE-001 Segment merge links adjacent segments
MERGE SHALL take two SEGs with equal `prog_d`, `L.post_d == R.pre_d` and `L.h_hi == R.h_lo`, and SHALL output `(prog_d, L.pre_d, R.post_d, L.h_lo, R.h_hi)`. A unary MERGE SHALL pass its child through. A LEAF child SHALL count as the SEG `(prog_d, pre_d, post_d, h_pre, h_pre + steps)`.

#### Scenario: Adjacent segments
- **WHEN** two adjacent SEGs are merged in either bracketing, or one slot is off
- **THEN** MockProver accepts the node with the output above (positives `pos_node_merge_adjacent`, `pos_node_merge_bracketing_same_claim`, `pos_node_unary_merge`), the Quint invariant `SegCoversExactInterval` of `nodeIntegration.qnt` holds, and O31 (NODE-L1, `BinaryNode.lean`) is open

#### Scenario: Broken link
- **WHEN** `L.post_d != R.pre_d`, the intervals gap or overlap, or the programs differ
- **THEN** the node is unsatisfiable (mutants `node_broken_boundary`, `node_gap_interval`, `node_overlap_interval`, `node_cross_program_merge`)

### Requirement: NODE-002 Heights and steps are range-checked
NODE SHALL range-check `h_lo` and `h_hi` below `2^64` with 8-bit byte decomposition only, so it adds no range-table tag (SRC-004). For a LEAF child NODE SHALL check `1 <= steps <= S_MAX` and `h_pre + steps < 2^64`.

#### Scenario: Height edge
- **WHEN** `h = 2^64 - 1`
- **THEN** MockProver accepts the node (positive `pos_node_height_2p64_minus_1`, a GP-001 acceptance line)

#### Scenario: Height wrap or bad step count
- **WHEN** `h = 2^64`, `steps = 0` or `steps = S_MAX + 1`
- **THEN** the node is unsatisfiable (mutants `node_height_wrap`, `node_steps_zero`, `node_steps_gt_smax`)

### Requirement: NODE-003 Seal requires genesis and a terminal state and erases physical fields
SEAL SHALL require `h_lo == 0`, a terminal opened `outcome_tag` and `deps_d == EMPTY_DEPS` from `open_state_header(post_d)`. It SHALL output `set_root = H(SHK_SET_LEAF, claim_d, n_steps)` with `claim_d = H(SHK_RUN, 1, prog_d, input_d, outcome_d)` and `n_steps = h_hi`, and no physical state field. The opening SHALL be measured by X10 (OD-19).

#### Scenario: Seal of every outcome
- **WHEN** a SEG with `h_lo == 0` and a Halted, Rejected or Exhausted state is sealed
- **THEN** the SET binds `claim_d` and `n_steps` and no `pre_d`, `post_d` or memory root (positives `pos_node_seal_all_outcomes`, `pos_node_header_opening_all_outcomes`), the Quint invariants `SealRequiresGenesisAndTerminal` and `SealErasesPhysicalAddresses` hold, and O32 (NODE-L2) is open

#### Scenario: Running, non-genesis or dependent seal
- **WHEN** the state is Running, `h_lo != 0`, a state digest enters the SET, an opened header word is forged, or `deps_d` is not `EMPTY_DEPS` in a unary SEAL
- **THEN** the node is unsatisfiable (mutants `node_seal_running`, `node_seal_nonzero_hlo`, `node_seal_state_digest_in_set`, `node_seal_header_word_forged`, `node_seal_nonempty_deps`)

### Requirement: NODE-004 Child keys are routed by one selector and disabled slots are neutral
Each child slot SHALL have `en_j` boolean and a one-hot `key_sel_j` over {LEAF, NODE, FOREIGN} that sums to `en_j`. One selector SHALL bind the child key representation, its permitted `stmt_tag` and its fixed-base coefficients. A disabled slot and the genesis carry SHALL use the canonical neutral encoding. Every enabled carry SHALL be folded.

#### Scenario: Wrong key destination
- **WHEN** a child proved under one key is routed to another key's coefficients, or the selector and the representation disagree
- **THEN** the node is unsatisfiable (mutants `node_wrong_key_destination`, `node_selector_repr_mismatch`), and O33 (NODE-L3, `NamedAccumulator.lean`) is open

#### Scenario: Garbage in a disabled slot
- **WHEN** a disabled slot carries a non-neutral accumulator, the genesis carry is not neutral, a child carry is dropped, or a child instance is unbound
- **THEN** the node is unsatisfiable (mutants `node_disabled_slot_garbage`, `node_genesis_non_neutral`, `node_drop_child_carry`, `node_unbound_child_instance`), and the positives `pos_node_genesis_neutral_slots` and `pos_node_key_kinds_each_slot` pass

### Requirement: NODE-005 The node relation table is complete
The node SHALL accept only the rows MERGE, MERGE-WORK, SEAL, SEAL-WITH-DEPS, DL-STEP, DESCEND, JOIN and FV-WORK of the kit's relation table. Every other combination of `stmt_tag`s, enables and work kind SHALL be unsatisfiable. Administrative modes SHALL execute no owner step and SHALL NOT change a height.

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

### Requirement: NODE-007 The node stays under its row gate
The binary node SHALL stay at or below 950,000 rows with its actual table rows and usable domain recorded with headroom. A row figure SHALL be reported MEASURED only with a `cost_model` run id.

#### Scenario: Node rows measured
- **WHEN** X5 and X7 measure the node over the integrated leaf key
- **THEN** the receipt records node rows at or below 950,000 and the table-row headroom, and until then every node figure is ESTIMATE
