# SDK-free host machine

Public path: `mori_core::sherlock`. `Machine` is re-exported; mutation remains private. No wire ABI. The [stream producer](../../../models/sherlock/qa/stream_observations.rs#L114) uses fixed normal inputs; see [observed run](doctorwatson.md).

## Machine methods

```rust
pub fn new(program: Arc<Program>, fuel: u128, stack_limit: usize) -> Self;
pub fn snapshot(&self) -> Snapshot;
pub fn step(&mut self) -> Option<StepRecord>;
pub fn run_steps(&mut self, max_events: usize) -> Vec<StepRecord>;
pub fn run(&mut self) -> Snapshot;
pub fn visit_steps(
    &mut self,
    max_events: usize,
    visitor: impl FnMut(&StepRecord) -> std::ops::ControlFlow<()>,
) -> VisitResult;
```

Signatures omit bodies/parameter `mut`. [Methods](../../../crates/mori-core/src/sherlock/reference.rs#L14), [visits](../../../crates/mori-core/src/sherlock/reference_stream.rs#L38).

`new` starts at `Eval(program.entry)`, empty environment/frames, full supplied fuel and `Pending`. `snapshot` clones the full observation. There is no public arbitrary-state importer. `step` returns no record after terminal outcome. A pending top-level return publishes `Done` at line 80, a pending error publishes `Rejected` at line 82, and otherwise zero fuel publishes `Exhausted` at line 84. Publication precedes exhaustion and costs zero. Other transitions consume one fuel.

`run_steps` retains at most the event budget and stops at publication. `visit_steps` borrows complete records and drops each before advancing; it retains no history. Observer break stops after a nonterminal event; publication takes precedence. Zero budget does not advance/call the observer. Later calls resume pending publication. `run` steps to completion.

Event budgets bound grouping only. Helpers, callbacks, cloning and allocation are unmetered; there is no host time/heap/stack admission policy or untrusted-input sandbox. No B/circuit/native evidence is supplied.

## Program and value data

`Name = String`, `Environment = BTreeMap<Name, Value>`, `ExprRef = Arc<Expr>`. `Value` supports UInt/Bool/List/Tree. Complete `Value`, `Tree`, `Type`, `Program`, `Function`, `Parameter`, `Expr`, `Pattern`, `Frame`, `Control` declarations: [reference_types.rs:15](../../../crates/mori-core/src/sherlock/reference_types.rs#L15). Function lookup selects the first matching name. Only canonical signed-decimal symbols belong to the K comparison domain.

## Complete observations

```rust
pub struct Snapshot {
    pub program: Arc<Program>,
    pub control: Option<Control>,
    pub environment: Environment,
    pub frames: Vec<Frame>,
    pub stack_limit: usize,
    pub remaining_fuel: u128,
    pub initial_fuel: u128,
    pub outcome: Outcome,
}
pub struct StepRecord {
    pub rule_start_line: usize,
    pub charge: u8,
    pub before: Snapshot,
    pub after: Snapshot,
}
```

Frames are top first. `None` control denotes K's `.K`. Machine-produced snapshots satisfy `remaining_fuel <= initial_fuel`. `depth()` returns frame count; `charged_steps()` requires that fuel invariant; `checked_charged_steps()` returns `None` for an invalid hand-constructed counter pair. A constructed snapshot is not evidence of reachable machine state. [Declarations](../../../crates/mori-core/src/sherlock/reference_types.rs#L162).

See [first local observations](../tutorials/first-local-observations.md), [find first difference](../how-to/find-first-difference.md) and [image boundary](image-layout.md).

## A trace continuity diagnostic

`mori_core::sherlock::compare::first_continuity_difference(trace, returned)` takes
one borrowed `TraceView` and an independently supplied `&Snapshot`. It returns
`Option<ContinuityDifference>`. This is separate from the unchanged pairwise
`first_difference(left, right)` observation comparison.

State edges are checked in this order:

1. Independent initial state against record0.before.
2. Each prior.after against next.before, in ascending record index order.
3. Last.after against the returned snapshot.

A present empty record slice instead compares initial with returned. Empty
records establish neither missing evidence nor terminality. A matching pending
prefix is permitted. Every edge compares all eight complete Snapshot fields in
`SnapshotField` order: program, control, environment, frames, stack limit,
remaining fuel, initial fuel and outcome. Equality is structural, including
program bodies and saved frame environments.

A state mismatch returns `ContinuityDifference::State { boundary, field }`.
`ContinuityBoundary` is `InitialToFirst`, `BetweenRecords { previous, next }`,
`LastToReturned { last }`, or `EmptyToReturned`; indices are zero based. These
positions select the two raw states in the original caller-owned inputs. No
state is copied into the diagnostic or reconstructed from expected data.

After all edges agree, the function checks the initial remaining fuel minus
returned remaining fuel with checked subtraction, sums every record's `u8`
charge with checked `u128` addition, and compares the totals exactly.
`UndefinedFuelDelta` identifies an undefined subtraction; `ChargeOverflow { index }`
identifies the first overflowing addition; `ChargeSumMismatch { recorded,
endpoint_delta }` preserves unequal totals. Nothing is clamped. On the current
64-bit host an addressable `u8`-charge slice cannot reach `u128` sum overflow;
the operation still uses checked arithmetic.

`None` establishes only supplied-state continuity and aggregate charge equality.
It does not check transition validity, individual rule charges, terminality,
profile/source authenticity, absent adapters/layers, proofs or B-profile
correspondence. Declared metadata is not authenticated by this API. Host
structural traversal remains unbounded; this adds no resource policy. The full
SCON-007/R02 and parent acceptance obligations remain separate.

## Owned B host observations

The source `helper_machine::complete_observation` exposes `capture`, `copy_observation`, `record_step` and `first_discontinuity`. Its records copy all current public typed admission/program/layout/image fields, preserve the full actual event and retain terminal/failure evidence. This is separate from the A comparator and does not align their clocks. The [production typecheck](../../results/sherlock-b-complete-observation-20261017/README.md) passed; the four new fixed-step tests and applicable source/execution-scope review remain pending. Host copying is unmetered diagnostic work; these snapshots are not a canonical checkpoint or a transition proof.
