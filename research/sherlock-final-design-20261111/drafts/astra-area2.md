# Area 2: Memory and values

Run: `sherlock-final-design-20261111`

Status: architecture proposal. The interfaces and constraints below are proposed requirements, not claims about the current implementation. All new row budgets are **ESTIMATE**. No file was written. No build, test, model checker or prover was run.

## 1. Architecture decision

Use four dense append-only arenas: `DATA`, `FIELDS`, `ENV` and `SLOTS`. Give each arena an eight-record tail and advance its length only when a machine operation appends a record. Keep `FRAMES` and `TASKS` in cached-top Poseidon hash-stacks owned by CORE and HASH.

Implement MEMORY BACKEND behind one interface with two implementations:

- `DenseMerkle`: completed eight-record segments in a depth-61 tree, with a separately authenticated tail.
- `WriteOnceLogUp`: run-wide write-once memory checking, with records committed before the challenges are derived.

Implement BOUNDARY COMMITMENT behind one interface with two implementations:

- `PoseidonBoundary`: a digest of the complete boundary state.
- `HornerBoundary`: a polynomial fingerprint of boundary words committed before the fingerprint challenge is derived.

Start integration with `DenseMerkle + PoseidonBoundary`. Build the alternate implementations against the same module tests. Select the production profile only after measuring the complete leaf, node and decider.

This preserves the modularity requirement that CORE and MEM remain unchanged when either implementation changes. Swapping an implementation changes the keyed relation and its manifest. It does not imply that two different relations share a verifying key. [M:7–12]

Four arenas preserve the current reference machine’s separate indices and the Init contract’s separate roots and lengths. The run-length studies disagree on whether one arena should replace them. That optimization remains a separate layout change. [SPEC:249–266; RUN-A:23–31; RUN-B:28–46; RUN-E:148–158]

### Source notation

Paths below are relative to `/home/hoskinson/Projects/Moriarty`. A citation such as `SPEC:94–103` identifies that path and its line range.

| Name | Source |
|---|---|
| M | [research/sherlock-final-design-20261111/MODULARITY.md](/home/hoskinson/Projects/Moriarty/research/sherlock-final-design-20261111/MODULARITY.md:1) |
| SPEC | [openspec/changes/sherlock-vm-consensus-20261107/design.md](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/design.md:74) |
| REQ | [openspec/changes/sherlock-vm-consensus-20261107/requirements.md](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/requirements.md:5) |
| GP | [openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:1) |
| DISSENT | [openspec/changes/sherlock-vm-consensus-20261107/dissent.md](/home/hoskinson/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/dissent.md:194) |
| C6 | [research/sherlock-c6-outcome-ledger9-review-20261110/review.md](/home/hoskinson/Projects/Moriarty/research/sherlock-c6-outcome-ledger9-review-20261110/review.md:102) |
| LOOKUP | [research/sherlock-lookup-cache-20261111/lanes/B/round1.md](/home/hoskinson/Projects/Moriarty/research/sherlock-lookup-cache-20261111/lanes/B/round1.md:73) |
| RUN-A … RUN-E | `research/sherlock-runlength-consensus-20261111/lanes/{A,B,C,D,E}/round2.md` |
| COST | [research/sherlock-processor-analogy-20261110/experiments-2.md](/home/hoskinson/Projects/Moriarty/research/sherlock-processor-analogy-20261110/experiments-2.md:120) |
| GP-ARCH | [research/sherlock-gp-architecture-20261110/architecture.md](/home/hoskinson/Projects/Moriarty/research/sherlock-gp-architecture-20261110/architecture.md:67) |
| MC | `/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.3.0/src` |
| MP | `/home/hoskinson/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src` |

## 2. Shared interface contract

### 2.1 Bus encoding

Retain the spike’s `CHIP_MEM = 2`. Reserve chip zero exclusively for the inactive tuple. The current bus already uses five-field tuples and two request lanes. [crates/mori-chip/src/bus.rs:1–20,29–37,47–61]

Memory serves:

```text
(CHIP_MEM, key, a, b, c)

key = 8 * address + 2 * arena + lane

arena:
    0 = DATA
    1 = FIELDS
    2 = ENV
    3 = SLOTS

lane:
    0 or 1
```

`address` is a logical record index within an arena. It is not an index into individual bus lanes. Both lanes of a record have the same address.

The initial profile requires:

```text
0 <= address < 2^64
0 <= length  < 2^64
0 <= key     < 2^67
```

MEM constrains the decomposition and its ranges. These bounds are integer bounds, not field comparisons. Packing must be injective in the circuit field.

Each record has six field words, served as two triples. A short record has a constrained zero second lane.

| Arena | Lane 0 | Lane 1 |
|---|---|---|
| DATA | `(value_tag, scalar_or_type_id, ctor_id)` | `(fields_start, arity, vd)` |
| FIELDS | `(ref_kind, ref_index, child_vd)` | `(0, 0, 0)` |
| ENV | `(binder_id, ref_kind, ref_index)` | `(previous_present, previous_index, child_vd)` |
| SLOTS | `(ref_kind, ref_index, child_vd)` | `(0, 0, 0)` |

This is a proposed storage encoding. It extends the spike’s three-word cells so that a value digest is part of the authenticated record. The reference representation already separates data records, field slices, environment cells and value references. [crates/mori-chip/src/mem.rs:27; crates/mori-core/src/sherlock/helper_machine/raw.rs:690–695,739–768; crates/mori-core/src/sherlock/helper_machine/shared.rs:702–725]

For DATA:

- `value_tag` uses the `sherlock-value/1` tags `0..6`.
- `scalar_or_type_id` contains the UInt, Bool or program-local type-table index.
- `ctor_id` contains a program-local constructor-table index.
- Scalar and Nil records have zero field start and zero arity.
- Builtin composite records have their specified arity.
- Constructor records obtain their ordinal and program-independent type digest from authenticated declaration metadata.
- Every unused word is zero.

A program-local type or constructor index may occur in a physical record. It must never occur directly in the canonical value digest. [C6:104–119; REQ:47]

For references:

```text
ref_kind = 0: Const(ref_index)
ref_kind = 1: Data(ref_index)
```

Both forms permit index zero. Absence uses a separate presence bit. No arena reserves eight records merely to provide a null address.

### 2.2 Range ownership

