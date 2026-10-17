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
