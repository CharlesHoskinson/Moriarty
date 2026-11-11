# Area 1 draft: Sherlock processor modules

Status: proposed architecture. This draft defines CORE, DEC, ALU + RANGE, HASH and BUS. Memory storage, boundary commitments and recursion remain separate modules.

The processor uses the narrow pair-row layout, one typed bus schema and named copy-constrained ports. It preserves the reference rule sequence, logical resource accounting and terminal outcomes. Changing the memory backend or boundary commitment must not change processor rules.

## 1. Evidence and scope

Source names below refer to these repository files. Line ranges identify the evidence used.

| Name | Source |
|---|---|
| MOD | [MODULARITY.md](/home/hoskinson/Projects/Moriarty/research/sherlock-final-design-20261111/MODULARITY.md:7), lines 7–12 |
| DESIGN | [design.md](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/design.md:74), especially lines 74–146 and 220–267 |
| REQ | [requirements.md](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/requirements.md:5), SVM lines 9–20, SRC-021–023 lines 46–48, GP lines 56–59 |
| GP | [general-purpose.md](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:7), lines 7–58 |
| COST | [experiments-2.md](/home/hoskinson/Projects/Moriarty/research/sherlock-processor-analogy-20261110/experiments-2.md:49), lines 49–146 |
| C6 | [review.md](/home/hoskinson/Projects/Moriarty/research/sherlock-c6-outcome-ledger9-review-20261110/review.md:85), lines 85–142 |
| CACHE | [lookup-cache lane B, round 1](/home/hoskinson/Projects/Moriarty/research/sherlock-lookup-cache-20261111/lanes/B/round1.md:77), lines 77–109, 120–160 and 183–199 |
| RUN-B | [run-length lane B, round 2](/home/hoskinson/Projects/Moriarty/research/sherlock-runlength-consensus-20261111/lanes/B/round2.md:19), lines 19–62 |
| RUN-E | [run-length lane E, round 2](/home/hoskinson/Projects/Moriarty/research/sherlock-runlength-consensus-20261111/lanes/E/round2.md:21), lines 21–44 |
| SKELETON | [builds.md](/home/hoskinson/Projects/Moriarty/research/skeleton-coding-20261108/builds.md:3), lines 3–16 and 92; [testing.md](/home/hoskinson/Projects/Moriarty/research/skeleton-coding-20261108/testing.md:29), lines 29–45 |

The following are facts about the current spike, not claims about the completed processor:

- The final spike uses 12 advice columns, 13 equality columns, 20 fixed columns, four lookup arguments and degree six. Its reported leaf, node and decider sizes are 6,511, 823,466 and 929,166 rows respectively. It still lacks the complete rule inventory, PROG, TRAP and Init. [DESIGN:125–136]
- Pair rows preserve rotations `{-1, 0, +1}`. They need gates for conditional register updates and copy constraints between consecutive slots. [COST:63–91]
- The register spike has 32 words and six stand-in rules. Its two-word cached frames do not represent the full frame record. [rf.rs](/home/hoskinson/Projects/Moriarty/crates/mori-chip/src/rf.rs:1), lines 1–26.
- The C6 publication decision does not adopt the chip leaf. The published run statement is `[prog_d, input_d, outcome_d, n_steps]`. Physical state commitments remain internal. [DESIGN:138–144; C6:85–100]
- The existing chip Quint models abstract hashing and use a small ISA. They do not establish refinement of all real rules. [chipLeaf.qnt](/home/hoskinson/Projects/Moriarty/models/sherlock/quint/chipLeaf.qnt:6), lines 6–46.

All contracts, encodings and signatures below are proposals unless identified as existing behavior. They are not implemented or verified by this draft.

## 2. Shared interface contract

### 2.1 Cells and ownership

The proof field is `midnight_curves::Fq`, as in [lib.rs](/home/hoskinson/Projects/Moriarty/crates/mori-chip/src/lib.rs:38), lines 38–42.

Every module interface uses either:

1. A tuple `(chip, key, a, b, c)` checked by BUS.
2. A named port containing assigned cells connected by equality constraints.

Passing the same host value to two assignments does not connect them. A host integer type does not establish an in-circuit range.

Use these circuit-level value classes:

| Class | Meaning | Constraint owner |
|---|---|---|
| `BitCell` | Integer in `{0,1}` | Module that creates the bit |
| `UIntCell<B>` | Integer in `[0,2^B)` | RANGE, through an explicit check or an authenticated invariant on the same cells |
| `TagCell<T>` | Member of a fixed finite tag set | Owning record decoder |
| `FieldCell` | Arbitrary field element, including a digest | No integer range check |
| `RecordCells<R>` | Fixed ordered lanes, canonical tags and zero inactive lanes | Owning record decoder |
| `BoundaryCells` | Ordered persistent state cells and named module state | Boundary composer copies cells supplied by their owners |

An authenticated invariant means a property established at Init and preserved through linked transitions. It does not mean a host assertion or an arbitrary resumed state.

Each port manifest records its name, lane order, range, producer, consumer and activation condition. Raw internal advice columns remain private.

### 2.2 One schema, with explicit lookup accounting

The current implementation has two BUS lookup arguments, one DEC lookup and one RANGE lookup. `bus.rs` creates a lookup for each request lane. [bus.rs](/home/hoskinson/Projects/Moriarty/crates/mori-chip/src/bus.rs:46), lines 46–61; [dec.rs](/home/hoskinson/Projects/Moriarty/crates/mori-chip/src/dec.rs:20), lines 20–30; [range.rs](/home/hoskinson/Projects/Moriarty/crates/mori-chip/src/range.rs:92), lines 92–102.

Retain that measured arrangement for the first complete processor:

- Two request lanes use the same five-field schema and the same provider table.
- DEC uses a named copy-constrained port backed by its fixed ROM lookup.
- RANGE uses named copy-constrained ports backed by one limb lookup.
- Additional requests occupy more rows. They do not create more lookup arguments.

Thus “one bus” means one schema and provider namespace. It does not mean one PLONK lookup argument. A literal single-argument implementation is a separate measurement candidate.

### 2.3 Chip identifiers and key spaces

Preserve the existing ALU, MEM and HASH identifiers: `1`, `2` and `3`. [bus.rs:29–31; poseidon.rs:52–61]