| Field | Range or condition | Owner |
|---|---|---|
| Arena, lane, enable, presence bits | Enumerated values or Boolean | MEM |
| Record address, arena length, slice start, slice length | `u64`, with checked additions | MEM through RANGE |
| Type, constructor, binder and constant indices | `u64`, then authenticated table bounds | DEC/admission |
| DATA UInt payload | `< 2^128` | VALUE DIGEST through RANGE |
| DATA Bool payload, reference kind | Boolean | VALUE DIGEST or typed record admission |
| Value and memory digests | Native field elements | HASH/backend relation |
| Error code | `u8` plus machine validity rules | CORE and VALUE DIGEST |
| Exhaust kind | Encoding `0..7`, then profile-specific admissibility | CORE and VALUE DIGEST |
| LogUp multiplicity | Nonnegative bounded integer, initially `u128` | LogUp backend through RANGE |

Range checks occur when a value enters the proved state. Carried values may use the inductive invariant only after genesis or an authenticated import establishes it. The current ALU relies on bounded operands. Importing an arbitrary field element and treating it as an integer breaks that premise. [SPEC:86; RUN-E:105–107]

### 2.3 Named copied cells

Modules exchange assigned cells through these records. Constructors that certify authentication remain private to the module that establishes it.

```rust
pub type Cell = AssignedCell<F, F>;

pub enum Arena { Data, Fields, Env, Slots }

pub struct RecordCells {
    pub arena: Cell,
    pub address: Cell,
    pub words: [Cell; 6],
}

pub struct ArenaFrontierCells {
    pub len: Cell,
    pub segment: Cell,
    pub offset: Cell,
}

pub struct MemoryTokenCells {
    pub roots: [Cell; 4],
    pub tails: [Cell; 4],
}

pub struct MemoryBoundaryCells {
    pub frontiers: [ArenaFrontierCells; 4],
    pub token: MemoryTokenCells,
}

pub struct MemoryObligationCells {
    pub sums: [Cell; 2],
}

pub struct BoundaryCommitmentCells {
    pub value: Cell,
}

pub struct RunChallengeCells {
    pub gamma: [Cell; 2],
    pub alpha: [Cell; 2],
    pub beta: Cell,
}
```

`segment` and `offset` are derived:

```text
len = 8 * segment + offset
segment < 2^61
offset < 8
```

The Merkle implementation uses all eight token words. The LogUp implementation constrains them to zero. LogUp obligations travel separately to NODE INTEGRATION and the seal.

This keeps the MEM interface stable without requiring LogUp to compute unused Merkle roots or tail hashes.

HASH and RANGE receive named copied cells through their Area 1 ports. Area 2 introduces no additional hash or range bus encoding. A production hash input derived from state must use a copy-constrained cell. The existing `HIn::Cell` supplies that mechanism. [crates/mori-chip/src/poseidon.rs:128–143]

## 3. MEM: in-leaf window and tail

### Purpose

MEM implements dense append and bounded reads within a leaf. It exposes authenticated records through the common bus while delegating persistence to MEMORY BACKEND.

### Interface

MEM receives:

- `pre.frontiers[arena]`, copied from the authenticated boundary.
- Every enabled operation’s slot, operation position and current arena length, copied from CORE.
- Ordered `ReadRecord` and `AppendRecord` events.
- Up to seven imported tail records per arena.
- Authenticated cold-read records from MEMORY BACKEND.

MEM provides:

- Both bus lanes for each active window record.
- The exact appended-record list to MEMORY BACKEND.
- The pre-tail and post-tail cell lists to MEMORY BACKEND.
- Final arena lengths, copied to CORE’s post boundary.
- A list of cold-read requests whose addresses precede the initial tail.

An append is a copied event, not a conclusion inferred from lookup membership. Set inclusion does not prove that each provider record corresponds to one append.

For each arena:

```text
base       = 8 * floor(pre.len / 8)
tail_count = pre.len mod 8

window = imported_tail || new_records
post.len = pre.len + number_of_new_records
```

The initial candidate provisions eight total new records per leaf. Each arena has a fixed 15-record window, with inactive records zeroed. At most 28 imported tail records and eight new records are active across all arenas.

A read selects exactly one provider class:

```text
address < base:
    MEMORY BACKEND supplies the record

base <= address < current_len:
    MEM supplies the record
```

`current_len` belongs to the read’s position in the constrained operation sequence. If the reference rule reads only its pre-state, CORE passes the pre-step length. Same-step read-after-append requires an explicitly ordered micro-operation and a refinement check.

### Guarantees

1. Each append uses the current arena length as its address.
2. Each append increments that length by exactly one.
3. Append events and newly active records correspond one-to-one.
4. Every read satisfies `address < current_len`.
5. Both bus lanes come from the same record.
6. The old-memory and window provider domains are disjoint.
7. Imported tail records retain their values.
8. A leaf close changes no arena length, logical counter or value.
9. Padding produces only the zero bus tuple.
10. Disabled steps have no memory effects.

MEM constrains unused words and inactive records to zero. Each active provider pins its chip identifier. These retain D1, D2, D4, D5, D6 and D8 while replacing D3’s leaf-close allocation rule. [SPEC:94–103]

### Assumptions

- CORE supplies the actual constrained operation sequence.
- VALUE DIGEST or typed admission validates new records’ semantic fields.
- MEMORY BACKEND authenticates imported tails and cold records.
- RANGE proves the integer bounds.
- An enabled machine step fits the configured resources.

The last premise needs a worst-case rule census. Early close cannot repair a step that exceeds capacity in an otherwise empty leaf.

### Row budget

**ESTIMATE**, excluding HASH and shared RANGE work:

- 120 provider rows for four windows of 15 records, with two lanes per record.
- 200–600 rows for append provenance, routing, frontier arithmetic and tail selection.
- Initial module allowance: **320–720 rows**.

The spike’s eight-row segment region is not a full-record estimate. [crates/mori-chip/src/mem.rs:122–130; LOOKUP:19–30]

### Verification

Create `models/sherlock/quint/denseMemory.qnt`.

Model four finite sequences, ordered operations, explicit provider ownership and a `close` action. The reference contains no leaf allocation. Prove preservation of dense prefixes, read causality, immutable records and decoded-state identity on close.

