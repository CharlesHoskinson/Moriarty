## ADDED Requirements

### Requirement: ALU-001 The tuple is `(ALU, op + 8 flag, x, y, z)` and `(z, flag)` is unique
ALU SHALL provide the tuple `(1, op + 8 flag, x, y, z)` with `flag` boolean. For each `(op, x, y)` the gates and the range checks SHALL admit exactly one `(z, flag)`. The identity of a provider row SHALL be `(op, x, y)`, not the key field alone.

#### Scenario: Second result for the same inputs
- **WHEN** a second result is offered for the same `(op, x, y)`, or a carry or borrow is forged
- **THEN** the correct circuit refuses the witness (mutants `alu_second_result_for_same_inputs`, `alu_carry_forged`, `alu_borrow_forged`), and the Quint model `aluArith.qnt` shows uniqueness of `(z, flag)` at a reduced width

#### Scenario: Lean obligations
- **WHEN** the kit is offered for completion
- **THEN** O4 (`AluRange.lean`, unique `(z, flag)`, no field wrap in the multiply) is proved, and P5 (`toy_contracts`, `toy_refines`, `toy_needs_bus`, proved in C3) is cited only as the composition pattern, and today O4 is open

### Requirement: ALU-002 Operands and results are ranged inside the chip
ALU SHALL range-check `x`, `y`, `z` and every internal witness below `2^128` inside the chip (LC AUDIT item 21). It SHALL NOT leave the operand range to the requester.

#### Scenario: Field-wrapped operand
- **WHEN** `x = p - 5` is offered to ADD, or `x = p - 1`, `y = 2` is offered to LT with the claim `x < y`
- **THEN** the correct circuit refuses the witness (mutants `alu_operand_range_missing`, `alu_lt_field_wrap`), and the control circuit without the in-chip range check accepts the first witness

#### Scenario: Boundary vectors
- **WHEN** 0, 1, `2^64 - 1`, `2^64` and `2^128 - 1` are used for every op
- **THEN** the constraint checker accepts each (positive `alu_boundary_vectors`), and an unranged output is refused (mutant `alu_output_range_missing`)

### Requirement: ALU-003 Overflow is provable with flag 1
An overflowing ADD or MUL SHALL be provable with `flag = 1`, so that the outcome `Rejected(10)` stays provable (L-7). The multiply SHALL keep the high half and SHALL have no field residue.

#### Scenario: Honest overflow
- **WHEN** an overflowing ADD or MUL is proved with `flag = 1`
- **THEN** the constraint checker accepts it (the completeness check `alu_overflow_unprovable` passes)

#### Scenario: Overflow hidden
- **WHEN** an overflowing product is offered with `flag = 0`, the C1 term overflows, or a field residue replaces the product
- **THEN** the correct circuit refuses the witness (mutants `alu_mul_high_half_dropped`, `alu_mul_c1_overflow`, `alu_mul_field_residue`)

### Requirement: ALU-004 The op list is the reference machine's
The ALU op list SHALL be the ops of the reference machine only: ADD, LT (landed), SUB, MUL and EQ (new). ALU SHALL NOT add division, signed or bitwise ops that the machine lacks (L-7).

#### Scenario: Op outside the list
- **WHEN** a request names an op outside the list
- **THEN** the lookup refuses it (mutant `bus_key_namespace_alias`, ALU case)

#### Scenario: Comparator and equality shape
- **WHEN** LT has its boolean constraint removed, or EQ has its reverse implication removed
- **THEN** the correct circuit refuses the witness (mutants `alu_lt_boolean_removed`, `alu_eq_reverse_implication_removed`), and the positives `alu_sub_underflow_flag` and `alu_unused_slot_zero` pass