| ID | Name | Tuple |
|---:|---|---|
| 0 | Disabled | `(0,0,0,0,0)` only |
| 1 | ALU | `(1, op + 8·flag, x, y, z)` |
| 2 | CELL | `(2, cell_key, w0, w1, w2)` |
| 3 | HASH2 | `(3, 2, x, y, H([x,y]))` |
| 4 | PROG | `(4, prog_key, w0, w1, w2)` |
| 5 | Reserved DEC | No active bus provider in this version |
| 6 | Reserved RANGE | No active bus provider in this version |
| 7 and above | Reserved | Rejected until a versioned interface assigns them |

Reserve DEC and RANGE identifiers to prevent another provider from taking their names. Their initial interfaces use copied cells, so they do not emit bus tuples.

Define memory keys as:

```text
cell_key = index + 2^64 · arena + 2^67 · chunk

arena:
  0 = DATA
  1 = FIELDS
  2 = ENV
  3 = SLOTS

0 <= index < 2^64
0 <= chunk < 2^16
```

`chunk` selects three consecutive lanes of one fixed-width record. The record schema determines its permitted chunks and zero padding. A wider record consumes additional tuples. It does not compress unrelated lanes into an unchecked field value.

Define program keys as:

```text
prog_key = index + 2^64 · table + 2^68 · chunk

table:
  0 = expressions
  1 = expression children
  2 = functions
  3 = constants
  4 = types
  5 = constructors
  6 = match plans

0 <= index < 2^64
0 <= chunk < 2^16
```

Other table codes are reserved. These packings are injective within their declared ranges and fit below the field modulus.

These keys identify logical records. The memory module can map them to one physical arena or four without changing CORE. That mapping is authenticated by Area 2.

The CELL or PROG provider checks the packed key, namespace and chunk bounds. The requester checks that the key comes from the intended reference or instruction and that any required index comparison holds. Record payload ranges belong to the record schema, not to BUS.

### 2.4 Functional identities

“Function of its key” needs a precise meaning.

| Provider | Inputs that determine the result |
|---|---|
| ALU | `(op, x, y)` determines both `flag` and `z` |
| CELL | `(arena, index, chunk)` determines `(w0,w1,w2)` within the authenticated leaf context |
| PROG | `(prog_root, table, index, chunk)` determines `(w0,w1,w2)` |
| HASH2 | `(x,y)` determines the output |
| DEC | Dispatch class determines the rule metadata |

For ALU and HASH, the bus `key` field alone is not the functional identity. The existing ALU already documents the stronger identity `(op,x,y)`. [alu.rs](/home/hoskinson/Projects/Moriarty/crates/mori-chip/src/alu.rs:1), lines 1–9.

A leaf has one authenticated program context. It must not import foreign program rows into the PROG namespace without a separate context-binding interface.

## 3. BUS module

### Purpose

BUS connects requests to constrained provider rows through a common tuple schema. It supplies membership, while each provider supplies the relation and uniqueness properties that make membership sufficient.

### Interface

BUS accepts the tuples in §2.3.

A request contains an active bit and five assigned cells. Its caller constrains the chip identifier, packed key and operands. BUS constrains an inactive request to the all-zero tuple.

A provider contains a fixed row role, an active bit where required, and five assigned cells. Its owning module constrains the chip identifier and the full relation on every active provider row.

BUS exports request and provider cell handles for copy connections. It exports no semantic result without a provider contract.

### Guarantees

1. Every active request matches an active provider tuple.
2. Request rows cannot serve themselves or other requests.
3. The table side is gated by the provider row selector.
4. Only the zero tuple has chip identifier zero.
5. Each active provider row belongs to a registered provider implementation.
6. Every request lane is checked.
7. Repeating a read request is permitted.

These preserve D1, D4 and D8. [DESIGN:94–101]

### Assumptions

Each provider proves its relation even when no request uses that row. CELL providers additionally prove consistent records and exclusive ownership between old memory, the carried tail and current-leaf appends.

Set membership does **not** prove that an append happened exactly once. CORE and MEM therefore share an ordered append-event port:

```text
AppendEvent {
    active: Bit,
    slot: UInt<slot_bits>,
    ordinal: UInt<event_bits>,
    arena: Tag<MemoryArena>,
    index: UInt<64>,
    record: canonical record lanes
}
```

CORE constrains every event slot, including inactive events. MEM copies the complete fixed-capacity event array and derives its new records from that array. MEM cannot add an unrequested append, omit an event or reinterpret a read as a write.

Cache rows may repeat an authenticated immutable record. Duplicate cache entries must not introduce different payloads for the same identity.

### Selector packing

BUS owns the shared column allocator and row-role schedule. Individual modules request row roles without allocating independent copies of the 12-column set.

A packed fixed column uses codes `0..=n`. Code zero disables every role. For role `c`, the selector is the Lagrange indicator:

```text
L_c(q) = product over j != c of (q - j)/(c - j)
```

Every packed gate satisfies:

```text
selector_degree + body_degree <= 6
```

This is the existing `vsel` construction. [vsel.rs](/home/hoskinson/Projects/Moriarty/crates/mori-chip/src/vsel.rs:1), lines 1–14 and 65–119.

The allocator must also check row disjointness. Degree-compatible gates cannot share different codes on the same row. The current `pack` function accepts degrees, so its API alone cannot establish that property.

Round gates with body degree five get a degree-one selector. First, chain and tail absorb gates can share a code column only where the full composed degree permits it.

### Row budget

**ESTIMATE:**

- A pair of requests costs one request row unless it occupies an existing CORE row.
- A provider tuple costs zero additional rows when it occupies the provider’s checked operation row, otherwise one.
- More requests increase rows, not lookup arguments.
- A third lookup argument would add roughly 22,600 node rows under the reported fit.
- A new filled selector column costs roughly 6,460 node rows in the reported selector experiment.

The 12 shared advice columns are a design target. The complete key must be measured. [DESIGN:125–134; COST:125–136]

### Verification

Refine the bus portion of `models/sherlock/quint/chipLeaf.qnt`. Add explicit provider roles, chunked keys, exclusive memory ownership and copied append events.

Positive MockProver cases:

- Repeated requests served by one valid provider.
- Both request lanes active.
- Disabled lanes and provider padding.
- Repeated authenticated cache entries.
- Each registered chip using the same numeric key without cross-chip aliasing.

Named negative mutants:

- `bus_provider_gate_removed`
- `bus_request_serves_itself`
- `bus_second_lane_unchecked`
- `bus_chip_id_unpinned`
- `bus_zero_chip_payload`
- `bus_key_namespace_alias`
- `bus_append_event_omitted`
- `bus_unrequested_append`
- `vsel_overlapping_roles`
- `vsel_degree_overflow`

