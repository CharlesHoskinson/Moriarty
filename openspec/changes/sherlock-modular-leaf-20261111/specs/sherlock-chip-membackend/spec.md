## ADDED Requirements

### Requirement: MBK-001 One seam with an opaque token
The memory backend SHALL implement the `MemBackend` trait (`crates/mori-chip/src/membackend.rs:95-128`): `persist`, `cold_read`, and one new method for 8-record block lines. The token SHALL be opaque to MEM and CORE. The dense token SHALL be `[root, tail_d]`, and `len` SHALL stay a MEM frontier cell. A backend swap SHALL NOT change CORE, MEM, DEC, ALU, RANGE, HASH, STACKS, VALUE DIGEST or the bus schema.

#### Scenario: Second backend behind the seam
- **WHEN** the MEM harness is compiled against a second `MemBackend` that authenticates nothing
- **THEN** `WindowCfg` and the CORE stand-in are unchanged (the seam test of the kit)

### Requirement: MBK-002 The tail is canonical and every leaf opens it
`tail_d` SHALL be canonical for the cells of the tail. Every leaf SHALL authenticate the imported tail against `pre`, also a leaf with no append. Every tagged hash SHALL carry its domain in the sponge capacity (E-2).

#### Scenario: Forged or non-canonical tail
- **WHEN** a leaf imports a forged tail across a leaf boundary, a non-canonical tail, or changes `tail_d` with no append
- **THEN** the correct circuit refuses the witness (mutants `forged_tail_across_leaf_boundary`, `non_canonical_tail`, `tail_open_skipped_when_no_append`), and P12 (`Mem.wf_unique`, proved in C3) holds

#### Scenario: Domain tag dropped
- **WHEN** a domain tag is dropped or a node domain is swapped across heights
- **THEN** the C1 lane model mutant `domain_tag_dropped` violates `invRefine`, and the correct circuit refuses the witness (mutant `domain_tag_swapped_node_height`)

### Requirement: MBK-003 A flush happens iff a segment boundary is crossed, into an empty slot, at every TOP
`persist` SHALL flush a segment exactly when `floor(len / 8)` grows, into an empty tree slot, with old and new walks that share siblings. It SHALL hold at every `TOP >= 1`. The working depth SHALL be TOP = 63, gated on OD-1 (OWNER).

#### Scenario: Skipped, spurious or occupied flush
- **WHEN** a flush is skipped, spurious, onto an occupied slot, or forges an empty segment, also at TOP = 1
- **THEN** the correct circuit refuses the witness (mutants `flush_skipped`, `flush_spurious`, `address_reuse_occupied_slot`, `empty_seg_forgery`, `c1_gap_top1_empty_seg_forgery`), and the C1 lane model mutant `top1_walk_empty_shadowed` violates `invRefine`

#### Scenario: Walk tampered
- **WHEN** a walk sibling differs between the old and the new walk, or a walk bit is not boolean
- **THEN** the correct circuit refuses the witness (mutants `walk_sibling_differs_old_new`, `walk_bit_nonboolean`), and a wrong sibling count is refused at synthesis (`persist_wrong_sibling_count`, recorded apart)

#### Scenario: Lean evidence for the flush rule
- **WHEN** the backend is offered for completion
- **THEN** P13 and P14 (proved in C3, P14 only for leaves of at most 8 appends) hold, and O14 (`FitsWindow W`) and O15 (single-step fit) are open

### Requirement: MBK-004 An old read opens the record and its kind against the pre-state root
A scalar cold read SHALL open the record and its kind against `pre.root` of the leaf, never against a root after a flush in the same leaf. The `ColdRead` enable SHALL come from the read port of CORE.

#### Scenario: Forged old record
- **WHEN** the value or the kind of an old record is forged, or the read is opened against the running root
- **THEN** the correct circuit refuses the witness (mutants `cold_value_forged`, `wrong_kind_cold`, `cold_read_uses_running_root`), and O16 (`MemContract.lean`, read soundness) is open

### Requirement: MBK-005 The landed dense backend is conformed to the frozen ports
The landed dense backend (C1, `crates/mori-chip/src/dense.rs`) SHALL be conformed to the ports frozen in group F. The C1 Quint model SHALL move to `models/sherlock/quint/denseMemory.qnt` with its verdict kept.

#### Scenario: Landed tests after conformance
- **WHEN** the backend is conformed to the frozen ports
- **THEN** every landed C1 positive (for example `c1_partition_independence`, `c1_top1_positive`) and every landed backend mutant (for example `flush_skipped`, `cold_value_forged`) still gives its result, and the moved model `denseMemory.qnt` re-runs with `invRefine` and all 23 mutant modules caught
