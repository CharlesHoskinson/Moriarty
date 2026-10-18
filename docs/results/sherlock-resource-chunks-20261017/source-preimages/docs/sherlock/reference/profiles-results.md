# Profiles, clocks and result types

| Profile or layer | Clock and scope | Result boundary |
| --- | --- | --- |
| `sherlock-k-a/1` | Frozen K source; typed host common domain has valid UInt128 values and nonnegative fuel/stack budgets. Charged transitions cost one; terminal publication costs zero. Pure helpers and allocation are unmetered. | `Outcome`, `Snapshot`, `StepRecord` |
| `sherlock-quint-a/1` | Distinct finite Quint projection; no automatic equality with the K clock | Model results remain model evidence; explicit adapters are required |
| `sherlock-helper-microstep/1` | B owner helper visits, helper work, reference cursor and total consumed work are distinct counters | `OutcomeC`, `RunSummary`, `StepEventC`, `Tooling`, `HostViolation` |
| L4 typed observation | Encoding of trusted A observations under caller capacities; no execution clock | `Result<PreparedA, PrepareFailure>` or `StateResult<T>` |
| A102 / A102→A80 relation | One charged literal rule / literal plus zero-charge return publication | Public statement carriers and native-specific result types; no general dispatch or canonical checkpoint |

The A source pin is `K_SOURCE_SHA256 = b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`; `StepRecord.rule_start_line` resolves only against that source. [Declarations](../../../crates/mori-core/src/sherlock/reference_types.rs#L4).

## A machine results

```rust
pub enum Outcome {
    Pending,
    Done(Value),
    Rejected(u8),
    Exhausted,
}
pub enum VisitStop { Budget, Terminal, Observer }
pub struct VisitResult { pub events: usize, pub stop: VisitStop }
```

`Outcome::is_resource_limit()` recognizes only A's `Exhausted` and frame-cap `Rejected(4)`. It does not transfer to Quint/B. `VisitStop` describes an observation call, not outcome, commitment or suspension. Zero budget reports `Budget` even when terminal. [Outcome](../../../crates/mori-core/src/sherlock/reference_types.rs#L129), [visit results](../../../crates/mori-core/src/sherlock/reference_stream.rs#L6).

## B machine and report results

`shared::OutcomeC` has `Running`, `Halted { value: RefC }`, `Rejected { code: u8 }`, and `Exhausted { reason: ExhaustReason }`. Admission uses `Tooling::{AdmissionRejected, CapacityNotAdmitted, UnsupportedFeature}`. Runtime owner violations use `HostViolation`; they are not semantic `OutcomeC` variants. Exact declarations: [shared.rs:257](../../../crates/mori-core/src/sherlock/helper_machine/shared.rs#L257), [shared.rs:352](../../../crates/mori-core/src/sherlock/helper_machine/shared.rs#L352).

`RunSummary` carries `outcome`, `events`, `consumed_work`, `helper_work`, `helper_visits`, `reference_cursor`. `resource_report::ResourceReport` adds profile/inputs, owner observations, lengths, supplied bounds and unmeasured groups. Format: `moriarty-sherlock-b-resource-report/1`; counters are not physical/economic measurements. [Declaration](../../../crates/mori-core/src/sherlock/helper_machine/resource_report.rs#L14).

## Observation agreement

`compare::first_difference(left: TraceView<'_>, right: TraceView<'_>) -> Option<TraceDifference>` compares profile/source, initial state, records, then lengths. `None` means supplied observations agree, not rule validity, continuity, authenticity or layer presence. Empty records are a present zero-step observation. [Contract](../../../crates/mori-core/src/sherlock/compare.rs#L1).

QA `pass`, supplied `match`, runner `success`, model/native/settlement results remain distinct. [Stream](../../results/sherlock-stream-20261017/): eight tests/216 runs. [Composed](../../results/sherlock-composed333-20261017/): the same 333 carriers match named/ordered forms, not 666 carriers or full typed traces. Context/checked335/circuit/native/target remain unbound or unobserved. See [DoctorWatson](doctorwatson.md), [boundaries](../explanation/observation-boundaries.md).