Selector schedule and degree errors are configuration failures. Advice tampering must fail constraints.

### Code skeleton

Owned files: `bus.rs`, `vsel.rs`, new `ports.rs`, new `layout.rs`.

```rust
pub struct BusTuple<T> {
    pub chip: T,
    pub key: T,
    pub a: T,
    pub b: T,
    pub c: T,
}

pub enum ChipId {
    Alu = 1,
    Cell = 2,
    Hash2 = 3,
    Prog = 4,
}

pub struct ProviderSpec {
    pub chip: ChipId,
    pub role: RowRole,
}

pub trait BusPort {
    /// Constrain membership and the all-zero inactive encoding.
    fn request(
        &self,
        region: &mut Region<'_, F>,
        row: usize,
        lane: RequestLane,
        active: &BitCell,
        tuple: &BusTuple<Cell>,
    ) -> Result<(), Error>;
}

impl BusConfig {
    /// Allocate two request arguments over one registered provider table.
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        columns: SharedColumns,
        schedule: &LayoutManifest,
    ) -> Result<Self, ConfigError>;

    /// Expose only rows assigned to this provider's fixed role.
    pub fn provider(
        &self,
        spec: ProviderSpec,
    ) -> Result<ProviderPort, ConfigError>;
}

impl SelectorPlan {
    /// Check role disjointness and composed degree before assigning codes.
    pub fn freeze(
        roles: &[RoleSpec],
        max_degree: usize,
    ) -> Result<Self, ConfigError>;
}
```

The BUS agent reads this contract, `bus.rs`, `vsel.rs`, D1–D8 and the lookup cost evidence. It must not need CORE rule bodies, memory tree traversal code or recursion internals.

## 4. CORE module

### Purpose

CORE proves the selected reference transition for each enabled owner-step slot. It carries persistent registers, issues typed operations and exposes the exact boundary cells used by the commitment module.

### State interface

Use the complete record schemas, not `Regs32`.

| Named port | Contents and ranges | Ownership |
|---|---|---|
| `control` | Full control record and origin. Finite tags, bounded indices, canonical inactive lanes | CORE |
| `outcome` | Running, Halted, Rejected or Exhausted, with the appropriate payload | CORE |
| `fuel` | Initial fuel, remaining fuel and consumed work, each u128 | CORE through RANGE |
| `logical_counters` | Helper work, visits, cursor and allocation counters, using the reference widths and limits | CORE through RANGE |
| `stack_limit` | u64 policy value authenticated by `input_d` | Init, then CORE |
| `frame_count`, `task_count` | u64 counts, with non-wrapping updates | CORE through RANGE |
| `environment` | Presence bit and typed ENV reference | CORE and MEM |
| `top_frame` | Complete 13-lane canonical frame | CORE |
| `top_task` | Complete 7-lane canonical task | CORE |
| `helper` | Complete 23-lane canonical helper | CORE |
| `result` | Complete 4-lane canonical result | CORE |
| `frame_below`, `task_below` | Field-valued stack commitments | CORE and HASH |
| `memory_lengths` | Four logical arena lengths from the MEM interface | MEM, copied into CORE |
| `program_context` | `prog_d`, `prog_root`, entry metadata and authenticated table lengths | Program admission |
| `input_context` | `input_d`, value opening and resource policy | Init/input admission |
| `dependencies` | Dependency commitment carried through the run | Area 3 dependency interface |
| `scratch` | Slot-local typed temporaries | CORE |

The complete record widths and Init fields come from DESIGN:220–267. Their expansion into lanes must be frozen in `state_schema.rs`. The 39 Init fields include records, so they are not 39 scalar lanes.

Memory roots, tail records and backend-specific evidence are exported by MEM directly to the boundary composer. CORE does not inspect them. CORE consumes logical records and lengths.

All persistent cells affect future execution and enter `BoundaryCells`. Scratch is excluded only when it cannot affect a later slot before a constrained write. Reset scratch at every slot entry. This also gives every leaf a canonical scratch entry.

### Requests and provided cells

CORE requests:

- ALU tuples for arithmetic and integer comparisons.
- CELL tuples for authenticated data, field, environment and slot records.
- PROG tuples for instruction and metadata reads.
- HASH2 tuples where the required operation is exactly a two-input hash.
- Named HASH ports for stack hashing.
- Named RANGE ports for newly introduced or updated integer cells.
- Named DEC ports for dispatch.
- A named VALUE DIGEST port when a rule constructs or publishes a value.

CORE provides:

- `SlotInput` and `SlotOutput`.
- `DispatchInputs`.
- Ordered `AppendEvent` cells.
- `StackOperation` cells.
- Per-slot `enable`, rule lane and publication bit.
- `steps` and the ordered rule lanes used to derive `rules_d`.
- Its portion of `BoundaryCells`.
- The authenticated terminal tag and payload for outcome encoding.

A reference used to compute `vd` carries its semantic digest through the VALUE DIGEST interface. A physical address must not become a hash input to the canonical value encoding. [C6:104–124]

### Slot and transition invariants

For each slot:

1. `enable` is Boolean.
2. Enables form a prefix.
3. Every operation of the slot uses the same enable.
4. DEC selects exactly one supported rule when enabled.
5. Every register in the rule’s write set has its specified next value.
6. Every other persistent register is unchanged.
7. Slot post-state cells are copied to the next slot’s pre-state cells.
8. A disabled slot is the identity on persistent state and contributes zero steps.
9. A publishing step is the last enabled step.
10. `steps = Σ enable`, with `1 <= steps <= S` for an execution leaf.

A micro-operation is a circuit scheduling unit. It does not increment the reference step count. One enabled owner-step slot increments that count once.

CORE implements each rule’s exact charge behavior. It must not copy the spike’s “subtract one fuel on every enabled step” rule into the full machine. The scalar fragment, for example, distinguishes an uncharged HelperWork refusal from its charged arithmetic branch. [rf.rs:10–11; c6_scalar_arithmetic_step.rs](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_scalar_arithmetic_step.rs:45), lines 45–53.

Leaf closure changes no logical counter, fuel, outcome or logical length. Physical capacity failure causes proof scheduling to stop before an enabled step. It does not produce `Rejected` or `Exhausted`. [REQ:18–19]

### Memory access and append ordering

A read uses a reference from constrained state or a prior constrained micro-operation. The address is never selected by a free provider payload.

