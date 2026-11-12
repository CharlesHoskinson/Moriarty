## ADDED Requirements

### Requirement: VDG-001 `sherlock-value/1` bit for bit
VALUE DIGEST SHALL compute exactly the preimages of the seven forms, the constructor fold step and the outcome wrappers of `sherlock-value/1` (`openspec/changes/sherlock-vm-consensus-20261107/design.md:175-191`). It SHALL use the zk-stdlib sponge with the length in the start state and no capacity domain tag. A single precomputed sponge prefix for all lengths SHALL NOT be used (L-3).

#### Scenario: Every form and outcome
- **WHEN** every form and every outcome is digested, including UInt 0 and `2^128 - 1` and constructors of 0, 1 and 3 fields
- **THEN** the circuit equals a reference written from the specification text (differential `vd_differential_every_form`)

#### Scenario: Wrong start state
- **WHEN** a capacity tag is added, or one start state is used for all lengths
- **THEN** the differential fails (mutants `vd_capacity_tag_added`, `vd_single_prefix_constant`)

### Requirement: VDG-002 Whole-sponge slots with copied inputs
Each digest SHALL be one sponge in HASH slots with copy-constrained inputs. Constant chunks SHALL be precomputed per preimage length and form. No chain of two-input bus hashes SHALL be used. A constant-start HASH mode SHALL be used only if X9 passes (V-5).

#### Scenario: Bus re-hash chain
- **WHEN** a two-input bus chain replaces a sponge
- **THEN** the differential fails (mutant `vd_bus_rehash_chain`), and O26 (`ValueDigest.lean`, constant-start equivalence) is open

### Requirement: VDG-003 Records and ROM rows only through the read port
VALUE DIGEST SHALL issue no bus request. Every child record and ROM row SHALL be an `AuthRec` or `AuthRom` token of the read port of CORE.

#### Scenario: Future child, wrong reference or row overflow
- **WHEN** a child record is at or above the running length, at another address than the held reference, or a ROM row index overflows `D_row`
- **THEN** the correct circuit refuses the witness (mutants `vd_child_future_record`, `vd_child_wrong_reference`, `vd_rom_row_overflow`), and the control port that checks window membership only accepts the first witness

### Requirement: VDG-004 Digests from authenticated children
The stored `vd` SHALL be computed from the tag and the child `vd` lanes of kind-constrained records. `b` SHALL be boolean and UInt SHALL be below `2^128`. The constructor fold SHALL advance one field per step and SHALL complete at the arity. The digest SHALL be copied to the record.

#### Scenario: Host or wrong child digest
- **WHEN** a host digest, a wrong child digest, an unconstrained child kind, a wrong tag or arity, or swapped fields are supplied
- **THEN** the correct circuit refuses the witness (mutants `host_supplied_vd`, `wrong_child_digest`, `child_kind_unconstrained`, `wrong_value_tag`, `wrong_arity`, `field_order_swapped`)

#### Scenario: Fold cursor or accumulator tampered
- **WHEN** the fold cursor skips, or the accumulator resets across a leaf
- **THEN** the correct circuit refuses the witness (mutants `fold_cursor_skipped`, `fold_accumulator_reset`), and a constructor fold split across leaves is accepted (VALUE DIGEST positive)

#### Scenario: Scalar and padding forged
- **WHEN** a scalar is out of range, a bool selection is not boolean, a raw type id or an unbound ordinal enters the digest, Running carries an outcome, padding is used as a value, or the digest is not copied to the record
- **THEN** the correct circuit refuses the witness (mutants `scalar_out_of_range`, `bool_nonboolean_selection`, `raw_type_id_in_digest`, `constructor_ordinal_unbound`, `running_has_outcome`, `padding_as_value`, `digest_not_copied_to_record`)

### Requirement: VDG-005 Bounded value opening
`open_value_node` SHALL open a value held by digest one node per call. A constructor field SHALL be usable only after its opening fold reaches the digest. Under input admission form (b) (AD-17), an input field SHALL be read through `AuthIn` under the certified `view_root` instead.

#### Scenario: Field used before the fold completes
- **WHEN** a field is used before the fold completes, or an opened child digest differs from the hashed one
- **THEN** the correct circuit refuses the witness (mutants `vd_open_field_used_before_completion`, `vd_open_node_wrong_child`)

### Requirement: VDG-006 Layout independence and injectivity
Constants and data of one value, and shared and unfolded structure, SHALL have equal `vd`. Distinct values SHALL give distinct preimage sequences.

#### Scenario: Relocated or shared value
- **WHEN** one value is stored at relocated addresses, as a constant and as data, or shared
- **THEN** `vd` is equal (Quint invariants `VdRelocationInvariant`, `VdConstDataEqual`, `VdSharingEqual` of `valueDigest.qnt`), and O25 (layout independence) is open

#### Scenario: Lean injectivity obligations
- **WHEN** the kit is offered for completion
- **THEN** O23 (encoding injectivity), O24 (equal `vd` implies equal values, under collision resistance) and O27 (VALUE DIGEST as a `Module`) are proved, the Quint invariant `VdFormsDisjoint` holds, and today O23, O24 and O27 are open

### Requirement: VDG-007 The co-processor digests are of the `sherlock-value/2` form
Under `sherlock-processor-final-20261111` the co-processor digests SHALL be of the `sherlock-value/2` form: Poseidon2 over BabyBear inside the STARK layers and `PoseidonFr(tag, pack(d2))` at the Fr boundary, with salted `input_d` and `outcome_d` (HDM-001, HDM-002, ZK-006, SPL-003). The `sherlock-value/1` form of VDG-001 is the halo2 prototype form and SHALL NOT be a co-processor digest. VALUE DIGEST SHALL compute the `vd` rows of the FP-4 table of `FD/ARCHITECTURE.md` section 1.11 (FPR-011).

#### Scenario: Prototype digest offered as a co-processor digest
- **WHEN** a co-processor public digest is computed with the `sherlock-value/1` zk-stdlib sponge in place of the `sherlock-value/2` form
- **THEN** it differs from the `PoseidonFr(tag, pack(d2))` value that the wrap checks (HDM-002), so the co-processor uses the FP-4 table (FPR-011) and refuses the `sherlock-value/1` value (mutant `digest_tag_swapped`)
