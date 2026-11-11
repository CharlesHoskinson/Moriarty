## ADDED Requirements

### Requirement: RNG-001 A checked cell is below its static width
The RANGE port SHALL take a source cell and a static width and SHALL return the same cell, range-checked below `2^width`, with no wrap in the field. Limbs SHALL be below `2^12` and SHALL be looked up in the tagged fixed table under `TAG_RANGE`. The 128-bit and the 64-bit shapes SHALL both be offered.

#### Scenario: Value at or above the width
- **WHEN** a value of `2^128` or `p - 1` is checked at 128 bits
- **THEN** the correct circuit refuses the witness (mutants `range_value_2_128`, `range_field_wrap`)

#### Scenario: Boundary values accepted
- **WHEN** the values 0, 1, `2^64` and `2^128 - 1` are checked at 128 bits, the 64-bit boundaries at 64 bits, and small widths at their width
- **THEN** MockProver accepts each (positives `range_128_boundaries`, `range_64_boundaries`, `range_small_widths`)

### Requirement: RNG-002 The decomposition is tied to the source cell
The limb decomposition SHALL reconstruct exactly the source cell through a running sum. The top limb SHALL be bounded to the exact width. Every limb SHALL be in the table.

#### Scenario: Decomposition of another value
- **WHEN** the limbs decompose a value other than the source cell, the top limb exceeds the exact width, a limb is outside the table, or the running sum is broken
- **THEN** the correct circuit refuses the witness (mutants `range_source_copy_removed`, `range_top_pad_unlinked`, `range_limb_out_of_table`, `range_running_sum_broken`)

### Requirement: RNG-003 RANGE is a module instance in Lean
RANGE SHALL be stated as a `Module` instance in Lean: limbs in the table and the reconstruction gate SHALL imply `source < 2^B` with no wrap in the field. The kit SHALL NOT be reported complete while this obligation is open.

#### Scenario: Lean obligation O3
- **WHEN** the kit is offered for completion
- **THEN** O3 (`AluRange.lean`, premise: table soundness) is a proved theorem with an axiom report of only `propext`, `Quot.sound` and `Classical.choice`, and today O3 is open

#### Scenario: Reduced-width model
- **WHEN** the Quint lane runs for RANGE
- **THEN** the range behaviour is checked through `aluArith.qnt` at a reduced width, because RANGE has no model of its own