For an ordinary pre-state read, prove:

```text
0 <= index < arena_len_pre
```

If a reference rule permits a later micro-operation to read a record appended earlier in the same owner step, use the constrained intermediate length after that append. Do not apply a blanket “all reads use final slot length” rule.

Each append event uses the current logical length as its index and advances that length by one. The order of multiple appends follows the reference rule. Closing a leaf does not advance any length.

This ordering is also the causality premise required by a write-once LogUp backend. [CACHE:84–89]

### Cached frame and task stacks

Use cached tops with Poseidon commitments for the records below them. This follows the run-length studies’ stack direction and removes stack spills from random-access memory. [RUN-B:28–35; run-length lane A round 2](/home/hoskinson/Projects/Moriarty/research/sherlock-runlength-consensus-20261111/lanes/A/round2.md:19), lines 19–21.

For frames, define:

```text
spill_frame(depth, below, frame) =
    H([DOM_FRAME_V1, depth, below, frame[0], ..., frame[12]])
```

For tasks, use a distinct domain and the seven task lanes.

Protocol:

- At count zero, the cached record is zero and `below` equals its fixed empty-stack constant.
- Pushing onto an empty stack installs the new top without spilling a sentinel.
- Pushing onto a nonempty stack hashes the previous top and previous `below`, then installs the new top.
- Popping count one clears the top and preserves the empty-stack invariant.
- Popping a larger stack opens the current `below` commitment to the next top, predecessor commitment and count.
- Rewriting a nonempty top changes only the top. It performs no spill.
- Pop and rewrite at count zero follow the reference refusal behavior.

The count used in a pop opening equals the depth at which that record was spilled. Hash domains separate frames and tasks. Every recovered integer lane receives its canonical record checks.

### Layout and budget

Use narrow pair rows with copied operands and copied flags. A constraint without a valid location at rotations `-1`, `0` or `+1` is a configuration error. [rfcore.rs](/home/hoskinson/Projects/Moriarty/crates/mori-chip/src/rfcore.rs:5), lines 5–22.

**ESTIMATE:**

```text
core_rows_per_slot =
    2 * ceil(register_word_count / 6) + 1
    + additional_operand_routing
    + additional_request_rows
    + rule_local_rows
```

The first term extrapolates the measured 13-row CORE for 32 words. It is not a measured formula for the complete state. A 128–256-word state gives 45–87 base pair rows before routing and rule work. [COST:73–105]

Stack hashes belong to HASH’s budget:

- Frame spill/open: 16 inputs, eight permutations, approximately 560 rows.
- Task spill/open: ten inputs, five permutations, approximately 350 rows.
- Cached-top rewrite: zero stack-hash rows.

These are **ESTIMATE**, using 70 rows per permutation. They exclude record checks.

Each rule must publish its maximum requests, appends, range checks and hashes. The minimum leaf capacities must fit the largest single supported owner step. An early-close policy cannot repair a step that cannot fit an empty leaf.

### Verification

Create `models/sherlock/quint/processorCore.qnt`. Reuse slot obligations from `chipLeaf.qnt` and stack obligations from `chipLeafCached.qnt`, but replace the stand-in ISA with rule-family transition contracts.

The refinement reference must not allocate at leaf boundaries. Prove:

- Each enabled slot projects to one owner transition.
- Closure projects to identity.
- Partition changes preserve the logical trace, resource accounting and terminal outcome.
- A stack opening recovers exactly the previous logical top.
- Every append event corresponds to one reference allocation.

Positive MockProver cases:

- One instance of every admitted rule and every branch of its charge behavior.
- Full and partial leaves.
- Zero-allocation loops.
- Nested frame/task push, rewrite and pop sequences.
- Halted values, rejection and exhaustion.
- The same trace under full, one-step and mixed leaf partitions.
- A rule crossing two arena flush boundaries, through the MEM interface.

Named negative mutants:

- `core_selector_forged`
- `core_enable_split_across_microops`
- `core_enable_nonprefix`
- `core_post_pre_copy_removed`
- `core_unlisted_register_changed`
- `core_disabled_state_changed`
- `core_terminal_followed_by_step`
- `core_steps_counts_microops`
- `core_close_charges_fuel`
- `core_close_advances_length`
- `core_future_read`
- `core_append_address_free`
- `core_append_order_reversed`
- `core_scratch_read_before_write`
- `core_empty_stack_nonzero`
- `core_stack_predecessor_forged`
- `core_stack_count_unchecked`
- `core_value_digest_host_supplied`

### Code skeleton

Owned files: refactor `rf.rs` and `rfcore.rs`; add `core/mod.rs`, `core/state_schema.rs`, `core/slots.rs`, `core/stacks.rs` and rule-family files under `core/rules/`.

```rust
pub struct CorePorts {
    pub pre: CoreStateCells,
    pub post: CoreStateCells,
    pub steps: UIntCell<64>,
    pub rule_lanes: Vec<Cell>,
    pub append_events: Vec<AppendEventCells>,
    pub boundary: CoreBoundaryCells,
}

pub struct RuleContract {
    pub rule: RuleIdC,
    pub reads: &'static [RegisterId],
    pub writes: &'static [RegisterId],
    pub effects: EffectBounds,
}

pub trait RuleEmitter {
    /// Constrain one reference rule using only declared ports and write lanes.
    fn constrain(
        &self,
        ctx: &mut RuleContext<'_>,
        selected: &BitCell,
        pre: &CoreStateCells,
        post: &CoreStateCells,
    ) -> Result<(), Error>;
}

impl CoreConfig {
    /// Configure fixed slot geometry and the complete admitted inventory.
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        kit: &ProcessorKit,
        inventory: &RuleInventory,
    ) -> Result<Self, ConfigError>;

    /// Assign and constrain linked slots, all effects and boundary copies.
    pub fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        witness: Value<CoreWitness>,
        ports: &ProcessorServices,
    ) -> Result<CorePorts, Error>;
}

/// Constrain the complete write set and equality of all remaining registers.
pub fn constrain_frame_condition(
    ctx: &mut RuleContext<'_>,
    contract: &RuleContract,
    selected: &BitCell,
    pre: &CoreStateCells,
    post: &CoreStateCells,
) -> Result<(), Error>;

/// Authenticate the cached-stack transition, including zero-count cases.
pub fn constrain_stack(
    ctx: &mut RuleContext<'_>,
    operation: &StackOperationCells,
    pre: &StackCells,
    post: &StackCells,
) -> Result<(), Error>;
```

