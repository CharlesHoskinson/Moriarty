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

## Cumulative A resource observations (ordinary host result observed)

`ReferenceSlice::resource_report()` reports a single present K-A slice.
`reference_slice::cumulative_resource_report(&[&first, &second])` reports ordered
technical chunks from the same observed computation. It checks the existing
complete-state continuity contract, exact profile, each supplied prefix-work
column and checked cumulative record charges. It does not execute or validate
transitions. Original complete slices and boundary annotations stay caller-owned.

The plain std-only `reference_slice::ReferenceResourceReport` names `sherlock-k-a/1` in a versioned
`ResourceManifest` and retains the K source-definition pin separately from missing
rule-inventory/program-image/lowering identity. A's absent B layout/bounds remain
explicitly unavailable. Core-only serialization adapters in the already-registered resource-report module
use exact decimal counter strings; standalone A needs no serde or B module. The report keeps
reference work, recorded reference events and maximum observed logical frame
depth distinct; recorded events are not measured physical transitions.

All original unmeasured groups remain present. Helper work/visits, frame and
environment bytes, preparation/serialization bytes, host time/peak RSS, circuit
rows/degree, proof/key bytes and ledger fees remain explicitly unmeasured.
Grouping boundaries add no semantic charge or financial commitment. One complete
slice and two technical chunks with the same records produce the same cumulative
report. Empty slices are present observations; an absent chunk list is an error.
This ordinary typed observation contract does not authenticate a producer, admit
untrusted input, equate A/B/Quint fuel, measure a cost winner, or establish complete
UIREQ-013/SCON-015, constrained execution, proof/native or target acceptance.

The [actual fixed host result](../../results/sherlock-resource-chunks-20261017/actual/attempt-01/RESULT.json)
passed the complete four-report equality check once after the normal production
metadata/library and external test builds succeeded. Whole/first/second/cumulative
reports each compare all87 scalar leaves against independently frozen expectations.
The fixture has two supplied records (charges1 and0), depth0, and unchanged
input slices. It does not generate a live Machine trace or exercise nonzero-depth,
error, overflow, cross-profile or native cases. Reporting-source and exact-result
acceptance remain distinct from broader interpreter requirements.


## Offline K-capture decoding

`k_capture_observations.decode_capture` keeps its default limit of 64 captures.
An explicit keyword `max_captures=F03_MAX_CAPTURES` selects the ceiling of 95;
other byte, depth, log and argument limits remain unchanged. The [five selected
offline checks](../../results/sherlock-k-f03-offline-decoder-20261018/README.md)
passed against the installed decoder and literal source identities. The F03 test
uses supplied boundary states with placeholder intermediate states and empty
engine artifacts, so `comparison_ready` remains false. These tests establish no
actual F03 engine capture, paired K/Quint agreement or UIREQ-021 acceptance.


## Native06 retained-audit successor — observed refusal

The [exact successor audit](../../results/sherlock-native06-retained-audit-regressions-20261018/fixed-successor-r2-20261018/README.md) exited2 after19.7301seconds: `run-unit lacks dep-info-associated compiled build script`. Five selected SDK-free parser/output methods and one new synthetic successor method passed beforehand; they are distinct from the failed real audit. No Cargo/passed-prefix replay or SDK executable/proof/target run is claimed. The prospective inventory comparison reports3,971 target stat records and3,544 total old regular-file hashes unchanged; outputs remain quarantined/unaccepted and cleanup owed. Host acceptance remains13/35; this result closes no clause and adds no circuit iteration.
