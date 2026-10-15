# Opus core foundation: stage API (early report for the admit/decode owner)

Status: SOURCE-WRITTEN in progress. This is not execution approval. Nothing was built or run.

## `mori_core::stage` (defined in `stage/mod.rs`)

Module tree: `mod admit; mod cost; pub mod decode; mod digest; mod plan; mod profile; mod validate;`

Public re-exports:
- `pub use crate::LOGICAL_COST_PROFILE_2;`
- `pub use admit::{resume, start};` — **admit.rs must define `pub fn start` and `pub fn resume` with the A3 signatures.**
- `pub use plan::{ActionPlan, AwaitSite, CaptureSlot, Origin, StageBinder, StagePlan};`
- `pub use profile::{require_core1, require_core2};`

Constants (all `pub`, defined only in `mod.rs`; admit/decode import from `super`):
`LANGUAGE_2`, `SEMANTICS_2`, `STAGE_PLAN_FORMAT = "moriarty-stage-plan/1"`, `CONTINUATION_FORMAT`, `STAGE_EVENT_FORMAT`, `MAX_AWAITS_PER_ACTION = 16`, `MAX_EVENT_BINDERS = 16`, `MAX_CAPTURE_SLOTS = 256`, `MAX_CONTINUATION_BYTES = 4 * 1_048_576`, `MAX_EVENT_BYTES = 65_536`, `STAGE_LABELS`.

Placement note: A3 places `STAGE_LABELS` in `admit.rs`, and A4 places `MAX_CONTINUATION_BYTES`/`MAX_EVENT_BYTES` in `decode.rs`. Per the task they live once in `mod.rs`. Do **not** redefine them. To keep the A4 paths valid, `decode.rs` should add `pub use super::{MAX_CONTINUATION_BYTES, MAX_EVENT_BYTES};`. The public path `stage::STAGE_LABELS` is unchanged, and so are the values.

DTOs (all fields `pub`):
- `NamedValue { name: String, value: Value }`: Serialize, Deserialize, `deny_unknown_fields`, `PartialEq`/`Eq`/`Clone`.
- `StageLogical { profile, cap: Option<UInt> /*required key, null*/, committed }`: same derives.
- `Continuation { format, program_hash, plan_hash, action, site: u16, captures: Vec<NamedValue>, logical: StageLogical, boundary: Snapshot }`: same derives.
- `StageEvent { format, action, label, values: Vec<NamedValue> }`: same derives.
- `PendingFrame { site: u16, captures: Vec<NamedValue> }`: same derives, plus `pub(crate) fn idle()`, which returns `{0, []}`.
- `StageSegment { action, frame_pre, frame_post, execution: Execution }`: **Serialize only.** No Deserialize, because it would decode shared Await-bearing `Statement` (FINAL-NONSEC wire rule; C4 never decodes history).
- `StageOutcome::{Waiting{advanced, segment: Option<StageSegment>, continuation, attempted, residual: Vec<Liability>}, Complete{segment: StageSegment, committed, attempted, residual: Vec<Liability>}}`: Serialize only.
- `StageRejection { error: Error, attempted: UInt, committed: UInt }`: Serialize and `thiserror::Error` (`"{error}"`).
- `pub(crate) fn residual(&Snapshot) -> Vec<Liability>`: liabilities with amount > 0, in boundary order.

Sealed program:
- `StagedProgram` has private fields and derives `Debug` and `Clone` only, with no `Default`, `Deserialize` or constructor.
- Accessors: `program()`, `plan()`, `program_hash() -> [u8;32]`, `plan_hash() -> [u8;32]`, and `pub(crate) action_index(&str) -> Option<usize>`.
- `pub fn load(src) -> Result<StagedProgram, Error>` runs:
  1. `parse_v2`
  2. `require_core2`
  3. `validate_v2`
  4. `plan::build`
  5. `plan_invariants`
  6. `cost::fill`
  7. `cost_invariants`
  8. `program_hash`, which is written into `plan.program_hash`
  9. `plan_hash`

## Helpers for `admit.rs` (`stage/cost.rs`, `pub(crate)`)

```rust
pub(crate) fn fill(program: &Program, plan: &mut StagePlan) -> Result<(), Error>;            // load only
pub(crate) fn statement_cost(program: &Program, statement: &Statement) -> Result<UInt, Error>; // 1 + distinct DAG nodes
pub(crate) fn dispatch_cost(program: &Program, plan: &ActionPlan, site: u16) -> Result<UInt, Error>; // attempted of a false guard at site
pub(crate) fn start_preflight(plan: &ActionPlan, cap: Option<UInt>) -> Result<UInt, Error>;
pub(crate) fn resume_preflight(plan: &ActionPlan, site: u16, logical: &StageLogical, cli_cap: Option<UInt>) -> Result<UInt, Error>;
pub(crate) fn segment_meter(limit: UInt) -> Meter;
pub(crate) fn require_static(meter: &Meter, expected: UInt) -> Result<(), Error>;
pub(crate) fn committed_after(plan: &ActionPlan, segment: usize) -> Result<UInt, Error>;
```

- **`start_preflight`** (Clarification §2.5, C3):
  - `None` → no check.
  - `Some(c)` → requires `plan_total ≤ c`, else `stage_logical`.
  - Returns the meter limit `segment_cost[0]`. Admit reports attempted 0 on error.
- **`resume_preflight` (R6).** Run it only after R1–R5.
  - `committed == committed_before[site]`, else `stage_logical`.
  - CLI cap `c`: the stored cap must be `None` or `c ≤ stored`, and `committed + segment_cost[site] ≤ c` (checked).
  - No CLI cap and stored `Some(s)`: `committed + segment_cost[site] ≤ s`.
  - Returns the meter limit `segment_cost[site]`. Every failure is `stage_logical`; admit reports attempted 0 and committed = the input committed.
  - The output continuation carries the **stored** cap unchanged.
- **`segment_meter(limit)`** is `Meter::new(Cost{logical: limit, ..})`.
- **`require_static(meter, expected)` (H26).** Requires `meter.used().logical == expected`, else `stage_logical` with the message "static cost premise violated".
  - Use it after `Ran`, with `segment_cost[j]`.
  - Use it after `NotAdvanced`, with `dispatch_cost(site)` (recommended).
  - If `run_segment` returns a `mori::core::meter` error under a limit of `segment_cost[j]`, that is also a premise violation. Map it to the same `stage_logical` error. A legitimate segment can never exceed its static cost.
- **`committed_after(plan, j)`** is `committed_before[j] + segment_cost[j]` (checked). It gives the next continuation's `committed` and the Complete `committed`.

Segment index convention: segment `j` starts at entry site `j` (0 = start). Resume at site `k` runs segment `k`.

## Other foundation functions

- **`profile::require_core1(&Program)`** requires `language == LANGUAGE` and `semantics == SEMANTICS`, and no `Statement::Await` anywhere. Otherwise `mori::core::profile`: "Unsupported language or semantics identity." (Await: "Await requires moriarty-core/2.").
- **`profile::require_core2(&Program)`** requires `LANGUAGE_2`/`SEMANTICS_2`, with the same code and message.
- **`digest`:**
  - `program_hash(&Program) -> [u8;32]` is `Sha256(intent::canonical(program))`. That is the backend `digest(program)` formula.
  - `plan_hash(&StagePlan) -> [u8;32]` requires a filled 64-lc-hex `program_hash`.
  - `hex([u8;32]) -> String`.
  - `matches(text, &[u8;32]) -> bool` does an exact 64-lc-hex comparison, for R2.
