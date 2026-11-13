## ADDED Requirements

### Requirement: BUS-001 One LogUp bus per chip id and a fixed registry
The leaf SHALL use one LogUp bus for each chip id of the FP-1 registry: 1 ALU, 2 MEM, 3 HASH, 4 ROM, 5 RANGE, 6 DEC, at index `BUS_BASE + id`, with one typed message layout each. A disabled request SHALL have count 0. A request on a reserved id 7 to 15 SHALL be unsatisfiable. CORE SHALL be the only requester of buses 1 to 4 and 6. The halo2 tuple `(chip, key, a, b, c)` over `Fq` is the prototype form only.

#### Scenario: Unassigned chip id
- **WHEN** a request is sent on the reserved bus of id 7 with a consistent payload
- **THEN** no provider row matches and the correct AIR refuses the witness (mutant `bus_unassigned_chip`), and the Quint invariant `unassignedChipUnserved` of `busContract.qnt` holds (`noZeroChipProvider` is evidence only for chip id 0)

#### Scenario: Two layouts on one bus
- **WHEN** a chip set puts two message layouts on one bus index
- **THEN** keygen refuses the chip set (refusal `bus_two_layouts_one_index`, recorded apart)

#### Scenario: A request row serves itself
- **WHEN** a request row offers its own message as a provider, or the second request interaction of a row is left unchecked
- **THEN** the correct AIR refuses the witness (mutants `bus_request_serves_itself`, `bus_second_lane_unchecked`)

#### Scenario: All chips on both lanes
- **WHEN** ALU, MEM, HASH and ROM requests use both request interactions of one row, with the same numeric message words on two buses
- **THEN** the stock verifier accepts a real proof of the leaf (positive `bus_both_lanes_all_chips`), and an unused provider row with a repeated request is legal (positive `bus_unused_provider_and_repeated_request`)

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs the `rec` lanes of the MEM message (`W_rec`)
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-5 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: BUS-002 Provider rows sit on their own bus and padding has count 0
A provider AIR SHALL offer its messages only on the bus index of its chip id, a constant of the interaction. A padding provider row SHALL have multiplicity 0. An unused request SHALL have count 0, and its count SHALL be the boolean that its requester constrains. Every provider row SHALL satisfy its relation whether or not its multiplicity is 0.

#### Scenario: Inactive slot keeps a payload
- **WHEN** an inactive request slot has a non-zero count, or a padding provider row offers a message that fails the provider relation
- **THEN** the correct AIR refuses the witness (mutants `bus_inactive_slot_leaks_payload`, `bus_zero_chip_payload`)

#### Scenario: Chip id not pinned
- **WHEN** a provider AIR sends on a bus index that the chip set does not assign to it, or the provider relation is removed
- **THEN** keygen refuses the first (refusal `bus_chip_id_unpinned`, recorded apart), and the correct AIR refuses the witness of the second (mutant `bus_provider_gate_removed`)

### Requirement: BUS-003 RANGE and DEC are two buses over fixed tables
RANGE and DEC SHALL be two LogUp buses over fixed tables (L-6 in the FP-1 form). The tag SHALL be the bus index, a constant of the interaction, and SHALL NOT be a witness or be multiplied by a witness. A query on one bus SHALL NOT match a row of the other.

#### Scenario: DEC query shaped as a RANGE row
- **WHEN** a DEC query is shaped to match a RANGE row, or a RANGE query to match a DEC row
- **THEN** the correct AIR refuses the witness (mutants `tagged_dec_query_matches_range_row`, `tagged_range_query_matches_dec_row`), the control AIR `tagged_tag_from_advice`, with both tables on one bus and the tag as a witness word, accepts the same witnesses, and the Quint invariant `taggedNoAlias` holds

#### Scenario: RANGE and DEC in one table
- **WHEN** RANGE and DEC queries run in one leaf shard
- **THEN** the stock verifier accepts a real proof of the leaf (positive `tagged_range_and_dec_coexist`)

### Requirement: BUS-004 A disabled DEC slot matches only the idle row
Every DEC slot, enabled or not, SHALL query the DEC bus with count 1 from a fixed selector. A disabled slot SHALL query only the DEC idle row `(0, 0)`. An enabled slot SHALL NOT match the idle row. A full leaf and a leaf with a disabled suffix SHALL prove under one key.

#### Scenario: Full and partial leaf under one key
- **WHEN** a leaf with every slot enabled and a leaf with a disabled suffix are proved under one key
- **THEN** the stock verifier accepts a real proof of both (positive `tagged_full_and_partial_leaf_same_key`), and the Quint invariant `decIdleOnlyWhenDisabled` holds

#### Scenario: Disabled slot with a control word
- **WHEN** a disabled slot has `dec_key = 0` and a non-zero control word, or an enabled slot queries the idle row
- **THEN** the correct AIR refuses the witness (mutants `tagged_disabled_dec_slot_nonzero_val`, `tagged_enabled_dec_slot_idle_row`)

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs the `dec_key` and `dec_val` parts
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-7 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: BUS-005 Messages are injective on their typed lanes
Each bus message SHALL be injective on its typed lanes (FP-1; Lean `bus_enc_inj`, open). No lane SHALL pack two parts: the packed keys `8 addr + kind`, `tbl 2^D_row + row` and `op + 8 flag` are halo2 prototype forms only. The owner of each part SHALL range-check it. Two provider classes that share a bus SHALL have disjoint key sets by a local rule.