Use `chipLeafCached.qnt` for bus and slot conventions. Replace its memory-backed stack reference when checking hash-stacks. Its existing reference deliberately models the cached spill layout. [models/sherlock/quint/chipLeafCached.qnt:17–24,35–48]

MockProver positives:

- Zero appends.
- Every initial tail occupancy `0..7`.
- Append sequences crossing an eight-record boundary.
- Repeated reads of one record.
- Reads of an imported tail and a newly appended record.
- Field slices spanning three leaves.
- One step appending to DATA and FIELDS together.
- A partially filled final leaf.

Named negative mutants:

```text
append_address_free
append_without_event
event_without_append
read_at_current_len
future_record_read
mixed_record_lanes
old_window_overlap
tail_rewritten
close_advances_len
close_changes_counter
padding_provider_active
frontier_wrap
```

### Code skeleton

Own:

```text
crates/mori-chip/src/memory/types.rs
crates/mori-chip/src/memory/window.rs
crates/mori-chip/tests/memory_window.rs
```

```rust
pub struct MemConfig { /* private columns and selectors */ }
pub struct MemWitness { /* raw events and record values */ }
pub struct MemAssignment { /* assigned window and event cells */ }

impl MemConfig {
    /// Ensures fixed-height windows and disjoint provider domains.
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        ports: &SharedPorts,
        limits: MemLimits,
    ) -> Self;

    /// Ensures all events refine dense append/read operations.
    pub fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        pre: &MemoryBoundaryCells,
        events: &CoreMemoryEventCells,
        witness: &MemWitness,
    ) -> Result<MemAssignment, Error>;

    /// Ensures post lengths equal pre lengths plus proved append counts.
    pub fn constrain_frontiers(
        &self,
        layouter: &mut impl Layouter<F>,
        assignment: &MemAssignment,
        post: &MemoryBoundaryCells,
    ) -> Result<(), Error>;
}
```

The implementing agent reads this contract, the shared ports, `mem.rs:94–240`, and the relevant reference append operations. It does not need to read Merkle walks, LogUp equations, recursive verifier internals or the decider.

## 4. MEMORY BACKEND

### Purpose

MEMORY BACKEND authenticates records that MEM cannot establish locally and carries their persistence across leaves. Its result includes every obligation that recursion must discharge before the run is accepted.

### Common interface

Both implementations receive the same `MemAssignment`, pre-boundary token and cold-read requests.

Both implementations provide:

- Authenticated imported tails.
- Cold-record bus providers below `base`.
- A post-boundary token.
- Two memory-obligation scalars.
- Any required precommitment cells and transcript obligations.

A Merkle leaf discharges memory authentication locally. A LogUp leaf proves a conditional statement whose memory obligations require a complete run seal. The API and Area 3 statement must preserve this distinction.

### 4.1 Dense Merkle implementation

#### State and hashing

For arena `a`:

```text
q = floor(len_a / 8)
r = len_a mod 8

root_a:
    tree of completed segments [0, q)

tail_a:
    commitment to r records followed by canonical padding
```

The top tree has depth 61. Each segment is a binary tree of eight record commitments.

Use exact length-aware Poseidon encodings:

```text
record_d = H(SHK_MEM_RECORD_V1, 1, arena, record_words...)
pad_d    = H(SHK_MEM_PAD_V1, 1, arena)

segment_d = Tree8(record_d[0], ..., record_d[7])
tail_d    = Tree8(active_record_d..., pad_d...)
```

DATA and ENV hash six words. FIELDS and SLOTS hash their three non-padding words. Their unused bus lane is still constrained to zero.

`Tree8` and the fixed-depth upper tree use the existing two-input hash. Record and padding encodings have different lengths and domains. The circuit fixes tree height, orientation and arena. Empty top-tree leaves equal `Tree8(pad_d × 8)`, not an unconstrained zero.

The existing Poseidon implementation includes the input length in the initial state and uses 70 rows per permutation. [crates/mori-chip/src/poseidon.rs:55–59,97–114]

#### Flush constraints

For each arena:

```text
total = pre.offset + appended_count
flush = floor(total / 8)
post.offset = total - 8 * flush
```

With at most eight total appends per leaf, each arena needs at most one flush.

When `flush = 1`:

1. Hash the first eight window records.
2. Open the old segment position as the canonical empty segment.
3. Insert the completed segment using the same sibling cells and path bits.
4. Bind both walks to `q = pre.segment`.
5. Form the remaining tail from the remaining records.

When `flush = 0`, constrain `post.root = pre.root`. When no records are appended, constrain the complete memory boundary to remain unchanged.

Provision two shared flush units initially. A single reference step can append to DATA and FIELDS simultaneously. Whether two units suffice for every current rule remains part of the executable census. [RUN-B:37–46]

#### Cold reads

A cold read opens a complete logical record:

```text
record hash
    -> three intra-segment levels
    -> 61 upper-tree levels
    -> pre.root[arena]
```

Authenticate against the pre-leaf root. Records appended in this leaf remain window providers even after their segment becomes full.

#### Assumptions and guarantees

The backend assumes MEM’s append provenance and frontier constraints. It guarantees record authentication, append-only persistence, exact tail preservation and canonical empty space, subject to Poseidon’s cryptographic assumptions.

No memory-specific challenge, second proving pass or committed instance column is required.

#### Row budget

Let `H(n) = 70 * ceil(n / 2)`.

**ESTIMATE**:

| Operation | Rows |
|---|---:|
| DATA or ENV record hash, nine inputs | 350 |
| FIELDS or SLOTS record hash, six inputs | 210 |
| Eight-record subtree, excluding record hashes | 490 |
| One depth-61 old/new walk pair | 8,540 |
| One flush, excluding record hashes | 9,030 |
| Cold DATA or ENV read | 4,830 |
| Cold FIELDS or SLOTS read | 4,690 |
| One tail commitment, excluding record hashes | 490 |

The often-cited 4,480-row old-memory read covers 64 binary hashes. It excludes hashing the record that is opened. [RUN-E:158; RUN-C:174]

A conservative module allowance with all four pre-tails opened, eight new records, two flushes, both tail commitments per arena and four cold reads is **ESTIMATE 51,940 rows**, before range and selection overhead. Deduplication and copying unchanged tail tokens can reduce active work. Fixed circuit capacity must still cover the selected worst case.

This is not evidence that the whole leaf fits `k <= 16`.

