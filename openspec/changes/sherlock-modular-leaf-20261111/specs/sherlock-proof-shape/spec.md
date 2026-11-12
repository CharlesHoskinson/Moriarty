## ADDED Requirements

### Requirement: SHP-001 Join is total at any depth with no claim count
JOIN SHALL compute `set_root = H(SHK_SET_JOIN, l.set_root, r.set_root)` with no counter and no range check, at any depth (R-5, AD-2 OWNER to confirm). A claim MAY be reused in two branches. One key set SHALL prove a chain, a balanced tree and a DAG with equal final proof size.

#### Scenario: Diamond reuse at depth 8 or more
- **WHEN** one claim feeds two consumers at depth >= 8 and the branches are joined
- **THEN** the root is accepted with the same final proof size as a chain and a tree (positives `pos_gp4_diamond_reuse`, `pos_gp4_chain_tree_dag`, and the real-proof acceptance test of GP-004), the Quint invariant `DiamondDagDepth64NoOverflow` of `pcdGraph.qnt` holds, and O40 (GP4-L1) is open

#### Scenario: Fan-out refused
- **WHEN** a valid claim fan-out is refused, or one execution interval is duplicated
- **THEN** the completeness check `graph_reject_valid_claim_fanout` fails the build and the node refuses the duplicate (mutant `graph_duplicate_execution_interval`)

### Requirement: SHP-002 The shape commitment is address-free and never decides acceptance
`shape_d` SHALL be `H(SHK_SHAPE, T_NODE, mode, slot_bits, key_sel_0, key_sel_1, shape_0, shape_1, work_kind, work_steps, out_a, out_b)` in this order. No field SHALL be `pre_d`, `post_d`, `body_d`, a memory root, a tail digest, a challenge or an address. Acceptance, replay and consumption SHALL NOT use `shape_d`. Bracketing disclosure SHALL follow N-7.

#### Scenario: State root in the shape
- **WHEN** a state root enters `shape_d`, `shape_d` is used as a replay key, the field order is swapped, the relation tag is omitted, the slot bits are unbound, or the children are swapped
- **THEN** it is refused (mutants `graph_state_root_in_shape`, `graph_shape_as_replay_key`, `graph_shape_field_order_swapped`, `graph_omit_relation_tag`, `graph_unbound_slot_bits`, `graph_swap_children`), and the Quint invariant `ShapeNotUsedForAcceptance` holds

#### Scenario: Same claims, other bracketing
- **WHEN** the same claims are joined under different bracketings
- **THEN** acceptance is equal (positive `pos_gp4_bracketing_same_claim`)

### Requirement: SHP-003 A node takes two prior proofs plus new work
MERGE-WORK SHALL take two enabled prior SEG proofs plus `k >= 1` real owner steps in one node, with `W.pre_d` equal to the right-most child's `post_d`, `W.h_pre` equal to its `h_hi`, and output height `W.h_pre + k` below `2^64`. Two successive nodes SHALL NOT satisfy this requirement. Its fit SHALL be measured by X7 (N-6, OWNER).

#### Scenario: Two prior SEGs and one step
- **WHEN** two enabled prior SEGs and one owner step are proved in one node
- **THEN** the node outputs the summed interval (positive `pos_gp4_two_prior_plus_step`), and O41 (GP4-L2, `BinaryNode.lean`) is open

#### Scenario: Work not bound
- **WHEN** inline work is not bound to the child state, its steps are not in the height, two children disable the work, or administrative work changes `steps`
- **THEN** the node is unsatisfiable (mutants `graph_work_not_bound_to_child_state`, `graph_work_steps_not_in_height`, `graph_two_children_disable_work`, `graph_admin_work_changes_steps`)

#### Scenario: Two nodes offered as evidence
- **WHEN** a run of two successive nodes is offered as evidence for MERGE-WORK
- **THEN** the acceptance check refuses it (`graph_two_nodes_for_merge_work`), and if X7 does not fit the shape bounds of the internal set GP-4 stays incomplete and the owner decides

### Requirement: SHP-004 Leaf, foreign and disabled children have canonical shape encodings
A LEAF child SHALL have `shape_j = H(SHK_SHAPE, T_LEAF, h_pre, steps)` from its authenticated lanes. A FOREIGN child SHALL forward `H(SHK_SHAPE, T_FOREIGN, fclaim_d)`. A disabled slot SHALL have the fixed non-zero constant `SHAPE_OFF = H(SHK_SHAPE, T_OFF, 0)`.

#### Scenario: Free leaf shape
- **WHEN** a leaf child's shape word is not derived from its lanes, or a disabled slot's shape is not `SHAPE_OFF`
- **THEN** the node is unsatisfiable (mutants `graph_leaf_shape_free`, `graph_disabled_slot_shape_not_off`)

### Requirement: SHP-005 Each output type has a fixed `(out_a, out_b)` pair
`(out_a, out_b)` SHALL be `(h_lo, h_hi)` for a SEG output, `(set_root, 0)` for a SET, `(chain_d, cover_root)` for a DL, `(set_root, sub_root)` for a SUB and `(0, 0)` for an FV, AW or VW output. A FOREIGN SET at FV completion, and an `ADM` or `VIEW` SET at the completion of ADMIT-WORK or VIEW-WORK, SHALL use the SET rule `(set_root, 0)` (`FD/ARCHITECTURE.md` kit GP-4 hook). `fv_state_d` SHALL NOT enter `shape_d`.

#### Scenario: FV state in the shape
- **WHEN** an FV output has `(out_a, out_b)` other than `(0, 0)`, for example `fv_state_d` in `out_a`
- **THEN** the node is unsatisfiable (mutant `shape_fv_state_in_shape`)

### Requirement: SHP-006 The inline kernel has one encoding of the machine relation
The inline kernel SHALL be built from the same kit `configure` and `assign` functions as the leaf (CORE, DEC, ALU, RANGE, HASH, STACKS, MEM, the memory backend, VALUE DIGEST, INIT), instantiated in the node's constraint system. A separate node emitter SHALL NOT be used. If the kernel does not fit the node gate, the owner SHALL decide under N-6 and GP-4 SHALL NOT be narrowed.

#### Scenario: Second encoding of the machine relation
- **WHEN** an inline kernel does not instantiate the leaf kits' `configure` and `assign` functions, for example an emitter checked only against differential vectors
- **THEN** the build check `graph_inline_kernel_separate_encoding` refuses it