The CORE framework agent reads the shared contracts, complete state schema, slot rules and pair-row implementation. A rule agent additionally reads its own reference family, existing fragment and tests. Neither needs Merkle walks, LogUp equations, boundary hashing internals or node verification code.

## 5. DEC module

### Purpose

DEC derives the selected rule from authenticated pre-state facts in reference priority order. It returns fixed rule metadata that CORE uses to select a micro-operation plan and write set.

### Interface

DEC uses named copied ports:

```text
DispatchInputs:
    enable
    canonical pre-control, outcome, helper, result, top-frame and top-task
    authenticated instruction and metadata cells
    authenticated value observations needed by guards
    range-checked counters and comparator results

DecodeOutput:
    class
    rule_ordinal
    microplan_id
    write_set_id
    flags
    semantic_rule_lane
    selected[]
```

All guard observations are linked to their actual source cells. A supplied Boolean flag is not a fact until its producer proves the corresponding predicate.

Use a finite priority network to derive the winning class. Initially assign one class to each rule in the frozen inventory:

```text
class = sum(selected[r] * (ordinal(r) + 1))
sum(selected) = enable
```

Class zero is disabled. Active classes and rule ordinals fit u16. The ordered inventory is explicit and versioned. Do not derive it from unstable enum discriminants.

This retains the complete priority logic while making the ROM small. Compression of a large Cartesian product of tags and guard bits into a new class encoding is not required for the first implementation.

The ROM stores two fixed columns:

```text
(class, packed_output)

control_word =
    microplan_id
    + 2^16 * write_set_id
    + 2^32 * flags
    + 2^40 * control_version

packed_output =
    rule_ordinal
    + 2^16 * control_word
    + 2^64 * semantic_rule_lane
```

Ranges:

- `microplan_id`, `write_set_id`, `rule_ordinal`: u16.
- `flags`, `control_version`: u8.
- `control_word`: 48 bits.
- `semantic_rule_lane`: 128 bits.
- `packed_output`: 192 bits.

DEC constrains the unpacking and component ranges. The disabled row has all-zero output. The semantic lane follows the rule-lane convention used for `rules_d`, including nonzero and collision checks when the inventory freezes. [DESIGN:42]

Flags are fixed metadata, not free permissions. Define bit zero as publication and reserve the remaining bits as zero until a rule contract assigns them.

### Guarantees and assumptions

DEC guarantees:

1. P0, P1, P2 and `owner_select` precedence.
2. Exactly one supported selected rule when enabled.
3. No selected rule when disabled.
4. A matching fixed microplan, write set and semantic rule lane.
5. Unsupported dispatch paths make an enabled slot unsatisfiable.
6. Exactly one ROM output for each active class.

The dispatch order and data dependencies are documented in [c6_dispatch_step.rs](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:4), lines 4–38.

DEC assumes canonical record ports and authenticated program/value reads. It must expose the list of those assumptions per guard. Program admission is not established merely because DEC found a ROM row.

### Row budget

**ESTIMATE:**

- ROM: one row per dispatch class plus the disabled row.
- Lookup arguments: one, as in the spike.
- Fixed ROM columns: two, by packing metadata into one field.
- Lookup can occupy an existing CORE dispatch row.
- Fact extraction and priority evaluation require additional serial rows determined by the guard inventory.

The six-row spike ROM is not a cost model for all real guard predicates. [dec.rs:1–5; rf.rs:87–88]

### Verification

Create `models/sherlock/quint/processorDecode.qnt` with separate facts and selection contracts. Enumerate abstract guard combinations by dispatch group and compare against reference priority. This follows the skeleton study’s proposed split. [SKELETON testing:31–41]

Positive MockProver cases:

- Every admitted dispatch class.
- P1 publication coinciding with zero remaining fuel.
- Helper-present versus result-present cases.
- Every comparator boundary.
- Disabled slots.
- Unknown and unsupported classes refused.

Named negative mutants:

- `dec_priority_swapped`
- `dec_guard_unlinked`
- `dec_wrong_instruction_source`
- `dec_wrong_value_reference`
- `dec_two_rules_selected`
- `dec_enabled_without_rule`
- `dec_metadata_forged`
- `dec_write_set_swapped`
- `dec_rule_lane_zero`
- `dec_unknown_class_accepted`

### Code skeleton

Owned files: `dec.rs`, new `dec/facts.rs`, `dec/priority.rs`, `dec/rom.rs`.

```rust
pub struct DecodeCells {
    pub class: UIntCell<16>,
    pub rule_ordinal: UIntCell<16>,
    pub microplan_id: UIntCell<16>,
    pub write_set_id: UIntCell<16>,
    pub flags: UIntCell<8>,
    pub semantic_rule_lane: UIntCell<128>,
    pub selected: Vec<BitCell>,
}

impl DecConfig {
    /// Configure one fixed ROM and priority logic from the frozen inventory.
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        kit: &ProcessorKit,
        inventory: &RuleInventory,
    ) -> Result<Self, ConfigError>;

    /// Derive every guard from named authenticated inputs.
    pub fn constrain_facts(
        &self,
        ctx: &mut RegionContext<'_>,
        inputs: &DispatchInputs,
    ) -> Result<DispatchFacts, Error>;

    /// Enforce reference precedence and return constrained rule metadata.
    pub fn decode(
        &self,
        ctx: &mut RegionContext<'_>,
        facts: &DispatchFacts,
    ) -> Result<DecodeCells, Error>;
}

/// Reject duplicate classes, zero/colliding rule lanes and missing plans.
pub fn validate_rom(
    inventory: &RuleInventory,
    rows: &[DecodeRow],
) -> Result<(), ConfigError>;
```

The DEC agent reads dispatch reference code, the facts schema, the admitted inventory and its local tests. It must not need CORE’s register layout, memory authentication code or arithmetic implementation.

## 6. ALU + RANGE module

### Purpose

ALU proves exact integer operations and their overflow results. RANGE proves the integer bounds that distinguish those operations from field arithmetic.

### ALU tuple contract

Preserve the existing encoding:

```text
(1, op + 8*flag, x, y, z)
```

| `op` | Operation | Result and flag |
|---:|---|---|
| 1 | ADD128 | `x+y = z + flag·2^128`, flag is carry |
| 2 | LT128 | `z = [x<y]`, flag zero |
| 3 | SUB128 | `x-y = z - flag·2^128`, flag is borrow |
| 4 | MUL128 | `z` is the low 128 bits, flag iff the high half is nonzero |
| 5 | EQ128 | `z = [x=y]`, flag zero |
| 6 | NOT_BOOL | `z=1-x`, `x` Boolean, `y=0`, flag zero |

