## ADDED Requirements

### Requirement: PAGE-001 Heap cache lines open against the pre-state root
A heap line SHALL authenticate its 8 records as `H_REC` leaves of `Tree8` and a TOP-level walk to the leaf's `pre.root`. Its walk bits SHALL be the decomposition of `blk`, and `blk` SHALL be below `q_pre`. It SHALL serve `(MEM, 8 (8 blk + j) + kind_j, a, b, c)` from the hashed cells, with `j` fixed by the row.

#### Scenario: Line against a segment of the same leaf
- **WHEN** a line authenticates a segment flushed in the same leaf, or `blk >= q_pre`
- **THEN** the correct circuit refuses the witness (mutants `line_uses_post_root`, `line_blk_ge_qpre`), and the Quint invariant `LineMatchesPreRoot` of `denseLineCache.qnt` holds

#### Scenario: Served cell not hashed
- **WHEN** a served cell differs from the hashed cell, or the walk bits are not `blk`
- **THEN** the correct circuit refuses the witness (mutants `line_provider_not_hashed`, `line_walk_bits_not_blk`), and O18 (`PageCache.lean`, line soundness) is open

#### Scenario: All 8 records and TOP = 63
- **WHEN** a line serves all 8 records at TOP = 63, beside a scalar cold read in the same leaf
- **THEN** the constraint checker accepts the leaf (positives `ok_line_all_8_records`, `ok_line_top63`, `ok_line_and_scalar_cold_read_same_leaf`)

### Requirement: PAGE-002 Disabled, duplicate and unordered lines
A line with `en = 0` SHALL serve the zero tuple and SHALL turn off its root equality. Duplicate or unordered lines SHALL need no extra constraint, and two lines that open one block SHALL serve identical tuples.

#### Scenario: Disabled line serves a tuple
- **WHEN** `en = 0` and a non-zero tuple is served
- **THEN** the correct circuit refuses the witness (mutant `line_disabled_nonzero`)

#### Scenario: Two lines open one block
- **WHEN** two lines open one block with equal records, and with different records
- **THEN** the constraint checker accepts the first (positive `ok_duplicate_and_unordered_lines`) and the correct circuit refuses the second (mutant `line_dup_block_two_values`)

### Requirement: PAGE-003 Lines change no boundary value
Opening heap or ROM lines SHALL NOT change `(len, root, tail_d)`, `pre_d`, `post_d` or the public statement.

#### Scenario: Zero, one or K_page lines
- **WHEN** a leaf opens 0, 1 or `K_page` lines
- **THEN** the post token and the digests are equal (positive `line_boundary_invariance`), the Quint invariant `BoundaryIndependentOfLines` holds, and O17 (`PageCache.lean`, boundary invariance) is open

### Requirement: PAGE-004 ROM lines under `prog_root`
ROM SHALL have its own chip id. Its key SHALL be `tbl 2^D_row + row`, with bits from the walk, and its leaf and node tags SHALL differ from the heap tags and carry heights. Padding SHALL be canonical. ROM SHALL trust row content only through the PROG canonicality premise, which the seal discharges with the `ADM` claim (NODE-010). `D_row` SHALL cover every table index of the language (AD-12).

#### Scenario: Table alias or row overflow
- **WHEN** a row of one table is served under another table's key, or the requested row overflows `D_row`
- **THEN** the correct circuit refuses the witness (mutants `rom_tbl_row_alias`, `rom_row_overflow_alias`)

#### Scenario: Heap node in a ROM walk
- **WHEN** a heap node is spliced into a ROM walk
- **THEN** the correct circuit refuses the witness (mutant `rom_heap_domain_splice`), the Quint invariants `RomKeyInjective` and `RomImmutable` hold, and O20 (`RomLine.lean`) is open

#### Scenario: Each table and the last partial line
- **WHEN** a ROM line is opened for each table and for the last partial line
- **THEN** the constraint checker accepts each (positives `ok_rom_line_each_table`, `ok_rom_last_partial_line`)

### Requirement: PAGE-005 Line capacity covers every rule and waits for X4 and X15
`K_page + n_cold`, `K_rom` and the lane count SHALL be at least the per-step maxima of every branch of every rule. A shape below them SHALL be refused before key generation. `K_rom` SHALL NOT be frozen before X15, and the line cost and ROM depth SHALL NOT be frozen before X4 (AD-3, AD-12).

#### Scenario: Shape below a rule maximum
- **WHEN** a `LeafShape` is below a rule maximum
- **THEN** the shape check refuses it (`leafshape_below_rule_max_refused`), and the maximal step proves at S = 1 (positive `ok_max_rule_step_at_s1`)

### Requirement: PAGE-006 Heap line rows repeat the kind gate while input admission is open
Heap line rows SHALL repeat the kind gate `kind` in `0..=4` (V-3 option b) until kit INIT proves the genesis memory (condition C4). The landed MEM and backend mutants SHALL keep their result with lines enabled.

#### Scenario: Landed mutants with lines on
- **WHEN** lines are enabled
- **THEN** the mutants `future_record_read`, `read_at_len`, `wrong_kind_warm`, `wrong_kind_cold`, `cold_value_forged`, `cold_route_forged`, `kind_out_of_range` and `empty_seg_forgery` are still refused, and O19 (the line as a `Module`) is open
