# Core2 financial observation adapter

Public Rust path: `mori_core::sherlock_financial_projection`. This module borrows
the existing local Core/2 stage carriers and derives diagnostic accounting for
one published successful segment. `stage::load`, `stage::start` and
`stage::resume` produce the program and results; projection does not execute
them or establish that a supplied program/input produced a supplied result.
See the [implementation](../../../crates/mori-core/src/sherlock/financial_projection.rs)
and [public export](../../../crates/mori-core/src/lib.rs).

## Entry points

All observation lifetimes borrow the caller's existing values.

| Function | Input and result |
| --- | --- |
| `project_execution` | `&Execution` to `ExecutionObservation` |
| `project_segment` | `&StageSegment` to `SegmentObservation` |
| `project_stage_result` | `&Result<StageOutcome, StageRejection>` to `FinancialObservation` |
| `project_resume_result` | Captured continuation, event, optional CLI action/cap and result to `ResumeFinancialObservation` |
| `project_invocation` | `&StagedProgram`, `InvocationInput` and raw result to `InvocationObservation` |
| `account_effects` | `&[Effect]` to `AccountingReport` |
| `financial_observation_matches` | Raw actual result and expected `FinancialObservation` to `bool` |
| `resume_financial_observation_matches` | Actual and expected `ResumeFinancialObservation` to `bool` |

`InvocationInput::Start` retains action, scenario and cap.
`InvocationInput::Resume` retains continuation, event, CLI action and CLI cap.
`InvocationObservation` retains the complete program reference, captured input,
raw result reference, projected outcome, accounting availability and tentative
publication classification. The original program/plan and their hashes remain
available through `StagedProgram`'s getters. The older
`ResumeFinancialObservation` omits the staged program argument; continuation
hash claims cannot substitute for that actual argument.

## Complete observations

`ExecutionObservation` retains semantics, action, full pre/post `Snapshot`s,
ordered `Step`s and ordered `Effect`s. Each step retains index, statement,
optional value, complete snapshot and effects. Snapshots retain every state
entry, balance and liability, including unaffected entries and liabilities
whose amount is present zero. `SegmentObservation` also retains action and both
`PendingFrame`s, including site and captures. These are the
[Core data types](../../../crates/mori-core/src/ir.rs) and
[stage data types](../../../crates/mori-core/src/stage/mod.rs), distinct from
Sherlock A/B machine snapshots.

| `FinancialObservation` | Preserved fields |
| --- | --- |
| `Waiting` | `advanced`, optional segment, full continuation, attempted work and ordered residual duties |
| `Complete` | Full segment, committed and attempted work, ordered residual duties |
| `Rejected` | Actual error, attempted and committed work |

A false await can return `Waiting { advanced: false, segment: None, .. }`.
It is not converted to an empty execution. A returned `Some(segment)` with zero
effects remains present and retains its steps, frames and state. Residual duties
are the stage carrier's positive-amount liabilities; complete snapshots also
retain present-zero rows.

`committed_boundary()` returns the waiting continuation boundary or complete
segment post-state, and returns `None` on rejection. `successful_segment()`
returns only this invocation's published segment. `committed_logical_work()`
returns the reported counter, not an independently validated charge. Early
resume refusal can echo an unvalidated input counter.
`ResumeFinancialObservation::input_boundary()` exposes caller input even after
rejection; that input is not a returned rollback continuation.

## Accounting and publication

`AccountingReport.effects` retains the entire original ordered effect slice.
`accounts` groups transfer and fee entries by exact `(AssetId, account)` in
first encounter order, visiting debit before credit. Rows contain
`transfer_debit`, `transfer_receipts`, `fee_debit`, `fee_receipts`,
`gross_debit`, `gross_receipts` and `net_flow`. State, liability and emit effects
remain in the raw effects; they create no monetary aggregate row. An account
without a monetary effect remains observable in the complete state.

`AmountAggregate::Exact(UInt)` uses checked addition. Overflow becomes
`AboveU128` and remains explicit; values are neither clamped nor wrapped.
`NetFlow` is `Balanced`, `Inflow(UInt)`, `Outflow(UInt)` or `Unavailable` if
either gross aggregate is unrepresentable. Fees already contribute to gross
debit/receipts. Reverse transfers do not erase gross debit. A self-transfer
records both its debit and receipt.

| Returned carrier | `AccountingAvailability` | `TentativePublication` |
| --- | --- | --- |
| Successful published segment | `PublishedSegment(report)` | `PublishedSuccessfulSegment` |
| Waiting with no segment | `NoPublishedSegment` | `NoAdvancedSegment` |
| Rejection | `RejectedNoPublishedDelta` | `UnavailableFromStageRejection` |

`StageRejection` has no failed tentative snapshot, discarded effects or
replacement continuation. The adapter synthesizes none. Accounting describes
one successful segment, not cumulative prefix history, native fees, DUST or
ledger phases. Preserve `stage::STAGE_LABELS` when publishing local stage
observations: authentication is absent and consumption is untracked.

## Observed verification and remaining scope

The [captured financial result](../../results/sherlock-financial-observation-20261018/README.md)
records the R176 external test compile/list and one passing positive test. The
canonical [integration test](../../../crates/mori-core/tests/sherlock_financial_observation_positive.rs)
has the historical name
`actual_core2_positive_financial_observations_match_complete_independent_r152`;
R175 independently revised the input identities. R189 compiled the canonical
test after include-locator changes; it did not execute that binary. The canonical
promotion review approved this exact comment/include-locator change.

The test fully compares eleven standalone expectations: Program, StagePlan,
three raw results, three accounting values and three publication values.
The 179,206-byte `EXPECTED.json` contributes only its program-hash and plan-hash
cells; its other cells are not compared. Borrow checks supplement these
independent values. Equality helpers do not certify oracle independence, and
Core/1 JSON decoding does not bind Core/2 Await-bearing steps by itself.

R188 returned `APPROVE_SCOPED_RESULT` with preserved factual qualifications.
The Core/2 first-paid-prefix scenario is observed; the general UIREQ-017 clause
remains open for the Sherlock financial extension's producer/adapter
relationship. UIREQ-018 universal lowering and UIREQ-019 later local failure
remain open. The positive test does not exercise rejection, aggregate overflow,
self-transfer or every helper. These results establish no financial circuit,
native proof or ledger settlement. Existing H1 and native boundaries remain.

See [observe a stage invocation](../how-to/observe-stage-financial-result.md)
and [financial boundaries](../explanation/chunks-and-financial-boundaries.md).
