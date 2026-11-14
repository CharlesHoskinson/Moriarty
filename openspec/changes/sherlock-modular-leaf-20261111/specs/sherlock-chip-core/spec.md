## ADDED Requirements

### Requirement: CORE-001 Each enabled slot is one reference step and a disabled slot is the identity
With `enable = 1`, a slot SHALL be the transition of `sherlock-k-a/1` for the rule that DEC selected: every register in the rule's write set SHALL take its specified next value and every other persistent register SHALL stay unchanged. A disabled slot SHALL be the identity on persistent state and SHALL add no step.

#### Scenario: Register outside the write set
- **WHEN** a register outside the write set changes, a disabled slot changes state, the selector is forged, or the post-to-pre copy is removed
- **THEN** the correct AIR refuses the witness (mutants `core_unlisted_register_changed`, `core_disabled_state_changed`, `core_selector_forged`, `core_post_pre_copy_removed`), and the Quint model `processorCore.qnt` checks that each enabled slot projects to one owner transition

#### Scenario: Lean evidence
- **WHEN** the CORE framework is offered for completion
- **THEN** P3 (`leaf_refines`) and P4 (`leaf_refines_step`, `follows_run`, `fun_det`) are proved in C3 in generic form, and O8 (`CoreRefinement.lean`, concrete contract over the full rule set) and O9 (`LeafSound.lean`, `leaf_sound` derived) are open

### Requirement: CORE-002 Enables form a non-empty prefix and heights stay below 2^64
`enable` SHALL be boolean, the enables SHALL form a prefix, and the first slot SHALL be enabled (`1 <= steps <= S`). Every micro-operation of a slot SHALL use the slot's enable. `steps` SHALL count enabled slots, not micro-operations. `h_post = h_pre + steps` SHALL hold with both below `2^64` (SRC-004).

#### Scenario: Zero steps, broken prefix or wrapped height
- **WHEN** `steps = 0`, the prefix is broken, an enable is split across micro-operations, micro-operations are counted as steps, or `h` reaches `2^64`
- **THEN** the correct AIR refuses the witness (mutants `core_zero_step_leaf`, `core_enable_nonprefix`, `core_enable_split_across_microops`, `core_steps_counts_microops`, `core_height_wrap_2_64`)

#### Scenario: Height edge
- **WHEN** `h_pre = 2^64 - 2` and one step is proved
- **THEN** the stock verifier accepts a real proof of the leaf (positive `core_height_edge`)

### Requirement: CORE-003 No step follows a terminal outcome and closing a leaf changes nothing
No enabled step SHALL start from a terminal outcome, and a publishing step SHALL be the last enabled step. Closing a leaf SHALL change no counter, fuel, outcome or length. A step that does not fit the remaining capacity SHALL NOT be enabled and SHALL NOT produce `Rejected` or `Exhausted` (SVM-010).

#### Scenario: Step after a terminal state or a charging close
- **WHEN** a step follows a terminal outcome, or closing a leaf charges fuel or advances the length
- **THEN** the correct AIR refuses the witness (mutants `core_terminal_followed_by_step`, `core_close_charges_fuel`, `core_close_advances_length`)

### Requirement: CORE-004 Every read goes through the read port below the running length
Every record and ROM row that a rule, VALUE DIGEST or INIT uses SHALL be returned by the read port of `core/ports.rs` as `AuthRec` or `AuthRom`. `read_record` SHALL constrain `addr < len_cur` with `len_cur` the cell `len_at[at]` of the append chain, the expected kind, a copy-linked address, and exactly one route: window, heap line or scalar cold read.

#### Scenario: Future read or free kind
- **WHEN** a read is at `len_cur`, of a later record, between two appends with `len_post` in place of the running length, with a free kind or with a free address
- **THEN** the correct AIR refuses the witness (mutants `core_future_read`, `core_read_len_post_instead_of_running`, `core_read_kind_unconstrained`, `core_read_address_free`), and the positive `core_read_between_two_appends` passes