#### Scenario: Key alias across a namespace
- **WHEN** a request offers lanes that alias another op, kind or table row, or one lane holds two parts
- **THEN** the correct AIR refuses the witness (mutants `bus_key_namespace_alias`, `bus_limb_over_range`, `bus_addr_top_limb_over`), the control AIR `bus_key_packed_alias` with the packed key `8 addr + kind` in one word accepts two `(addr, kind)` pairs with one key word, and the Quint invariant `memProviderClassesDisjoint` holds between the window class and the line class

### Requirement: BUS-006 Tuple levels and port dependencies are well founded
Bus messages SHALL have levels: RANGE and DEC rows at level 0, ALU, HASH and ROM at level 1, MEM at level 2 with the address as sub-level, and CORE requests above all. The port dependency graph SHALL be acyclic. A port guarantee SHALL travel only as a port certificate through the multiset equality of its FP-1 port (shared columns of one row, or one `PermutationCheckBus` message `(seq, cells)` sent once and received once).

#### Scenario: Level and port order of the module list
- **WHEN** the module list of the leaf is instantiated
- **THEN** every requested tuple has a lower level than the tuple that it is used to provide, and the Quint invariants `levelOrder` and `portOrderAcyclic` hold

#### Scenario: Lean composition evidence
- **WHEN** the composition argument is checked in Lean
- **THEN** `provided_valid` and `requested_valid` (P2, proved in C3, bus tuples only) apply, and O1 (`BusContract.lean`), O2 (`LookupSound` premise), O42 (port composition, `leaf_refines_ports` with `toy_ports`) and O43 (`MemHist`) are open and gate kit completion

### Requirement: BUS-007 The bus is a membership lookup under the LogUp bounds
Every argument about the bus SHALL use membership (`LookupSound`) only, never `BusBalanced`. The LogUp bounds 1 to 5 of FP-1 SHALL hold: messages of at most 128 words, per-bus and total count-times-height sums below `p`, each AIR inside its layer bounds, boolean request counts and witness provider multiplicities. Keygen and the verifier SHALL refuse a shape that breaks them. The halo2 rules (no challenge, rotations `-1, 0, +1`, degree at most 6) bind the prototype only.

#### Scenario: Balanced-bus theorem not credited
- **WHEN** a kit receipt cites `balanced_lookup` (P1, proved in C3)
- **THEN** the receipt is refused as evidence for the landed bus, because P1 applies to the Profile B LogUp form only

#### Scenario: Message too long or heights over p
- **WHEN** a message with its bus index has more than 128 words, or the heights of a proof break the per-bus bound
- **THEN** keygen or the verifier refuses it (refusals `bus_message_too_long`, `bus_height_constraint_over_p`, recorded apart from constraint refusals), and the halo2 configuration failures `vsel_overlapping_roles` and `vsel_degree_overflow` stay prototype checks only

### Requirement: BUS-008 The tuple freezes in its BabyBear encoding
The schema of the halo2 prototype over `Fq` SHALL NOT be the frozen form. The frozen form SHALL be the BabyBear form of `FD/ARCHITECTURE.md` section 1.11, FP-1: one typed lane for each field in 16-bit little-endian limbs (FPR-001), one LogUp bus index for each chip id (FPR-002), ports as shared columns or permutation buses (FPR-003) and the LogUp bounds of FPR-004. Lean `bus_enc_inj` is open.

#### Scenario: Packed key in one BabyBear element
- **WHEN** a MEM key `8 addr + kind` or a 128-bit ALU operand is placed in one BabyBear element
- **THEN** distinct tuples can alias modulo p, so the encoding of FP-1 is required, and the vectors `bus_enc_vectors` of FP-1 pass

### Requirement: BUS-009 The BUS AIRs meet the STARK kit gate
The provider AIRs of buses 1 to 6 SHALL meet the STARK kit gate SG-1 to SG-6 of `design.md` section 9 inside the app set (at most 5,000 constraints and 1,000 interactions per AIR, 100 AIRs, 30,000 columns), with every AIR required and a fixed height in its menu shape.

#### Scenario: AIR omitted from the proof
- **WHEN** a proof omits one of the provider AIRs of buses 1 to 6
- **THEN** N1, N3 and `L` refuse it (mutant `bus_air_omitted`), and the kit does not pass SG-1 before row T3 passes

#### Scenario: Positive and negative proofs on the stock verifier
- **WHEN** the positive `bus_both_lanes_all_chips` and the mutant `bus_unassigned_chip` are proved with the stark-backend prover
- **THEN** the stock verifier accepts the first, refuses the second and accepts the second under its weakened control AIR; a prover panic or tracegen error earns no rejection credit

#### Scenario: Shape, budget and D3
- **WHEN** the provider AIRs of buses 1 to 6 are keyed in a menu shape
- **THEN** each AIR has the fixed height of the shape, keygen refuses a shape outside the bounds above, D3 at the measured shape prints every term at 100 bits or more and every grindable term at 101 or more (SG-5), and the receipt records seconds per proved step (SG-6)
