## ADDED Requirements

### Requirement: BUS-001 One tuple schema and a fixed chip registry
The leaf SHALL use one bus tuple schema `(chip, key, a, b, c)` over `Fq` with two request lanes that look up one provider table. The chip registry SHALL be 0 disabled, 1 ALU, 2 MEM, 3 HASH and 4 ROM. A chip id of 5 or above SHALL be unsatisfiable until a versioned schema assigns it. CORE SHALL be the only bus requester (`FD/ARCHITECTURE.md` section 1.2).

#### Scenario: Unassigned chip id
- **WHEN** a request names the unassigned chip id 5 with a consistent payload
- **THEN** no provider row matches and the correct circuit refuses the witness (mutant `bus_unassigned_chip`), and the Quint invariant `unassignedChipUnserved` of `busContract.qnt` holds (`noZeroChipProvider` is evidence only for chip id 0)

#### Scenario: A request row serves itself
- **WHEN** a request row offers its own tuple as a provider, or the second lane is left unchecked
- **THEN** the correct circuit refuses the witness (mutants `bus_request_serves_itself`, `bus_second_lane_unchecked`)

#### Scenario: All chips on both lanes
- **WHEN** ALU, MEM, HASH and ROM requests use both lanes in one region, with the same numeric key on two chips
- **THEN** MockProver accepts the leaf (positive `bus_both_lanes_all_chips`), and an unused provider row with a repeated request is legal (positive `bus_unused_provider_and_repeated_request`)

### Requirement: BUS-002 Provider rows pin a non-zero chip id and padding is the zero tuple
An active provider row SHALL pin the fixed non-zero chip id of its provider. A padding provider row SHALL be the zero tuple in all five cells or SHALL be off the provider selector. An unused request lane SHALL be zero in all five cells. Every provider row SHALL satisfy its relation whether or not a request uses it.

#### Scenario: Inactive slot keeps a payload
- **WHEN** an inactive provider slot keeps a non-zero payload, or a chip-0 row carries a payload
- **THEN** the correct circuit refuses the witness (mutants `bus_inactive_slot_leaks_payload`, `bus_zero_chip_payload`)

#### Scenario: Chip id not pinned
- **WHEN** a provider row carries a chip id that its gate does not pin, or the provider gate is removed
- **THEN** the correct circuit refuses the witness (mutants `bus_chip_id_unpinned`, `bus_provider_gate_removed`)

### Requirement: BUS-003 One tagged fixed table serves RANGE and DEC
RANGE and DEC SHALL share one tagged fixed table `(tag, key, val)` in the three-column form (L-6). The tag SHALL be a fixed selector value on every query row and SHALL NOT be a witness or be multiplied by a witness. A query of one tag SHALL NOT match a row of the other tag.

#### Scenario: DEC query shaped as a RANGE row
- **WHEN** a DEC query is shaped to match a RANGE row, or a RANGE query to match a DEC row
- **THEN** the correct circuit refuses the witness (mutants `tagged_dec_query_matches_range_row`, `tagged_range_query_matches_dec_row`), the control circuit `tagged_tag_from_advice` with the tag in an advice cell accepts the same witnesses, and the Quint invariant `taggedNoAlias` holds

#### Scenario: RANGE and DEC in one table
- **WHEN** RANGE and DEC queries run in one leaf
- **THEN** MockProver accepts the leaf (positive `tagged_range_and_dec_coexist`)

### Requirement: BUS-004 A disabled DEC slot matches only the idle row
A disabled DEC slot SHALL keep the fixed tag `TAG_DEC` and SHALL query only the DEC idle row `(TAG_DEC, 0, 0)`. An enabled slot SHALL NOT match the idle row. A full leaf and a leaf with a disabled suffix SHALL prove under one key.

#### Scenario: Full and partial leaf under one key
- **WHEN** a leaf with every slot enabled and a leaf with a disabled suffix are proved under one key
- **THEN** MockProver accepts both (positive `tagged_full_and_partial_leaf_same_key`), and the Quint invariant `decIdleOnlyWhenDisabled` holds

#### Scenario: Disabled slot with a control word
- **WHEN** a disabled slot has `dec_key = 0` and a non-zero control word, or an enabled slot queries the idle row
- **THEN** the correct circuit refuses the witness (mutants `tagged_disabled_dec_slot_nonzero_val`, `tagged_enabled_dec_slot_idle_row`)

### Requirement: BUS-005 Packed keys are injective and stay below the field size
Each packed key SHALL be injective on its ranged parts and SHALL stay far below the field size: MEM `8 addr + kind < 2^(TOP+6)`, ROM `tbl 2^D_row + row < 2^(D_tbl+D_row)`, ALU `op + 8 flag < 16`. The owner of each part SHALL range-check it. Two provider classes that share a chip id SHALL have disjoint key sets by a local rule.

#### Scenario: Key alias across a namespace
- **WHEN** a request packs a key that aliases another op, kind or table row
- **THEN** the correct circuit refuses the witness (mutant `bus_key_namespace_alias`), and the Quint invariant `memProviderClassesDisjoint` holds between the window class and the line class

### Requirement: BUS-006 Tuple levels and port dependencies are well founded
Bus tuples SHALL have levels: tagged-table rows at level 0, ALU, HASH and ROM at level 1, MEM at level 2 with the address as sub-level, and CORE requests above all. The port dependency graph SHALL be acyclic. A port guarantee SHALL travel only as a port certificate through copy equalities.

#### Scenario: Level and port order of the module list
- **WHEN** the module list of the leaf is instantiated
- **THEN** every requested tuple has a lower level than the tuple that it is used to provide, and the Quint invariants `levelOrder` and `portOrderAcyclic` hold

#### Scenario: Lean composition evidence
- **WHEN** the composition argument is checked in Lean
- **THEN** `provided_valid` and `requested_valid` (P2, proved in C3, bus tuples only) apply, and O1 (`BusContract.lean`), O2 (`LookupSound` premise), O42 (port composition, `leaf_refines_ports` with `toy_ports`) and O43 (`MemHist`) are open and gate kit completion

### Requirement: BUS-007 The landed bus is a membership lookup
Every argument about the landed bus SHALL use membership (`LookupSound`) only. No argument SHALL use `BusBalanced` for the landed bus. The circuit SHALL use no challenge, no second phase, rotations `-1, 0, +1` only and degree at most 6 including the packed selector.

#### Scenario: Balanced-bus theorem not credited
- **WHEN** a kit receipt cites `balanced_lookup` (P1, proved in C3)
- **THEN** the receipt is refused as evidence for the landed bus, because P1 applies to the Profile B LogUp form only

#### Scenario: Selector roles overlap
- **WHEN** two row roles overlap in the packed selector, or a gate exceeds degree 6
- **THEN** configuration fails (configuration failures `vsel_overlapping_roles`, `vsel_degree_overflow`, recorded apart from constraint refusals)