#### Scenario: Unrouted or misrouted old read
- **WHEN** an old read has the bus request and the scalar cold read both off, the cold bit is forged, or `line = 1` with `cold = 0`
- **THEN** the correct AIR refuses the witness (mutants `core_read_old_record_unrouted`, `core_read_cold_bit_forged`, `core_read_line_bit_on_window_address`), and the control port with a free `ColdRead.enable` bit accepts the first witness

#### Scenario: Unranged ROM row
- **WHEN** `read_rom` is given a row that is not a RANGE handle of width `D_row`
- **THEN** the correct AIR refuses the witness (mutant `rom_row_overflow_alias`)

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs the `AuthRec` and `AuthRom` cells
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-9 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: CORE-005 Appends are dense and one to one with window rows
CORE SHALL own the append chain with conditions A1 to A4: the first append address equals `len_pre` (or `len_post = len_pre` with no append), each next address is `addr + 1`, the last output equals `len_post`, and each active append issues one MEM append request with its own enable and the rule's kind constant in `1..=4`.

#### Scenario: Append without a window row
- **WHEN** an append has no window row, its bus request is off, its address or kind is free, or the append order is reversed
- **THEN** the correct AIR refuses the witness (mutants `core_append_without_window_row`, `core_append_request_disabled`, `core_append_address_free`, `core_append_kind_free`, `core_append_order_reversed`), and O45 (`MemContract.lean`, append bijection) is open

#### Scenario: Zero-allocation loop
- **WHEN** a loop that allocates nothing runs over a full leaf
- **THEN** the stock verifier accepts a real proof of the leaf and no address is consumed (CORE positive, zero-allocation loop)

### Requirement: CORE-006 `rules_d` is the SRC-001 hash of the S slot rule lanes
CORE SHALL compute `rules_d = Poseidon(dom("SHK_RULES"), rid_0, ..., rid_(S-1))` over exactly S lanes in slot order in one whole-sponge HASH slot, with `rid_j = 0` for a disabled slot, and SHALL bind it to the leaf public value `rules_d` (word 32, FP-1). There SHALL be no special case for S = 1. S SHALL be 16 for the first build (AD-6).

#### Scenario: Forged or reordered rules digest
- **WHEN** the public value `rules_d` is not that hash, the lanes are reordered, a disabled slot has a non-zero lane, or at S = 1 the single lane is published without the hash
- **THEN** the correct AIR refuses the witness (mutants `core_rules_d_forged`, `core_rules_d_lanes_reordered`, `core_rules_d_disabled_lane_nonzero`, `core_rules_d_s1_shortcut`)

#### Scenario: Reference vectors
- **WHEN** `rules_d` is computed for S = 16 full, S = 16 with a disabled suffix, and an S = 1 key shape
- **THEN** it equals a host reference written from SRC-001 (positive `core_rules_d_reference_vectors`), and O44 (`CoreRefinement.lean`, `rules_d` binds the lane list) is open

### Requirement: CORE-007 A step is independent of its incoming scratch
For every slot, the persistent post state, every request and every port cell SHALL be a function of the persistent pre state and the authenticated inputs only. No constraint other than the next-state constraint of a scratch lane SHALL read `Pre(v_k)`. Configuration SHALL refuse a plan with a scratch use before its definition in the same slot.

#### Scenario: Free incoming scratch
- **WHEN** any value is placed in every scratch lane at every slot entry
- **THEN** acceptance and every public lane are unchanged (positive `core_scratch_incoming_free`), and O10 (`ScratchIndependence.lean`, non-interference) is open

#### Scenario: Plan reads scratch before it defines it
- **WHEN** a rule plan reads `v0` and writes `v0` later in the same slot, or a constraint reads `Pre(v_k)`
- **THEN** configuration refuses the plan (`core_plan_scratch_use_before_def`, `core_plan_reads_pre_scratch`), and the controls `ctl_scratch_read_then_write` and `ctl_scratch_cross_slot_link` accept two witnesses with different `post_d`

