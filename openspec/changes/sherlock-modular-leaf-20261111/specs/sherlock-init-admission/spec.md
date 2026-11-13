## ADDED Requirements

### Requirement: INIT-001 At `h_pre = 0` the pre state is `Init(prog_d, input_d)`, field by field
When `h_pre = 0`, INIT SHALL constrain every word of the first pre state, the memory frontier and token, and both stacks to `Init(prog_d, input_d)` with the one-arena field list of the kit. Equality with a supplied digest SHALL NOT replace the field relation. The SRC-021 text of the field list SHALL wait for AD-18 (OWNER).

#### Scenario: An Init field differs
- **WHEN** the control, fuel, a counter, the outcome, a cached top, a stack head, the memory or `deps_d` differs from Init at `h_pre = 0`
- **THEN** the correct AIR refuses the witness (mutants `init_wrong_control`, `init_fuel_differs_from_input_d`, `init_counter_nonzero`, `init_outcome_not_running`, `init_cached_top_nonzero`, `init_stack_head_not_empty_const`, `init_memory_not_genesis`, `init_deps_not_empty`), and the Quint invariant `GenesisStateIsInit` of `init.qnt` holds

#### Scenario: Schema coverage
- **WHEN** the frozen schema and the Init table are compared
- **THEN** every schema word has an Init value (host check `init_schema_coverage`, Quint invariant `InitFieldsCoverSchema`), and O49 (`InitAdmission.lean`) is open

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs the one-arena Init field list (SRC-021)
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-15 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: INIT-002 The genesis selector is `is_zero(h_pre)`
The genesis selector SHALL be `g = is_zero(h_pre)` from the leaf public value `h_pre` (FP-1), not a witness flag. Every INIT constraint SHALL be multiplied by `g`. At `h_pre != 0` INIT SHALL constrain nothing.

#### Scenario: Non-genesis leaf
- **WHEN** `h_pre != 0`
- **THEN** the stock verifier accepts a real proof of any pre state that the previous `post_d` binds (positive `init_non_genesis_leaf_any_state`), Init forced at `h_pre = 5` is a completeness failure (`init_gate_at_nonzero_height`), and the Quint invariant `NonGenesisUnconstrainedByInit` holds

#### Scenario: Gate from a witness flag
- **WHEN** the selector is taken from a witness column, or only a supplied `init_d` is compared
- **THEN** the controls `ctl_init_gate_from_witness_flag` and `ctl_init_supplied_digest_only` accept a wrong genesis state, which the correct AIR refuses

#### Scenario: Field waits on T5
- **WHEN** the Init field list that the genesis selector gates is needed for a freeze or a kit tick
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-15 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: INIT-003 `prog_d` and `input_d` are opened and bound
INIT SHALL open `prog_d` to its SVM-005 preimage and SHALL constrain the opened `prog_root` equal to the `prog_root` body word. INIT SHALL open `input_d = P2(SHK_IN, salt, val_d, fuel_lo, fuel_hi, stack_limit)`, with 8 salt words fixed for the run at INIT, and with each of the three limbs below `2^64`. PROG canonicality SHALL be the named premise `ProgCanonical(prog_root)` of the leaf. The seal SHALL discharge it with the `ADM` claim (NODE-010).

#### Scenario: Root or input not opened
- **WHEN** `prog_root` is not the root inside `prog_d`, a fuel limb is `2^64`, `input_d` is not opened, the stack limit is unbound, or the opening of another `input_d` is used
- **THEN** the correct AIR refuses the witness (mutants `init_prog_root_not_opened`, `init_fuel_limb_2_64`, `init_input_d_not_opened`, `init_stack_limit_unbound`, `init_input_d_swapped_opening`)

#### Scenario: Entry forms
- **WHEN** an `Expr(e0)` entry with no input and a `Main(in_fn, in_ty)` entry with an input are proved, with fuel limbs at 0 and `2^64 - 1`
- **THEN** the stock verifier accepts a real proof of each (positives `init_expr_entry_no_input`, `init_main_entry_with_input`, `init_fuel_limbs_boundaries`), and `Main` with no input or the wrong `in_ty` is refused (mutant `init_entry_input_mismatch`)

### Requirement: INIT-004 The genesis memory is canonical (condition C4)
The genesis memory `(len_0, root_0, tail_0)` SHALL be a function of `(prog_d, input_d)`, SHALL be canonical, and every record in it SHALL have `kind` in `1..=4`. INIT SHALL take it as the parameter `GenesisMem` and SHALL be tested with the empty arena until AD-17 is decided.