All other opcodes are rejected. Operations 1 and 2 exist in the spike. Operations 3–6 are proposed extensions.

For numeric operations, `x,y < 2^128`. Arithmetic results satisfy `z < 2^128`. Comparison and Boolean results are Boolean. ALU checks opcode and flag membership.

`EQ128` compares scalar integers. Structural value equality remains a reference helper process and must not be replaced with physical-address equality or an unqualified digest equality.

CORE converts arithmetic overflow to the reference staged error. ALU does not choose the machine outcome. The current scalar contract requires error code 10 for overflow and 11 for an invalid operand kind. [c6_scalar_arithmetic_step.rs:6–17]

### RANGE interface and ownership

RANGE accepts a copied source cell and a static bit width. It returns a range-checked handle to the same value.

For widths up to 128 bits, use 12-bit limbs and a running accumulator. If `m = ceil(B/12)` and `t = B - 12(m-1)`, constrain:

```text
limb[i] in [0,4096)
top_pad = top_limb * 2^(12-t)
top_pad in [0,4096)
source = reconstructed limbs
```

Use Boolean gates for bits. Every reconstruction connects to the original source cell.

The existing running mode takes 12 rows for a u128 and uses one lookup argument. [range.rs:12–19,92–127]

ALU may reuse a range proof on the same assigned operands. It may also rely on an authenticated, preserved state invariant explicitly recorded in its port contract. Standalone ALU tests must establish these premises themselves. A raw `Value<u128>` witness is insufficient.

Every provider row, including an unused row, satisfies its operation relation and required range premises.

### Arithmetic constraints

ADD and SUB use bounded equations below `2^129`.

LT uses the existing bounded difference relation:

```text
z Boolean
w = z*(y-x-1) + (1-z)*(x-y)
0 <= w < 2^128
```

Both operands must have valid ranges. [alu.rs:106–125]

MUL uses 64-bit limbs and bounded carries. A single equation `x*y=z` in the proof field is unsound for u128 multiplication because the integer product can exceed the field modulus. The existing reference fragment documents the required limb construction and exact overflow test. [c6_scalar_arithmetic_step.rs:31–43]

EQ uses both directions of equality: `z*(x-y)=0` and a constrained inverse witness for `1-z`. NOT requires a Boolean input.

### Row budget

**ESTIMATE:**

| Work | Rows |
|---|---:|
| RANGE table | 4,096 fixed-table rows |
| New u128 range check | 12 |
| New u64 range check | Approximately 7 |
| ADD/SUB/LT with already checked operands | Approximately 12–16 |
| ADD/SUB/LT including two fresh u128 operands | Approximately 36–40 |
| EQ/NOT with already checked operands | Approximately 1–4 |
| MUL including limb and carry checks | Reserve 64–128 until measured |

These are operation-region estimates, not total domain sizes. Table and advice regions can overlap in the domain. The spike’s 12-row running-range ALU cost excludes fresh checks of both operands.

Keep one limb lookup. Do not introduce bitwise tables unless the admitted language needs them. The current `BinaryOp` inventory is Add, Sub, Mul, Eq, Lt, Cons and Branch. [shared.rs](/home/hoskinson/Projects/Moriarty/crates/mori-core/src/sherlock/helper_machine/shared.rs:44), lines 44–57.

### Verification

Create `models/sherlock/quint/processorAlu.qnt` over a reduced bit width. Check exact integer results, carries, borrows and overflow. Integer Quint checks do not replace field-level MockProver negatives.

Positive cases:

- Zero, one, maximum values and carry boundaries.
- Equal operands and adjacent comparison operands.
- Subtraction underflow.
- Products with only cross-limb contributions.
- Products whose low half is zero and high half is nonzero.
- Both Boolean inputs.
- Reused checked operand cells.

Named negative mutants:

- `alu_operand_range_missing`
- `alu_output_range_missing`
- `alu_carry_forged`
- `alu_borrow_forged`
- `alu_lt_boolean_removed`
- `alu_mul_field_residue`
- `alu_mul_high_half_dropped`
- `alu_eq_reverse_implication_removed`
- `alu_not_nonboolean`
- `range_top_pad_unlinked`
- `range_source_copy_removed`
- `range_limb_out_of_table`
- `range_running_sum_broken`

### Code skeleton

Owned files: `alu.rs`, `range.rs`, optional `alu/mul.rs`.

```rust
pub enum AluOpcode {
    Add128 = 1,
    Lt128 = 2,
    Sub128 = 3,
    Mul128 = 4,
    Eq128 = 5,
    NotBool = 6,
}

pub struct AluResultCells {
    pub value: Cell,
    pub flag: BitCell,
}

impl RangeConfig {
    /// Use one limb lookup and constrain the source cell to its decomposition.
    pub fn check<const B: usize>(
        &self,
        ctx: &mut RegionContext<'_>,
        source: &Cell,
    ) -> Result<UIntCell<B>, Error>;
}

impl AluConfig {
    /// Constrain a provider operation with authenticated operand ranges.
    pub fn provide(
        &self,
        ctx: &mut RegionContext<'_>,
        op: AluOpcode,
        x: &UIntCell<128>,
        y: &UIntCell<128>,
        witness: Value<AluWitness>,
    ) -> Result<AluResultCells, Error>;

    /// Constrain the full product through bounded limbs and carries.
    pub fn multiply(
        &self,
        ctx: &mut RegionContext<'_>,
        x: &UIntCell<128>,
        y: &UIntCell<128>,
        witness: Value<MulWitness>,
    ) -> Result<AluResultCells, Error>;
}
```

The implementing agent reads `alu.rs`, `range.rs`, the scalar arithmetic fragment’s contract and the bus schema. It must not need dispatch, stack or memory internals.

## 7. HASH module

### Purpose

HASH implements the pinned Poseidon permutation and fixed-length sponge. It returns digests connected to their exact input cells, while callers own semantic domains and preimage schemas.

### Interface

Two-input bus service:

```text
(3, 2, x, y, H([x,y]))
```

`x`, `y` and the digest are field elements. HASH pins chip identifier 3 and key 2.

All other hashes use a named copied port:

```text
HashRequest {
    schema: fixed HashSchemaId,
    inputs: ordered assigned cells
}

HashResult {
    digest: assigned cell
}
```

