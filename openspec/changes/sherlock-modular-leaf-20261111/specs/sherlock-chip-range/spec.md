## ADDED Requirements

### Requirement: RNG-001 A checked cell is below its static width
The RANGE port SHALL take the limbs of a typed lane and a static width and SHALL check the value below `2^width` with no wrap in the field. Each limb SHALL be one RANGE bus query `(x, w)` against the stock `VariableRangeCheckerAir` at 16 bits (2^17 rows, LD-S10x-3), every limb below `2^16` and the top limb at its exact width. The 128-bit and 64-bit shapes (8 and 4 limbs) SHALL both be offered. Only FV work units SHALL use the 17-bit checker (O1).

#### Scenario: Value at or above the width
- **WHEN** a limb vector that encodes `2^128` is checked at 128 bits, or a limb holds `p - 1`
- **THEN** the correct AIR refuses the witness (mutants `range_value_2_128`, `range_field_wrap`)

#### Scenario: Boundary values accepted
- **WHEN** the values 0, 1, `2^64` and `2^128 - 1` are checked at 128 bits, the 64-bit boundaries at 64 bits, and small widths at their width
- **THEN** the stock verifier accepts a real proof of each (positives `range_128_boundaries`, `range_64_boundaries`, `range_small_widths`)

### Requirement: RNG-002 The decomposition is tied to the source cell
The limb decomposition SHALL reconstruct exactly the source cell through a running sum. The top limb SHALL be bounded to the exact width. Every limb SHALL be in the table.

#### Scenario: Decomposition of another value
- **WHEN** the limbs decompose a value other than the source cell, the top limb exceeds the exact width, a limb is outside the table, or the running sum is broken
- **THEN** the correct AIR refuses the witness (mutants `range_source_copy_removed`, `range_top_pad_unlinked`, `range_limb_out_of_table`, `range_running_sum_broken`)

### Requirement: RNG-003 RANGE is a module instance in Lean
RANGE SHALL be stated as a `Module` instance in Lean: limbs in the table and the reconstruction gate SHALL imply `source < 2^B` with no wrap in the field. The kit SHALL NOT be reported complete while this obligation is open.

#### Scenario: Lean obligation O3
- **WHEN** the kit is offered for completion
- **THEN** O3 (`AluRange.lean`, premise: table soundness) is a proved theorem with an axiom report of only `propext`, `Quot.sound` and `Classical.choice`, and today O3 is open

#### Scenario: Reduced-width model
- **WHEN** the Quint lane runs for RANGE
- **THEN** the range behaviour is checked through `aluArith.qnt` at a reduced width, because RANGE has no model of its own

### Requirement: RNG-004 The RANGE AIRs meet the STARK kit gate
The RANGE table AIR and the limb queries SHALL meet the STARK kit gate SG-1 to SG-6 of `design.md` section 9 inside the app set (at most 5,000 constraints and 1,000 interactions per AIR, 100 AIRs, 30,000 columns), with every AIR required and a fixed height in its menu shape. Where the inline kernel of SHP-006 carries them, each AIR SHALL also fit the per-AIR bounds of the internal set (1,000 constraints, 200 interactions).

#### Scenario: AIR omitted from the proof
- **WHEN** a proof omits one of the RANGE table AIR and the limb queries
- **THEN** N1, N3 and `L` refuse it (mutant `range_air_omitted`), and the kit does not pass SG-1 before row T3 passes

#### Scenario: Positive and negative proofs on the stock verifier
- **WHEN** the positive `range_64_boundaries` and the mutant `range_source_copy_removed` are proved with the stark-backend prover
- **THEN** the stock verifier accepts the first, refuses the second and accepts the second under its weakened control AIR; a prover panic or tracegen error earns no rejection credit

#### Scenario: Shape, budget and D3
- **WHEN** the RANGE table AIR and the limb queries are keyed in a menu shape
- **THEN** each AIR has the fixed height of the shape, keygen refuses a shape outside the bounds above, D3 at the measured shape prints every term at 100 bits or more and every grindable term at 101 or more (SG-5), and the receipt records seconds per proved step (SG-6)
