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


## Adopted finite F03 model-projection report — current15

Root's [original021 adoption](../../results/sherlock-k-quint-f03-actual-20261018/ROOT-ORIGINAL021-ADOPTION.json)
records the finite projection-report clause/scenario as satisfied after the final
[APPROVE_WITH_LIMITS review](../../results/sherlock-k-quint-f03-actual-20261018/reviews/actual-result-whole021/REPORT.md).
Current original closures are15/35 after this adoption; earlier dated counts above
retain their own cuts. The actual F03/fuel93/stack64 K/Quint comparison retains
95 complete snapshots and94 edges:93 charges and free publication rule80,
UInt6 and logical frame peak11. All eleven shared fields, full ordered rule/charge
mapping, numeric bounds, hidden steps, representation differences and unmatched
finance/wait behavior are named. Agreement is finite trace agreement, not refinement.

The [complete archive](../../results/sherlock-k-quint-f03-actual-20261018/README.md)
keeps the actual K195children+initializer/488files, real95-state Quint ITF,
three passive passes with517 unchanged pins, full outputs and independent oracle.
The later seven map-key helper/log guards passed with518 unchanged pins; they do
not cover decode_capture end-to-end chain propagation, the six assembler guard
categories or multiline single-item guards. General candidate04 assembler/decoder
installation remains unadopted. No full corpus, theorem, B/finance, circuit/native
proof or target result follows from original021's finite reporting acceptance.


## October 18 — retained native06 R3 provenance and completed cleanup

The corrected retained R3 audit passed and both independent result reviewers accepted its scoped build-provenance result. Root subsequently verified durable preservation of 3,554 files (3,498 target files, 55 packet files and one external marker), totaling 2,628,344,281 bytes, then completed all six cleanup operations. The three tmpfs mounts are gone, cgroups are absent and the final receipt records `cleanupOwed:false`. Archived files remain read-only/nonexecutable; native outputs remain unaccepted and the SDK target ELF was not executed. Proofs, native target semantics, stagenet and full Sherlock completion remain open. No requirement status changes; the current published count remains 15/35.

See the [complete result archive](../../results/sherlock-native06-association-audit-20261018/README.md), [Root preservation disposition](../../results/sherlock-native06-association-audit-20261018/actual/preservation/ROOT-PRESERVATION-DISPOSITION.json) and [actual cleanup result](../../results/sherlock-native06-association-audit-20261018/actual/preservation/POST-CLEANUP-RESULT.json). The outer audit measured 23.021559253 seconds and the worker 12.381874724997033 seconds; neither is target execution time. All file hashes were verified before cleanup; post-cleanup checks cover regularity, size and mode without a redundant payload rehash. Earlier pending/owed receipts remain dated history.


## Canonical TypeId carriers

Three Rust compiles and ten new selected tests passed: five V8 owner cases, four builtin cases through the real preparation/runtime path, and one nominal-storage case. All 484 input pins stayed unchanged. See the [complete scoped result](../../results/sherlock-canonical-runtime-types-20261018/ACTUAL-TEN-TESTS-SCOPE.json) and [Root compatibility adoption](../../results/sherlock-canonical-runtime-types-20261018/ROOT-CANONICAL-TYPEID-COMPATIBILITY-ADOPTION.json).

The Core harness compiled 331 tests; only the five selected V8 cases ran, leaving 326 other Core tests unrun. Eight independent mismatch vectors remain unrun. Absolute independent B clocks remain unbound. Nominal IDs 4/5/17 establish storage, not admission or execution; original UIREQ034 and all 25 refinements remain open. No new original clause, correspondence, proof/native/target or global H1 result is claimed.


## Bounded F03/N01 offline adapter installation — guard evidence

The [eleven complementary offline guards](../../results/sherlock-f03-general-adapter-guards-20261018/actual/eleven-01/RESULT.json) passed once: exit 0, 11 methods, 1.432471144 seconds, with 521 unchanged input pins and an empty isolated temporary directory. They cover the six R328 categories, multiline single-KItem guards, and whole decode_capture chain propagation and tampering. Earlier seven helper guards and three passive operations were not replayed. The [independent result/adoption disposition](../../results/sherlock-f03-general-adapter-guards-20261018/reviews/actual-result-general-adoption-7d/REPORT.md) accepts this result and permits the exact four modules as bounded passive offline F03/N01 utilities.

Canonical QA carries assembler 83a6424f, decoder 85e4e79a, and unchanged observations 540128fd and terms c79a852c. Only the existing decoder test's FIXED_SOURCES identity tuple changed; its historical preimage and the old decoder remain archived. The new eleven-method test uses retained R fixtures and is archived only; portable sourcing is separate source work. The default limit of 64 captures, explicit F03 ceiling of 95, and all other bounds, refusals and trust limits remain unchanged. No engine authenticity, arbitrary K-format support, theorem/refinement, finance/B, circuit/native/proof/target result or new EARS closure follows. Original021's finite projection acceptance, the current 15 original closures and all 25 open refinements remain unchanged.