The schema fixes the input count, domain constants and positions of variable inputs. The caller supplies constrained value cells. HASH copies every input and returns the constrained output.

Use the pinned sponge exactly: width three, rate two, the pinned round constants and MDS matrix, and input length in the initial capacity. [poseidon.rs](/home/hoskinson/Projects/Moriarty/crates/mori-chip/src/poseidon.rs:1), lines 1–32.

Do not replace this with a capacity-domain variant or a chain of `H2` operations where an interface specifies a fixed-length sponge. Such changes alter `sherlock-value/1`.

The current circuit method refuses input lengths below two. Its native helper accepts nonempty inputs. Keep the circuit port at `N>=2` until a one-input implementation has its own checks. [poseidon.rs:96–99,268–286]

### Guarantees and assumptions

HASH guarantees:

1. Correct initial state and exact input length.
2. Correct absorption order.
3. Every required round with the pinned constants.
4. Correct chain and odd-tail absorption.
5. Provider output copied from the final state.
6. Provider chip and key pinned.
7. No exposed internal state accepted as a digest.

The caller guarantees:

- Correct domain and encoding version.
- Canonical integers, tags and record structure.
- Authentication of any child digest.
- Exact semantic preimage.

For `sherlock-value/1`, the VALUE DIGEST module owns those guarantees. HASH cannot determine whether a supplied field is a child value digest or an address. [C6:104–124]

Within a leaf, repeated HASH2 requests can share one valid provider. Arbitrary-length hash sharing requires equality of schema and every input cell. Run-wide hash caching remains an optional authenticated cache service in Area 2.

### Row budget

**ESTIMATE:**

```text
hash_rows(N) = 70 * ceil(N/2), for N >= 2
```

This follows the existing Poseidon-only layout’s 70 rows per permutation. [poseidon.rs:17–19,55–59]

| Preimage | Rows |
|---|---:|
| Two fields | 70 |
| Four fields | 140 |
| Five or six fields | 210 |
| Seven or eight fields | 280 |
| Sixteen fields | 560 |

Input copies can occupy absorb rows. Callers count their requests, so module budgets must not count the same permutation twice.

Keep the Poseidon-only layout. The earlier stdlib composition added enough fixed-column cost to exceed the node gate in that experiment. [COST:28–33,190–198]

### Verification

Add `models/sherlock/quint/processorHashProtocol.qnt` for call framing, chunk order, domain separation and copy connections. Treat the cryptographic function as an injective abstraction only within the model. State that premise explicitly.

Positive MockProver cases:

- Differential vectors against the pinned stdlib for supported even and odd lengths.
- Zero inputs, repeated inputs and maximum field values.
- Multiple calls in the same region.
- HASH2 sharing.
- Frame and task schemas.
- Canonical VALUE DIGEST schemas supplied by Area 2.

Named negative mutants:

- `hash_state_tampered`
- `hash_round_constant_changed`
- `hash_round_skipped`
- `hash_absorb_skipped`
- `hash_length_changed`
- `hash_odd_tail_absorbs_padding`
- `hash_chip_id_forged`
- `hash_key_forged`
- `hash_output_copy_removed`
- `hash_input_copy_removed`
- `hash_schema_domain_swapped`

### Code skeleton

Owned file: `poseidon.rs`. Keep `hashchip.rs` as a calibration/reference implementation until migration ends.

```rust
pub struct HashSchema {
    pub id: HashSchemaId,
    pub inputs: &'static [HashInputKind],
}

pub struct HashResultCell {
    pub digest: Cell,
}

impl PoseidonConfig {
    /// Configure the pinned permutation with the shared selector plan.
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        kit: &ProcessorKit,
    ) -> Result<Self, ConfigError>;

    /// Hash the schema's exact ordered preimage and copy every variable input.
    pub fn hash(
        &self,
        ctx: &mut RegionContext<'_>,
        schema: &'static HashSchema,
        inputs: &[Cell],
    ) -> Result<HashResultCell, Error>;

    /// Publish only the exact two-input sponge relation on HASH2.
    pub fn provide_hash2(
        &self,
        ctx: &mut RegionContext<'_>,
        x: &Cell,
        y: &Cell,
    ) -> Result<HashResultCell, Error>;
}

/// Return a scheduling estimate without assigning cells.
pub fn rows_for_hash(input_count: usize) -> Result<usize, ConfigError>;
```

The HASH agent reads `poseidon.rs`, the pinned parameter definitions, the bus contract and differential vectors. It must not need value traversal, Merkle traversal, CORE rules or node code.

## 8. Integration obligations for Areas 2 and 3

These are processor dependencies, not implementations of the other areas.

| Interface | Required guarantee |
|---|---|
| MEM → CORE | Canonical records, range-checked logical indices and lengths, authenticated reads, exact copied append events |
| MEMORY BACKEND → MEM | Every supplied old record belongs to the committed memory context, or carries an explicit deferred obligation checked before run acceptance |
| VALUE DIGEST → CORE | Created values bind canonical `vd` to authenticated children and stored records |
| Init → CORE | Complete canonical state, range invariants, authenticated program/input/resource policy |
| CORE/MEM → BOUNDARY | Ordered copies of every persistent cell, including tails and stack commitments |
| BOUNDARY → NODE | Boundary equality authenticates the same state vector |
| NODE → accepted run | All deferred backend obligations and predecessor proofs are discharged |
| CONTINUATION → CORE | Resumed state has authenticated provenance and restored range/canonicality invariants |

A Horner boundary implementation requires the state vectors to be committed before its challenge. CORE supplies the same cells to either boundary implementation. It must not receive that challenge as a machine input.

The lookup-cache study describes a single-phase verifier and a two-pass committed-instance route. That is an Area 2/3 proof-system requirement, not a second phase to add to the processor. [CACHE:91–109,183–199]

The whole-leaf differential harness must run on every module change. It compares decoded transitions and ordered observations against `sherlock-k-a/1` and the owner rules, then checks the final public statement across partitions. Local MockProver suites remain necessary because final-output agreement does not expose every missing constraint. [MOD:12; SKELETON:9–14]

## 9. Open decisions and recommendations

