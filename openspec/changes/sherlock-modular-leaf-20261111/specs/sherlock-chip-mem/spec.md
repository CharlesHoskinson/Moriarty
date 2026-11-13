## ADDED Requirements

### Requirement: MEM-001 Dense append and `len` advances only by real appends
The MEM window SHALL serve each new active window row at the next dense address, from `len_pre` to `len_post - 1`, one row per address, with the record that CORE appended. `len` SHALL advance only by real appends. A closing leaf SHALL NOT advance `len`. An address SHALL NOT be reused.

#### Scenario: Address reuse or advancing close
- **WHEN** a window row reuses an address, closing advances `len`, or an appended value differs from the window row
- **THEN** the correct AIR refuses the witness (mutants `address_reuse_window_row`, `close_advances_len`, `append_value_mismatch`), and P8 (`appendCells_len`, `canon_len`, proved in C3) holds

### Requirement: MEM-002 Both lengths are below `2^(TOP+3)` and TOP binds after the step counter
MEM SHALL decompose `len_pre` and `len_post` into `TOP + 3` boolean bits. The tree depth SHALL satisfy `len_0 + a e 2^64 <= 2^(TOP+3) - 1` with `a = 3` (M-2). The working value SHALL be TOP = 63, gated on AD-1 (OWNER) and on `e = 1` from AD-11. A memory bound below the step bound SHALL NOT be an acceptance path.

#### Scenario: Counter wrap
- **WHEN** `len` wraps at `2^64` or wraps in the field
- **THEN** the correct AIR refuses the witness (mutants `counter_wrap_2_64`, `counter_field_wrap`), and the positives `c1_top61_and_top63_positive`, `c1_top1_positive` and `c1_counter_edge_2_64_minus_1` pass

#### Scenario: Address-bound obligation
- **WHEN** MEM is offered for completion
- **THEN** O12 (extend `PartitionIndependence.lean`, the bound over physical cells with the genesis length) is proved, and today O12 is open

### Requirement: MEM-003 The message is injective on `(kind, addr)` and padding is off the bus
The MEM message SHALL carry `kind` and `addr` as separate typed lanes (FP-1), with `kind` in `1..=4` on provider rows. A padding row SHALL be off the bus (multiplicity 0). In one arena the kind SHALL be hashed into the record. The prototype key `8 addr + kind` with kind 0 for padding is the halo2 form only.

#### Scenario: Kind out of range
- **WHEN** kind 9 is offered with a consistent key, a padding row is placed on the bus, or the arena kind is not pinned
- **THEN** the correct AIR refuses the witness (mutants `kind_out_of_range`, `pad_row_on_bus`, `arena_kind_pinned`), and property 4 of the C1 lane model `dense.qnt` holds

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs the record lanes of each kind
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-5 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: MEM-004 Window and old-memory providers are disjoint
The window class SHALL serve only `addr >= 8 q_pre` and the heap lines SHALL serve only `addr < 8 q_pre`. A read SHALL be routed to the old-memory path exactly when `addr < 8 q_pre`.

#### Scenario: Window address sent to the cold path
- **WHEN** a window address is sent to the cold path, or a warm read has the wrong kind
- **THEN** the correct AIR refuses the witness (mutants `cold_route_forged`, `wrong_kind_warm`)

#### Scenario: Read at or beyond the length
- **WHEN** a read is at `len`, beyond `len`, or of a future record
- **THEN** the correct AIR refuses the witness (mutants `read_at_len`, `read_beyond_len`, `future_record_read`), and `mem_read_between_two_appends` is added when MEM is wired to the real CORE

### Requirement: MEM-005 The state after a run does not depend on the partition
For every cut of a run into leaves that the leaf AIRs accept, the boundary `(len, root, tail_d)` at every common cut SHALL be equal (SRC-023). SRC-023 SHALL NOT be reported as proved for the circuit rule before O14 and O47 are proved.