### Requirement: CORE-008 A compound stack step is one owner event
CORE SHALL pass one `StackStep` per stack per slot to STACKS: an optional rewrite of the top, then one of none, push or pop, in that order. CORE SHALL NOT split a slot in two to express a rewrite followed by a push or a pop. The step count SHALL be the owner-event count.

#### Scenario: Rewrite then push in one slot
- **WHEN** `BHelperValidateField` or `BHelperEqualField` rewrites the top task and pushes a child on the non-tail branch
- **THEN** one slot proves it and `steps` counts one (positive `core_rewrite_then_push_one_slot`)

### Requirement: CORE-009 The request inventory is complete and unused lanes are zero
CORE SHALL issue only the requests of its inventory: ALU, MEM read, MEM append, ROM and two-input HASH. Every unused request SHALL have count 0, and an active request SHALL be on the bus of its operation. A value digest SHALL come from VALUE DIGEST and SHALL NOT be supplied by the host.

#### Scenario: Unused lane or host digest
- **WHEN** an unused request has a non-zero count, or a value digest is supplied by the host
- **THEN** the correct AIR refuses the witness (mutants `core_unused_lane_nonzero`, `core_value_digest_host_supplied`)

### Requirement: CORE-010 Leaf shape capacities fit every rule maximum
`LeafShape { s, w_flush, k_page, k_rom, n_cold }` SHALL fit the largest single step of every rule over every branch at one step per leaf and every tail occupancy. A shape below a rule maximum SHALL be refused before key generation. `k_page`, `k_rom` and `n_cold` SHALL NOT be frozen before X15 and X5 (AD-3, L-2).

#### Scenario: Shape below a rule maximum
- **WHEN** a `LeafShape` is below a rule maximum of the per-rule table
- **THEN** the shape check refuses it before key generation (`leafshape_below_rule_max_refused`), and the maximal step of the rule proves at one step per leaf

#### Scenario: Values known from X1
- **WHEN** the shape is configured for the first build
- **THEN** `s = 16`, `w_flush = 1` and `k_page + n_cold >= 4` hold by the X1 table `FD/maxima/rule_maxima.md` (M-2, M-3), a shape with `8 W` below the append maximum is refused (`mem_shape_step_exceeds_window`), and `k_rom` waits for X15

### Requirement: CORE-011 Each rule of a rule family is its reference transition
Each rule family sub-kit SHALL be generated from the `RuleContract`s of its rules (read set, write set, ordered plan, charge behaviour, effect bounds). Each rule SHALL be its reference transition on every branch, checked against the REFERENCE BRIDGE kit. A family that builds constructors SHALL wait for X6 (AD-5).

#### Scenario: One positive per branch
- **WHEN** the rule fires on each branch of its reference code
- **THEN** the post state of the slot equals the reference post state (one positive per branch, compared through the REFERENCE BRIDGE differential)

#### Scenario: One mutant per written register class
- **WHEN** a register outside the rule's write set changes, or a write has another value
- **THEN** the correct AIR refuses the witness (one named mutant per written register class of the family)

### Requirement: CORE-012 Each rule keeps the reference charge, outcome and effect bounds
Each rule SHALL charge fuel and counters exactly as the reference does on every branch, and each failure branch SHALL publish the reference outcome. The spike's one-fuel-per-step rule SHALL NOT be copied. The appends, reads, ROM rows and stack step of the maximal branch SHALL be within the `LeafShape`.

#### Scenario: Wrong charge or wrong failure outcome
- **WHEN** fuel or a counter is charged differently, or a failure branch publishes another outcome
- **THEN** the correct AIR refuses the witness (one named mutant per charge and per failure branch of the family)