### 4.2 Run-wide write-once LogUp implementation

#### Memory relation

Each appended record produces one write entry for each bus lane. Each authenticated cold-cache lane and imported-tail lane produces one read entry. Reads served from newly appended window records need no additional global entry.

For each repetition `j`:

```text
E_j(chip, key, a, b, c)
    = chip + gamma_j*key + gamma_j^2*a
           + gamma_j^3*b + gamma_j^4*c

sum_j =
    Σ write_lane multiplicity / (alpha_j - E_j(write_lane))
  - Σ authenticated_read_lane 1 / (alpha_j - E_j(read_lane))
```

The seal requires both run sums to be zero.

The logarithmic-derivative lookup construction uses reciprocal sums with a multiplicity function. Its application here requires a separate soundness argument for the committed transcript, recursion and bounded event counts. [Haböck, *Multivariate lookups based on logarithmic derivatives*](https://eprint.iacr.org/2022/1530)

Use two independently derived `(gamma, alpha)` pairs in the initial experiment. A one-pair profile is an optimization requiring an explicit whole-run security bound.

The multiplicity counts authenticated cache entries, not CPU lookup requests. An active cache entry may serve repeated local requests. Its tuple must contribute to the global check even when the host supplied the entry unnecessarily.

#### Commitment and challenge order

Pass 1 commits every challenge-dependent choice:

```text
P_i = [
    profile and schema identifiers,
    leaf index and execution interval,
    enabled-event flags,
    write tuples and multiplicities,
    authenticated-read tuples,
    pre-boundary words,
    post-boundary words,
    canonical padding
]

C_i = KZGCommit(P_i)
```

The run commitment binds the complete ordered sequence of `C_i`, its count, the relation profile, `prog_d`, `input_d` and any initialization certificate.

Derive challenges with separate domain labels:

```text
gamma_0, alpha_0, gamma_1, alpha_1, beta
    = Challenges(run_commitment, relation_profile, prog_d, input_d)
```

Pass 2 proves each leaf under those fixed challenges. Every used tuple, multiplicity, activity bit and boundary word is constrained equal to its committed instance entry.

The verifier supports committed instances and asserts that the circuit has one phase. Therefore, these challenges must enter as recursively checked scalar inputs. They cannot be introduced as a new Halo2 witness phase. [MC/verifier/verifier_gadget.rs:274–310]

The existing spike allocates a committed instance column followed by a scalar instance column. The committed-instance helper zero-pads a vector and calls `commit_lagrange`. These are available primitives, not an implemented two-pass protocol. [crates/mori-chip/src/rfcore.rs:830–844; MP/plonk/prover.rs:32–48]

#### Recursive obligations

Area 3 must constrain all of the following:

- The commitment supplied to the leaf verifier is the same `C_i` absorbed into the run commitment.
- Children carry the same profile, run commitment and challenges.
- Execution intervals and commitment-chain endpoints are adjacent.
- Memory sums add exactly.
- The final commitment-chain endpoint determines the challenges.
- Both final memory sums equal zero.
- Genesis or input admission accounts for all initially allocated records.
- A terminal outcome is proved before a RUN claim is emitted.

An ordered commitment hash chain permits different aggregation bracketings of the same leaf sequence. Repartitioning execution changes the commitments and challenges and requires new leaf proofs. It must leave the external claim unchanged.

#### Inverse and padding constraints

For an active entry:

```text
(alpha_j - E_j(tuple)) * inverse_j = 1
```

For an inactive entry:

```text
tuple = 0
multiplicity = 0
inverse_j = 0
contribution_j = 0
```

Reject active zero denominators. Do not omit the term or select another challenge after seeing the witness. A retry policy, if required, must be part of the transcript definition.

Stage tuple compression over rows. A direct expression containing powers of a witnessed challenge can violate the degree budget.

#### Initial memory and checkpoints

A nonzero initial length requires an authenticated source for its write entries. The backend accepts an initialization certificate whose records, lengths and input digest are proved together. A host-generated table is insufficient.

The complete run’s commitments and read multiplicities must exist before pass 2. Consequently:

- Pass 2 can prove leaves in parallel.
- A completed pass-1 bundle can be paused and resumed.
- Proving a final prefix before future execution is known is not supported by this protocol.
- Resetting sums at a checkpoint is unsound when later leaves read earlier writes.

A streaming checkpoint bridge needs an additional authenticated persistence relation. Until that relation exists, use Merkle memory for streaming execution and resumable prefixes.

#### Assumptions and guarantees

MEM proves unique append addresses and read causality. Commitment binding fixes records and multiplicities before the challenges. The completed run check then authenticates imported records with the statistical error of the chosen argument.

The proof must count all entries, including both record lanes and imported tails. “Fewer than `2^64` steps” is not by itself an event-count bound.

#### Row budget

Let `W` and `R` count active write and read **lanes**, and let `P` count committed words.

**ESTIMATE**, initial allowance:

```text
8–16 rows per lane for two compression/inverse/sum tracks
up to P rows for committed-instance binding
100–300 rows for counters and transcript glue
```

This excludes CORE, VALUE DIGEST, BOUNDARY COMMITMENT and shared RANGE costs.

Do not reuse the lookup study’s 599-row whole-leaf estimate. That estimate uses the smaller spike state and records. [LOOKUP:151–160,249–268]

### Verification for both implementations

Create:

```text
models/sherlock/quint/memoryBackend.qnt
models/sherlock/quint/writeOnceLogup.qnt
```

The common model proves that both implementations refine the same finite map and dense frontiers.

The LogUp model separates the precommit phase from the challenge phase. A small-field model may expose counterexamples, but it cannot establish the production cryptographic error bound.

MockProver positives:

- All MEM tests under both backends.
- Two simultaneous arena flushes.
- Cold reads repeated across several leaves.
- Initially admitted input records.
- Unused writes with multiplicity zero.
- Two-leaf and three-leaf LogUp cancellation.
- Identical leaves aggregated with different bracketings.

Named negative mutants:

| Merkle | LogUp |
|---|---|
| `old_segment_not_empty` | `multiplicity_uncommitted` |
| `old_new_paths_differ` | `activity_uncommitted` |
| `wrong_arena_root` | `tuple_uncommitted` |
| `path_bit_nonboolean` | `commitment_substituted_in_node` |
| `path_index_alias` | `challenge_not_derived` |
| `record_hash_omits_lane` | `inverse_unconstrained` |
| `flush_not_iff_full` | `inactive_term_nonzero` |
| `tail_suffix_wrong` | `read_entry_not_charged` |
| `cold_read_uses_post_root` | `seal_sum_unchecked` |
| `empty_root_unbound` | `checkpoint_resets_debt` |

MockProver can check the scalar binding constraints. Real proof tests must separately check committed-instance binding and the recursive connection between `C_i` and the run transcript.

### Code skeleton

Own:

```text
crates/mori-chip/src/memory/backend.rs
crates/mori-chip/src/memory/merkle.rs
crates/mori-chip/src/memory/logup.rs
crates/mori-chip/src/memory/precommit.rs
crates/mori-chip/tests/memory_backend.rs
```

```rust
pub struct BackendAssignment {
    pub post: MemoryBoundaryCells,
    pub obligation: MemoryObligationCells,
}

pub trait MemoryBackend {
    type Witness;

    /// Ensures imported records and post memory satisfy this backend's relation.
    fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        memory: &MemAssignment,
        pre: &MemoryBoundaryCells,
        challenges: &RunChallengeCells,
        committed: &CommittedPayloadCells,
        witness: &Self::Witness,
    ) -> Result<BackendAssignment, Error>;
}

pub struct DenseMerkleConfig { /* private */ }
pub struct WriteOnceLogUpConfig { /* private */ }

/// Ensures canonical field order, lengths and zero padding before commitment.
pub fn encode_precommit_payload(
    profile: &BackendProfile,
    leaf: &LeafPrecommitWitness,
) -> Result<Vec<F>, WitnessError>;

impl DenseMerkleConfig {
    /// Ensures one empty-to-full insertion with shared path cells.
    pub fn constrain_flush(
        &self,
        layouter: &mut impl Layouter<F>,
        flush: &FlushCells,
    ) -> Result<(), Error>;
}

impl WriteOnceLogUpConfig {
    /// Ensures a tuple contributes the specified signed rational term.
    pub fn constrain_entry(
        &self,
        layouter: &mut impl Layouter<F>,
        entry: &CommittedMemoryEntryCells,
        challenges: &RunChallengeCells,
    ) -> Result<[Cell; 2], Error>;
}
```

The Merkle agent reads this contract, `m3.rs:51–143`, the HASH port and the common backend tests. The LogUp agent reads this contract, `LOOKUP:73–109,183–199`, and the cited committed-instance verifier code.

Neither agent needs CORE’s rule implementations or the other backend’s internal columns.

## 5. VALUE DIGEST: `sherlock-value/1`

### Purpose

VALUE DIGEST computes the canonical address-free digest of a value. It binds that digest to the authenticated record when the value is created or imported.

### Interface

VALUE DIGEST uses copied cells rather than a separate bus namespace.

It requests:

- A DATA record from MEM or an authenticated constant record from DEC/admission.
- Child references and their authenticated digests.
- Ordered field cells selected by `fields_start + field_position`.
- Authenticated declaration metadata for a constructor.
- RANGE checks on scalars, tags, arity and indices.
- HASH computations over exact ordered preimages.

It provides:

- `ValueDigestCells { vd }`, certified by this relation.
- The same `vd` copied into the DATA record’s digest word.
- Authenticated child digests copied into FIELDS, ENV and SLOTS records.
- A constructor-fold continuation copied into CORE’s helper state.
- An outcome digest copied to the terminal-state opening.
- An input-value digest copied into the `input_d` computation.

A host witness may propose a digest. It cannot construct an authenticated digest object without satisfying these constraints.

### Exact encoding

Let:

```text
V = dom("SHK_VALUE_V1")
O = dom("SHK_OUTCOME_V1")
header(tag, arity) = [V, 1, tag, arity]
```

| Value | Preimage |
|---|---|
| UInt `n` | `[V, 1, 0, 0, n]` |
| Bool `b` | `[V, 1, 1, 0, b]` |
| Nil | `[V, 1, 2, 0]` |
| Cons `(h,t)` | `[V, 1, 3, 2, vd(h), vd(t)]` |
| Leaf `x` | `[V, 1, 4, 1, vd(x)]` |
| Branch `(l,r)` | `[V, 1, 5, 2, vd(l), vd(r)]` |
| Constructor | `[V, 1, 6, m, ty_d, ordinal, acc_m]` |

Constructor fields use:

```text
acc_0 = 0
acc_(k+1) = H(V, 1, 7, 2, acc_k, vd(field_k))
```

A reference has the digest of its value. Neither its reference kind nor its address enters `vd`. These encodings follow the adopted C6 table. [C6:104–124; DISSENT:216]

Outcomes:

```text
Halted(v):       H(O, 1, 0, vd(v))
Rejected(code):  H(O, 1, 1, code)
Exhausted(kind): H(O, 1, 2, kind)
Running:        no outcome digest
```

The encoding range does not authorize a machine outcome forbidden by the active profile. In particular, exhaust kind 7 remains a recorded conflict with profile `/1`. [C6:120; DISSENT:226]

### Guarantees

1. Every digest uses the exact tag, version, arity and field order.
2. Every child digest is authenticated.
3. Constructor folding advances by one authenticated field position.
4. Constructor completion requires `cursor = arity`.
5. The resulting digest is bound to the immutable DATA record.
6. Constants and dynamic data with the same value have equal digests.
7. Shared and duplicated equal subvalues have equal digests.
8. Halt copies an authenticated result digest.
9. Padding, dangling references and out-of-range scalars are rejected.

The constructor helper’s accumulator, cursor, arity, type identity and ordinal must survive a leaf boundary. They are named CORE boundary cells.

### Assumptions and open type identity

DEC or admission must authenticate the mapping:

```text
(program-local type_id, ctor_id)
    -> (program-independent ty_d, constructor ordinal, arity)
```

`decl_schema_d` has no adopted normal form. The module must expose this dependency explicitly and refuse key freeze until its contract is fixed. Substituting a raw type index or producer program digest would change the adopted cross-program value semantics. [DISSENT:204–207; REQ:47]

Input loading and lazy unfolding use the same digest checks. They also establish scalar ranges and reference validity. This does not settle whether those operations count as reference steps. That is an Init/refinement decision for Areas 1 and 3.

### Row budget

**ESTIMATE**, excluding child authentication, ranges and record storage:

| Operation | Rows |
|---|---:|
| UInt, Bool or Leaf | 210 |
| Nil | 140 |
| Cons or Branch | 210 |
| One constructor field fold | 210 |
| Constructor completion | 280 |
| Outcome wrapper | 140 |

These use the adopted preimages and 70 rows per permutation. [C6:126–139]

Do not price a constructor with arbitrary arity as one bounded hash request. Its existing field helper supplies the incremental fold.

### Verification

Create `models/sherlock/quint/valueDigest.qnt`.

Use a structural value model with explicit field order and separate physical representations. Model digest equality abstractly for semantic refinement, and use concrete hash vectors in circuit tests.

MockProver positives:

- Every builtin form.
- UInt values `0` and `2^128 - 1`.
- Constructors with zero, one and three fields.
- A constructor fold split across leaves.
- Constant/data representations of the same value.
- Shared and duplicated structures.
- Input and consumed-output digest parity.

Named negative mutants:

```text
host_supplied_vd
wrong_child_digest
wrong_value_tag
wrong_arity
field_order_swapped
fold_cursor_skipped
fold_accumulator_reset
raw_type_id_in_digest
constructor_ordinal_unbound
scalar_out_of_range
digest_not_copied_to_record
running_has_outcome
padding_as_value
```

### Code skeleton

Own:

```text
crates/mori-chip/src/value/mod.rs
crates/mori-chip/src/value/encoding.rs
crates/mori-chip/src/value/digest.rs
crates/mori-chip/tests/value_digest.rs
```

```rust
pub struct ValueDigestConfig { /* private */ }
pub struct ValueDigestCells { pub vd: Cell }
pub struct ConstructorFoldCells { /* cursor, arity, acc and declaration cells */ }

impl ValueDigestConfig {
    /// Ensures the builtin digest matches authenticated fields and scalar ranges.
    pub fn constrain_builtin(
        &self,
        layouter: &mut impl Layouter<F>,
        value: &AuthenticatedBuiltinCells,
    ) -> Result<ValueDigestCells, Error>;

    /// Ensures exactly one ordered field extends the constructor fold.
    pub fn fold_field(
        &self,
        layouter: &mut impl Layouter<F>,
        pre: &ConstructorFoldCells,
        field: &AuthenticatedFieldCells,
    ) -> Result<ConstructorFoldCells, Error>;

    /// Ensures completion at arity and binds the digest to the DATA append.
    pub fn finish_constructor(
        &self,
        layouter: &mut impl Layouter<F>,
        fold: &ConstructorFoldCells,
        record: &RecordCells,
    ) -> Result<ValueDigestCells, Error>;

    /// Ensures the wrapper represents a proved terminal outcome.
    pub fn constrain_outcome(
        &self,
        layouter: &mut impl Layouter<F>,
        outcome: &TerminalOutcomeCells,
    ) -> Result<Cell, Error>;
}

/// Returns the canonical preimage for host/circuit differential vectors.
pub fn encode_value_preimage(value: &CanonicalValueNode) -> Vec<F>;
```

The implementing agent reads this encoding table, C6’s canonical table, the authenticated record ports and the constructor helper’s field-order contract. It does not need Merkle paths, LogUp accumulators, recursion keys or ledger serialization.

## 6. BOUNDARY COMMITMENT

### Purpose

BOUNDARY COMMITMENT binds every state cell required to continue execution. Its output permits recursion to compare adjacent segment endpoints without exposing physical state in the final run statement.

### Common interface

Receive a compile-time ordered `BoundarySchema` and assigned boundary cells.

The serialization is:

```text
[
    SHK_BOUNDARY_V1,
    1,
    schema_id,
    memory_backend_id,
    boundary_backend_id,
    prog_d,
    input_d,

    CORE persistent words in schema order,

    DATA.len, FIELDS.len, ENV.len, SLOTS.len,
    DATA.root, FIELDS.root, ENV.root, SLOTS.root,
    DATA.tail, FIELDS.tail, ENV.tail, SLOTS.tail
]
```

CORE persistent words include control, outcome, logical counters, fuel policy and remaining fuel, environments, cached frame/task contents, stack commitments and counts, helper state, result and dependency commitment.

The schema must account for every register. A register may be omitted only if the circuit enforces initialization before every read within each leaf. The default is to include it.

The spike omits `v0..v2` from its 29-word digest. That is not sufficient evidence that the complete machine can omit them. Merely zeroing them at a boundary also requires a refinement argument that the reset is unobservable. [crates/mori-chip/src/rf.rs:4–8,32–67; RUN-A:33–37]

The boundary module provides one commitment cell for each endpoint and named opening cells for Init, continuation and terminal sealing.

No module reads the boundary module’s internal columns.

### 6.1 Poseidon implementation

Compute:

```text
boundary_d = H(serialized_boundary_words)
```

The input length is fixed by the keyed schema. Every input is a copied cell or a fixed domain/schema constant.

Requirements:

- One ordinary proof pass.
- No extra memory or boundary challenges.
- No required committed payload.
- One scalar instance column can carry the internal statement.
- The existing reserved committed instance column may remain unused.

**ESTIMATE:** for `N` serialized words, two endpoint digests cost:

```text
140 * ceil(N / 2) rows
```

For example, `N = 96` costs **ESTIMATE 6,720 rows**, excluding assignment glue. The final `N` must come from Area 1’s complete schema, not the spike’s `NDIG`.

### 6.2 Horner implementation

Compute:

```text
fingerprint(words, beta) = Σ words[i] * beta^i
```

Use the same fixed serialization as Poseidon.

The boundary words must be included in the pass-1 committed payload before `beta` is derived. Binding only the final fingerprint is insufficient. The leaf’s constrained boundary cells must equal the committed words.

Requirements:

- Two-pass precommitment protocol, even with Merkle memory.
- One committed instance column and one scalar instance column.
- A run-wide `beta` with its own challenge domain.
- Recursive enforcement of the commitment transcript and challenge equality.
- A statistical equality argument in place of deterministic digest injectivity.

**ESTIMATE:** a narrow Horner chain takes `N + O(1)` rows per boundary. Two boundaries plus separate binding rows need up to **ESTIMATE `4N + O(1)` rows**. Sharing existing binding rows can lower that cost. Do not credit overlapping columns until the layout is measured.

Horner replaces boundary hashing only. It does not replace `vd`, `input_d`, `outcome_d`, Merkle record hashes or replay-key hashing.

### Guarantees and assumptions

Both implementations guarantee that their output derives from the exact constrained serialization.

Poseidon equality relies on its cryptographic binding assumption. Horner equality additionally relies on precommitment, correct challenge derivation and the polynomial identity error bound.

A Horner fingerprint is valid only within its proved run context. It is not a portable checkpoint identifier or a consumable value digest.

### Verification

Create `models/sherlock/quint/boundaryCommitment.qnt`.

Separate state serialization from commitment checking. Model adjacency over the complete state and represent fingerprint equality as a conditional cryptographic premise. Do not reuse the aggregation model’s injective-digest premise unchanged. [models/sherlock/quint/aggregation.qnt:12–15]

MockProver positives:

- Adjacent leaves with all persistent state fields represented.
- Every terminal outcome.
- Constructor folds and cached stack tops crossing a boundary.
- Both memory implementations under both boundary implementations.
- Horner boundaries with fixed pass-1 payloads.

Named negative mutants:

```text
boundary_word_omitted
boundary_word_reordered
post_uses_pre_cell
memory_token_unbound
schema_id_unbound
scratch_read_before_init
horner_words_uncommitted
horner_beta_free
horner_beta_mismatch
boundary_commitment_substituted
terminal_opening_unbound
```

An unconstrained-Horner negative should deliberately modify two state words while preserving a chosen fingerprint. The committed implementation must reject the modified words.

### Code skeleton

Own:

```text
crates/mori-chip/src/boundary/mod.rs
crates/mori-chip/src/boundary/schema.rs
crates/mori-chip/src/boundary/poseidon.rs
crates/mori-chip/src/boundary/horner.rs
crates/mori-chip/tests/boundary_commitment.rs
```

```rust
pub struct BoundarySchema { /* fixed ordered field descriptors */ }
pub struct BoundaryCells { /* named assigned state fields */ }
pub struct SerializedBoundaryCells { pub words: Vec<Cell> }

/// Ensures every schema field occurs once in the keyed order.
pub fn serialize_boundary(
    schema: &BoundarySchema,
    state: &BoundaryCells,
) -> Result<SerializedBoundaryCells, LayoutError>;

pub trait BoundaryCommitment {
    /// Ensures the result binds the serialized, constrained boundary.
    fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        state: &SerializedBoundaryCells,
        committed: &CommittedBoundaryCells,
        challenges: &RunChallengeCells,
    ) -> Result<BoundaryCommitmentCells, Error>;
}

pub struct PoseidonBoundaryConfig { /* private */ }
pub struct HornerBoundaryConfig { /* private */ }
```

The implementing agent reads the literal boundary schema, the HASH port, committed-payload binding contract and module tests. It does not need rule bodies, memory path construction or ledger verification code.

## 7. SRC-023 partition independence

The external run statement remains:

```text
[prog_d, input_d, outcome_d, n_steps]
```

The replay identity remains:

```text
claim_d = H(SHK_RUN, 1, prog_d, input_d, outcome_d)
```

Memory roots, tail commitments, fingerprints, LogUp sums, precommitments and challenges remain internal. They do not enter a consumed RUN claim or determine replay identity. [REQ:31–32,47–48; SPEC:138–144]

| Combination | Partition-independence argument |
|---|---|
| Merkle + Poseidon | Dense allocation and canonical full-segment/tail representation give the same logical memory at each execution point. Complete state binding and reference refinement determine the same outcome and step count. |
| Merkle + Horner | Memory is as above. Fingerprints may differ after repartition because the challenge changes. Precommitted boundary equality still links one valid execution, subject to the fingerprint error bound. |
| LogUp + Poseidon | Each partition produces a different committed event bundle. The completed run memory check establishes the same write-once semantics. Poseidon binds the corresponding boundary state. |
| LogUp + Horner | Both memory and adjacency are conditional on the precommitment protocol. The seal discharges both obligations before emitting the same canonical outcome claim. |

The proof obligations are:

1. Every enabled machine step refines one reference transition.
2. Closing a leaf is an identity operation on decoded logical state.
3. Resource accounting depends on logical events.
4. Canonical value digests erase addresses and sharing.
5. Initialization authenticates the same input and resource policy.
6. A terminal seal discharges all backend obligations.

Three-partition testing must use full leaves where resources permit, one step per leaf and mixed boundaries. Compare all four public values and the replay key. Include shared/duplicated values and a constructor whose fields span several leaves.

Do not require proof bytes, challenges or physical commitments to match. SRC-023 requires equal public execution results. [REQ:48]

## 8. Area 3 integration obligations

Area 2 exports obligations, not a new recursion architecture.

NODE INTEGRATION must:

- Compare endpoint commitments under the same boundary profile.
- Link heights, program identity and input identity.
- Carry and combine LogUp sums when enabled.
- Bind the verified committed-instance point to the ordered precommitment transcript.
- Prevent an unresolved memory or boundary obligation from becoming a RUN claim.

DECIDER or the run seal must:

- Establish Init and a terminal state.
- Verify the complete challenge transcript for challenge-based profiles.
- Check both LogUp sums.
- Open the terminal value through VALUE DIGEST.
- Publish the C6 statement without physical commitments.

GP-2 consumption receives a sealed claim and authenticates the opening of its outcome value. It receives no open LogUp accumulator. GP-3 adapters must authenticate any claimed value digest. GP-4 permits aggregation rebracketing without confusing the proof graph with run identity. [C6:168–172]

For CONTINUATION:

- Merkle plus Poseidon supports retained-state continuation once Area 3 proves checkpoint linkage.
- Horner endpoints cannot be compared across unrelated challenges.
- An address-free checkpoint needs its own encoding and reconstruction relation.
- A LogUp prefix retains its unresolved memory obligation until a supported bridge or terminal seal discharges it.
- Checkpoint creation does not reset fuel or publish a terminal outcome.

The ledger review establishes a compatibility route through a recursive wrapper. It does not establish that these new modules fit or verify on the deployed network. Its reported decider also has a tight table-row budget. [C6:208–213]

## 9. Cost and implementation controls

The measured final spike has 12 advice columns, 13 equality columns, 20 fixed columns, four lookups and maximum degree six. Its reported node has 823,466 rows. [SPEC:125–134]

Use the following **ESTIMATE** only for architectural comparison:

```text
Δ node rows ≈
    7,298 * Δ advice columns
  + 8,189 * Δ equality columns
  + 6,004 * Δ fixed columns
  + 22,600 * Δ lookup arguments
```

The column fit has residual error, and fixed-column contents affect node cost. Measure actual populated keys. Do not assume adding one equality-enabled advice column costs only 7,298 rows. [COST:120–130,223–244]

Area 2 must reuse the narrow shared column bank and serialize work vertically. Keep rotations within `{-1,0,+1}` and total gate degree at most six. Selector packing must include the body’s degree.

Build order within Area 2:

1. Shared record types, serialization and test vectors.
2. MEM with a test backend.
3. VALUE DIGEST and Poseidon BOUNDARY COMMITMENT.
4. Dense Merkle backend.
5. Complete-leaf differential harness with Area 1.
6. Committed-payload binding and recursive transcript fixture.
7. LogUp backend and Horner BOUNDARY COMMITMENT.
8. All four combinations against identical semantic fixtures.

Each module kit includes its interface, assumptions, guarantees, row allowance, model, positive tests and named mutants. Function contracts must identify copied inputs and constrained outputs. This follows the skeleton experiment’s contract format. [research/skeleton-coding-20261108/pilot/prompts/skeleton-derived.md:17–32]

Every module change runs the whole-leaf differential harness against `sherlock-k-a/1` when implementation begins. Test results must distinguish MockProver constraints, formal model checks, real recursive proof binding and ledger acceptance.

## 10. Open decisions and conflicts

| Decision or conflict | Options | Recommendation |
|---|---|---|
| One arena versus four | One tagged arena with a proved address mapping, or four reference-aligned arenas | Use four initially. RUN-A and RUN-B retain four, while RUN-C, RUN-D and RUN-E recommend one. Their record shapes and cost assumptions also differ. [RUN-A:23–31; RUN-B:178–186; RUN-C:167–178; RUN-E:154–158] |
| D3 allocation | Pad at leaf close, or advance only on actual append | Replace padding. The current gate enforces `post.len = base + 8`; it cannot implement this draft unchanged. [SPEC:96; crates/mori-chip/src/m3.rs:125–133] |
| Memory backend | Merkle streaming or two-pass LogUp | Integrate Merkle first. Keep LogUp as a measured alternate with explicit seal obligations. |
| Alleged LogUp impossibility | Treat single-phase restriction as prohibiting all LogUp, or use external precommitment | Reject the categorical impossibility claim. The verifier restriction forbids the proposed extra phase, not necessarily a two-pass committed-instance protocol. Its complete soundness remains to be established. [RUN-A:183; RUN-B:175; LOOKUP:91–109; MC/verifier/verifier_gadget.rs:274–310] |
| Boundary implementation | Poseidon or precommitted Horner | Poseidon first. Horner requires the full transcript protocol even when memory uses Merkle. |
| “One instance column” | Literal single column or one scalar plus one committed column | Update the profile description explicitly. The spike allocates two columns, and challenge-based profiles use both. [SPEC:80; crates/mori-chip/src/rfcore.rs:833–834] |
| Leaf statement | Existing seven lanes or a versioned statement carrying backend obligations | Area 3 must define a new exact schema before integration. Do not silently append fields to `sherlock-leaf-stmt/1`. [REQ:26] |
| Type identity | Program-local identity or canonical declaration identity | Retain program-independent `ty_d`. Define `decl_schema_d` before key freeze. [DISSENT:204–207] |
| Input admission | Loading proof or lazy authenticated unfolding | Both must use VALUE DIGEST and establish record invariants. Select one with a reference-step accounting proof. Init must not scan the whole input. [REQ:46; C6:124,164] |
| Scratch registers | Include them, or prove safe omission | Include every future-readable register initially. Remove one only with a checked initialization-before-read obligation. |
| Full memory domain | Assume 64-bit frontiers suffice, or prove capacity coverage | Keep GP-1 unchanged. A depth-61 segment tree has `2^64` record positions, but a `u64` frontier represents at most `2^64 - 1` allocated records. Multiple appends per step and initial input mean that this alone does not prove coverage of every run below `2^64` steps. Establish a capacity theorem or widen the address/tree profile before freeze. [GP:9–14; RUN-B:40–46] |
| Atomic-step capacity | Average census or worst-case proof | Require a rule-by-rule maximum for cold reads, appends, flushes and hashes. A fixed leaf that cannot prove one required step fails completeness. |
| LogUp security budget | One repetition or multiple repetitions | Start with two independent repetitions. Derive a whole-run bound including all lanes, imported records, boundaries and Fiat–Shamir retries before selecting a production count. |
| Exhaust kind 7 | Encoding permits it, profile `/1` rejects it | Preserve profile rejection until the specification resolves the conflict. [DISSENT:226] |
| Raising `S` | Keep `S_MAX = 16` or adopt 64/128 | Keep the current bound for this draft. Larger values require a separate spec change and complete measurement. [REQ:39; LOOKUP:160; RUN-E:111–116] |
| Whole-leaf fit | Reuse spike/study totals or measure full records and state | Measure the complete design. This draft’s authenticated record widths invalidate several smaller study estimates. |

## 11. Checks not performed

The run-length and lookup-cache `CONSENSUS.md` files were absent when checked. The cited lane reports remain proposals with disagreements.

The source review covered the named design sections, relevant requirements, all run-length lanes’ memory recommendations, the lookup proposal, value encoding, selected spike code, verifier phase handling and committed-instance helper. It did not independently audit every reference-machine allocation path.

The following remain unverified:

- The complete 108-rule boundary schema.
- Worst-case resources for one indivisible reference step.
- Four-arena versus one-arena refinement.
- The canonical declaration schema for constructor types.
- Initialization and checkpoint reconstruction.
- The LogUp argument’s full recursive soundness and security bound.
- Privacy of any exposed precommitment bundle. The inspected commitment helper has no explicit random-blinding argument.
- Complete leaf, node and decider row counts.
- Compatibility of the revised recursive proof with the target ledger deployment.

The signatures above are interface skeletons. They have not been compiled.