#### Scenario: Four cuts of 37 appends
- **WHEN** 37 appends are cut in four ways
- **THEN** `(len, root, tail_d)` is equal (positive `c1_partition_independence`), and the invariant `invRefine` of the C1 lane model `dense.qnt` holds (verdict `qa/gemini-quint/c1-dense/verdict.md`)

#### Scenario: Lean scope of the proved theorems
- **WHEN** the partition-independence evidence is cited
- **THEN** P7, P9, P10 and P14 (proved in C3) are credited only for leaves of at most 8 appends, and O14 (`FitsWindow W`) and O47 (every accepted leaf satisfies `FitsWindow W`) are open

### Requirement: MEM-006 A leaf holds at most `8 W + 7 - r_pre` new records and every single step fits
A leaf with `W` flush units SHALL accept at most `8 W + 7 - r_pre` new records, and the last window row SHALL be inactive. Every rule SHALL append at most `8 W` physical cells in one step, so a single step fits at every `r_pre` (GP-1). The working value SHALL be `W = 1` (M-3).

#### Scenario: Nine to fifteen appends at W = 1
- **WHEN** a `W = 1` leaf appends 9, 12 or `15 - r_pre` records at each `r_pre`
- **THEN** the stock verifier accepts a real proof of it with the same post state as smaller leaves (positive `mem_w1_leaf_9_to_15_appends`)

#### Scenario: Leaf sum exceeds the window
- **WHEN** steps that each fit sum to `16 - r_pre` appends in one `W = 1` leaf, or a `LeafShape` has `8 W` below the per-rule maximum
- **THEN** the leaf is unsatisfiable (mutant `mem_leaf_sum_exceeds_window`) and configuration refuses the shape (`mem_shape_step_exceeds_window`), and O15 and O47 are open

### Requirement: MEM-007 One arena, hidden by the interface
The MEM interface SHALL hide the arena count. The built configuration SHALL be one arena with kind-tagged records (M-1, decided by X2). The record layout SHALL be three-word records at consecutive addresses (AD-11), frozen before the CORE rule families start.

#### Scenario: Arena count behind the interface
- **WHEN** the MEM harness runs with one arena and with four arenas
- **THEN** both configurations pass (positive `c1_four_arenas` and the one-arena positives), and CORE is unchanged

#### Scenario: Lean module instance
- **WHEN** MEM is offered for completion
- **THEN** O13 (`MemContract.lean`, MEM window as a `Module`) and O43 (`MemHist` across leaves) are proved, and today both are open

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs the three-word record layout and the physical cells per record (AD-11)
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-5 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: MEM-008 The MEM window AIRs meet the STARK kit gate
The MEM window AIR SHALL meet the STARK kit gate SG-1 to SG-6 of `design.md` section 9 inside the app set (at most 5,000 constraints and 1,000 interactions per AIR, 100 AIRs, 30,000 columns), with every AIR required and a fixed height in its menu shape. Where the inline kernel of SHP-006 carries them, each AIR SHALL also fit the per-AIR bounds of the internal set (1,000 constraints, 200 interactions).

#### Scenario: AIR omitted from the proof
- **WHEN** a proof omits one of the MEM window AIR
- **THEN** N1, N3 and `L` refuse it (mutant `mem_air_omitted`), and the kit does not pass SG-1 before row T3 passes

#### Scenario: Positive and negative proofs on the stock verifier
- **WHEN** the positive `c1_partition_independence` and the mutant `address_reuse_window_row` are proved with the stark-backend prover
- **THEN** the stock verifier accepts the first, refuses the second and accepts the second under its weakened control AIR; a prover panic or tracegen error earns no rejection credit

#### Scenario: Shape, budget and D3
- **WHEN** the MEM window AIR are keyed in a menu shape
- **THEN** each AIR has the fixed height of the shape, keygen refuses a shape outside the bounds above, D3 at the measured shape prints every term at 100 bits or more and every grindable term at 101 or more (SG-5), and the receipt records seconds per proved step (SG-6)