#### Scenario: Maximal branch at one step per leaf
- **WHEN** the rule runs its maximal branch at one step per leaf
- **THEN** the stock verifier accepts a real proof of the leaf (a per-rule maximal-step positive, checked against the row of `FD/maxima/rule_maxima.md`)

### Requirement: CORE-013 Input nodes are read through `AuthIn` under the certified view
Under input admission form (b) (AD-17, OWNER), `read_input` SHALL return an `AuthIn` token for the input root or for field `k < m` of an earlier token, opened by a Merkle path of depth `D_view` against the `view_root` header word (FPR-012). A read SHALL add no MEM append and SHALL charge no DataNodes or DataFields cap, unless X11 shows a reference charge; then the charge SHALL be a constant per read stated in the rule contract.

#### Scenario: Read outside the view
- **WHEN** a rule uses an input record that is not under `view_root`, or an index that is not reached from the input root
- **THEN** the correct AIR refuses the witness (mutant `in_read_outside_view`)

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs `D_view`, the view form and the `AuthIn` cells
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-10 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: CORE-014 The CORE AIRs meet the STARK kit gate
The CORE slot AIRs SHALL meet the STARK kit gate SG-1 to SG-6 of `design.md` section 9 inside the app set (at most 5,000 constraints and 1,000 interactions per AIR, 100 AIRs, 30,000 columns), with every AIR required and a fixed height in its menu shape. Where the inline kernel of SHP-006 carries them, each AIR SHALL also fit the per-AIR bounds of the internal set (1,000 constraints, 200 interactions).

#### Scenario: AIR omitted from the proof
- **WHEN** a proof omits one of the CORE slot AIRs
- **THEN** N1, N3 and `L` refuse it (mutant `core_air_omitted`), and the kit does not pass SG-1 before row T3 passes

#### Scenario: Positive and negative proofs on the stock verifier
- **WHEN** the positive `core_height_edge` and the mutant `core_rules_d_forged` are proved with the stark-backend prover
- **THEN** the stock verifier accepts the first, refuses the second and accepts the second under its weakened control AIR; a prover panic or tracegen error earns no rejection credit

#### Scenario: Shape, budget and D3
- **WHEN** the CORE slot AIRs are keyed in a menu shape
- **THEN** each AIR has the fixed height of the shape, keygen refuses a shape outside the bounds above, D3 at the measured shape prints every term at 100 bits or more and every grindable term at 101 or more (SG-5), and the receipt records seconds per proved step (SG-6)

### Requirement: CORE-015 The slice value set proves a Halted run
The slice leaf SHALL have a CORE rule subset chip set 2 (the value set) under its own per-program leaf key in `key_root`, with the rules `XAdtEvalConstructNullary` and `KAdminPublishValue`. The Halted payload SHALL equal the `vd` word of the constant's ROM row, and the INIT opening of `input_d` SHALL take a witnessed input value digest `val_d`. Chip sets 0 and 1 SHALL keep `VAL_NONE` and their keys.

#### Scenario: Rules, reads and digests
- **WHEN** chip set 2 proves a step
- **THEN** the rules are T5 3.4 rows 119 and 2, the ROM reads are `expressions[e]`, `constructors[ctor]` and `constants[cid]`, the payload is the constant VALUE DIGEST under the PROG premise, and the input enters by digest (AD-17) and is bound at the seal by the input link of CLM-005

#### Scenario: Halted producer and consumer at the app set
- **WHEN** `type U { A(); } entry construct U::A();` (producer) and the consumer whose input `val_d` is the producer's Halted `vd` are proved under chip set 2
- **THEN** the stock verifier accepts both, and a forged payload, a relabelled outcome tag, a forged constant row `vd` and a forged INIT input value are refused by the debug checker and by the stock verifier (example `value_set`)

#### Scenario: Chip set 0 is unchanged
- **WHEN** the chip set 0 and 1 app keys are built after the change
- **THEN** their pre-hashes equal the T8 values (`T7KEY` lines)