#### Scenario: Non-canonical genesis token
- **WHEN** the genesis token is a non-empty tree or a non-canonical tail at `h_pre = 0`
- **THEN** the correct AIR refuses the witness (mutant `init_memory_not_genesis`), the Quint invariant `GenesisMemCanonical` holds, and O50 (`InitAdmission.lean`) is open

#### Scenario: Genesis record kind
- **WHEN** AD-17 selects a form with genesis records and one record has a kind outside `1..=4`
- **THEN** the correct AIR refuses the witness (mutant `init_genesis_record_kind_out_of_range`)

### Requirement: INIT-005 Input admission scans nothing and bounds nothing
Any input admission form SHALL meet I1 to I6 of the kit: no scan of the input, no input-size bound, every used input node authenticated against `val_d` in bounded work, the reference outcome and step count of `apply(prog, x)`, a canonical genesis memory, and one `val_d` per structured value. The form SHALL be AD-17 (OWNER), measured by X11.

#### Scenario: Unauthenticated input node
- **WHEN** a step uses an input node that was not opened against `val_d`
- **THEN** the correct AIR refuses the witness (mutant `init_input_node_unauthenticated`)

#### Scenario: Form not yet decided
- **WHEN** the input part of INIT is offered for a freeze before AD-17 and X11
- **THEN** the input part stays open, and the rest of the kit is built and tested with the empty genesis memory (`init_memory_not_genesis` against the empty arena, Quint invariant `GenesisMemCanonical`)

### Requirement: INIT-006 Under form (b) the input type is checked on inspected nodes
Under input admission form (b) (AD-17, OWNER), INIT SHALL set the `view_root` header word (FPR-012) from the witness, or to `VIEW_NONE` when `val_d` is the absent-input constant; `read_input` SHALL refuse `VIEW_NONE`. `in_ty` admissibility SHALL be checked on inspected nodes only (I7); an uninspected subtree SHALL be carried by digest. A full check of `ty(x) == in_ty` by VIEW-WORK SHALL be used only by owner decision.

#### Scenario: Inspected node of the wrong type
- **WHEN** a step reads an input node through `AuthIn` whose tag or `ty_d` is not the one its rule requires
- **THEN** the correct AIR refuses the witness (mutant `init_entry_input_mismatch`)

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs `D_view` and the `ty_d` normal form
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-10 and I-14 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: INIT-007 The co-processor forms of `prog_d` and `input_d`
Under `sherlock-processor-final-20261111`, `prog_d` SHALL be `P2(DOM_PROG, image_d, plan_d, key_root, machine_version)` (SPL-003) and `input_d` SHALL be `P2(SHK_IN, salt, val_d, fuel_lo, fuel_hi, stack_limit)` with 8 salt words fixed at INIT (ZK-006). The forms of INIT-003 are the halo2 prototype forms. INIT SHALL open `prog_d` through `image_d` to `prog_root` with the preimages of `FD/ARCHITECTURE.md` section 1.11, FP-4 (FPR-011).

#### Scenario: Prototype opening
- **WHEN** INIT opens `prog_d` to the SVM-005 preimage without `plan_d` and `key_root`, or opens an unsalted `input_d`
- **THEN** the opened value is not the co-processor `prog_d` or `input_d`, so INIT opens the FP-4 preimages (FPR-011)

### Requirement: INIT-008 The INIT AIRs meet the STARK kit gate
The INIT AIRs SHALL meet the STARK kit gate SG-1 to SG-6 of `design.md` section 9 inside the app set (at most 5,000 constraints and 1,000 interactions per AIR, 100 AIRs, 30,000 columns), with every AIR required and a fixed height in its menu shape. Where the inline kernel of SHP-006 carries them, each AIR SHALL also fit the per-AIR bounds of the internal set (1,000 constraints, 200 interactions).

#### Scenario: AIR omitted from the proof
- **WHEN** a proof omits one of the INIT AIRs
- **THEN** N1, N3 and `L` refuse it (mutant `init_air_omitted`), and the kit does not pass SG-1 before row T3 passes

#### Scenario: Positive and negative proofs on the stock verifier
- **WHEN** the positive `init_expr_entry_no_input` and the mutant `init_wrong_control` are proved with the stark-backend prover
- **THEN** the stock verifier accepts the first, refuses the second and accepts the second under its weakened control AIR; a prover panic or tracegen error earns no rejection credit

#### Scenario: Shape, budget and D3
- **WHEN** the INIT AIRs are keyed in a menu shape
- **THEN** each AIR has the fixed height of the shape, keygen refuses a shape outside the bounds above, D3 at the measured shape prints every term at 100 bits or more and every grindable term at 101 or more (SG-5), and the receipt records seconds per proved step (SG-6)
