## ADDED Requirements

### Requirement: DEC-001 Exactly one enabled rule per enabled slot, in reference priority
DEC SHALL select exactly one rule of the frozen inventory for each enabled slot, in the priority of the reference machine, and SHALL return its `rule_lane` and bounded control parts through the DEC port. The inventory SHALL be the 120 enabled step rules of X1 (AD-10, M-4), explicit, versioned and ordered.

#### Scenario: Two rules or none
- **WHEN** two rules are selected, none is selected with `enable = 1`, or the priority is swapped
- **THEN** the correct AIR refuses the witness (mutants `dec_two_rules_selected`, `dec_enabled_without_rule`, `dec_priority_swapped`), and the Quint model `decDispatch.qnt` checks the dispatch against the reference priority

#### Scenario: Every enabled rule
- **WHEN** each of the 120 enabled rules is decoded
- **THEN** the stock verifier accepts a real proof of each (positive `dec_every_enabled_rule_roundtrip`), including `dec_publication_with_zero_fuel` and `dec_comparator_boundaries`

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs the AD-9 guard list of the 120 rules
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-7 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: DEC-002 Disabled and extension rules have no row
The DEC table SHALL hold no row for a disabled rule or a `/2` extension rule. DEC SHALL enforce the profile `/1`. The treatment of exhaust kind 7 under `/1` SHALL follow the resolution of AD-16 (OWNER) and SHALL be refused until then.

#### Scenario: Disabled or extension rule queried
- **WHEN** a slot queries the pre-state of a disabled rule or a `/2` rule
- **THEN** the correct AIR refuses the witness (mutants `dec_disabled_rule_query`, `dec_extension_rule_query`), and O6 (`DecDispatch.lean`, DEC as a `Module` over the frozen inventory) is the Lean evidence, open today

### Requirement: DEC-003 Every unpacked control part is bounded
Every key part and control word part SHALL be bounded by its width (LC AUDIT item 20), so that the packing is injective in the field and the unpacked parts are the parts of the table row.

#### Scenario: Wrap forgery
- **WHEN** a control part is changed and compensated in another part modulo the field
- **THEN** the correct AIR refuses the witness (mutant `dec_wrap_forgery`), the control test without the part bounds accepts it, and O5 (`DecDispatch.lean`, packing injectivity) is the Lean evidence, open today

#### Scenario: Forged metadata
- **WHEN** the metadata, the write set or the instruction source of a row is forged
- **THEN** the correct AIR refuses the witness (mutants `dec_metadata_forged`, `dec_write_set_swapped`, `dec_wrong_instruction_source`, `dec_guard_unlinked`)

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs the control word layout `dec_key`, `dec_val`
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-7 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: DEC-004 A disabled slot returns lane 0 and lane 0 is no rule
A disabled slot SHALL return `rule_lane = 0`. No rule of the inventory SHALL have lane 0 or key 0. A DEC query SHALL NOT match a RANGE row.

#### Scenario: Non-zero lane on a disabled slot
- **WHEN** a disabled slot carries a non-zero `rule_lane`, an enabled rule has lane 0, or a DEC query is shaped as a RANGE row
- **THEN** the correct AIR refuses the witness (mutants `dec_disabled_slot_nonzero_lane`, `dec_rule_lane_zero`, `dec_range_row_alias`), and the positive `dec_disabled_slot_idle_row` passes

#### Scenario: Bad table at load
- **WHEN** the table has a duplicate key or a rule with key 0
- **THEN** configuration fails (configuration failures `dec_duplicate_key_at_load`, `dec_rule_with_key_zero_at_load`)

### Requirement: DEC-005 The DEC AIRs meet the STARK kit gate
The DEC table AIR SHALL meet the STARK kit gate SG-1 to SG-6 of `design.md` section 9 inside the app set (at most 5,000 constraints and 1,000 interactions per AIR, 100 AIRs, 30,000 columns), with every AIR required and a fixed height in its menu shape. Where the inline kernel of SHP-006 carries them, each AIR SHALL also fit the per-AIR bounds of the internal set (1,000 constraints, 200 interactions).

#### Scenario: AIR omitted from the proof
- **WHEN** a proof omits one of the DEC table AIR
- **THEN** N1, N3 and `L` refuse it (mutant `dec_air_omitted`), and the kit does not pass SG-1 before row T3 passes

#### Scenario: Positive and negative proofs on the stock verifier
- **WHEN** the positive `dec_every_enabled_rule_roundtrip` and the mutant `dec_two_rules_selected` are proved with the stark-backend prover
- **THEN** the stock verifier accepts the first, refuses the second and accepts the second under its weakened control AIR; a prover panic or tracegen error earns no rejection credit

#### Scenario: Shape, budget and D3
- **WHEN** the DEC table AIR are keyed in a menu shape
- **THEN** each AIR has the fixed height of the shape, keygen refuses a shape outside the bounds above, D3 at the measured shape prints every term at 100 bits or more and every grindable term at 101 or more (SG-5), and the receipt records seconds per proved step (SG-6)