| Decision | Options | Recommendation |
|---|---|---|
| Number of bus lookup arguments | Preserve two request lanes; serialize one lane | Preserve the measured two-lane configuration initially. Measure serialization after complete rule request counts exist |
| Decoder class construction | Packed raw facts; priority-derived finite class | Use checked priority logic and a finite winning class. Avoid an unbounded Cartesian-product ROM |
| Physical memory arenas | One tagged arena; four arenas | Keep four logical namespaces at the processor interface. Area 2 may choose the physical representation after its refinement proof |
| Stack storage | Memory-linked spill records; hash stacks | Use cached tops with separate frame/task hash stacks |
| Range reuse | Check every use; reuse checked cells and preserved invariants | Reuse only with explicit cell provenance. Check every new external or resumed integer |
| ALU extensions | Only spike ADD/LT; full scalar operations | Add SUB, MUL and Boolean/scalar equality support required by the admitted rules |
| `S` | 16; 64; 128 | Keep 16 for this contract. Larger values require the SRC-014 amendment and measurements |
| One-step capacity | Census maximum; complete rule effect audit | Derive capacities from every admitted rule. Census data cannot establish completeness |
| Backend selection | Merkle; run-wide LogUp | CORE remains unchanged. Select in Area 2 after proof-system and continuation checks |
| Memory length endpoint | u64 length; endpoint-capable length encoding | Area 2 must distinguish addresses from lengths and specify the final endpoint. Do not silently wrap `2^64` to zero |
| Full GP-1 storage bound | Assume depth 61 suffices; prove allocation bound or continuation refinement | Require the proof. Multiple appends per step mean a 64-bit step bound alone does not establish a 64-bit storage bound |

No decision here narrows GP-1 through GP-4.

## 10. Conflicts and required specification edits

1. **Literal “one lookup” conflicts with the spike.** DESIGN:80 says one `lookup_any`. `bus.rs:50–61` creates two, and DEC/RANGE add two more. Record schema count and lookup-argument count separately.

2. **“Function of key” is ambiguous.** DESIGN:80 requires it. `alu.rs:3–9` makes the operation a function of `(op,x,y)`. Adopt the explicit identities in §2.4.

3. **D3 allocation conflicts with dense partition-independent execution.** DESIGN:96 and `m3.rs:20–32` close with `len_post=base+8`. The run-length studies replace this with real-append advancement and carried tails. [RUN-B:48–54; run-length lane D round 2:11–16] The old walker cannot be reused unchanged.

4. **The studies do not settle the arena count.** RUN-B:28–46 recommends four arenas and two possible flushes. RUN-E:21–32 recommends one arena and one flush after a different allocation analysis. Preserve this disagreement. It does not block a logical namespace interface.

5. **D7’s memory links differ from the proposed hash stacks.** DESIGN:100 stores frame links at memory addresses. The Init description and later studies use stack commitments. Replace the stored-link protocol explicitly and prove its reference projection.

6. **The spike state is incomplete.** `rf.rs:5–8` uses two-word frame/task records. DESIGN:241–244 specifies 13, 7, 23 and 4 lanes for frame, task, helper and result records. The 32-word cost must not be presented as the full processor cost.

7. **The admitted inventory is unresolved for the final universal target.** REQ:12 and `c6_dispatch_step.rs:30–32` exclude rule families that later studies analyze. GP:9–16 requires every program of the language. Freeze the complete intended inventory and close each rule’s implementation and evidence gap. Do not turn a temporary unsupported rule into a permanent language restriction.

8. **Scratch cannot cross boundaries without a contract.** `rf.rs:7–8,34` omits scratch from the digest. This draft resets scratch per slot and forbids read-before-write. Any persistent scratch must instead enter the state schema.

9. **The existing ALU relies on operand ranges.** `alu.rs:8–9` states that premise. Arbitrary continuation entry does not establish it. Init and resume interfaces must preserve or recheck it.

10. **HASH’s one-input support differs between native and circuit helpers.** `poseidon.rs:97–99` accepts nonempty native inputs, while `poseidon.rs:283–286` rejects circuit inputs below length two. Do not advertise circuit parity at length one without a new implementation.

11. **The run-length study proposes weakening GP-1.** RUN-B:125–149 proposes “physically realizable” runs and memory guards in replacement requirements. The architecture brief forbids narrowing GP-1. This draft retains the existing target and records storage completeness as an obligation.

12. **The publication and leaf schemas need reconciliation.** DESIGN:13 uses the seven-lane leaf statement. The GP architecture proposes an eight-lane statement with `input_d` and `deps_d` at `architecture.md:97`. CORE exports semantic ports and does not hard-code public lane positions. Area 3 must freeze the adapter.

13. **The GP architecture’s run claim predates the C6 correction.** `research/sherlock-gp-architecture-20261110/architecture.md:99` includes `n_steps` in `out_d`. C6:89 and `dissent.md:209–212` exclude it from the consumed claim and replay key. The processor still proves the count, but it must not alter the canonical value or claim digest.

14. **Profile `/1` and exhaust kind seven conflict.** REQ:17 excludes profile `/2`. `dissent.md:226` records the unresolved `LogicalCap` encoding conflict. DEC must enforce the active profile rather than admit every representable outcome tag.

## 11. Implementation kit and acceptance boundary

Each module kit contains its owned files, frozen port types, invariants, row formula, small model, positive vectors and named mutants. Each CORE rule kit additionally contains its exact read set, write set, charge behavior and maximum operation counts.

Build the shared types and row-role allocator first. Then implement BUS, RANGE, HASH and the DEC ROM independently against those contracts. Assemble CORE’s slot framework before adding rule families. Complete dispatch facts and rule transitions together through their named ports.

Skeletons describe obligations. They do not establish soundness. The skeleton study explicitly identifies disconnected assigned cells as a composition failure. [SKELETON:12–14]

The final processor acceptance record must include:

- Every admitted rule connected to its dispatch and transition constraints.
- Every persistent lane covered by a boundary copy.
- Every provider’s range, relation and authentication obligations.
- One-step capacity for every admitted rule.
- Whole-leaf differential and partition tests.
- Named negative-mutant results.
- Configured and keyed rotation checks.
- Complete leaf, node and decider measurements.

## 12. What was not checked

No files were written. No builds, tests, models, key generation or proving were run.

This draft does not establish the complete rule inventory, the final expanded state-lane count, the maximum per-rule memory requests or the final node cost. It does not independently verify the external literature cited by the studies, the pinned verifier’s implementation, the proposed committed-instance LogUp transcript or ledger acceptance.

The run-length `CONSENSUS.md` was absent when checked. The five round-two positions contain unresolved differences, including arena count and tail representation. Those differences are retained here rather than treated as a settled design.

The signatures are interface sketches. They have not been compiled.